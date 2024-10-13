#![feature(type_changing_struct_update)]
#![feature(test)]
extern crate test;

use core::fmt;
use logic::{get_move, shared::e_game_state::EGameState};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::{collections::HashMap, env};

pub mod logic;

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    pub height: u32,
    pub width: i32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Battlesnake,
}

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

const DIR: &str = "requests/";

pub fn read_game_state(path: &str) -> GameState {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let game_state: GameState = serde_json::from_reader(reader).unwrap();
    game_state
}

pub fn get_move_from_json_file(path: &str) -> Direction {
    let game_state = read_game_state(&(DIR.to_string() + path));
    let print = EGameState::from(&game_state.board, &game_state.you);
    println!("{}", print);
    env::set_var("MODE", "test");
    let m = get_move(
        &game_state.game,
        &game_state.turn,
        &game_state.board,
        &game_state.you,
        env::var("VARIANT").unwrap_or("breadth_first".to_string()),
    );
    m
}
