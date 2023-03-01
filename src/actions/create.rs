use std::process::{self, Command};

use anyhow::{Result, bail};

use crate::{state::ActiveOutput, spawn_command};

use super::ActionHandler;

pub fn create(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config, mut state, args, preset_names } = action_handler;

    let preset = match config.presets.get(args.value.as_str()) {
        Some(preset) => preset,
        None => {
            println!("Preset '{}' not found. Available presets are {}", args.value, preset_names.join(", "));
            process::exit(1);
        }
    };

    if let Some(_) = state.active_outputs.get(preset.name.as_str()) {
        //TODO: prompt for killing it?
        println!("There's already an output created for this preset. Kill it with 'vdctl kill {}', or create another preset", preset.name);
        process::exit(1);
    }

    let output_name = format!("HEADLESS-{}", &state.next_output_number);
    spawn_command("swaymsg", vec!["create_output"]).expect("Error creating output");
    println!("Created output, presumably '{}'", &output_name);

    state.next_output_number += 1;

    let resolution = format!("{}x{}", &preset.resolution.width, &preset.resolution.height);
    spawn_command("swaymsg", vec!["output", &output_name, "resolution", &resolution])?;
    println!("Set resolution of '{}' to {}", &output_name, &resolution);
    spawn_command("swaymsg", vec!["output", &output_name, "scale", &preset.scale_factor.to_string()])?;
    println!("Set scale factor of '{}' to {}", &output_name, &preset.scale_factor);

    if let Some(pos) = &preset.position {
        spawn_command("swaymsg", vec!["output", &output_name, "pos", &pos.x.to_string(), &pos.y.to_string()])?;
        println!("Set position of '{}' to x: {}, y:{}", &output_name, &pos.x, &pos.y);
    }

    if let Some(rot) = &preset.rotation {
        let rot = rot.clone();

        match rot {
            90 | 180 | 270 => {},
            _ => bail!("Rotation must be 0, +/- 90, 180, 270. Negative number will rotate counter-clockwise.")
        }

        let direction: &str = if rot < 0 {
            "anticlockwise"
        } else {
            "clockwise"
        };

        spawn_command("swaymsg", vec!["output", &output_name, "transform", &rot.to_string(), direction])?;
        println!("Set rotation of '{}' to {} ({})", &output_name, rot, direction);
    }

    Command::new("wayvnc").arg(format!("-o {}", &output_name));

    let host = match config.host {
        Some(host) => host,
        None => "0.0.0.0".to_string()
    };

    let mut vnc_process_pid: u32 = 0;
    if args.novnc {
        let vnc_cmd = spawn_command("wayvnc", vec![&host, &preset.port.to_string()])?;
        vnc_process_pid = vnc_cmd.id();
    }

    let active_output = ActiveOutput {
        preset: preset.clone(),
        vnc_process_pid: Some(vnc_process_pid)
    };

    state.active_outputs.insert(preset.name.to_owned(), active_output);

    state.save()?;

    Ok(())
}
