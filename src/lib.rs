#![feature(type_changing_struct_update)]
#![feature(test)]
extern crate test;

use core::fmt;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::{collections::HashMap, env};

use crate::logic::{game::direction::Direction, get_move};

pub mod logic;

#[derive(Deserialize, Serialize, Debug)]
pub struct OriginalGame {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OriginalBoard {
    pub height: u32,
    pub width: i32,
    pub food: Vec<OriginalCoord>,
    pub snakes: Vec<OriginalBattlesnake>,
    pub hazards: Vec<OriginalCoord>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OriginalBattlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<OriginalCoord>,
    pub head: OriginalCoord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub struct OriginalCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OriginalGameState {
    pub game: OriginalGame,
    pub turn: i32,
    pub board: OriginalBoard,
    pub you: OriginalBattlesnake,
}

impl fmt::Display for OriginalGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Game ID: {}", self.game.id)?;
        writeln!(f, "Turn: {}", self.turn)?;
        writeln!(f, "Board: {}x{}", self.board.width, self.board.height)?;
        writeln!(f, "You: {}", self.you.name)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum OriginalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for OriginalDirection {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => OriginalDirection::Up,
            Direction::Down => OriginalDirection::Down,
            Direction::Left => OriginalDirection::Left,
            Direction::Right => OriginalDirection::Right,
        }
    }
}

impl fmt::Display for OriginalDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OriginalDirection::Up => write!(f, "up"),
            OriginalDirection::Down => write!(f, "down"),
            OriginalDirection::Left => write!(f, "left"),
            OriginalDirection::Right => write!(f, "right"),
        }
    }
}

impl Serialize for OriginalDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OriginalDirection::Up => serializer.serialize_str("up"),
            OriginalDirection::Down => serializer.serialize_str("down"),
            OriginalDirection::Left => serializer.serialize_str("left"),
            OriginalDirection::Right => serializer.serialize_str("right"),
        }
    }
}

const DIR: &str = "requests/";

pub fn read_game_state(path: &str) -> OriginalGameState {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .try_init();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let gamestate: OriginalGameState = serde_json::from_reader(reader).unwrap();
    check_game_state(&gamestate);
    gamestate
}

fn check_game_state(state: &OriginalGameState) {
    let height = state.board.height as i32;
    for snake in state.board.snakes.iter() {
        let snake_sum = snake.head.x + snake.head.y;
        assert!(snake_sum % 2 == state.turn % 2);
        assert!(snake.body.len() as i32 == snake.length);
        assert!(snake.body[0] == snake.head);
        assert!(snake.health > 0);
        assert!(snake.body.windows(2).all(|w| {
            let dx = (w[0].x - w[1].x).abs();
            let dy = (w[0].y - w[1].y).abs();
            // Adjacent segments differ by 1 in one axis, or are stacked (eating/start)
            (dx == 1 && dy == 0) || (dx == 0 && dy == 1) || (dx == 0 && dy == 0)
        }));
        assert!(snake.body.iter().all(|coord| {
            coord.x >= 0 && coord.x < state.board.width && coord.y >= 0 && coord.y < height
        }));
        // Different snakes must not overlap
        assert!(snake.body.iter().all(|coord| {
            !state
                .board
                .snakes
                .iter()
                .filter(|other| other.id != snake.id)
                .any(|other| other.body.contains(coord))
        }));
        assert!(snake.length >= 3);
    }
    assert!(state.board.food.iter().all(|coord| {
        coord.x >= 0 && coord.x < state.board.width && coord.y >= 0 && coord.y < height
    }));
    assert!(state.board.hazards.iter().all(|coord| {
        coord.x >= 0 && coord.x < state.board.width && coord.y >= 0 && coord.y < height
    }));
}

pub fn get_move_from_json_file(path: &str) -> OriginalDirection {
    let gamestate = read_game_state(&(DIR.to_string() + path));

    unsafe {
        env::set_var("MODE", "test");
    }

    get_move(
        &gamestate,
        env::var("VARIANT").unwrap_or("single_gamestate_nodes".to_string()),
    )
}
