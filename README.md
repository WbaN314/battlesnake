# Battlesnake Rust Project

Competing at [play.battlesnake.com](https://play.battlesnake.com). Each participant provides the brain of his own snake on a self hosted server to survive the longest on the playing field. Games run daily and a leaderboard of the smartest snakes is created. [Here](https://play.battlesnake.com/profile/wban314) is the current ranking of this snake.

## Technologies Used

This project uses [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs) for performance reasons (simulating future gamestates scales exponentially so optimization is key). It also comes with an [Dockerfile](https://docs.docker.com/engine/reference/builder/) that is used for easy Google Could Run deployment.

## Commands

```
cargo run
cargo test
cargo bench
```

To test snakes against stored states

```
VARIANT=simple_hungry cargo test
VARIANT=simple_tree_search cargo test
VARIANT=depth_first cargo test
VARIANT=breadth_first cargo test
```

## Benchmarks

https://wban314.github.io/battlesnake/dev/bench

## Automated Testing from Game Logs

Run games with logging enabled, then analyze lost games to find decisions that would differ with more thinking time. Differing states are saved as regression tests.

```bash
# 1. Run games with logging (first snake's logs split into game_logs/game_N.log or game_N_lost.log)
cargo run --release --bin run_local_simulation -- -10 -l single_gamestate_nodes depth_first breadth_first simple_hungry

# 2. Analyze lost games (default: 10s re-evaluation timeout, 20 turns back)
cargo run --release --bin analyze_local_simulation

# With custom log directory, timeout (ms) and max turns back per game:
cargo run --release --bin analyze_local_simulation -- game_logs 5000 10

# 3. Run the generated regression tests
cargo run --release --bin run_generated_tests
```


## Flamegraph
Use to find hot spots with the benchmarks. For example:

```
CARGO_PROFILE_BENCH_DEBUG=true cargo flamegraph --unit-bench battlesnake_game_of_chicken_lib --open -- game::field::benchmarks::bench_next_state_with_basic_field
```

