use super::eldp;
use crate::arcmut;
use crossterm::style::Stylize;
use eyre::bail;
use eyre::Context;
use eyre::Result;
use prost::Message;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;
use tokio_serial::{SerialPortBuilder, SerialPortInfo, SerialPortType};

pub trait DevicePort: AsyncRead + AsyncWrite + Unpin {}
impl<T: AsyncRead + AsyncWrite + Unpin> DevicePort for T {}

pub type DevicePortBox = Box<dyn DevicePort>;

pub fn serial_port(port: String) -> SerialPortBuilder {
    tokio_serial::new(port, 115200).timeout(Duration::from_secs(5))
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

pub async fn send_request(
    device_port: arcmut!(DevicePortBox),
    request: eldp::Request,
) -> Result<eldp::Response> {
    let mut device_port = device_port.lock().await;

    let msg_data = eldp::encode(request).wrap_err("Failed to encode ELDP request")?;
    let mut buf: Vec<u8> = vec![0; 2048];

    let mut retries = 0;
    let mut try_again = |err: _| {
        retries += 1;
        if retries == 6 {
            bail!("ELDB did not respond ({})", err);
        } else {
            println!(
                "{} {}",
                "Warning:".yellow().bold(),
                format!("ELDB did not respond ({}), retry {}", err, retries).yellow()
            );
        }
        Ok(())
    };
    loop {
        device_port
            .write_all(&msg_data)
            .await
            .wrap_err("Failed to send ELDP request")?;

        let result = tokio::time::timeout(Duration::from_secs(3), device_port.read(&mut buf)).await;
        match result {
            Ok(result) => match result {
                Ok(size) => {
                    let buf = &buf[..size - 1];
                    // println!("Received data: {:?}", String::from_utf8_lossy(&buf));
                    let result = eldp::Response::decode(buf)?;
                    return Ok(result);
                }
                Err(err) => try_again(err)?,
            },
            Err(err) => try_again(err.into())?,
        }
    }
}
