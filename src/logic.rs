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
use std::collections::HashMap;

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

    return json!({
        "apiversion": "1",
        "author": "WbaN", // TODO: Your Battlesnake Username
        "color": "#f5982f", // TODO: Choose color
        "head": "fang", // TODO: Choose head
        "tail": "rattle", // TODO: Choose tail
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
    info!("MOVE {}: {}", turn, next_move);
    return next_move;
}

#[cfg(test)]
mod json_requests {
    use std::env;

    use crate::logic::Direction;

    use super::{efficient_game_objects::e_game_state::EGameState, get_move};

    const DIR: &str = "requests/";

    fn read_game_state(path: &str) -> crate::GameState {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let game_state: crate::GameState = serde_json::from_reader(reader).unwrap();
        game_state
    }

    fn get_move_from_json_file(path: &str) -> Direction {
        let game_state = read_game_state(&(DIR.to_string() + path));
        let print = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", print);
        let m = get_move(
            &game_state.game,
            &game_state.turn,
            &game_state.board,
            &game_state.you,
            env::var("VARIANT").unwrap_or("smart_snake".to_string()),
        );
        println!("{}", m);
        m
    }

    #[test]
    fn example_move_request() {
        let chosen_move = get_move_from_json_file("example_move_request.json");
        assert_eq!(chosen_move, Direction::Up);
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
        assert_ne!(chosen_move, Direction::Up);
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
}
