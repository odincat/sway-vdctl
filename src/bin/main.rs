use std::{process, env};

use anyhow::Result;
use clap::Parser;

use vdctl::{config::{load_config, Config}, state::State, Args, Action, actions::{handle_action, ActionHandler}, is_command_installed};

const STATE_FILEPATH: &str = "/tmp/vdctl-state";

fn main() -> Result<()> {
    let args = Args::parse();
    let state = State::load(STATE_FILEPATH).unwrap();

    let config_path = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => format!("{}/vdctl/config.json", path),
        Err(_) => {
            let home = env::var("HOME")?;
            format!("{}/.config/vdctl/config.json", home)
        }
    };

    let config = match load_config(&config_path) {
        Ok(sp) => sp,
        Err(err) => {
            match args.action {
                Action::NextOutputNumber | Action::SyncNumber => {
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
    let preset_names: Vec<String> = config.presets.iter().map(|preset| preset.name.clone()).collect();

    if args.value.is_empty() {
        match args.action {
            Action::SyncNumber | Action::List => {},
            Action::Create => {
                println!("A preset must be supplied with this action. Available presets: {}", &preset_names.join(", "));
                process::exit(1);
            },
            Action::Kill => {
                let active_presets: Vec<String> = state.active_outputs.keys().cloned().collect();
                println!("A preset, that's currently active must be supplied with this action. Currently active presets: {}", active_presets.join(", "));
                process::exit(1);
            },
            _ => {
                println!("A value must be supplied with this action");
                process::exit(1);
            }
        }
    }

    if check_dependencies() == false {
        process::exit(1);
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

pub fn check_dependencies() -> bool {
    let commands = vec!["swaymsg", "wayvnc"];

    for command in &commands {
        if is_command_installed(command) == false {
            println!("{} needs to be available, exiting...", command);
            println!("Required commands: {}", commands.join(", "));

            return false;
        }
    }

    true
}
