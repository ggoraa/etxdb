use crate::edgetx::eldp::SystemInfo;
use std::path::PathBuf;

pub struct State {
    pub proj_root: PathBuf,
    pub system_info: SystemInfo,
}
