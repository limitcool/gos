use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use tracing::info;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub mods: Vec<String>,
}

impl Config {
    pub fn new(config_path: &str) -> Result<Config, Box<dyn Error>> {
        info!(config_path);
        let mut config = Config { mods: vec![] };
        if std::path::Path::new(config_path).exists() {
            let file = fs::File::open(config_path)?;
            config = serde_yaml::from_reader(file)?;
        } else {
            let file = fs::File::create(config_path)?;
            serde_yaml::to_writer(file, &config)?;
            info!("Write config to file successfully!");
        }
        Ok(config)
    }
}
