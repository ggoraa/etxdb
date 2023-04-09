use crate::edgetx;
use crate::edgetx::comm::DevicePort;
use crate::edgetx::eldp;
use anyhow::anyhow;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpSocket;
use tokio::sync::Mutex;
use tokio_serial::SerialPortBuilderExt;

use self::state::State;

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
    let device_port = new_arcmut!(device_port);

    // TODO: Use CLI arguments
    let request = eldp::make_request(eldp::request::Content::StartDebug(eldp::StartDebug {
        target_type: Some(eldp::start_debug::Target::Standalone.into()),
        target_name: Some("badapple.lua".to_owned()),
    }));

    let response = edgetx::comm::send_request(device_port.clone(), request).await?;

    match response.content.unwrap() {
        eldp::response::Content::Error(error) => {
            return Err(anyhow!(
                "Failed to start ELDB ({}): {}",
                eldp::error::Type::from_i32(error.r#type.unwrap()).unwrap(),
                error.message.unwrap_or("(no message)".to_string())
            ))
        }
        eldp::response::Content::SystemInfo(info) => {
            let inf = info.clone();
            println!(
                "OS: {} {}-{} {} ({})\nDevice target: {}",
                inf.os_name.unwrap(),
                inf.version.unwrap(),
                inf.version_tag.unwrap(),
                inf.codename.unwrap(),
                inf.git_tag.unwrap(),
                inf.device_identifier.unwrap()
            );

            let state = State {
                proj_root: project_src,
                system_info: info,
            };

            session::begin(device_port.clone(), state).await?;
        }
        _ => {
            return Err(anyhow!("Failed to start ELDB: Expected system info but received else. Are etxdb and EdgeTX versions compatible?"));
        }
    }

    Ok(())
}
