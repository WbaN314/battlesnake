//! Top-level benchmark for the `GamestateNodesSnake` — exercises `logic()`
//! end-to-end (tree search + flood fill + situation evaluation), which is the
//! path that runs in production per move. This is the target used by the
//! flamegraph command in the README.

extern crate test;

use super::GamestateNodesSnake;
use super::bench_fixtures;
use crate::logic::legacy::shared::brain::Brain;
use std::hint::black_box;

// Ignored so `cargo bench` skips it: each fixture runs to the SIMULATION_TIME_MS
// budget, and libtest fixes the sample count regardless of per-call cost, so this
// takes minutes. It exists for flamegraph profiling — the README command passes
// `--include-ignored` to run it.
#[bench]
#[ignore = "only for flamegraph with --include-ignored"]
fn bench_snake_logic(b: &mut test::Bencher) {
    let snake = GamestateNodesSnake::new();
    let states = bench_fixtures::test_gamestates();
    // One measured iteration = one state, cycling through the fixtures. Because
    // every `logic()` call runs to the same SIMULATION_TIME_MS budget, per-call
    // cost is constant regardless of state, so cycling introduces no sampling
    // variance — and each sample is ~one budget instead of a full 20-fixture
    // sweep, cutting the flamegraph run's wall-clock roughly by the fixture count.
    let mut i = 0;
    b.iter(|| {
        let state = &states[i % states.len()];
        i += 1;
        black_box(snake.logic(state));
    });
}

// Node- and depth-bounded variants of the production tree search. Unlike
// `bench_snake_logic` these terminate on a fixed budget rather than wall-clock
// time, so their runtime is deterministic and safe for `cargo bench` — the
// harness can size its sample count meaningfully.
#[bench]
fn bench_simulation_max_nodes(b: &mut test::Bencher) {
    let states = bench_fixtures::basic_field_states();
    b.iter(|| {
        for state in &states {
            let mut tree = GamestateNodesSnake::configured_tree(state.clone()).max_nodes(10000);
            black_box(tree.simulate());
        }
    });
}

#[bench]
fn bench_simulation_max_depth(b: &mut test::Bencher) {
    let states = bench_fixtures::basic_field_states();
    b.iter(|| {
        for state in &states {
            let mut tree = GamestateNodesSnake::configured_tree(state.clone()).max_depth(4);
            black_box(tree.simulate());
        }
    });
}
