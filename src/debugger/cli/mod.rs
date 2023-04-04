use std::cell::Cell;

use crate::arcmut;

use super::state::State;
use crossterm::style::Stylize;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_serial::SerialStream;

pub mod handlers;

const COMMAND_HANDLERS: [Command; 4] = [
    Command {
        name: "continue",
        shorthand: Some("c"),
        help: "continue command help text", // TODO: Fill
        handler: handlers::continue_command,
    },
    Command {
        name: "breakpoint",
        shorthand: Some("b"),
        help: "breakpoint command help text", // TODO: Fill
        handler: handlers::breakpoint_command,
    },
    Command {
        name: "print",
        shorthand: Some("p"),
        help: "print command help text", // TODO: Fill
        handler: handlers::print_command,
    },
    Command {
        name: "quit",
        shorthand: Some("q"),
        help: "Stops current debugging session and exits etxdb.", // TODO: Fill
        handler: handlers::quit_command,
    },
];

fn valid_commands() -> Vec<&'static str> {
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
}

lazy_static! {
    static ref VALID_COMMANDS: Vec<&'static str> = valid_commands();
}

pub fn execute(
    command: String,
    args: Vec<String>,
    halt: &Cell<bool>,
    state: arcmut!(State),
    serial_port: arcmut!(SerialStream),
) {
    match command.as_str() {
        "h" | "help" => {
            show_help(None);
        }
        _ => {
            if let Some(handler) = COMMAND_HANDLERS
                .into_iter()
                .find(|handler| handler.name == command || handler.shorthand == Some(&command))
            {
                (handler.handler)(args, state.clone(), serial_port.clone(), halt);
            } else {
                println!(
                    "{} {}.",
                    "Unknown command".yellow(),
                    command.yellow().italic()
                );
            }
        }
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

type CommandHandler =
    fn(Vec<String>, arcmut!(State), serial_port: arcmut!(SerialStream), &Cell<bool>);

struct Command<'a> {
    name: &'a str,
    shorthand: Option<&'a str>,
    help: &'a str,
    handler: CommandHandler,
}
