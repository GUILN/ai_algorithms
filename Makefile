format:
	cargo fmt

test:
	cargo test

## Run algorithms:
run_all: run_bfs run_dfs run_greedy_best_first_search run_a_star

run_bfs:
	cargo run --bin bfs

run_dfs:
	cargo run --bin dfs

run_greedy_best_first_search:
	cargo run --bin greedy_best_first_search

run_a_star:
	cargo run --bin a_star

