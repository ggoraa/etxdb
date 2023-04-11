#![allow(unused_variables)] // TODO: Remove unused_variables before a release
use clap::Parser;
use cli::commands;
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
        cli::Commands::List { show_all } => commands::list(show_all)?,
        cli::Commands::Start { port } => commands::start(port).await?,
        cli::Commands::Init { port } => commands::init(port)?,
        cli::Commands::Stop { port } => commands::stop(port)?,
        cli::Commands::Attach { port } => todo!(),
    }

    disable_raw_mode()?;
    Ok(())
}
