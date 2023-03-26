use std::path::PathBuf;

use colored::{Colorize, ColoredString};
use tokio_serial::{SerialPortInfo, SerialPortType};
use anyhow::Error;

pub fn list(show_all: bool) -> Result<(), Error> {
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
        .filter(|port| {
            if show_all { true } else {
                !port.port_name.contains("tty") && cfg!(target_os = "macos")
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
            let port_type: Option<String>;
            match port.port_type {
                SerialPortType::UsbPort(info) => {
                    port_type = Some(format!("{}({})",
                     "USB".bright_blue().bold(), 
                     if info.product == None { "unknown".yellow() } else { ColoredString::from(info.product.unwrap().as_str()) }))
                },
                SerialPortType::PciPort => port_type = Some(format!("{}", "PCI".green().bold())),
                SerialPortType::BluetoothPort => port_type = Some(format!("{}", "Bluetooth".blue().bold())),
                SerialPortType::Unknown => port_type = Some(format!("{}", "Unknown".yellow().bold())),
            }
            println!("- {}: {}", port_type.unwrap(), port.port_name.white().bold());
        }
    }
    Ok(())
}

pub fn connect(port: String, project_src: Option<PathBuf>) -> Result<(), Error> {
    todo!()
    // let serial_port = tokio_serial::new(port, 115200).open()?;
}