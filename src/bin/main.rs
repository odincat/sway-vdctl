use std::{collections::HashMap, process};

use anyhow::Result;
use clap::Parser;

use vdctl::{config::{load_config, Config}, state::State, Args, Action, actions::{handle_action, ActionHandler}};

const STATE_FILEPATH: &str = "/tmp/vdctl-state";
const PRESET_FILEPATH: &str = "/home/odincat/.config/vdctl/presets.json";

fn main() -> Result<()> {
    let args = Args::parse();
    let state = State::load(STATE_FILEPATH).unwrap();

    let config = match load_config(PRESET_FILEPATH) {
        Ok(sp) => sp,
        Err(err) => {
            match args.action {
                Action::NextOutputNumber => {
                    println!("WARN: Unable to load config: {:?}", err);
                    println!("Still continuing, as the operation you are performing doesn't require any presets");
                },
                _ => {
                    println!("Unable to load presets: {:?}", err);
                    process::exit(1);
                }
            }

            Config::default()
        }
    };

    // let presets: HashMap<String, Preset> = HashMap::new();
    let mut preset_names: Vec<String> = vec![];
    for (name, _) in config.clone().presets {
        preset_names.push(name);
    }

    let action_handler = ActionHandler {
        args: args.clone(),
        state,
        config,
        preset_names
    };

    handle_action(action_handler)?;

    Ok(())
}

