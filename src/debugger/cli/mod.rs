use std::cell::Cell;

use crate::arcmut;

use super::state::State;
use crossterm::style::Stylize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod handlers;

const COMMAND_HANDLERS: [Command; 5] = [
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
    Command {
        name: "help",
        shorthand: Some("h"),
        help: "Displays help. Wow.", // TODO: Fill
        handler: handlers::quit_command,
    },
];

pub fn execute(command: String, args: Vec<String>, halt: &Cell<bool>, state: arcmut!(State)) {
    if let Some(handler) = COMMAND_HANDLERS
        .into_iter()
        .find(|handler| handler.name == command || handler.shorthand == Some(&command))
    {
        (handler.handler)(args, state.clone(), halt);
    } else {
        println!(
            "{} {}.",
            "Unknown command".yellow(),
            command.yellow().italic()
        );
    }
}

struct Command<'a> {
    name: &'a str,
    shorthand: Option<&'a str>,
    help: &'a str,
    handler: fn(Vec<String>, arcmut!(State), &Cell<bool>),
}
