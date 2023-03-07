use anyhow::Result;

use super::ActionHandler;

pub fn list(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config: _, state, args: _, preset_names: _ } = action_handler;

    if state.active_outputs.is_empty() == false {
        for (_, output) in state.active_outputs {
            println!("Preset '{}' ('{}': {}x{}) active on port {}", output.preset.name, output.output_name, output.preset.resolution.width, output.preset.resolution.height, output.preset.port);
        }
    } else {
        println!("No outputs seem to be active right now")
    }

    Ok(())
}
