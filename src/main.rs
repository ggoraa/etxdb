use anyhow::{Error, Result};
use clap::Parser;
use cli::handlers;

pub mod cli;
pub mod debugger;
pub mod edgetx;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::List { show_all } => handlers::list(show_all)?,
        cli::Commands::Start { port, project_src } => handlers::start(port, project_src).await?,
        cli::Commands::Init { port } => handlers::init(port)?,
        cli::Commands::Stop { port } => handlers::stop(port)?,
    }
    Ok(())
}
