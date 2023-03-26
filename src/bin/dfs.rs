use std::collections::{HashMap, VecDeque};
use std::error::Error;

use algoritmos_rust::{WorldState, WorldStateResult};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut already_queued_states: HashMap<String, bool> = HashMap::new();
    let mut next_states_to_visit_stack: VecDeque<WorldState> = VecDeque::new();

    const INITIAL_STATE: &str = "0 0 3 3 right";
    let initial_state: WorldStateResult = WorldState::try_from(INITIAL_STATE);
    let initial_state = initial_state.expect("faulty state");

    next_states_to_visit_stack.push_front(initial_state);
    already_queued_states.insert(INITIAL_STATE.to_string(), true);

    let mut visited_states = 0;
    let solution_state: Option<WorldState> = loop {
        if let Some(state_to_visit) = next_states_to_visit_stack.pop_front() {
            let mut solution: Option<WorldState> = None;
            visited_states += 1;
            for child_state in state_to_visit.get_child_states() {
                let child_world_state = child_state.expect("faulty state!");
                if child_world_state.is_game_over() {
                    continue;
                } else if child_world_state.is_solution() {
                    solution = Some(child_world_state);
                    break;
                }

                let world_state_str_representation: String = (&child_world_state).into();
                // Checks if the world state is already in the queue to be visited.
                if already_queued_states.contains_key(&world_state_str_representation) {
                    continue;
                }
                next_states_to_visit_stack.push_front(child_world_state);
                already_queued_states.insert(world_state_str_representation, true);
            }
            if solution != None {
                break solution;
            }
        } else {
            break None;
        }
    };

    if let Some(state) = solution_state {
        println!("Follow the steps:");
        let step_by_step_vec = state.get_step_by_step_vec();
        let n_of_steps = step_by_step_vec.len() - 1;

        println!("visited states: {}", visited_states);
        println!("number of steps: {}", n_of_steps);
        step_by_step_vec
            .into_iter()
            .for_each(|step| println!("{}", step))
    } else {
        println!("no solution was found!");
    }

    Ok(())
}
