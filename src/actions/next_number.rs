use anyhow::{Result, bail};

use super::ActionHandler;

pub fn set_next_output_number(action_handler: ActionHandler) -> Result<()> {
    let ActionHandler { config: _, mut state, args, preset_names: _ } = action_handler;

    let next_number: u8 = match args.value.parse() {
        Ok(num) => num,
        Err(err) => {
            bail!("Error parsing number from value: {}", err)
        }
    };

    state.next_output_number = next_number;
    state.save()?;

    Ok(())
}
