use std::{path::PathBuf, cell::Cell};

use tokio::io::AsyncReadExt;
use tokio_serial::{SerialPort, SerialStream};

pub async fn begin(mut serial_port: SerialStream, src: PathBuf) {
    let halt = Cell::new(false);

    // starting handlers for several actions
    tokio::join!(
        cli_input(&halt),
        serial_port_response(&halt, &mut serial_port)
    );
}

async fn cli_input(halt: &Cell<bool>) {
    while !halt.get() {

    }
}

async fn serial_port_response(halt: &Cell<bool>, serial_port: &mut SerialStream) {
    let mut rx_buf = Vec::<u8>::new();

    while !halt.get() {
        _ = serial_port.read(&mut rx_buf).await;
    }
}