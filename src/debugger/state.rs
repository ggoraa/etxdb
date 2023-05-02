use crate::edgetx::eldp::SystemInfo;
use std::path::PathBuf;

pub struct SessionState {
    pub script: PathBuf,
    pub system_info: SystemInfo,
}
