use std::io::Cursor;

use clap::Parser;
use prost::Message;

mod cli;
pub mod edgetx {
    pub mod eldp {
        include!(concat!(env!("OUT_DIR"), "/edgetx.lua.debugger.rs"));
    }
}

fn main() {
    let args = cli::Arguments::parse();
    let proj_src = args.project_src.unwrap_or(std::env::current_dir().unwrap());

    let mut testmsg_data = edgetx::eldp::SetBreakpoint::default();
    testmsg_data.breakpoint = Some(edgetx::eldp::Breakpoint { file: Some("main.lua".to_owned()), line: Some(1) });
    testmsg_data.state = Some(edgetx::eldp::set_breakpoint::State::Enabled.into());
    let mut data_buf = Vec::new();
    data_buf.reserve(testmsg_data.encoded_len());
    testmsg_data.encode(&mut data_buf).unwrap();

    let mut testmsg = edgetx::eldp::MessageContainer::default();
    testmsg.bytes = data_buf;
    testmsg.message_type = edgetx::eldp::message_container::MessageType::SetBreakpoint.into();

    let mut buf = Vec::new();
    buf.reserve(testmsg.encoded_len());
    testmsg.encode(&mut buf).unwrap();
    println!("{:?}", buf);

    let decoded = edgetx::eldp::MessageContainer::decode(&mut Cursor::new(buf)).unwrap();
    let decoded_data = edgetx::eldp::SetBreakpoint::decode(&mut Cursor::new(&decoded.bytes)).unwrap();
    println!("{:?} {:?}", decoded, decoded_data);
}
