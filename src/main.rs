use anyhow::{Result, Error};
use clap::Parser;

pub mod cli;
pub mod edgetx;
pub mod error;

fn main() -> Result<(), Error> {
    let args = cli::args::Arguments::parse();

    match args.command {
        cli::args::Commands::List { show_all } => cli::subcommands::list(show_all)?,
        cli::args::Commands::Connect { port, project_src } => cli::subcommands::connect(port, project_src)?,
    }
    Ok(())
}
