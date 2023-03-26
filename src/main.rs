use anyhow::{Context, Result, Error};
use clap::Parser;
use colored::{Colorize, ColoredString};
use prost::Message;
use std::io::Cursor;
use tokio_serial::{SerialPortInfo, SerialPortType};

pub mod cli;
pub mod edgetx;
pub mod error;

fn main() -> Result<(), Error> {
    let args = cli::Arguments::parse();
    match args.command {
        cli::Commands::List { show_all } => {
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

            if ports.is_empty() {
                println!("{}", "No devices found.".yellow());
            } else {
                if show_all {
                    println!("{}", "All connected devices:".white().bold());
                } else {
                    println!("{}", "Connected USB devices:".white().bold());
                }
                for port in ports {
                    if show_all {
                        let port_type: Option<ColoredString>;
                        match port.port_type {
                            SerialPortType::UsbPort(_) => port_type = Some("USB".bright_blue().bold()),
                            SerialPortType::PciPort => port_type = Some("PCI".green().bold()),
                            SerialPortType::BluetoothPort => port_type = Some("Bluetooth".blue().bold()),
                            SerialPortType::Unknown => port_type = Some("Unknown".bold()),
                        }
                        println!("{}: {}", port_type.unwrap(), port.port_name);
                    } else {

                    }
                }
            }
        },
        cli::Commands::Connect { port, project_src } => todo!(),
    }
    Ok(())
}
