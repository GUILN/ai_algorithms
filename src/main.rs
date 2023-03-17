pub mod canibals;
pub use canibals::*;

fn main() {
    let left_state = SideState::new(3, 3);
    println!("left state: {}", left_state)
}
