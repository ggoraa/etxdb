use std::cell::Cell;

use crate::{arcmut, edgetx::comm::DevicePortBox};

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
        "h" | "help" => commands::help_command(args),
        "c" | "continue" => commands::continue_command(device_port).await,
        "b" | "breakpoint" => commands::breakpoint_command(args, state, device_port).await,
        "p" | "print" => commands::print_command(args, state, device_port).await,
        "q" | "quit" => Ok(commands::quit_command(state, device_port, halt).await),
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
}

pub struct Command<'a> {
    name: &'a str,
    shorthand: Option<&'a str>,
    help: &'a str,
}
