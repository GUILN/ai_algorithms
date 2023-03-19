use std::error::Error;

use crate::{WorldState, WorldStateError, WorldStateResult};

fn main() -> Result<(), Box<dyn Error>> {
    let initial_state: WorldStateResult = WorldState::try_from("0 0 3 3 right");

    let mut initial_state = initial_state?;

    let son_states = initial_state.get_child_states();

    Ok(())
}
