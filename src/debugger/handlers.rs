use std::cell::Cell;

use crate::arcmut;

use super::session::State;
use std::sync::Arc;
use dbg_cli_derive::Commands;
use tokio::sync::Mutex;

#[derive(Commands)]
enum CliCommands {

}

// TODO: Rewrite using #[derive]
pub fn cli(command: &Vec<String>, halt: &Cell<bool>, state: arcmut!(State)) {
    match command[0].as_str() {
        "c" | "continue" => {
            println!("continue");
        }
        "b" | "breakpoint" => {
            println!("breakpoint");
        }
        "p" | "print" => {
            println!("breakpoint");
        }
        "q" | "quit" => {
            println!("quit");
            halt.set(true);
        }
        "h" | "help" => {
            todo!()
        }
        _ => println!("Unknown command {}", command[0].as_str()),
    }
}
