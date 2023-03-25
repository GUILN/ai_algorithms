use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::rc::Rc;

use algoritmos_rust::{
    WorldState, WorldStateHeapWrapper, WorldStateResult, WorldStateWrapperCostFunctionType,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut already_queued_states: HashMap<String, bool> = HashMap::new();
    let mut next_states_to_visit_heap: BinaryHeap<Reverse<WorldStateHeapWrapper>> =
        BinaryHeap::new();
    const COST_FUNCTION_TYPE: WorldStateWrapperCostFunctionType =
        WorldStateWrapperCostFunctionType::HeuristicPlusBranchCost;

    const INITIAL_STATE: &str = "0 0 3 3 right";
    let initial_state: WorldStateResult = WorldState::try_from(INITIAL_STATE);
    let initial_state = initial_state.expect("faulty state");

    next_states_to_visit_heap.push(Reverse(WorldStateHeapWrapper::new(
        Rc::new(initial_state),
        COST_FUNCTION_TYPE,
    )));
    already_queued_states.insert(INITIAL_STATE.to_string(), true);
    let mut visited_states = 0;

    let solution_state: Option<Rc<WorldState>> = loop {
        if let Some(Reverse(state_to_visit)) = next_states_to_visit_heap.pop() {
            let mut solution: Option<Rc<WorldState>> = None;
            visited_states += 1;
            for child_world_state in state_to_visit.get_world_state().get_child_states() {
                let child_world_state = child_world_state.expect("faulty state!");
                let child_world_state = Rc::new(child_world_state);
                if child_world_state.is_game_over() {
                    continue;
                } else if child_world_state.is_solution() {
                    solution = Some(child_world_state);
                    break;
                }

                let world_state_str_representation: String = child_world_state.as_ref().into();
                // Checks if the world state is already in the queue to be visited.
                if already_queued_states.contains_key(&world_state_str_representation) {
                    continue;
                }
                next_states_to_visit_heap.push(Reverse(WorldStateHeapWrapper::new(
                    Rc::clone(&child_world_state),
                    COST_FUNCTION_TYPE,
                )));
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
