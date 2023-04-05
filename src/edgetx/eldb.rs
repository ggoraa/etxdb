use std::collections::VecDeque;
use std::time::Duration;

use crate::arcmut;

use super::{comm::DevicePortBox, eldp};
use anyhow::anyhow;
use anyhow::Result;
use crossterm::style::Stylize;
use std::sync::Arc;
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
    loop {
        device_port.write_all(&msg_data).await?;

        let result = tokio::time::timeout(Duration::from_secs(3), device_port.read(&mut buf)).await;
        match result {
            Ok(_) => {
                return Ok(buf);
            }
            Err(err) => {
                retries += 1;
                if retries == 6 {
                    return Err(anyhow!("ELDB did not respond ({})", err));
                } else {
                    println!(
                        "{} {}",
                        "Warning:".yellow().bold(),
                        format!("ELDB did not respond ({}), retry {}", err, retries).yellow()
                    );
                }
            }
        }
    }
}
