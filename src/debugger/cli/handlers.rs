use crate::{arcmut, debugger::state::State};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn continue_command(state: arcmut!(State)) {
    println!("continue");
}

pub fn breakpoint_command(state: arcmut!(State)) {
    println!("breakpoint");
}

pub fn print_command(state: arcmut!(State)) {
    println!("print");
}
