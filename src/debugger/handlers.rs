use std::cell::Cell;

use crate::arcmut;

use std::sync::Arc;
use tokio::sync::Mutex;

use super::state::State;

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
            println!("print");
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
