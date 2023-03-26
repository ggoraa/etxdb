use std::{path::PathBuf, time::Duration};

use anyhow::{anyhow, Result};
use colored::{ColoredString, Colorize};
use prost::Message;
use serialport::{SerialPortBuilder, SerialPortInfo, SerialPortType};

use crate::edgetx;

pub fn list(show_all: bool) -> Result<()> {
    let ports: Vec<SerialPortInfo> = serialport::available_ports()?
        .into_iter()
        .filter(|port| {
            if !show_all {
                match port.port_type {
                    SerialPortType::UsbPort(_) => true,
                    _ => false,
                }
            } else {
                true
            }
        })
        .filter(|port| {
            if show_all {
                true
            } else {
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
                    port_type = Some(format!(
                        "{}({})",
                        "USB".bright_blue().bold(),
                        if info.product == None {
                            "unknown".yellow()
                        } else {
                            ColoredString::from(info.product.unwrap().as_str())
                        }
                    ))
                }
                SerialPortType::PciPort => port_type = Some(format!("{}", "PCI".green().bold())),
                SerialPortType::BluetoothPort => {
                    port_type = Some(format!("{}", "Bluetooth".blue().bold()))
                }
                SerialPortType::Unknown => {
                    port_type = Some(format!("{}", "Unknown".yellow().bold()))
                }
            }
            println!(
                "- {}: {}",
                port_type.unwrap(),
                port.port_name.white().bold()
            );
        }
    }
    Ok(())
}

pub fn start(port: String, project_src: Option<PathBuf>) -> Result<()> {
    todo!()
}

pub fn init(port: String) -> Result<()> {
    let mut serial_port = serial_port(port).open()?;
    serial_port.write(
        "init_eldp\n\r"
        .as_bytes(),
    )?;
    let success_msg = "ELDP started".to_owned();
    let success_msg_len = success_msg.as_bytes().len();
    let mut buf: [u8; 30] = [0; 30];
    serial_port.read(&mut buf)?;

    let response = String::from_utf8(buf.to_vec())?;

    if response.contains(&success_msg) {
        return Ok(());
    } else {
        return Err(anyhow!(
            "Failed to init debugger, received response \"{}\", expected \"{}\"",
            response,
            success_msg
        ));
    }
}

pub fn stop(port: String) -> Result<()> {
    let msg = edgetx::eldp::StopDebugger::default();
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf)?;
    let mut serial_port = serial_port(port).open()?;
    serial_port.write(&buf)?;
    Ok(())
}

fn serial_port(port: String) -> SerialPortBuilder {
    return serialport::new(port, 115200).timeout(Duration::from_secs(2));
}
