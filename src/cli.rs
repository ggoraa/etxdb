use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {
        #[arg(short, long)]
        show_all: bool
    },
    Connect {
        port: String,
        project_src: Option<PathBuf>
    }
}