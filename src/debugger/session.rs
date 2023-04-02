use std::io::Write;
use std::{cell::Cell, path::PathBuf};

use colored::Colorize;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio_serial::SerialStream;

use anyhow::Result;

use crate::debugger;
use crate::edgetx::eldp;

pub async fn begin(mut serial_port: SerialStream, _src: PathBuf) -> Result<()> {
    let halt = Cell::new(false);

    // starting handlers for several actions
    tokio::join!(cli_task(&halt), serial_port_task(&halt, &mut serial_port));
    stop_session(&mut serial_port)?;
    Ok(())
}

async fn cli_task(halt: &Cell<bool>) {
    let mut reader = io::BufReader::new(io::stdin());
    let mut buf = Vec::new();

    while !halt.get() {
        prompt();

        reader.read_until(b'\n', &mut buf).await.unwrap();
        let command_string = String::from_utf8(buf.clone()).unwrap();
        let command_vec: Vec<String> = command_string
            .split(' ')
            .map(|x| x.replace("\n", ""))
            .collect();

        match command_vec[0].as_str() {
            "c" | "continue" => {
                println!("continue");
            }
            "b" | "breakpoint" => {
                println!("breakpoint");
            }
            "p" | "print" => {
                println!("breakpoint");
            }
            "q" | "quit" => {
                println!("quit");
                halt.set(true);
            }
            _ => todo!(),
        }
        buf.clear();
    }
}

fn stop_session(serial_port: &mut SerialStream) -> Result<()> {
    let request = eldp::make_request(eldp::request::Content::StopDebug(eldp::StopDebug::default()));
    let buf = eldp::encode(request).unwrap(); // will never fail
    serial_port.write(&buf)?;
    Ok(())
}

#[inline]
fn prompt() {
    print!("{} ", debugger::consts::PROMPT_INPUT_NAME.white().bold());
    std::io::stdout().flush().unwrap();
}

async fn serial_port_task(halt: &Cell<bool>, serial_port: &mut SerialStream) {
    let mut rx_buf = Vec::<u8>::new();

    while !halt.get() {
        _ = serial_port.read(&mut rx_buf).await;
        if rx_buf.is_empty() {}
    }
}
