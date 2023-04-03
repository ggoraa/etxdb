use std::cell::Cell;

use crate::arcmut;

use super::session::State;
use std::sync::Arc;
use tokio::sync::Mutex;

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
        _ => todo!(),
    }
}
