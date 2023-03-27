use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Continue,
    C,
    Breakpoint,
    B,
    Print,
    P,
    Quit,
    Q,
}
