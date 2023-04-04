use anyhow::Result;
use std::time::Duration;
use tokio_serial::{SerialPortBuilder, SerialPortInfo, SerialPortType};

pub fn serial_port(port: String) -> SerialPortBuilder {
    tokio_serial::new(port, 115200).timeout(Duration::from_secs(4))
}

pub fn available_devices(show_all: bool) -> Result<Vec<SerialPortInfo>> {
    Ok(tokio_serial::available_ports()?
        .into_iter()
        .filter(|port| {
            if !show_all {
                matches!(port.port_type, SerialPortType::UsbPort(_))
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
