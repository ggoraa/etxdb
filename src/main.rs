#![allow(unused_variables, incomplete_features)] // TODO: Remove unused_variables before a release
use anyhow::Result;
use clap::Parser;
use cli::handlers;
use crossterm::terminal::disable_raw_mode;

pub mod cli;
pub mod debugger;
pub mod edgetx;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    match args.command {
        cli::Commands::List { show_all } => handlers::list(show_all)?,
        cli::Commands::Start { port, project_src } => handlers::start(port, project_src).await?,
        cli::Commands::Init { port } => handlers::init(port)?,
        cli::Commands::Stop { port } => handlers::stop(port)?,
    }
    disable_raw_mode()?;
    Ok(())
}
