use std::cell::Cell;

use crate::arcmut;

use colored::Colorize;
use inquire::Select;
use std::sync::Arc;
use tokio::sync::Mutex;

use self::handlers::{breakpoint_command, continue_command, print_command};

use super::state::State;

pub mod handlers;

pub fn execute(command: &Vec<String>, halt: &Cell<bool>, state: arcmut!(State)) {
    match command[0].as_str() {
        "c" | "continue" => continue_command(state.clone()),
        "b" | "breakpoint" => breakpoint_command(state.clone()),
        "p" | "print" => print_command(state.clone()),
        "q" | "quit" => {
            const YES_CHOICE: &str = "Yes, stop and quit";
            const NO_CHOICE: &str = "No, abort!";
            let answer = Select::new(
                "You sure you want to stop this session and quit?",
                vec![YES_CHOICE, NO_CHOICE],
            )
            .prompt();

            match answer {
                Ok(choice) => match choice {
                    YES_CHOICE => halt.set(true),
                    _ => {}
                },
                Err(_) => println!("{}", "An error occured. Abort.".red().bold()),
            }
        }
        "h" | "help" => todo!("Help does not yet exist"),
        _ => println!("Unknown command {}", command[0].as_str()),
    };
}
