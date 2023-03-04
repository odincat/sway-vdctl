use anyhow::Result;

use super::ActionHandler;

pub fn list(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config: _, state, args: _, preset_names: _ } = action_handler;

    if state.active_outputs.is_empty() == false {
        for (_, output) in state.active_outputs {
            println!("Preset '{}' active on port {} ({}x{})", output.preset.name, output.preset.port, output.preset.resolution.width, output.preset.resolution.height);
        }
    } else {
        println!("No outputs seem to be active right now")
    }

    Ok(())
}
