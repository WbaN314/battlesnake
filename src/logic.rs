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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Up => write!(f, "up"),
            Move::Down => write!(f, "down"),
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
        }
    }
}

impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Move::Up => serializer.serialize_str("up"),
            Move::Down => serializer.serialize_str("down"),
            Move::Left => serializer.serialize_str("left"),
            Move::Right => serializer.serialize_str("right"),
        }
    }
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
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
pub fn get_move(_game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Move {
    let next_move = hungry_simple_snake(you, board, turn);
    info!("MOVE {}: {}", turn, next_move);
    return next_move;
}

fn hungry_simple_snake(you: &Battlesnake, board: &Board, _turn: &i32) -> Move {
    let mut is_move_safe: HashMap<Move, _> = vec![
        (Move::Up, true),
        (Move::Down, true),
        (Move::Left, true),
        (Move::Right, true),
    ]
    .into_iter()
    .collect();
    let my_head = &you.body[0];
    let board_width = board.width;
    let board_height = board.height as i32;
    if my_head.x + 1 == board_width {
        is_move_safe.insert(Move::Right, false);
    }
    if my_head.x == 0 {
        is_move_safe.insert(Move::Left, false);
    }
    if my_head.y + 1 == board_height {
        is_move_safe.insert(Move::Up, false);
    }
    if my_head.y == 0 {
        is_move_safe.insert(Move::Down, false);
    }
    let snakes = &board.snakes;
    for s in snakes {
        for i in 0..s.body.len() {
            if s.body[i].y == my_head.y {
                if s.body[i].x == my_head.x + 1 {
                    is_move_safe.insert(Move::Right, false);
                }
                if s.body[i].x + 1 == my_head.x {
                    is_move_safe.insert(Move::Left, false);
                }
            }
            if s.body[i].x == my_head.x {
                if s.body[i].y == my_head.y + 1 {
                    is_move_safe.insert(Move::Up, false);
                }
                if s.body[i].y + 1 == my_head.y {
                    is_move_safe.insert(Move::Down, false);
                }
            }
        }
    }
    let foods = &board.food;
    let middle = Coord {
        x: board_width / 2,
        y: board_height / 2,
    };
    let closest_food = if foods.len() == 0 {
        &middle
    } else {
        let mut closest_distance = u32::MAX;
        let mut tmp = &foods[0];
        for food in foods {
            let distance = my_head.x.abs_diff(food.x) + my_head.y.abs_diff(food.y);
            if distance <= closest_distance {
                closest_distance = distance;
                tmp = food;
            }
        }
        tmp
    };
    let chosen_move = if closest_food.x > my_head.x && *is_move_safe.get(&Move::Right).unwrap() {
        Move::Right
    } else if closest_food.x < my_head.x && *is_move_safe.get(&Move::Left).unwrap() {
        Move::Left
    } else if closest_food.y > my_head.y && *is_move_safe.get(&Move::Up).unwrap() {
        Move::Up
    } else if closest_food.y < my_head.y && *is_move_safe.get(&Move::Down).unwrap() {
        Move::Down
    } else {
        if *is_move_safe.get(&Move::Right).unwrap() {
            Move::Right
        } else if *is_move_safe.get(&Move::Left).unwrap() {
            Move::Left
        } else if *is_move_safe.get(&Move::Up).unwrap() {
            Move::Up
        } else if *is_move_safe.get(&Move::Down).unwrap() {
            Move::Down
        } else {
            Move::Up
        }
    };
    return chosen_move;
}
