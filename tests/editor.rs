#[cfg(test)]
use battlesnake_game_of_chicken_lib::{OriginalDirection, get_move_from_json_file};
#[macro_use]
mod common;

situation_test!(editor_1, Right => "Winner if just going right");
situation_test!(editor_2, Up, Right => "Up could draw, but if it works guarantees a win");
