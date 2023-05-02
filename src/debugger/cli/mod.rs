use crate::{arcmut, edgetx::comm::DevicePortBox};

use self::consts::COMMANDS;
use super::state::State;
use crossterm::style::Stylize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod commands;
pub mod consts;
pub mod interactive_stdin;

pub async fn execute(
    command: String,
    args: Vec<String>,
    state: arcmut!(State),
    device_port: arcmut!(DevicePortBox),
) {
    #![allow(clippy::unit_arg)]
    let found_command = COMMANDS
        .iter()
        .find(|c| c.name.starts_with(&command.replace('\r', "")));
    if let Some(found_command) = found_command {
        let result = match found_command.name {
            "help" => commands::help_command(args),
            "continue" => commands::continue_command(device_port).await,
            "breakpoint" => commands::breakpoint_command(args, state, device_port).await,
            "print" => commands::print_command(args, state, device_port).await,
            "quit" => Ok(commands::quit_command(state, device_port).await),
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
                "{} {:?}",
                format!("Error in {}:", command.italic()).red().bold(),
                err
            );
        }
    } else {
        println!("Command {command} was not found. Run help to see all available commands.");
    }
}

pub struct Command<'a> {
    name: &'a str,
    short_help: &'a str,
    help: &'a str,
}

pub struct CommandAlias<'a> {
    name: &'a str,
    aliased_to: &'a str,
}
