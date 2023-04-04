use crate::{arcmut, debugger::state::State};
use std::{sync::Arc, cell::Cell};
use inquire::Select;
use tokio::sync::Mutex;
use tokio_serial::SerialStream;

pub fn continue_command(_: Vec<String>, state: arcmut!(State), _: arcmut!(SerialStream), _: &Cell<bool>) {
    println!("continue");
}

pub fn breakpoint_command(args: Vec<String>, state: arcmut!(State), _: arcmut!(SerialStream), _: &Cell<bool>) {
    println!("breakpoint");
}

pub fn print_command(args: Vec<String>, state: arcmut!(State), _: arcmut!(SerialStream), _: &Cell<bool>) {
    println!("print");
}

pub fn quit_command(_: Vec<String>, state: arcmut!(State), _: arcmut!(SerialStream), halt: &Cell<bool>) {
    const YES_CHOICE: &str = "Yes, stop and quit";
    const NO_CHOICE: &str = "No, abort!";
    let answer = Select::new(
        "You sure you want to stop this session and quit?",
        vec![YES_CHOICE, NO_CHOICE],
    )
    .prompt();

    if answer.unwrap() == YES_CHOICE {
        halt.set(true);
    }
}