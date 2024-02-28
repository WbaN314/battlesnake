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

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Coord, Game};

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
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Value {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    let my_head = &you.body[0]; // Coordinates of your head

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    let board_width = board.width;
    let board_height = board.height as i32;

    if my_head.x + 1 == board_width {
        is_move_safe.insert("right", false);
    }
    if my_head.x == 0 {
        is_move_safe.insert("left", false);
    }
    if my_head.y + 1 == board_height {
        is_move_safe.insert("up", false);
    }
    if my_head.y == 0 {
        is_move_safe.insert("down", false);
    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    let snakes = &board.snakes;

    for s in snakes {
        for i in 0..s.body.len() {
            if s.body[i].y == my_head.y {
                if s.body[i].x == my_head.x + 1 {
                    is_move_safe.insert("right", false);
                }
                if s.body[i].x + 1 == my_head.x {
                    is_move_safe.insert("left", false);
                }
            }
            if s.body[i].x == my_head.x {
                if s.body[i].y == my_head.y + 1 {
                    is_move_safe.insert("up", false);
                }
                if s.body[i].y + 1 == my_head.y {
                    is_move_safe.insert("down", false);
                }
            }
        }
    }
    
    // find closest food
    let foods = &board.food;
    let middle = Coord {x: board_width / 2, y: board_height / 2};
    let closest_food = 
    if foods.len() == 0 {
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
    
    // try to move towards closest food
    let chosen_move: &&str = 
    if closest_food.x > my_head.x && *is_move_safe.get("right").unwrap() {
        &"right"
    } else if closest_food.x < my_head.x && *is_move_safe.get("left").unwrap() {
        &"left"
    } else if closest_food.y > my_head.y && *is_move_safe.get("up").unwrap() {
        &"up"
    } else if closest_food.y < my_head.y && *is_move_safe.get("down").unwrap() {
        &"down"
    } else {
        if *is_move_safe.get("right").unwrap() {
            &"right"
        } else if *is_move_safe.get("left").unwrap() {
            &"left"
        } else if *is_move_safe.get("up").unwrap() {
            &"up"
        } else if *is_move_safe.get("down").unwrap() {
            &"down"
        } else {
            &"up"
        }
    };

    info!("MOVE {}: {}", turn, chosen_move);
    return json!({ "move": chosen_move });
}
