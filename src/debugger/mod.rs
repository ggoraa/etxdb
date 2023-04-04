use std::path::PathBuf;

use anyhow::Result;
use prost::Message;
use tokio::io::AsyncWriteExt;
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

pub async fn start(port: String, project_src: PathBuf) -> Result<()> {
    #[cfg(target_family = "windows")]
    let mut device_port = {
        if port.contains("COM") { // serial
            edgetx::comm::serial_port(port).open_native_async()?
        } else { // TCP socket
            todo!("TCP Socket support")
        }
    };

    #[cfg(target_family = "unix")]
    let mut device_port = {
        if port.contains("/dev/tty") || port.contains("/dev/cu") { // serial
            edgetx::comm::serial_port(port).open_native_async()?
        } else { // TCP socket
            todo!("TCP Socket support")
        }
    };

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
