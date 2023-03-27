use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use prost::Message;

use crate::edgetx;
use crate::edgetx::eldp;

pub mod args;
pub mod session;
pub mod consts;

pub fn start(port: String, project_src: PathBuf) -> Result<()> {
    let mut serial_port = edgetx::serial::cli_port(port).open()?;
    println!("{}", "Starting a debug session...".white().italic());

    let mut msg_container = eldp::MessageContainer::default();
    let mut msg = eldp::StartDebug::default();
    msg.target_type = Some(eldp::DebugTarget::Script.into());
    msg.target_name = Some("file.lua".to_owned());
    msg_container.message = Some(eldp::message_container::Message::StartDebug(msg));

    let mut msg_buf = Vec::<u8>::new();
    msg_buf.reserve(msg_container.encoded_len());
    msg_container.encode(&mut msg_buf)?;

    serial_port.write(&mut msg_buf)?;

    session::begin(serial_port, project_src);

    Ok(())
}
