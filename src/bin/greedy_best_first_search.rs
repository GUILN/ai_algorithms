use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::error::Error;

use algoritmos_rust::{WorldState, WorldStateResult};

pub fn main() -> Result<(), Box<dyn Error>> {
    const INITIAL_STATE: &str = "0 0 3 3 right";
    let initial_state: WorldStateResult = WorldState::try_from(INITIAL_STATE);
    let initial_state = initial_state.expect("faulty state");
    let mut already_queued_states: HashMap<String, bool> = HashMap::new();

    let mut next_states_to_visit: VecDeque<WorldState> = VecDeque::new();

    // let mut next_states_to_visit_heap: BinaryHeap<WorldState> = BinaryHeap::new();

    already_queued_states.insert(INITIAL_STATE.to_string(), true);

    let mut initial_child_states = initial_state
        .get_child_states()
        .into_iter()
        .map(|w_result| w_result.expect("faulty state"))
        .collect::<Vec<WorldState>>();
    initial_child_states.sort_by(|a, b| return b.get_heuristic().cmp(&a.get_heuristic()));

    initial_child_states
        .into_iter()
        .for_each(|world_state_result| {
            let ref_world_state_result = &world_state_result;
            let world_state_str_representation: String = ref_world_state_result.into();
            next_states_to_visit.push_front(world_state_result);
            already_queued_states.insert(world_state_str_representation, true);
        });

    let solution_state: Option<WorldState> = loop {
        if let Some(state_to_visit) = next_states_to_visit.pop_front() {
            if state_to_visit.is_solution() {
                break Some(state_to_visit);
            } else if state_to_visit.is_game_over() {
                continue;
            } else {
                let mut child_states = state_to_visit
                    .get_child_states()
                    .into_iter()
                    .map(|w_result| w_result.expect("faulty state"))
                    .collect::<Vec<WorldState>>();
                child_states.sort_by(|a, b| return b.get_heuristic().cmp(&a.get_heuristic()));

                for child_world_state in child_states {
                    let ref_child_w_state = &child_world_state;
                    let world_state_str_representation: String = ref_child_w_state.into();

                    // Checks if the world state is already in the queue to be visited.
                    if already_queued_states.contains_key(&world_state_str_representation) {
                        continue;
                    }
                    next_states_to_visit.push_front(child_world_state);
                    already_queued_states.insert(world_state_str_representation, true);
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
