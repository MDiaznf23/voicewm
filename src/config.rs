use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub audio: AudioConfig,
    pub commands: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub model_path: String,
    pub language: String,
    pub confidence_threshold: f32,
    pub threads: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default_path() -> String {
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("voicewm")
            .join("config.toml")
            .to_string_lossy()
            .to_string()
    }
}
