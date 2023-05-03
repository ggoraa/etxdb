use crate::{
    arcmut,
    debugger::{cli::consts::COMMANDS, state::SessionState},
    dyn_async,
    edgetx::{self, comm::DevicePortBox, eldp},
};
use eyre::{bail, Result};

use crossterm::style::Stylize;
use inquire::Select;

use macro_rules_attribute::macro_rules_attribute;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::consts::quit_choice;

#[macro_rules_attribute(dyn_async!)]
pub async fn continue_command(
    args: Vec<String>,
    state: arcmut!(SessionState),
    device_port: arcmut!(DevicePortBox),
) -> Result<()> {
    let msg = eldp::RunExecutionFlowCommand {
        command: Some(eldp::run_execution_flow_command::Command::Continue.into()),
    };
    let request = eldp::make_request(eldp::request::Content::RunExecutionFlowCommand(msg));
    let response = edgetx::comm::send_request(device_port, request).await?;
    Ok(())
}

#[macro_rules_attribute(dyn_async!)]
pub async fn breakpoint_command(
    args: Vec<String>,
    state: arcmut!(SessionState),
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

#[macro_rules_attribute(dyn_async!)]
pub async fn print_command(
    args: Vec<String>,
    state: arcmut!(SessionState),
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

#[macro_rules_attribute(dyn_async!)]
pub async fn quit_command(
    args: Vec<String>,
    state: arcmut!(SessionState),
    device_port: arcmut!(DevicePortBox),
) -> Result<()> {
    let answer = Select::new(
        "You sure you want to stop this session and quit?",
        vec![
            quit_choice::STOP_AND_QUIT,
            quit_choice::QUIT,
            quit_choice::ABORT,
        ],
    )
    .prompt();

    match answer.unwrap() {
        quit_choice::STOP_AND_QUIT => {
            let response = edgetx::comm::send_request(
                device_port.clone(),
                eldp::make_request(edgetx::eldp::request::Content::RunExecutionFlowCommand(
                    eldp::RunExecutionFlowCommand {
                        command: Some(eldp::run_execution_flow_command::Command::Stop.into()),
                    },
                )),
            );
            todo!("Implement shutdown");
        }
        quit_choice::QUIT => todo!("Implement shutdown"),
        quit_choice::ABORT => println!("Aborted."),
        _ => panic!("What the shit??"),
    }

    Ok(())
}

pub fn help_command(args: Vec<String>) -> Result<()> {
    if let Some(command) = args.get(0) {
        if let Some(command) = COMMANDS.iter().find(|c| c.name.starts_with(command)) {
            todo!()
        } else {
            bail!("Unknown command {}", command);
        }
    } else {
        let longest_cmd = COMMANDS.iter().fold(&COMMANDS[0], |acc, item| {
            if item.name.len() > acc.name.len() {
                item
            } else {
                acc
            }
        });

        println!("{}", "Debugger commands:".white().bold());
        for command in COMMANDS.iter() {
            println!(
                "- {}  {}",
                format!("{:width$}", command.name, width = longest_cmd.name.len()).bold(),
                command.short_help.italic()
            );
        }
    }
    Ok(())
}
