use std::cell::Cell;

use crate::arcmut;

use colored::Colorize;
use inquire::Select;
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
        "q" | "quit" => {
            const YES_CHOICE: &str = "Yes, stop and quit";
            const NO_CHOICE: &str = "No, abort!";
            let answer = Select::new(
                "You sure you want to stop this session and quit?",
                vec![YES_CHOICE, NO_CHOICE],
            )
            .prompt();

            match answer {
                Ok(choice) => {
                    match choice {
                        YES_CHOICE => halt.set(true),
                        _ => {}
                    }
                }
                Err(_) => println!("{}", "An error occured. Abort.".red().bold()),
            }
        }
        "h" | "help" => todo!("Help does not yet exist"),
        _ => println!("Unknown command {}", command[0].as_str()),
    };
}
