use anyhow::Result;
use serialport::{SerialPortBuilder, SerialPortInfo, SerialPortType};
use std::time::Duration;

pub fn cli_port(port: String) -> SerialPortBuilder {
    return serialport::new(port, 115200).timeout(Duration::from_secs(2));
}

pub fn available_devices(show_all: bool) -> Result<Vec<SerialPortInfo>> {
    Ok(serialport::available_ports()?
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
        .collect())
}