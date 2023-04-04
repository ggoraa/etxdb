use crate::{
    arcmut,
    debugger::state::State,
    edgetx::{comm::DevicePortBox, eldp},
};
use anyhow::Result;
use inquire::Select;
use std::{cell::Cell, sync::Arc};
use tokio::{io::AsyncWriteExt, sync::Mutex};

pub async fn continue_command(device_port: arcmut!(DevicePortBox)) -> Result<()> {
    let msg = eldp::ExecuteDebuggerCommand {
        command: Some(eldp::Command::Continue.into()),
    };
    let request = eldp::make_request(eldp::request::Content::ExecuteDebuggerCommand(msg));
    let data = eldp::encode(request)?;
    let mut device_port = device_port.lock().await;
    device_port.write_all(&data).await?;
    println!("continue");
    Ok(())
}

pub fn breakpoint_command(
    args: Vec<String>,
    state: arcmut!(State),
    device_port: arcmut!(DevicePortBox),
) -> Result<()> {
    println!("breakpoint");
    Ok(())
}

pub fn print_command(args: Vec<String>, state: arcmut!(State)) -> Result<()> {
    println!("print");
    Ok(())
}

pub fn quit_command(state: arcmut!(State), device_port: arcmut!(DevicePortBox), halt: &Cell<bool>) {
    const YES_CHOICE: &str = "Yes, stop and quit";
    const NO_CHOICE: &str = "No, abort!";
    let answer = Select::new(
        "You sure you want to stop this session and quit?",
        vec![YES_CHOICE, NO_CHOICE],
    )
    .prompt();

    if answer.unwrap() == YES_CHOICE {
        halt.set(true);
    }
}
