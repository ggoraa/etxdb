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

use std::{sync::Arc};
use tokio::sync::Mutex;

use super::consts::{QUIT_NO_CHOICE, QUIT_STOP_YES_CHOICE, QUIT_YES_CHOICE};

pub async fn continue_command(device_port: arcmut!(DevicePortBox)) -> Result<()> {
    let msg = eldp::ExecuteCommand {
        command: Some(eldp::execute_command::Command::Continue.into()),
    };
    let request = eldp::make_request(eldp::request::Content::ExecuteCommand(msg));
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
            line: args.get(0).map(|val| val.parse::<u32>().unwrap()),
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

pub async fn quit_command(state: arcmut!(State), device_port: arcmut!(DevicePortBox)) {
    let answer = Select::new(
        "You sure you want to stop this session and quit?",
        vec![QUIT_STOP_YES_CHOICE, QUIT_YES_CHOICE, QUIT_NO_CHOICE],
    )
    .prompt();

    match answer.unwrap() {
        QUIT_STOP_YES_CHOICE => {
            let response = edgetx::comm::send_request(
                device_port.clone(),
                eldp::make_request(edgetx::eldp::request::Content::ExecuteCommand(
                    eldp::ExecuteCommand {
                        command: Some(eldp::execute_command::Command::Stop.into()),
                    },
                )),
            );
            todo!("Implement shutdown");
        }
        QUIT_YES_CHOICE => todo!("Implement shutdown"),
        QUIT_NO_CHOICE => {}
        _ => panic!("What the shit??"),
    }
}

pub fn help_command(args: Vec<String>) -> Result<()> {
    if let Some(command) = args.get(0) {
        if let Some(command) = COMMANDS
            .iter()
            .into_iter()
            .find(|c| c.name == command || c.aliases.contains(&command.as_str()))
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
        for command in COMMANDS.iter() {
            println!(
                "- {}{}  {}",
                if command.aliases.is_empty() {
                    "".to_string()
                } else {
                    format!("({:?})", command.aliases)
                },
                format!("{:width$}", command.name, width = longest_str.len()).bold(),
                command.help.italic()
            );
        }
    }
    Ok(())
}
