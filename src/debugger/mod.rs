use std::path::PathBuf;

use anyhow::Result;
use prost::Message;
use tokio::io::AsyncWriteExt;
use tokio_serial::SerialPortBuilderExt;

use crate::edgetx;
use crate::edgetx::eldp;

pub mod consts;
pub mod handlers;
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
    let mut serial_port = edgetx::serial::cli_port(port).open_native_async()?;

    // TODO: Use CLI arguments
    let mut msg = eldp::StartDebug::default();
    msg.target_type = Some(eldp::DebugTarget::Script.into());
    msg.target_name = Some("file.lua".to_owned());
    let request = eldp::make_request(eldp::request::Content::StartDebug(msg));

    let mut msg_buf = Vec::<u8>::new();
    msg_buf.reserve(request.encoded_len());
    request.encode(&mut msg_buf)?;

    serial_port.write_all(&msg_buf).await?;

    session::begin(serial_port, project_src).await?;

    Ok(())
}
