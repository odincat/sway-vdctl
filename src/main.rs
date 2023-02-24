use std::{path::Path, fs::{self, File, OpenOptions}, io::Write, collections::HashMap, env, process::{Command, self}, os};
use anyhow::{Result, bail, Context};
use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
enum Action {
    Create,
    Kill,
    Restart
}

#[derive(Parser, Debug)]
struct Args {
    preset: String,
    #[arg(value_enum)]
    action: Action,
    #[arg(long, default_value_t = false)]
    novnc: bool
}

#[derive(serde::Deserialize, serde::Serialize)]
struct State {
    output_number: u8,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Resolution {
    width: u32,
    height: u32
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Preset {
    name: String,
    resolution: Resolution,
    scale_factor: u8
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct PresetConfig {
    preset: Vec<Preset>
}

const STATE_FILEPATH: &str = "/tmp/vdctl-state";

fn main() -> Result<()> {
    let args = Args::parse();

    let mut state = load_state().unwrap();
    // save_state(&state).unwrap();

    let mut presets: HashMap<String, Preset> = HashMap::new();
    let mut preset_names: Vec<String> = vec![];
    for preset in load_presets().unwrap().preset {
        presets.insert(preset.name.clone(), preset.clone());
        preset_names.push(preset.name.clone());
    }

    match args.action {
        Action::Create => {
            let preset = match presets.get(args.preset.as_str()) {
                Some(preset) => preset,
                None => {
                    println!("Preset '{}' not found. Available presets are {}", args.preset, preset_names.join(", "));
                    process::exit(1);
                }
            };

            Command::new("swaymsg").arg("create_output").spawn().expect("Error creating output");

            let output_name = format!("HEADLESS-{}", state.output_number);
            println!("{}", output_name);

            Command::new("swaymsg").arg("output").arg(output_name).arg("resolution").arg(format!("{}x{}", preset.resolution.width, preset.resolution.height)).spawn().expect("Error setting resolution");

            state.output_number += 1;
            save_state(&state)?;
        },
        _ => {}
    }

    Ok(())
}

fn load_state() -> Result<State> {
    if Path::new(STATE_FILEPATH).exists() == false {
        let mut new_file = File::create(STATE_FILEPATH).unwrap();

        let default_state = State {
            output_number: 1
        };

        let toml = toml::to_string(&default_state)?;

        new_file.write_all(toml.as_bytes())?;

        Ok(default_state)
    } else {
        let state_file = fs::read_to_string(STATE_FILEPATH)?;

        let toml: State = toml::from_str(&state_file)?;

        Ok(toml)
    }
}

fn save_state(state: &State) -> Result<()> {
    let state_string = toml::to_string(&state)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(STATE_FILEPATH)?;

    file.set_len(0)?;

    file.write_all(state_string.as_bytes())?;

    Ok(())
}

fn load_presets() -> Result<PresetConfig> {
    let preset_filepath = "/home/odincat/.config/vdctl/presets.toml";

    if Path::new(preset_filepath).exists() == false {
        bail!("No preset config at '{}'", preset_filepath);
    } else {
        let preset_file = fs::read_to_string(preset_filepath)?;

        let toml: PresetConfig = toml::from_str(&preset_file)?;

        Ok(toml)
    }
}

fn create_output() {

}
