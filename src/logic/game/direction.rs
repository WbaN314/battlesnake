use crate::logic::game::{coord::Coord, moves::MoveVector};
use std::fmt::Display;

pub static DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

impl TryFrom<Coord> for Direction {
    type Error = ();

    fn try_from(coord: Coord) -> Result<Self, Self::Error> {
        match coord {
            Coord { x: 0, y: 1 } => Ok(Direction::Up),
            Coord { x: 0, y: -1 } => Ok(Direction::Down),
            Coord { x: -1, y: 0 } => Ok(Direction::Left),
            Coord { x: 1, y: 0 } => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl TryFrom<MoveVector> for Direction {
    type Error = ();

    fn try_from(value: MoveVector) -> Result<Self, Self::Error> {
        match *value {
            Some(arr) => arr.try_into(),
            None => Err(()),
        }
    }
}

impl TryFrom<[bool; 4]> for Direction {
    type Error = ();

    fn try_from(value: [bool; 4]) -> Result<Self, Self::Error> {
        match value {
            [true, false, false, false] => Ok(Direction::Up),
            [false, true, false, false] => Ok(Direction::Down),
            [false, false, true, false] => Ok(Direction::Left),
            [false, false, false, true] => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}
