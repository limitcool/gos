#[allow(unused_imports)]
use crate::constants::vscode_launch::CONFIG;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::str::FromStr;
use std::{fs, io};
use tracing::info;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub mods: Vec<String>,
    pub create_vscode_launch: bool,
    pub create_license: bool,
    pub create_gitignore: bool,
    pub create_readme: bool,
}

impl Config {
    pub fn config_file() -> PathBuf {
        let proj_dirs =
            ProjectDirs::from("com", "initcool", "gos").expect("Failed to get project directories");
        let mut config_file = PathBuf::from(proj_dirs.config_dir());
        std::fs::create_dir_all(&config_file).expect("");
        config_file.push("config.yaml");
        return config_file;
    }
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let mut config = Config {
            mods: vec![],
            create_vscode_launch: false,
            create_license: false,
            create_gitignore: false,
            create_readme: false,
        };
        match std::fs::File::open(Config::config_file()) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();

                config = serde_yaml::from_str(&content)?;
            }
            Err(_e) => {
                let file = fs::File::create(Config::config_file())?;
                serde_yaml::to_writer(file, &config)?;
                info!("Write config to file successfully!");
            }
        }
        Ok(config)
    }

    fn create_file(&self, project_path: &Path, file_name: &str, content: &[u8]) -> io::Result<()> {
        let file_path = project_path.join(file_name);
        let mut file = File::create(&file_path)?;
        file.write_all(content)?;
        Ok(())
    }
}
