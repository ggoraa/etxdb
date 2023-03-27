use anyhow::{Error, Result};
use clap::Parser;
use cli::{args, handlers};

pub mod cli;
pub mod debugger;
pub mod edgetx;

fn main() -> Result<(), Error> {
    let args = cli::args::Arguments::parse();

    match args.command {
        args::Commands::List { show_all } => handlers::list(show_all)?,
        args::Commands::Start { port, project_src } => handlers::start(port, project_src)?,
        args::Commands::Init { port } => handlers::init(port)?,
        args::Commands::Stop { port } => handlers::stop(port)?,
    }
    Ok(())
}
