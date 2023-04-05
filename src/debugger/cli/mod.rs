use std::cell::Cell;

use crate::{
    arcmut,
    debugger::cli::consts::{COMMANDS, VALID_COMMANDS},
    edgetx::comm::DevicePortBox,
};

use super::state::State;
use crossterm::style::Stylize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod commands;
pub mod consts;

pub async fn execute(
    command: String,
    args: Vec<String>,
    halt: &Cell<bool>,
    state: arcmut!(State),
    device_port: arcmut!(DevicePortBox),
) {
    #![allow(clippy::unit_arg)]
    let command_str = command.replace('\r', "");
    let command_str = command_str.as_str();
    let result = match command_str {
        "h" | "help" => Ok(show_help(None)),
        "c" | "continue" => commands::continue_command(device_port).await,
        "b" | "breakpoint" => commands::breakpoint_command(args, state, device_port),
        "p" | "print" => commands::print_command(args, state, device_port).await,
        "q" | "quit" => Ok(commands::quit_command(state, device_port, halt)),
        _ => {
            println!(
                "{} {}.",
                "Unknown command".yellow(),
                command.clone().yellow().italic()
            );
            Ok(())
        }
    };

    if let Err(err) = result {
        println!(
            "{} {}",
            format!("Error in {}:", command.italic()).red().bold(),
            err.to_string().red().italic()
        );
    }
}

fn show_help(command: Option<Command>) {
    if command.is_none() {
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
    } else {
        todo!()
    }
}

pub struct Command<'a> {
    name: &'a str,
    shorthand: Option<&'a str>,
    help: &'a str,
}
