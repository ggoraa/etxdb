use eyre::{Result, Context};
use serde::{Deserialize, Serialize};

use tokio::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub target: String,
}

pub async fn read_fs() -> Result<Config> {
    let contents = fs::read_to_string("etxdb.yml")
        .await
        .wrap_err("Failed to retrieve config file (etxdb.yml)")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
