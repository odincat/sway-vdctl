use std::process;

use anyhow::Result;

use crate::{kill_by_pid, spawn_command};

use super::ActionHandler;

pub fn kill(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config, mut state, args, preset_names } = action_handler;

    let preset_name = args.value.as_str().to_lowercase();

    if let Some((preset_name, output)) = state.get_active_output(&preset_name) {
        println!("Closing '{}'...", output.preset.name);

        if let Some(vnc_process_pid) = output.vnc_process_pid {
            match kill_by_pid(vnc_process_pid) {
                Ok(_) => {},
                Err(err) => {
                    println!("There was an error killing the VNC server process, maybe it has been terminated manually: {}", err)
                }
            }
        } else {
            println!("Info: No VNC server was started")
        }

        spawn_command("swaymsg", vec!["output", &output.output_name, "unplug"])?;

        state.active_outputs.remove(&preset_name);
        state.save()?;
    } else {
        match config.get_preset(&preset_name) {
            Some(_) => {
                println!("Preset '{}' is currently not active", args.value)
            },
            None => {
                println!("Preset '{}' not found. Available presets are: {}", args.value, preset_names.join(", "));
                process::exit(1);
            }
        };
    }

    Ok(())
}
