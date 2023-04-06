use super::{comm::DevicePortBox, eldp};
use crate::arcmut;
use anyhow::anyhow;
use anyhow::Result;
use crossterm::style::Stylize;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub async fn send_request(
    device_port: arcmut!(DevicePortBox),
    request: eldp::Request,
) -> Result<Vec<u8>> {
    let msg_data = eldp::encode(request)?;
    let mut device_port = device_port.lock().await;

    let mut buf: Vec<u8> = vec![0; 2048];

    let mut retries = 0;
    let mut try_again = |err: _| {
        retries += 1;
        Ok(if retries == 6 {
            return Err(anyhow!("ELDB did not respond ({})", err));
        } else {
            println!(
                "{} {}",
                "Warning:".yellow().bold(),
                format!("ELDB did not respond ({}), retry {}", err, retries).yellow()
            );
        })
    };
    loop {
        device_port.write_all(&msg_data).await?;

        let result = tokio::time::timeout(Duration::from_secs(3), device_port.read(&mut buf)).await;
        match result {
            Ok(result) => {
                match result {
                    Ok(size) => {
                        return Ok(buf[..size - 1].to_vec());
                    },
                    Err(err) => try_again(err)?,
                }
            }
            Err(err) => {
                try_again(err.into())?;
            }
        }
    }
}
