use anyhow::Result;

use crate::state::ActiveOutput;

use super::ActionHandler;

pub fn list(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config, mut state, args, preset_names } = action_handler;

    let active_outputs: Vec<ActiveOutput> = vec![];

    Ok(())
}
