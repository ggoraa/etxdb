use anyhow::{Context, Result, Error};
use clap::Parser;
use prost::Message;
use std::io::Cursor;
use tokio_serial::{SerialPortInfo, SerialPortType};

pub mod cli;
pub mod edgetx;
pub mod error;

fn main() -> Result<(), Error> {
    let args = cli::Arguments::parse();
    match args.command {
        cli::Commands::ListPorts { show_all } => {
            let ports: Vec<SerialPortInfo> = tokio_serial::available_ports()?
                .into_iter()
                .filter(|port| {
                    if !show_all {
                        match port.port_type {
                            SerialPortType::UsbPort(_) => true,
                            _ => false
                        }
                    } else {
                        true
                    }
                })
                .collect();
            println!("{:?}", ports);
        },
        cli::Commands::Connect { port, project_src } => todo!(),
    }
    Ok(())
}
