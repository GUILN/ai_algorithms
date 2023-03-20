pub mod canibals;
pub use canibals::*;

fn main() {
    println!("BFS:");
    run_bfs();
    println!("DFS:");
    run_dfs();
}
