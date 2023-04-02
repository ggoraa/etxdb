use anyhow::Result;
use std::time::Duration;
use tokio_serial::{SerialPortBuilder, SerialPortInfo, SerialPortType};

pub fn cli_port(port: String) -> SerialPortBuilder {
    return tokio_serial::new(port, 115200).timeout(Duration::from_secs(4));
}

pub fn available_devices(show_all: bool) -> Result<Vec<SerialPortInfo>> {
    Ok(tokio_serial::available_ports()?
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
                !port.port_name.contains("cu") && cfg!(target_os = "macos")
            }
        })
        .collect())
}
