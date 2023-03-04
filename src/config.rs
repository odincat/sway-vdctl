use std::{path::Path, fs};

use anyhow::{Result, bail};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Resolution {
    pub width: u32,
    pub height: u32
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Preset {
    pub name: String,
    pub scale_factor: u8,
    pub rotation: Option<i32>,
    pub position: Option<Position>,
    pub port: u16,
    pub resolution: Resolution
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            scale_factor: 1,
            rotation: Some(0),
            position: Some(Position { x: 0, y: 0 }),
            port: 0,
            resolution: Resolution { width: 1920, height: 1080 }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct Config {
    pub host: Option<String>,
    pub presets: Vec<Preset>,
}

impl Config {
    pub fn get_preset(&self, name: &str) -> Option<Preset> {
        for preset in &self.presets {
            if preset.name.to_lowercase() == name.to_lowercase() {
                return Some(preset.clone())
            }
        }

        None
    }
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

