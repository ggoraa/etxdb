use crate::{
    arcmut,
    debugger::{
        cli::consts::{COMMANDS, VALID_COMMANDS},
        state::State,
    },
    edgetx::{self, comm::DevicePortBox, eldp},
};
use eyre::{bail, Result};

use crossterm::style::Stylize;
use inquire::Select;

use std::{cell::Cell, sync::Arc};
use tokio::sync::Mutex;

use super::consts::{QUIT_NO_CHOICE, QUIT_STOP_YES_CHOICE, QUIT_YES_CHOICE};

pub async fn continue_command(device_port: arcmut!(DevicePortBox)) -> Result<()> {
    let msg = eldp::ExecuteCommand {
        command: Some(eldp::execute_command::Command::Continue.into()),
    };
    let request = eldp::make_request(eldp::request::Content::ExecuteDebuggerCommand(msg));
    let response = edgetx::comm::send_request(device_port, request).await?;
    Ok(())
}

pub async fn breakpoint_command(
    args: Vec<String>,
    state: arcmut!(State),
    device_port: arcmut!(DevicePortBox),
) -> Result<()> {
    if args.get(0).is_none() || args.get(0).unwrap().is_empty() {
        bail!("No arguments supplied");
    }
    let request = eldp::make_request(eldp::request::Content::SetBreakpoint(eldp::SetBreakpoint {
        breakpoint: Some(eldp::Breakpoint {
            file: None,
            line: args.get(0).map(|val| val.parse::<u32>().unwrap())
        }),
        state: Some(eldp::set_breakpoint::State::Enabled.into()),
    }));
    edgetx::comm::send_request(device_port, request).await;
    Ok(())
}

pub async fn print_command(
    args: Vec<String>,
    state: arcmut!(State),
    device_port: arcmut!(DevicePortBox),
) -> Result<()> {
    if args.get(0).is_none() || args.get(0).unwrap().is_empty() {
        bail!("No expression was passed");
    }
    let request = eldp::make_request(eldp::request::Content::ExecuteExpression(
        eldp::ExecuteExpression {
            expression: Some(args[0].clone()),
        },
    ));

    let response = edgetx::comm::send_request(device_port, request).await?;

    println!("Received message: {:?}", response);

    Ok(())
}

pub async fn quit_command(state: arcmut!(State), device_port: arcmut!(DevicePortBox), halt: &Cell<bool>) {
    let answer = Select::new(
        "You sure you want to stop this session and quit?",
        vec![QUIT_STOP_YES_CHOICE, QUIT_YES_CHOICE, QUIT_NO_CHOICE],
    )
    .prompt();

    match answer.unwrap() {
        QUIT_STOP_YES_CHOICE => {
            let response = edgetx::comm::send_request(
                device_port.clone(),
                eldp::make_request(edgetx::eldp::request::Content::ExecuteDebuggerCommand(
                    eldp::ExecuteCommand {
                        command: Some(eldp::execute_command::Command::Stop.into()),
                    },
                )),
            );
            halt.set(true);
        }
        QUIT_YES_CHOICE => halt.set(true),
        QUIT_NO_CHOICE => {}
        _ => panic!("What the shit??"),
    }
}

pub fn help_command(args: Vec<String>) -> Result<()> {
    if let Some(command) = args.get(0) {
        if let Some(command) = COMMANDS
            .into_iter()
            .find(|c| c.name == command || c.shorthand == Some(command))
        {
            todo!()
        } else {
            bail!("Unknown command {}", command);
        }
    } else {
        let longest_str = VALID_COMMANDS.iter().fold(VALID_COMMANDS[0], |acc, &item| {
            if item.len() > acc.len() {
                item
            } else {
                acc
            }
        });

        println!("{}", "Debugger commands:".white().bold());
        for command in COMMANDS {
            println!(
                "- {}{}  {}",
                if command.shorthand.is_none() {
                    "".to_string()
                } else {
                    format!("({})", command.shorthand.unwrap())
                },
                format!("{:width$}", command.name, width = longest_str.len()).bold(),
                command.help.italic()
            );
        }
    }
    Ok(())
}
