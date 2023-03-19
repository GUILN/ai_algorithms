pub mod canibals;
pub use canibals::*;

fn main() {
    let world = WorldState::new(
        SideState::new(0, 0),
        SideState::new(3, 3),
        BoatSide::RightSide,
        (None, "root state".to_string()),
    )
    .unwrap();

    let child_states = world.get_child_states();
    child_states.into_iter().for_each(|child_state| {
        let mut step = 0;
        if let Ok(world_state) = child_state {
            world_state
                .get_step_by_step_vec()
                .into_iter()
                .for_each(|s| {
                    println!("{}. {}", step, s);
                    step += 1;
                })
        }
    });

    println!("left state: {}", world);
    println!("is game over?: {}", world.is_game_over())
}
