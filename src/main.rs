#![allow(unused_variables)] // TODO: Remove unused_variables before a release
use clap::Parser;
use cli::handlers;
use crossterm::terminal::disable_raw_mode;
use eyre::Result;

pub mod cli;
pub mod config;
pub mod debugger;
pub mod edgetx;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = cli::Args::parse();

    match args.command {
        cli::Commands::List { show_all } => handlers::list(show_all)?,
        cli::Commands::Start { port } => handlers::start(port).await?,
        cli::Commands::Init { port } => handlers::init(port)?,
        cli::Commands::Stop { port } => handlers::stop(port)?,
        cli::Commands::Attach { port } => todo!(),
    }

    disable_raw_mode()?;
    Ok(())
}
