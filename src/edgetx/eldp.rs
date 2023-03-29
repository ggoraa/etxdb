use anyhow::Result;

include!(concat!(env!("OUT_DIR"), "/edgetx.lua.debugger.rs"));

pub fn container(for_message: message_container::Message) -> MessageContainer {
    let mut container = MessageContainer::default();
    container.message = Some(for_message);
    container
}

pub fn encode<T: prost::Message>(message: T) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve(message.encoded_len());
    message.encode(&mut buf)?;
    Ok(buf)
}