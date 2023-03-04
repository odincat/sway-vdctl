use std::process::Command;

use anyhow::{Result, bail};

use super::ActionHandler;

pub fn sync_next_output_number(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config: _, mut state, args: _, preset_names: _ } = action_handler;

    let output_command = Command::new("swaymsg").args(vec!["-t", "get_outputs", "-r"]).output()?;
    let output_string = String::from_utf8(output_command.stdout)?;


    let sway_outputs: SwayGetOutputs = match serde_json::from_str(&output_string) {
        Ok(outputs) => outputs,
        Err(err) => {
            bail!("unable to deserialize sway outputs: {}", err)
        }
    };

    let mut headless_outputs: Vec<String> = vec![];
    let mut headless_output_numbers: Vec<u8> = vec![];

    for output in sway_outputs {
        if output.name.starts_with("HEADLESS") {
            let number_str: Vec<&str> = output.name.split("-").collect();

            let number: u8 = if let Some(number_str) = number_str.last() {
                if let Ok(number) = number_str.parse::<u8>() {
                    number
                } else {
                    continue;
                }
            } else {
                continue;
            };

            headless_outputs.push(output.name);
            headless_output_numbers.push(number)
        }
    }

    //TODO: maybe sort headless_output_numbers, just to be sure. But should still work fine
    if let Some(last_number) = headless_output_numbers.last() {
        state.next_output_number = last_number + 1;
        state.save().unwrap();

        println!("Set next output number to {}", state.next_output_number)
    } else {
        println!("No headless outputs seem to be present at the moment")
    }

    Ok(())
}

// Parsing of 'swaymsg -t get_outputs'
type SwayGetOutputs = Vec<SwayOutput>;

#[derive(serde::Deserialize, Default)]
struct SwayOutput {
    #[serde(rename = "name")]
    name: String
}

// ^
// |
// This is way simpler and more compatible too, but it took some debugging to make ALL of it
// deserialize properly, so here it is. Maybe it will be useful someday:
//
// #[allow(dead_code)]
// #[derive(serde::Deserialize, Default)]
// struct SwayOutput {
//     id: u32,
//     #[serde(rename = "type")]
//     output_type: String,
//     orientation: String,
//     percent: f32,
//     urgent: bool,
//     marks: Vec<Value>,
//     layout: String,
//     border: String,
//     current_border_width: i32,
//     rect: Rect,
//     deco_rect: Rect,
//     window_rect: Rect,
//     geometry: Rect,
//     name: String,
//     window: Value,
//     nodes: Vec<Value>,
//     floating_nodes: Vec<Value>,
//     focus: Vec<u32>,
//     fullscreen_mode: u32,
//     sticky: bool,
//     primary: bool,
//     make: String,
//     model: String,
//     serial: String,
//     modes: Vec<Mode>,
//     non_desktop: bool,
//     active: bool,
//     dpms: bool,
//     power: bool,
//     scale: f32,
//     scale_filter: String,
//     transform: String,
//     adaptive_sync_status: String,
//     current_workspace: String,
//     current_mode: Mode,
//     max_render_time: u32,
//     focused: bool,
//     subpixel_hinting: String
// }
//
//
// #[allow(dead_code)]
// #[derive(serde::Deserialize, Default)]
// struct Rect {
//     x: i32,
//     y: i32,
//     width: u32,
//     height: u32
// }
//
// #[allow(dead_code)]
// #[derive(serde::Deserialize, Default)]
// struct Mode {
//     width: u32,
//     height: u32,
//     refresh: u32,
//     picture_aspect_ratio: Option<String>
// }
