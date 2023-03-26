use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Default, Debug)]
pub struct Arguments {
    pub project_src: Option<PathBuf>
}
