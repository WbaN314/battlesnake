//! Top-level benchmark for the `GamestateNodesSnake` — exercises `logic()`
//! end-to-end (tree search + flood fill + situation evaluation), which is the
//! path that runs in production per move. This is the target used by the
//! flamegraph command in the README.

extern crate test;

use super::GamestateNodesSnake;
use super::bench_fixtures;
use crate::logic::legacy::shared::brain::Brain;
use std::hint::black_box;

#[bench]
fn bench_snake_logic(b: &mut test::Bencher) {
    let snake = GamestateNodesSnake::new();
    let states = bench_fixtures::test_gamestates();
    // One measured iteration = one full sweep of all fixtures, so every sample
    // does identical aggregate work (avoids variance from time-bounded per-state
    // cost differences being sampled unevenly across the harness's batches).
    b.iter(|| {
        for state in &states {
            black_box(snake.logic(state));
        }
    });
}
