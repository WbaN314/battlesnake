#[cfg(test)]
use battlesnake_game_of_chicken_lib::{OriginalDirection, get_move_from_json_file};

#[test]
fn editor_1() {
    let chosen_move = get_move_from_json_file("editor_1.json");
    assert_eq!(chosen_move, OriginalDirection::Right);
}

#[test]
fn editor_2() {
    let chosen_move = get_move_from_json_file("editor_2.json");
    assert_eq!(chosen_move, OriginalDirection::Right);
}