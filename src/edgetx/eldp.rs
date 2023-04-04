use anyhow::Result;

include!(concat!(env!("OUT_DIR"), "/edgetx.eldp.rs"));

pub fn make_request(content: request::Content) -> Request {
    Request { content: Some(content) }
}

pub fn encode<T: prost::Message>(message: T) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve(message.encoded_len());
    message.encode(&mut buf)?;
    Ok(buf)
}
