use std::{path::Path, fs::{self, File, OpenOptions}, io::Write, collections::HashMap, process::{Command, self}};
use anyhow::{Result, bail};
use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
enum Action {
    Create,
    Kill,
    Restart,
    NextOutputNumber
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_enum)]
    action: Action,
    #[arg(help = "Preset name to apply, alternatively a value")]
    preset: String,
    #[arg(long, default_value_t = false, help = "skip launching vnc server")]
    novnc: bool
}

const STATE_FILEPATH: &str = "/tmp/vdctl-state";

fn main() -> Result<()> {
    let args = Args::parse();

    let mut state = State::load(STATE_FILEPATH).unwrap();

    let saved_presets = match load_presets() {
        Ok(sp) => sp,
        Err(err) => {
            match args.action {
                Action::NextOutputNumber => {
                    println!("WARN: Unable to load presets: {:?}", err);
                    println!("Still continuing, as the operation you are performing doesn't require them");
                },
                _ => {
                    println!("Unable to load presets: {:?}", err);
                    process::exit(1);
                }
            }

            PresetConfig {
                preset: vec![]
            }
        }
    };

    let mut presets: HashMap<String, Preset> = HashMap::new();
    let mut preset_names: Vec<String> = vec![];
    for preset in saved_presets.preset {
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

            let output_name = format!("HEADLESS-{}", &state.next_output_number);
            Command::new("swaymsg").arg("create_output").spawn().expect("Error creating output");
            println!("Created output, presumably '{}'", &output_name);

            let resolution = format!("{}x{}", &preset.resolution.width, &preset.resolution.height);
            Command::new("swaymsg").arg("output").arg(&output_name).arg("resolution").arg(&resolution).spawn().expect("Error setting resolution");
            println!("Set resolution of '{}' to {}", &output_name, &resolution);
            Command::new("swaymsg").arg("output").arg(&output_name).arg("scale").arg(&preset.scale_factor.to_string()).spawn().expect("Error setting scale");
            println!("Set scale factor of '{}' to {}", &output_name, &preset.scale_factor);

            state.next_output_number += 1;

            let next_preset_number = match state.next_preset_number.get(preset.name.as_str()) {
                Some(num) => num.clone(),
                None => 1
            };

            state.next_preset_number.insert(preset.name.to_owned(), next_preset_number);

            state.save()?;
        },
        Action::NextOutputNumber => {
            let preset_as_number: u8 = match args.preset.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please provide a number");
                    process::exit(1);
                }
            };

            state.next_output_number = preset_as_number;
state.save()?;
        }
        _ => {}
    }

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ActiveOutput {
    preset: Preset,
    vnc_port: i32,
    vnc_process_pid: i32
}

#[derive(serde::Deserialize, serde::Serialize)]
struct State {
    next_output_number: u8,
    active_outputs: HashMap<String, ActiveOutput>,
    next_preset_number: HashMap<String, i32>
}

impl State {
    fn load(filepath: &str) -> Result<Self> {
        if Path::new(filepath).exists() == false {
            let mut new_file = File::create(filepath).unwrap();

            let default_state = State {
                next_output_number: 1,
                active_outputs: HashMap::new(),
                next_preset_number: HashMap::new()
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

    fn save(&self) -> Result<()> {
        let state_string = toml::to_string(&self)?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(STATE_FILEPATH)?;

        file.set_len(0)?;

        file.write_all(state_string.as_bytes())?;

        Ok(())
    }
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

fn load_presets() -> Result<PresetConfig> {
    let preset_filepath = "/home/odincat/.config/vdctl/presets.toml";

    if Path::new(preset_filepath).exists() == false {
        bail!("No file found at '{}'", preset_filepath);
    } else {
        let preset_file = fs::read_to_string(preset_filepath)?;

        let toml: PresetConfig = toml::from_str(&preset_file)?;

        Ok(toml)
    }
}

// Specific to linux, but that's where sway only runs anyway :shrug:
fn kill_by_pid(pid: i32) -> Result<()> {
    let output = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        bail!("Error killing process")
    }
}
