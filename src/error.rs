use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
enum DebugError {
    #[error("Timeout: debugger is not responding.")]
    TimeOut,
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}