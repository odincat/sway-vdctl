use std::{collections::HashMap, path::Path, fs::{File, OpenOptions, self}, io::Write};

use anyhow::Result;

use crate::config::Preset;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ActiveOutput {
    pub preset: Preset,
    pub vnc_process_pid: Option<u32>
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct State {
    #[serde(skip_serializing, skip_deserializing)]
    pub filepath: String,
    
    pub next_output_number: u8,
    pub active_outputs: HashMap<String, ActiveOutput>,
}

impl State {
    pub fn load(filepath: &str) -> Result<Self> {
        if Path::new(filepath).exists() == false {
            let mut new_file = File::create(filepath).unwrap();

            let default_state = State {
                filepath: filepath.to_owned(),
                next_output_number: 1,
                active_outputs: HashMap::new(),
            };

            let json = serde_json::to_string(&default_state)?;

            new_file.write_all(json.as_bytes())?;

            Ok(default_state)
        } else {
            let state_file = fs::read_to_string(&filepath)?;

            let mut json: State = serde_json::from_str(&state_file)?;

            json.filepath = filepath.to_owned();

            Ok(json)
        }
    }

    pub fn save(&self) -> Result<()> {
        let state_string = serde_json::to_string(&self)?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.filepath)?;

        file.set_len(0)?;

        file.write_all(state_string.as_bytes())?;

        Ok(())
    }
}
