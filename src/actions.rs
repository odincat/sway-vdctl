use std::collections::HashMap;

use anyhow::Result;

use crate::{state::State, Action, Args, config::{Preset, Config}};

mod create;

#[derive(Debug)]
pub struct ActionHandler {
    pub state: State,
    pub args: Args,
    pub config: Config,
    pub presets: HashMap<String, Preset>,
    pub preset_names: Vec<String>
}

pub fn handle_action(action_handler: ActionHandler) -> Result<()> {

    // let ActionHandler { action, mut state, args, presets, preset_names } = action_handler;

    match action_handler.args.action {
        Action::Create => create::create(action_handler)?,
        // Action::NextOutputNumber => {
        //     let preset_as_number: u8 = match args.value.parse() {
        //         Ok(num) => num,
        //         Err(_) => {
        //             println!("Please provide a number");
        //             process::exit(1);
        //         }
        //     };
        //
        //     state.next_output_number = preset_as_number;
        //     state.save()?;
        // }
        _ => {}
    }

    Ok(())
}
