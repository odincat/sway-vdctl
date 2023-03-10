use std::{process::{self, Command}, time::Duration};

use anyhow::{Result, bail};

use crate::state::ActiveOutput;

use super::ActionHandler;

pub fn create(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config, mut state, args, preset_names } = action_handler;

    let preset_name = args.value.as_str().to_lowercase();

    let preset = match config.get_preset(&preset_name) {
        Some(preset) => preset,
        None => {
            println!("Preset '{}' not found. Available presets are: {}", args.value, preset_names.join(", "));
            process::exit(1);
        }
    };

    if let Some(_) = state.active_outputs.get(preset.name.as_str()) {
        //TODO: prompt for killing it?
        println!("There's already an output created for this preset. Kill it with 'vdctl kill {}', or create another preset", preset.name);
        process::exit(1);
    }

    
    if port_scanner::local_port_available(preset.port) == false {
        println!("Port {} seems to be unavailable, aborting...", preset.port);
        process::exit(1);
    }

    let output_name = format!("HEADLESS-{}", &state.next_output_number.clone());
    Command::new("swaymsg").arg("create_output").spawn().expect("Error creating output");
    println!("Created output, presumably '{}'", &output_name);

    state.next_output_number += 1;

    let resolution = format!("{}x{}", &preset.resolution.width, &preset.resolution.height);
    Command::new("swaymsg").args(vec!["output", &output_name, "resolution", &resolution]).spawn()?;
    println!("Set resolution of '{}' to {}", &output_name, &resolution);
    Command::new("swaymsg").args(vec!["output", &output_name, "scale", &preset.scale_factor.to_string()]).spawn()?;
    println!("Set scale factor of '{}' to {}", &output_name, &preset.scale_factor);

    if let Some(pos) = &preset.position {
        Command::new("swaymsg").args(vec!["output", &output_name, "pos", &pos.x.to_string(), &pos.y.to_string()]).spawn()?;
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

        Command::new("swaymsg").args(vec!["output", &output_name, "transform", &rot.to_string(), direction]).spawn()?;
        println!("Set rotation of '{}' to {} ({})", &output_name, rot, direction);
    }


    let host = match config.host {
        Some(host) => host,
        None => "0.0.0.0".to_string()
    };

    let mut active_output = ActiveOutput {
        preset: preset.clone(),
        vnc_process_pid: None,
        output_name: output_name.clone()
    };

    if args.novnc == false {
        std::thread::sleep(Duration::from_millis(500));

        let vnc_output = format!("-o={}", &output_name);
        let vnc_socket = format!("-S=/tmp/{}.sock", &output_name.to_lowercase());

        let vnc_cmd = Command::new("wayvnc")
            .arg(vnc_output)
            .arg(vnc_socket)
            .arg(&host)
            .arg(&preset.port.to_string())
            .spawn()?;

        //TODO: error handling of vnc command

        active_output.vnc_process_pid = Some(vnc_cmd.id());
    } 

    state.active_outputs.insert(preset.name.to_owned(), active_output);

    state.save()?;

    println!("Preset '{}' ('{}': {}x{}) is now active on port {}", preset.name, &output_name, preset.resolution.width, preset.resolution.height, preset.port);

    Ok(())
}
