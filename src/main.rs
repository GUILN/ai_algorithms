pub mod canibals;
pub use canibals::*;

fn main() {
    let world = WorldState::new(
        SideState::new(2, 1),
        SideState::new(1, 2),
        BoatSide::LeftSide,
    )
    .unwrap();

    println!("left state: {}", world);
    println!("is game over?: {}", world.is_game_over())
}
