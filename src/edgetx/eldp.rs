use std::fmt::Display;

use eyre::Result;

include!(concat!(env!("OUT_DIR"), "/edgetx.eldp.rs"));

impl Display for error::Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            error::Type::Unknown => write!(f, "Unknown"),
            error::Type::BadMessage => write!(f, "Bad message"),
            error::Type::NotStartedYet => write!(f, "Not started yet"),
            error::Type::AlreadyStarted => write!(f, "Already started"),
            error::Type::FailedStart => write!(f, "Failed start"),
            error::Type::UnknownTarget => write!(f, "Unknown target"),
            error::Type::FileDoesNotExist => write!(f, "File does not exist"),
            error::Type::AlreadyStopped => write!(f, "Already stopped"),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn make_request(content: request::Content) -> Request {
    Request {
        content: Some(content),
    }
}

pub fn encode<T: prost::Message>(message: T) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    buf.reserve(message.encoded_len());
    message.encode(&mut buf)?;
    Ok(buf)
}
