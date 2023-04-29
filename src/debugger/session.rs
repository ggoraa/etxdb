use std::cell::Cell;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;

use crossterm::style::Stylize;
use tokio::io::{self, AsyncWriteExt};
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::sync::Mutex;
use tokio::time::timeout;

use eyre::Result;

use super::cli;
use super::state::State;
use crate::edgetx::comm::DevicePortBox;
use crate::edgetx::eldp;
use crate::{arcmut, debugger, new_arcmut};

pub async fn begin(device_port: arcmut!(DevicePortBox), mut state: State) -> Result<()> {
    let halt = Cell::new(false);

    // all of this is so that they can be safely accessed
    // between tasks
    let state = new_arcmut!(state);

    tokio::join!(
        cli_task(&halt, state.clone(), device_port.clone()),
        device_port_task(&halt, device_port.clone(), state.clone())
    );

    // on exit (basically halt set to true)
    stop_session(device_port.clone()).await?;
    Ok(())
}

async fn cli_task(halt: &Cell<bool>, state: arcmut!(State), device_port: arcmut!(DevicePortBox)) {
    let mut reader = io::BufReader::new(io::stdin());
    let mut buf = Vec::new();

    while !halt.get() {
        prompt();

        reader.read_until(b'\n', &mut buf).await.unwrap();
        let (command, args) = parse_command(&buf);

        cli::execute(command, args, halt, state.clone(), device_port.clone()).await;

        buf.clear();
    }
}

async fn device_port_task(
    halt: &Cell<bool>,
    device_port: arcmut!(DevicePortBox),
    state: arcmut!(State),
) {
    let mut rx_buf = vec![0; 1024];

    while !halt.get() {
        if let Ok(result) = timeout(
            Duration::from_millis(200),
            device_port.lock().await.read(&mut rx_buf),
        )
        .await
        {
            if result.is_ok() {
                #[cfg(debug_assertions)]
                println!(
                    "Received data from device port: {}",
                    String::from_utf8_lossy(&rx_buf)
                );
            }
            // TODO: Implement serial communication
        }
        if rx_buf.is_empty() {}
    }
}

async fn stop_session(device_port: arcmut!(DevicePortBox)) -> Result<()> {
    let request = eldp::make_request(eldp::request::Content::ExecuteCommand(
        eldp::ExecuteCommand {
            command: Some(eldp::execute_command::Command::Stop.into()),
        },
    ));
    let buf = eldp::encode(request).unwrap(); // will never fail
    device_port.lock().await.write_all(&buf).await?;
    Ok(())
}

#[inline]
fn prompt() {
    print!("{} ", debugger::consts::PROMPT_INPUT_NAME.white().bold());
    std::io::stdout().flush().unwrap();
}

fn parse_command(buf: &[u8]) -> (String, Vec<String>) {
    let command_string = String::from_utf8(buf.to_owned()).unwrap();
    let mut command_vec: Vec<String> = command_string
        .split(' ')
        .map(|x| x.replace('\n', ""))
        .collect();
    let command = command_vec[0].clone();
    command_vec.remove(0);
    (command, command_vec)
}
