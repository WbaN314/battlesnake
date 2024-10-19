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

use legacy::shared::brain::Brain;
use log::info;
use serde_json::{json, Value};
use std::env;

use crate::{Battlesnake, Board, Direction, Game};

mod depth_first;
pub mod legacy;

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
        "simple_hungry" => Box::new(legacy::simple_hungry::SimpleHungrySnake::new()),
        "simple_tree_search" => Box::new(legacy::simple_tree_search::SimpleTreeSearchSnake::new()),
        "breadth_first" => Box::new(legacy::breadth_first::BreadthFirstSnake::new()),
        "depth_first" => Box::new(depth_first::DepthFirstSnake::new()),
        _ => panic!("No VARIANT given for snake"),
    };
    let next_move = brain.logic(game, turn, board, you);
    // info!("MOVE {}: {}", turn, next_move);
    return next_move;
}
