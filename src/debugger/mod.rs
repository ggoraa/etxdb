use std::path::PathBuf;

use anyhow::Result;
use prost::Message;
use tokio::io::{AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::net::TcpSocket;
use tokio_serial::SerialPortBuilderExt;

use crate::edgetx;
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

trait DevicePort: AsyncRead + AsyncWrite + Unpin {}
impl<T: AsyncRead + AsyncWrite + Unpin> DevicePort for T {}

#[cfg(target_family = "windows")]
async fn get_device_port(port: String) -> Result<Box<dyn DevicePort>> {
    // TODO: Test this on a real Windows machine
    if port.contains("COM") { // serial
        let serial_stream = edgetx::comm::serial_port(port).open_native_async()?;
        Ok(Box::new(serial_stream))
    } else { // TCP socket
        let sock = TcpSocket::new_v4()?;
        let tcp_stream = sock.connect(port.parse().unwrap()).await?;
        Ok(Box::new(tcp_stream))
    }
}

#[cfg(target_family = "unix")]
async fn get_device_port(port: String) -> Result<Box<dyn DevicePort>> {
    if port.contains("/dev/tty") || port.contains("/dev/cu") { // serial
        let serial_stream = edgetx::comm::serial_port(port).open_native_async()?;
        Ok(Box::new(serial_stream))
    } else { // TCP socket
        let sock = TcpSocket::new_v4()?;
        let tcp_stream = sock.connect(port.parse().unwrap()).await?;
        Ok(Box::new(tcp_stream))
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
