use std::path::PathBuf;

use anyhow::{anyhow, Result};
use crossterm::style::Stylize;
use prost::Message;
use tokio_serial::SerialPortType;

use crate::edgetx::eldp;
use crate::{debugger, edgetx};

pub fn list(show_all: bool) -> Result<()> {
    let ports = edgetx::comm::available_devices(show_all)?;

    if ports.is_empty() {
        println!("{}", "No devices found.".yellow());
    } else {
        if show_all {
            println!("{}", "All connected devices:".white().bold());
        } else {
            println!("{}", "Connected USB devices:".white().bold());
        }
        for port in ports {
            let port_type = match port.port_type {
                SerialPortType::UsbPort(info) => Some(format!(
                    "{}({})",
                    "USB".blue().bold(),
                    match info.product {
                        Some(val) => val.reset(),
                        None => String::from("unknown").yellow(),
                    }
                )),
                SerialPortType::PciPort => Some(format!("{}", "PCI".green().bold())),
                SerialPortType::BluetoothPort => Some(format!("{}", "Bluetooth".blue().bold())),
                SerialPortType::Unknown => Some(format!("{}", "Unknown".yellow().bold())),
            };
            println!(
                "- {}: {}",
                port_type.unwrap(),
                port.port_name.white().bold()
            );
        }
    }
    Ok(())
}

pub async fn start(port: String, project_src: Option<PathBuf>) -> Result<()> {
    debugger::start(port, project_src.unwrap_or(std::env::current_dir()?)).await?;
    Ok(())
}

pub fn init(port: String) -> Result<()> {
    let mut device_port = edgetx::comm::serial_port(port).open()?;
    device_port.write_all(edgetx::consts::ELDP_INIT_CLI_COMMAND.as_bytes())?;
    let success_msg = edgetx::consts::ELDP_INIT_SUCCESS_RESPONSE.to_owned();
    let mut buf: [u8; 30] = [0; 30];
    if let Err(err) = device_port.read(&mut buf) {
        return Err(anyhow!(
            "Failed to init debug connection ({}), maybe it's already initialised?",
            err
        ));
    }

    let response = String::from_utf8(buf.to_vec())?;

    if response.contains(&success_msg) {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to init debug connection, received response \"{}\", expected \"{}\"",
            response,
            success_msg
        ))
    }
}

pub fn stop(port: String) -> Result<()> {
    let msg = eldp::SwitchSerialMode {
        mode: Some(eldp::switch_serial_mode::Mode::Cli.into()),
    };
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve(msg.encoded_len());
    eldp::make_request(eldp::request::Content::SwitchSerialMode(msg)).encode(&mut buf)?;
    let mut device_port = edgetx::comm::serial_port(port).open()?;
    device_port.write_all(&buf)?;
    Ok(())
}
