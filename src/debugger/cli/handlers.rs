use crate::{arcmut, debugger::state::State};
use std::{sync::Arc, cell::Cell};
use colored::Colorize;
use inquire::Select;
use tokio::sync::Mutex;

pub fn continue_command(_: Vec<String>, state: arcmut!(State), _: &Cell<bool>) {
    println!("continue");
}

pub fn breakpoint_command(args: Vec<String>, state: arcmut!(State), _: &Cell<bool>) {
    println!("breakpoint");
}

pub fn print_command(args: Vec<String>, state: arcmut!(State), _: &Cell<bool>) {
    println!("print");
}

pub fn quit_command(_: Vec<String>, state: arcmut!(State), halt: &Cell<bool>) {
    const YES_CHOICE: &str = "Yes, stop and quit";
    const NO_CHOICE: &str = "No, abort!";
    let answer = Select::new(
        "You sure you want to stop this session and quit?",
        vec![YES_CHOICE, NO_CHOICE],
    )
    .prompt();

    match answer {
        Ok(choice) => if choice == YES_CHOICE { halt.set(true) },
        Err(_) => println!("{}", "An error occured. Abort.".red().bold()),
    }
}

pub fn help_command(_: Vec<String>, state: arcmut!(State), halt: &Cell<bool>) {
    println!("Help not yet here");
}
