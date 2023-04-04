use std::cell::Cell;

use crate::arcmut;

use super::state::State;
use crossterm::style::Stylize;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::Mutex,
};

pub mod handlers;

const COMMAND_HANDLERS: [Command; 4] = [
    Command {
        name: "continue",
        shorthand: Some("c"),
        help: "continue command help text", // TODO: Fill
    },
    Command {
        name: "breakpoint",
        shorthand: Some("b"),
        help: "breakpoint command help text", // TODO: Fill
    },
    Command {
        name: "print",
        shorthand: Some("p"),
        help: "print command help text", // TODO: Fill
    },
    Command {
        name: "quit",
        shorthand: Some("q"),
        help: "Stops current debugging session and exits etxdb.", // TODO: Fill
    },
];

lazy_static! {
    static ref VALID_COMMANDS: Vec<&'static str> = {
        COMMAND_HANDLERS
            .into_iter()
            .flat_map(|command| {
                let mut vec = vec![command.name];
                if command.shorthand.is_some() {
                    vec.push(command.shorthand.unwrap());
                }
                vec
            })
            .collect()
    };
}

pub async fn execute<T: AsyncRead + AsyncWrite + Unpin>(
    command: String,
    args: Vec<String>,
    halt: &Cell<bool>,
    state: arcmut!(State),
    device_port: arcmut!(T),
) {
    #![allow(clippy::unit_arg)]
    let result = match command.as_str() {
        "h" | "help" => Ok(show_help(None)),
        "c" | "continue" => handlers::continue_command(device_port).await,
        "b" | "breakpoint" => handlers::breakpoint_command(args, state, device_port),
        "p" | "print" => handlers::print_command(args, state),
        "q" | "quit" => Ok(handlers::quit_command(state, device_port, halt)),
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
        for command in COMMAND_HANDLERS {
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

struct Command<'a> {
    name: &'a str,
    shorthand: Option<&'a str>,
    help: &'a str,
}
