use std::cell::Cell;

use crate::arcmut;

use std::sync::Arc;
use tokio::sync::Mutex;

use super::state::State;

fn continuec(state: arcmut!(State)) {
    println!("continue");
}

fn breakpointc(state: arcmut!(State)) {
    println!("breakpoint");
}

fn printc(state: arcmut!(State)) {
    println!("print");
}

pub fn cli(command: &Vec<String>, halt: &Cell<bool>, state: arcmut!(State)) {
    match command[0].as_str() {
        "c" | "continue" => continuec(state.clone()),
        "b" | "breakpoint" => breakpointc(state.clone()),
        "p" | "print" => printc(state.clone()),
        "q" | "quit" => halt.set(true),
        "h" | "help" => todo!("Help does not yet exist"),
        _ => println!("Unknown command {}", command[0].as_str()),
    };
}
