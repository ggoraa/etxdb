use crate::{arcmut, edgetx::comm::DevicePortBox};

use self::consts::COMMANDS;
use super::state::SessionState;
use crossterm::style::Stylize;
use eyre::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod commands;
pub mod consts;
pub mod interactive_stdin;

pub async fn execute(
    command: String,
    args: Vec<String>,
    state: arcmut!(SessionState),
    device_port: arcmut!(DevicePortBox),
) {
    #![allow(clippy::unit_arg)]
    let found_command = COMMANDS
        .iter()
        .find(|c| c.name.starts_with(&command.replace('\r', "")));
    if let Some(found_command) = found_command {
        let result = (found_command.handler)(args, state, device_port).await;

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

type CommandHandler = fn(
    Vec<String>,
    Arc<Mutex<SessionState>>,
    Arc<Mutex<DevicePortBox>>,
) -> Pin<Box<(dyn Future<Output = Result<()>> + Send + 'static)>>;

pub struct Command<'a> {
    name: &'a str,
    short_help: &'a str,
    help: &'a str,
    handler: CommandHandler,
}

// uncomment when it becomes needed
// pub struct CommandAlias<'a> {
//     name: &'a str,
//     aliased_to: &'a str,
// }
