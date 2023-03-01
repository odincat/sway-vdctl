use std::process::{self, Command};

use anyhow::Result;

use crate::state::ActiveOutput;

use super::ActionHandler;

pub fn create(action_handler: ActionHandler) -> Result<()> {
    // let ActionHandler { action: _, mut state, args, presets, preset_names } = action_handler;

    println!("creating... {:?}", action_handler);

    return Ok(());

    // let preset = match presets.get(args.value.as_str()) {
    //     Some(preset) => preset,
    //     None => {
    //         println!("Preset '{}' not found. Available presets are {}", args.value, preset_names.join(", "));
    //         process::exit(1);
    //     }
    // };
    //
    // if let Some(_) = state.active_outputs.get(preset.name.as_str()) {
    //     //TODO: prompt for killing it?
    //     println!("There's already an output created for this preset. Kill it with 'vdctl kill {}', or create another preset", preset.name);
    //     process::exit(1);
    // }
    //
    // let output_name = format!("HEADLESS-{}", &state.next_output_number);
    // Command::new("swaymsg").arg("create_output").spawn().expect("Error creating output");
    // println!("Created output, presumably '{}'", &output_name);
    //
    // state.next_output_number += 1;
    //
    // let resolution = format!("{}x{}", &preset.resolution.width, &preset.resolution.height);
    // Command::new("swaymsg").arg("output").arg(&output_name).arg("resolution").arg(&resolution).spawn().expect("Error setting resolution");
    // println!("Set resolution of '{}' to {}", &output_name, &resolution);
    // Command::new("swaymsg").arg("output").arg(&output_name).arg("scale").arg(&preset.scale_factor.to_string()).spawn().expect("Error setting scale");
    // println!("Set scale factor of '{}' to {}", &output_name, &preset.scale_factor);
    //
    //
    // Command::new("wayvnc").arg(format!("-o {}", &output_name));
    //
    // let active_output = ActiveOutput {
    //     preset: preset.clone(),
    //     vnc_process_pid: Some(213)
    // };
    //
    // state.active_outputs.insert(preset.name.to_owned(), active_output);
    //
    // state.save()?;
    //
    // Ok(())
}
