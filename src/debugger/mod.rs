use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use lazy_static::lazy_static;
use prost::Message;
use regex::Regex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpSocket;
use tokio_serial::SerialPortBuilderExt;

use crate::edgetx;
use crate::edgetx::comm::DevicePort;
use crate::edgetx::eldp;

pub mod cli;
pub mod consts;
pub mod session;
pub mod state;

#[macro_export]
macro_rules! arcmut {
    ($typename:ident) => {
        Arc<Mutex<&mut $typename>>
    };
}

#[macro_export]
macro_rules! new_arcmut {
    ($value:expr) => {
        Arc::new(Mutex::new(&mut $value))
    };
}

lazy_static! {
    static ref IP_ADDRESS_REGEX: Regex = Regex::new(r"(\d+\.\d+\.\d+\.\d+):(\d+)").unwrap();
}

#[cfg(target_family = "windows")]
fn is_port_serial(port: &str) -> bool {
    port.contains("COM")
}

#[cfg(target_family = "unix")]
fn is_port_serial(port: &str) -> bool {
    port.contains("/dev/tty") || port.contains("/dev/cu")
}

async fn get_device_port(port: String) -> Result<Box<dyn DevicePort>> {
    if is_port_serial(&port) {
        let serial_stream = edgetx::comm::serial_port(port).open_native_async()?;
        Ok(Box::new(serial_stream))
    } else if IP_ADDRESS_REGEX.is_match(port.as_str()) {
        let sock = TcpSocket::new_v4()?;
        let tcp_stream = sock.connect(port.parse().unwrap()).await?;
        Ok(Box::new(tcp_stream))
    } else {
        Err(anyhow!(
            "Supplied port is neither a serial port nor an IP address"
        ))
    }
}

pub async fn start(port: String, project_src: PathBuf) -> Result<()> {
    let mut device_port = get_device_port(port).await?;

    // TODO: Use CLI arguments
    let msg = eldp::StartDebug {
        target_type: Some(eldp::DebugTarget::Script.into()),
        target_name: Some("file.lua".to_owned()),
        ..Default::default()
    };
    let request = eldp::make_request(eldp::request::Content::StartDebug(msg));

    let mut msg_buf = Vec::<u8>::new();
    msg_buf.reserve(request.encoded_len());
    request.encode(&mut msg_buf)?;

    device_port.write_all(&msg_buf).await?;

    session::begin(device_port, project_src).await?;

    Ok(())
}
