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
pub fn get_move(game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
    let brain: Box<dyn Brain> = if let Ok(value) = env::var("VARIANT") {
        debug!("{}", value);
        if value == "hungry_simple".to_string() {
            Box::new(hungry_simple_snake::HungrySimpleSnake::new())
        } else if value == "simple_tree_search" {
            Box::new(simple_tree_search_snake::SimpleTreeSearchSnake::new())
        } else if value == "smart_snake" {
            Box::new(smart_snake::SmartSnake::new())
        } else {
            Box::new(hungry_simple_snake::HungrySimpleSnake::new())
        }
    } else {
        Box::new(hungry_simple_snake::HungrySimpleSnake::new())
    };
    let next_move = brain.logic(game, turn, board, you);
    info!("MOVE {}: {}", turn, next_move);
    return next_move;
}

#[cfg(test)]
mod tests {
    use crate::logic::Direction;

    use super::get_move;

    fn read_game_state(path: &str) -> crate::GameState {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let game_state: crate::GameState = serde_json::from_reader(reader).unwrap();
        game_state
    }

    #[test]
    fn get_move_2() {
        let game_state = read_game_state("requests/example_move_request_2.json");
        let chosen_move = get_move(
            &game_state.game,
            &game_state.turn,
            &game_state.board,
            &game_state.you,
        );
        assert_eq!(chosen_move, Direction::Up);
    }

    #[test]
    fn get_move_3() {
        let game_state = read_game_state("requests/example_move_request_3.json");
        let chosen_move = get_move(
            &game_state.game,
            &game_state.turn,
            &game_state.board,
            &game_state.you,
        );
        assert_eq!(chosen_move, Direction::Down);
    }
}
