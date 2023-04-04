use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use std::{cell::Cell, path::PathBuf};

use crossterm::style::Stylize;
use tokio::io::{self, AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::sync::Mutex;
use tokio::time::timeout;

use anyhow::Result;

use super::cli;
use super::state::State;
use crate::edgetx::eldp;
use crate::{arcmut, debugger, new_arcmut};

pub async fn begin<T: AsyncRead + AsyncWrite + Unpin>(
    mut device_port: T,
    src: PathBuf,
) -> Result<()> {
    let halt = Cell::new(false);

    let mut state = State { proj_root: src };

    // all of this is so that they can be safely accessed
    // between tasks
    let state = new_arcmut!(state);
    let device_port = new_arcmut!(device_port);

    tokio::join!(
        cli_task(&halt, state.clone(), device_port.clone()),
        device_port_task(&halt, device_port.clone(), state.clone())
    );

    // on exit (basically halt set to true)
    stop_session(device_port.clone()).await?;
    Ok(())
}

async fn cli_task<T: AsyncRead + AsyncWrite + Unpin>(
    halt: &Cell<bool>,
    state: arcmut!(State),
    device_port: arcmut!(T),
) {
    let mut reader = io::BufReader::new(io::stdin());
    let mut buf = Vec::new();

    while !halt.get() {
        prompt();

        reader.read_until(b'\n', &mut buf).await.unwrap();
        let command_string = String::from_utf8(buf.clone()).unwrap();
        let mut command_vec: Vec<String> = command_string
            .split(' ')
            .map(|x| x.replace('\n', ""))
            .collect();
        let command = command_vec[0].clone();
        command_vec.remove(0);

        cli::execute(
            command,
            command_vec,
            halt,
            state.clone(),
            device_port.clone(),
        )
        .await;

        buf.clear();
    }
}

async fn device_port_task<T: AsyncRead + AsyncWrite + Unpin>(
    halt: &Cell<bool>,
    device_port: arcmut!(T),
    state: arcmut!(State),
) {
    let mut rx_buf = Vec::<u8>::new();
    rx_buf.resize(1, 0);

    while !halt.get() {
        if timeout(
            Duration::from_millis(200),
            device_port.lock().await.read(&mut rx_buf),
        )
        .await
        .is_ok()
        {
            // TODO: Implement serial communication
        }
        if rx_buf.is_empty() {}
    }
}

async fn stop_session<T: AsyncRead + AsyncWrite + Unpin>(device_port: arcmut!(T)) -> Result<()> {
    let request = eldp::make_request(eldp::request::Content::StopDebug(eldp::StopDebug::default()));
    let buf = eldp::encode(request).unwrap(); // will never fail
    device_port.lock().await.write_all(&buf).await?;
    Ok(())
}

#[inline]
fn prompt() {
    print!("{} ", debugger::consts::PROMPT_INPUT_NAME.white().bold());
    std::io::stdout().flush().unwrap();
}
