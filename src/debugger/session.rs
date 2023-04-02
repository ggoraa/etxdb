use std::io::Write;
use std::{path::PathBuf, cell::Cell};

use tokio::io::{AsyncReadExt, AsyncBufReadExt};
use tokio::io;
use tokio_serial::{SerialStream};

use crate::debugger;

pub async fn begin(mut serial_port: SerialStream, _src: PathBuf) {
    let halt = Cell::new(false);

    // starting handlers for several actions
    tokio::join!(
        cli_task(&halt),
        serial_port_task(&halt, &mut serial_port)
    );
}

async fn cli_task(halt: &Cell<bool>) {
    let mut reader = io::BufReader::new(io::stdin());
    let mut buf = Vec::new();

    prompt();

    while !halt.get() {
        match reader.read_until(b'\n', &mut buf).await {
            Ok(_) => {
                println!("received data: {:?}", String::from_utf8(buf.clone()));
            },
            Err(err) => panic!("{}", err),
        }
        buf.clear();
        prompt();
    }
}

#[inline]
fn prompt() {
    print!("{} ", debugger::consts::PROMPT_INPUT_NAME);
    std::io::stdout().flush().unwrap();
}

async fn serial_port_task(halt: &Cell<bool>, serial_port: &mut SerialStream) {
    let mut rx_buf = Vec::<u8>::new();

    while !halt.get() {
        _ = serial_port.read(&mut rx_buf).await;
        if rx_buf.is_empty() {
            
        }
    }
}