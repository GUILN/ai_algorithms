use std::collections::VecDeque;
use std::error::Error;

use crate::{WorldState, WorldStateResult};

pub fn run_dfs() -> Result<(), Box<dyn Error>> {
    const INITIAL_STATE: &str = "0 0 3 3 right";
    let initial_state: WorldStateResult = WorldState::try_from(INITIAL_STATE);
    let initial_state = initial_state.expect("faulty state");

    let mut next_states_to_visit: VecDeque<WorldState> = VecDeque::new();
    let mut queued_states: Vec<String> = Vec::new();

    queued_states.push(INITIAL_STATE.to_string());

    initial_state
        .get_child_states()
        .into_iter()
        .for_each(|world_state_result| {
            let world_state_result = world_state_result.expect("expects non faulty state");
            let ref_world_state_result = &world_state_result;
            let world_state_str_representation: String = ref_world_state_result.into();
            next_states_to_visit.push_front(world_state_result);
            queued_states.push(world_state_str_representation);
        });

    let solution_state: Option<WorldState> = loop {
        if let Some(state_to_visit) = next_states_to_visit.pop_front() {
            if state_to_visit.is_solution() {
                break Some(state_to_visit);
            } else if state_to_visit.is_game_over() {
                continue;
            } else {
                for child_state in state_to_visit.get_child_states() {
                    let child_world_state = child_state.expect("expects non faulty state.");
                    let ref_child_w_state = &child_world_state;
                    let world_state_str_representation: String = ref_child_w_state.into();

                    // Checks if the world state is already in the queue to be visited.
                    if queued_states.contains(&world_state_str_representation) {
                        continue;
                    } else if ref_child_w_state.is_game_over() {
                        continue;
                    }
                    next_states_to_visit.push_front(child_world_state);
                    queued_states.push(world_state_str_representation);
                }
            }
        } else {
            break None;
        }
    };

    if let Some(state) = solution_state {
        println!("Follow the steps:");
        let step_by_step_vec = state.get_step_by_step_vec();
        let n_of_steps = step_by_step_vec.len() - 1;

        println!("number of steps: {}", n_of_steps);
        step_by_step_vec
            .into_iter()
            .for_each(|step| println!("{}", step))
    } else {
        println!("no solution was found!");
    }

    Ok(())
}
