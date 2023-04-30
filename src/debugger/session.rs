use std::io::Write;
use std::sync::Arc;
use std::time::Duration;

use crossterm::style::Stylize;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use tokio::time::timeout;

use eyre::Result;

use super::cli;
use super::state::State;
use crate::debugger::cli::interactive_stdin::InteractiveStdin;
use crate::edgetx::comm::DevicePortBox;
use crate::{arcmut, debugger, new_arcmut};

pub async fn begin(device_port: arcmut!(DevicePortBox), mut state: State) -> Result<()> {
    // all of this is so that they can be safely accessed
    // between tasks
    let state = new_arcmut!(state);

    let stdin = InteractiveStdin::new();

    tokio::try_join!(
        device_port_task(device_port.clone(), state.clone()),
        cli_task(state.clone(), device_port.clone(), stdin)
    )?;

    Ok(())
}

async fn cli_task(
    state: arcmut!(State),
    device_port: arcmut!(DevicePortBox),
    mut stdin: InteractiveStdin,
) -> Result<()> {
    prompt();

    loop {
        let line = stdin.next_line().await?;
        if line.is_some() {
            let (command, args) = parse_command(&line.unwrap());
            cli::execute(command, args, state.clone(), device_port.clone()).await;
            prompt();
        }
    }
}

async fn device_port_task(
    device_port: arcmut!(DevicePortBox),
    state: arcmut!(State),
) -> Result<()> {
    let mut rx_buf = vec![0; 1024];

    loop {
        let mut device_port = device_port.lock().await;
        if let Ok(result) = timeout(Duration::from_millis(200), device_port.read(&mut rx_buf)).await
        {
            println!("{:?}", result);
            let result = result?;
            #[cfg(debug_assertions)]
            println!(
                "Received data from device port: {}",
                String::from_utf8_lossy(&rx_buf)
            );
            // TODO: Implement serial communication
        }
        if rx_buf.is_empty() {}
    }
}

#[inline]
fn prompt() {
    print!("{} ", debugger::consts::PROMPT_INPUT_NAME.white().bold());
    std::io::stdout().flush().unwrap();
}

fn parse_command(string: &str) -> (String, Vec<String>) {
    let mut command_vec: Vec<String> = string.split(' ').map(|x| x.replace('\n', "")).collect();
    let command = command_vec[0].clone();
    command_vec.remove(0);
    (command, command_vec)
}
