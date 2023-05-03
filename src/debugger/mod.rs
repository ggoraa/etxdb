use self::state::SessionState;
use crate::config;
use crate::edgetx;
use crate::edgetx::comm::DevicePortBox;
use crate::edgetx::eldp;
use crate::new_arcmut;
use eyre::bail;
use eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpSocket;
use tokio::sync::Mutex;
use tokio_serial::SerialPortBuilderExt;

pub mod cli;
pub mod consts;
pub mod session;
pub mod state;

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

async fn get_device_port(port: String) -> Result<DevicePortBox> {
    if is_port_serial(&port) {
        let serial_stream = edgetx::comm::serial_port(port).open_native_async()?;
        Ok(Box::new(serial_stream))
    } else if IP_ADDRESS_REGEX.is_match(port.as_str()) {
        let sock = TcpSocket::new_v4()?;
        let tcp_stream = sock.connect(port.parse().unwrap()).await?;
        Ok(Box::new(tcp_stream))
    } else {
        bail!("Supplied port is neither a serial port nor an IP address")
    }
}

pub async fn start(port: String) -> Result<()> {
    let config = config::read_fs().await?;

    let device_port = get_device_port(port).await?;
    let device_port = new_arcmut!(device_port);

    // TODO: Use config values
    // TODO: Support different kinds of scripts
    let request = eldp::make_request(eldp::request::Content::StartDebug(eldp::StartDebug {
        target_type: Some(eldp::start_debug::Target::Standalone.into()),
        target_name: Some(config.target.clone()),
    }));

    let response = edgetx::comm::send_request(device_port.clone(), request).await?;

    match response.content.unwrap() {
        eldp::response::Content::Error(error) => {
            bail!(
                "Failed to start ELDB ({}): {}",
                eldp::error::Type::from_i32(error.r#type.unwrap()).unwrap(),
                error.message.unwrap_or("(no message)".to_string())
            );
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

            let state = SessionState {
                script: PathBuf::from(config.target),
                system_info: info,
            };

            session::begin(device_port.clone(), state).await?;
        }
        _ => {
            bail!("Failed to start ELDB: Expected system info but received else. Are etxdb and EdgeTX FW versions compatible?");
        }
    }

    Ok(())
}
