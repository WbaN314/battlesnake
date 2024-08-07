// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use core::fmt;
use log::info;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::{collections::HashMap, env};

use crate::{Battlesnake, Board, Coord, Game};

mod efficient_game_objects;
mod hungry_simple_snake;
mod simple_tree_search_snake;
mod smart_snake;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Direction::Up => serializer.serialize_str("up"),
            Direction::Down => serializer.serialize_str("down"),
            Direction::Left => serializer.serialize_str("left"),
            Direction::Right => serializer.serialize_str("right"),
        }
    }
}

trait Brain {
    fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction;
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    let color = env::var("SNAKE_COLOR").unwrap_or("#f5982f".to_string());
    let head = env::var("SNAKE_HEAD").unwrap_or("chicken".to_string());
    let tail = env::var("SNAKE_TAIL").unwrap_or("duck".to_string());

    return json!({
        "apiversion": "1",
        "author": "WbaN",
        "color": color,
        "head": head,
        "tail": tail,
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are Move::Up, Move::Down, Move::Left, or Move::Right
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(
    game: &Game,
    turn: &i32,
    board: &Board,
    you: &Battlesnake,
    variant: String,
) -> Direction {
    let brain: Box<dyn Brain> = match variant.as_str() {
        "hungry_simple" => Box::new(hungry_simple_snake::HungrySimpleSnake::new()),
        "simple_tree_search" => Box::new(simple_tree_search_snake::SimpleTreeSearchSnake::new()),
        "smart_snake" => Box::new(smart_snake::SmartSnake::new()),
        _ => panic!("No VARIANT given for snake"),
    };
    let next_move = brain.logic(game, turn, board, you);
    // info!("MOVE {}: {}", turn, next_move);
    return next_move;
}

#[cfg(test)]
mod json_requests {
    use std::env;

    use crate::logic::Direction;

    use super::{efficient_game_objects::e_game_state::EGameState, get_move};

    const DIR: &str = "requests/";

    pub fn read_game_state(path: &str) -> crate::GameState {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let game_state: crate::GameState = serde_json::from_reader(reader).unwrap();
        game_state
    }

    fn get_move_from_json_file(path: &str) -> Direction {
        let game_state = read_game_state(&(DIR.to_string() + path));
        let print = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", print);
        env::set_var("MODE", "test");
        let m = get_move(
            &game_state.game,
            &game_state.turn,
            &game_state.board,
            &game_state.you,
            env::var("VARIANT").unwrap_or("smart_snake".to_string()),
        );
        m
    }

    #[test]
    fn example_move_request_2() {
        let chosen_move = get_move_from_json_file("example_move_request_2.json");
        assert_eq!(chosen_move, Direction::Up);
    }

    #[test]
    fn example_move_request_3() {
        let chosen_move = get_move_from_json_file("example_move_request_3.json");
        assert_eq!(chosen_move, Direction::Down);
    }
    #[test]
    fn failure_1() {
        let chosen_move = get_move_from_json_file("failure_1.json");
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_2() {
        let chosen_move = get_move_from_json_file("failure_2.json");
        assert_ne!(chosen_move, Direction::Right);
    }

    #[test]
    fn failure_3() {
        let chosen_move = get_move_from_json_file("failure_3.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_4() {
        let chosen_move = get_move_from_json_file("failure_4.json");
        assert_ne!(chosen_move, Direction::Right);
        assert_ne!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_5() {
        let chosen_move = get_move_from_json_file("failure_5.json");
        assert_eq!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_6() {
        let chosen_move = get_move_from_json_file("failure_6.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_7() {
        let chosen_move = get_move_from_json_file("failure_7.json");
        assert_ne!(chosen_move, Direction::Up);
        assert_ne!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_8() {
        let chosen_move = get_move_from_json_file("failure_8.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_9() {
        let chosen_move = get_move_from_json_file("failure_9.json");
        assert_eq!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_10() {
        let chosen_move = get_move_from_json_file("failure_10.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_11() {
        let chosen_move = get_move_from_json_file("failure_11.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_12() {
        let chosen_move = get_move_from_json_file("failure_12.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_13() {
        let chosen_move = get_move_from_json_file("failure_13.json");
        assert_ne!(chosen_move, Direction::Up);
        assert_ne!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_14() {
        let chosen_move = get_move_from_json_file("failure_14.json");
        assert_ne!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_15() {
        let chosen_move = get_move_from_json_file("failure_15.json");
        assert_ne!(chosen_move, Direction::Down);
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_16() {
        let chosen_move = get_move_from_json_file("failure_16.json");
        assert_ne!(chosen_move, Direction::Down);
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_17() {
        let chosen_move = get_move_from_json_file("failure_17.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_18() {
        let chosen_move = get_move_from_json_file("failure_18.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_19() {
        let chosen_move = get_move_from_json_file("failure_19.json");
        assert_ne!(chosen_move, Direction::Left);
        assert_ne!(chosen_move, Direction::Right);
    }

    #[test]
    fn failure_20_for_improved_area_evaluation() {
        let chosen_move = get_move_from_json_file("failure_20_for_improved_area_evaluation.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_21_bait_into_trap_with_top_wall() {
        // TODO: Space should be able to judge this if improved
        let chosen_move = get_move_from_json_file("failure_21_bait_into_trap_with_top_wall.json");
        assert_eq!(chosen_move, Direction::Right);
    }

    #[test]
    fn failure_22_bait_into_trap_with_top_wall_modified() {
        let chosen_move =
            get_move_from_json_file("failure_22_bait_into_trap_with_top_wall_modified.json");
        assert_eq!(chosen_move, Direction::Right);
    }

    #[test]
    fn failure_23_go_for_kill_here() {
        // Down leads to guaranteed kill in 2, right leads to guaranteed kill in 4
        let chosen_move = get_move_from_json_file("failure_23_go_for_kill_here.json");
        assert_ne!(chosen_move, Direction::Up);
        assert_ne!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_24_debug_space() {
        let chosen_move = get_move_from_json_file("failure_24_debug_space.json");
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_25_continue_down_for_kill() {
        let chosen_move = get_move_from_json_file("failure_25_continue_down_for_kill.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_26_continue_down_for_kill() {
        let chosen_move = get_move_from_json_file("failure_26_continue_down_for_kill.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_27_grab_food() {
        let chosen_move = get_move_from_json_file("failure_27_grab_food.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_28_grab_food() {
        let chosen_move = get_move_from_json_file("failure_28_grab_food.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_29_move_down_towards_food() {
        let chosen_move = get_move_from_json_file("failure_29_move_down_towards_food.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_30_grab_food_leads_to_death() {
        let chosen_move = get_move_from_json_file("failure_30_grab_food_leads_to_death.json");
        assert_ne!(chosen_move, Direction::Right);
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_31_going_right_leads_to_death() {
        let chosen_move = get_move_from_json_file("failure_31_going_right_leads_to_death.json");
        assert_ne!(chosen_move, Direction::Right);
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_32_right_certain_death_down_maybe_death() {
        let chosen_move =
            get_move_from_json_file("failure_32_right_certain_death_down_maybe_death.json");
        // Assuming perfect opponent play, right is the best move as it leads to certain death in 3 turns whereas down is in 2 turns
        // If opponents do not play perfectly, down is better as it might not lead to death
        assert_ne!(chosen_move, Direction::Up);
        assert_ne!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_33_do_not_move_left_as_you_can_get_killed() {
        let chosen_move =
            get_move_from_json_file("failure_33_do_not_move_left_as_you_can_get_killed.json");
        assert_eq!(chosen_move, Direction::Right);
    }

    #[test]
    fn failure_34_follow_own_tail() {
        // Too long to really evaluate with current visualisation tools, therefore left and right accepted
        let chosen_move = get_move_from_json_file("failure_34_follow_own_tail.json");
        assert_ne!(chosen_move, Direction::Up);
        assert_ne!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_35_up_2() {
        let chosen_move = get_move_from_json_file("failure_35_up_2.json");
        assert_eq!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_36_tail_2_food_4() {
        let chosen_move = get_move_from_json_file("failure_36_tail_2_food_4.json");
        assert_eq!(chosen_move, Direction::Right);
    }

    #[test]
    fn failure_37_unclear_best_move() {
        let chosen_move = get_move_from_json_file("failure_37_unclear_best_move.json");
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_38_left_possible_wall_squeeze() {
        // Moving right would equally allow for squeeze
        let chosen_move = get_move_from_json_file("failure_38_left_possible_wall_squeeze.json");
        assert_ne!(chosen_move, Direction::Up);
        assert_ne!(chosen_move, Direction::Down);
    }

    #[test]
    fn failure_39_grab_food_in_middle() {
        // This might be fixed with chicken logic
        // or adding length to simulate timed evaluation
        let chosen_move = get_move_from_json_file("failure_39_grab_food_in_middle.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_40_should_go_up_to_food() {
        // Food can be secured in both ways either up or right
        let chosen_move = get_move_from_json_file("failure_40_should_go_up_to_food.json");
        assert_ne!(chosen_move, Direction::Down);
        assert_ne!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_41_area_suggests_right_but_left_might_be_better() {
        let chosen_move =
            get_move_from_json_file("failure_41_area_suggests_right_but_left_might_be_better.json");
        assert_eq!(chosen_move, Direction::Left);
    }

    #[test]
    fn failure_42_going_right_enables_getting_killed() {
        let chosen_move =
            get_move_from_json_file("failure_42_going_right_enables_getting_killed.json");
        assert_ne!(chosen_move, Direction::Left);
        assert_ne!(chosen_move, Direction::Up);
    }

    #[test]
    fn failure_43_going_down_guarantees_getting_killed() {
        let chosen_move =
            get_move_from_json_file("failure_43_going_down_guarantees_getting_killed.json");
        assert_eq!(chosen_move, Direction::Up);
    }
}
