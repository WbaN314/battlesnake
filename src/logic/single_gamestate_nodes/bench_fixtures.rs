//! Shared benchmark fixtures: a curated 20-state selection spanning 2/3/4-snake
//! boards across early/mid/late game. All `single_gamestate_nodes` benchmarks
//! draw from this same set so they profile a representative, consistent workload.

use crate::{
    OriginalGameState,
    logic::general::{field::BasicField, game_state::GameState},
    read_game_state,
};

pub(crate) const PATHS: [&str; 20] = [
    // 4 snakes — early
    "requests/failure_27.json", // t=1
    "requests/failure_39.json", // t=9
    // 4 snakes — mid
    "requests/failure_82.json", // t=32
    "requests/failure_60.json", // t=38
    "requests/failure_79.json", // t=72
    // 4 snakes — late
    "requests/failure_81.json", // t=115
    "requests/failure_83.json", // t=192
    // 3 snakes — early
    "requests/failure_01.json", // t=8
    "requests/failure_04.json", // t=21
    // 3 snakes — mid
    "requests/failure_38.json", // t=73
    "requests/failure_85.json", // t=87
    // 3 snakes — late
    "requests/failure_35.json", // t=115
    "requests/failure_68.json", // t=155
    "requests/failure_73.json", // t=163
    // 2 snakes — early
    "requests/failure_48.json", // t=11
    "requests/failure_13.json", // t=14
    // 2 snakes — mid
    "requests/failure_46.json", // t=49
    "requests/failure_03.json", // t=65
    // 2 snakes — late
    "requests/failure_06.json", // t=256
    "requests/failure_71.json", // t=371
];

/// Raw gamestates as received over the wire.
pub(crate) fn test_gamestates() -> Vec<OriginalGameState> {
    PATHS.iter().map(|p| read_game_state(p)).collect()
}

/// Converted to the internal `BasicField` representation.
pub(crate) fn basic_field_states() -> Vec<GameState<BasicField>> {
    PATHS
        .iter()
        .map(|p| GameState::<BasicField>::from(&read_game_state(p)))
        .collect()
}
