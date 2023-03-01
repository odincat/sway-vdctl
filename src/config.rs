use std::{path::Path, fs, collections::HashMap};

use anyhow::{Result, bail};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Resolution {
    pub width: u32,
    pub height: u32
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Preset {
    pub name: String,
    pub resolution: Resolution,
    pub scale_factor: u8,
    pub port: u32
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Config {
    pub host: Option<String>,
    pub presets: HashMap<String, Preset>,
}

pub fn load_config(filepath: &str) -> Result<Config> {
    if Path::new(filepath).exists() == false {
        bail!("No file found at '{}'", filepath);
    } else {
        let preset_file = fs::read_to_string(filepath)?;

        let json: Config = serde_json::from_str(&preset_file)?;

        Ok(json)
    }
}

