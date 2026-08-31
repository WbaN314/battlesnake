#[cfg(test)]
use battlesnake_game_of_chicken_lib::{OriginalDirection, get_move_from_json_file};
#[macro_use]
mod common;

// A: Row 1, B: Row 0
// Kill Situations
// 1 Ahead
situation_test!(editor_01, Right => "Always right to win.");
// 1 Behind
situation_test!(editor_02, Right => "If longer, right to win.");

// A: Row 2, B: Row 0
// Kill Situations
// Same x
situation_test!(editor_03, Down => "If longer, down goes back to editor_02 kill situation.");
// Restrict Situations
// 2 Behind
situation_test!(editor_04, Right => "If longer, right forces opponent on bottom two rows.");

// A: Row 3, B: Row 0
// Restrict Situations
// 1 Ahead
situation_test!(editor_05, Down => "If longer, down forces opponent on bottom 2 rows if he does not make a risky move.");
// 1 Behind
situation_test!(editor_06, Right => "If longer, right to make opponent take a decision first. Then we can force bottom two rows.");

// A: Row 4, B: Row 0
// Restrict Situations
// Same x
situation_test!(editor_07, Down => "If longer, down twice lets opponent decide first to get a good row 2 scenario to force bottom 2 rows.");

// Winner Situation
situation_test!(editor_08, Right => "Right is only valid choice");
situation_test!(editor_09, Left, Up => "Right is only valid choice");
