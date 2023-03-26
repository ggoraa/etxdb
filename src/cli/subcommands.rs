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
            if cfg!(target_os = "macos") { return true }
            if show_all { true } else { !port.port_name.contains("tty") }
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
            let port_type: Option<ColoredString>;
            match port.port_type {
                SerialPortType::UsbPort(_) => port_type = Some("USB".bright_blue().bold()),
                SerialPortType::PciPort => port_type = Some("PCI".green().bold()),
                SerialPortType::BluetoothPort => port_type = Some("Bluetooth".blue().bold()),
                SerialPortType::Unknown => port_type = Some("Unknown".bold()),
            }
            println!("{}: {}", port_type.unwrap(), port.port_name);
        }
    }
    Ok(())
}

pub fn connect(port: String, project_src: Option<PathBuf>) -> Result<(), Error> {
    todo!()
}