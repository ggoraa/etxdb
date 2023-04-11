pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Lists connected USB devices.
    List {
        /// Shows all devices instead of only USB.
        #[arg(short = 'a', long = "all")]
        show_all: bool,
    },
    /// Initiates ELDP connectivity with the debugger.
    Init { port: String },
    /// Stops ELDP connectivity on the radio.
    Stop { port: String },
    /// Starts a new debug session.
    Start { port: String },
    /// Connect to an already running debug session.
    Attach { port: String },
}
