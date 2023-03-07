use anyhow::Result;

use crate::{state::State, Action, Args, config::Config};

mod create;
mod kill;
mod list;
mod next_number;
mod sync;

#[derive(Debug, Clone)]
pub struct ActionHandler {
    pub state: State,
    pub args: Args,
    pub config: Config,
    pub preset_names: Vec<String>
}

pub fn handle_action(action_handler: ActionHandler) -> Result<()> {
    match action_handler.args.action {
        Action::Create => create::create(action_handler)?,
        Action::Kill => kill::kill(action_handler)?,
        Action::List => list::list(action_handler)?,
        Action::SyncNumber => sync::sync_next_output_number(action_handler)?,
        Action::NextOutputNumber => next_number::set_next_output_number(action_handler)?,
    }

    Ok(())
}
