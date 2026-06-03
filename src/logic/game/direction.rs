use crate::logic::game::{coord::Coord, moves::MoveVector};
use std::fmt::Display;

pub static DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub struct Directions {
    bools: [bool; 4],
    checkpoint: [bool; 4]
}

impl Directions {
    pub fn new() -> Self {
        Self { bools: [true; 4], checkpoint: [true; 4] }
    }

    pub fn reset(&mut self) {
        self.bools = self.checkpoint;
    }

    pub fn reset_if_exhausted(&mut self) {
        if self.exhausted() {
            self.reset();
        }
    }

    pub fn set_checkpoint(&mut self) {
        self.checkpoint = self.bools;
    }

    pub fn get(&self, direction: Direction) -> bool {
        self.bools[direction as usize]
    }

    pub fn get_index(&self, index: usize) -> bool {
        self.bools[index]
    }

    pub fn set(&mut self, direction: Direction, value: bool) {
        self.bools[direction as usize] = value;
    }

    pub fn set_index(&mut self, index: usize, value: bool) {
        self.bools[index] = value;
    }

    pub fn exhausted(&self) -> bool {
        self.iter().next().is_none()
    }

    pub fn only_one_left(&self) -> Option<Direction> {
        let mut iter = self.iter();
        let first = iter.next()?;
        if iter.next().is_none() {
            Some(first)
        } else {
            None
        }
    }

    pub fn iter(&self) -> DirectionsIter {
        DirectionsIter {
            bools: self.bools,
            index: 0,
        }
    }
}
pub struct DirectionsIter {
    bools: [bool; 4],
    index: usize,
}

impl Iterator for DirectionsIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 4 {
            let i = self.index;
            self.index += 1;
            if self.bools[i] {
                return Direction::try_from(i).ok();
            }
        }
        None
    }
}

impl IntoIterator for Directions {
    type Item = Direction;
    type IntoIter = DirectionsIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionsIter {
            bools: self.bools,
            index: 0,
        }
    }
}

impl Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let directions: Vec<String> = self
            .iter()
            .map(|d| d.to_string())
            .collect();
        write!(f, "{}", directions.join(", "))
    }
}

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

impl From<Direction> for usize {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}
