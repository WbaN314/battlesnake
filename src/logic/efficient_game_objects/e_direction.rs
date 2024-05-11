use core::fmt;
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::logic::Direction;

use super::e_coord::ECoord;

pub type EBoolDirections = [bool; 4];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl EDirection {
    pub fn from_usize(u: usize) -> EDirection {
        match u {
            0 => EDirection::Up,
            1 => EDirection::Down,
            2 => EDirection::Left,
            3 => EDirection::Right,
            _ => panic!("Invalid usize for Direction conversion"),
        }
    }

    pub fn to_usize(self) -> usize {
        match self {
            EDirection::Up => 0,
            EDirection::Down => 1,
            EDirection::Left => 2,
            EDirection::Right => 3,
        }
    }

    pub fn to_direction(self) -> Direction {
        match self {
            EDirection::Up => Direction::Up,
            EDirection::Down => Direction::Down,
            EDirection::Left => Direction::Left,
            EDirection::Right => Direction::Right,
        }
    }
}

impl Display for EDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &EDirection::Up => write!(f, "U"),
            &EDirection::Down => write!(f, "D"),
            &EDirection::Left => write!(f, "L"),
            &EDirection::Right => write!(f, "R"),
        }
    }
}

pub const EDIRECTIONS: [EDirection; 4] = [
    EDirection::Up,
    EDirection::Down,
    EDirection::Left,
    EDirection::Right,
];

pub const EDIRECTION_VECTORS: [ECoord; 4] = [
    ECoord { x: 0, y: 1 },
    ECoord { x: 0, y: -1 },
    ECoord { x: -1, y: 0 },
    ECoord { x: 1, y: 0 },
];

#[derive(Clone, Debug)]
pub struct EDirectionVec(Vec<EDirection>);

impl EDirectionVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[allow(dead_code)]
    pub fn from(v: Vec<EDirection>) -> Self {
        Self(v)
    }
}

impl Deref for EDirectionVec {
    type Target = Vec<EDirection>;
    fn deref(&self) -> &Vec<EDirection> {
        &self.0
    }
}

impl DerefMut for EDirectionVec {
    fn deref_mut(&mut self) -> &mut Vec<EDirection> {
        &mut self.0
    }
}

impl PartialOrd for EDirectionVec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EDirectionVec {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for EDirectionVec {}

impl Ord for EDirectionVec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.len() > other.len() {
            std::cmp::Ordering::Greater
        } else if self.len() < other.len() {
            std::cmp::Ordering::Less
        } else {
            let mut i = 0;
            loop {
                if i >= self.len() {
                    break std::cmp::Ordering::Equal;
                } else if self[i].to_usize() > other[i].to_usize() {
                    break std::cmp::Ordering::Greater;
                } else if self[i].to_usize() < other[i].to_usize() {
                    break std::cmp::Ordering::Less;
                }
                i += 1;
            }
        }
    }
}

impl Display for EDirectionVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for direction in &self.0 {
            write!(f, "{} ", direction)?;
        }
        Ok(())
    }
}
