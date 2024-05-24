use std::ops::{Add, AddAssign};

use super::e_direction::{EDirection, EDIRECTION_VECTORS};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ECoord {
    pub x: i8,
    pub y: i8,
}

impl ECoord {
    pub fn from(x: i8, y: i8) -> Self {
        ECoord { x, y }
    }

    pub fn distance(&self, other: &ECoord) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn move_in_direction(self, direction: EDirection) -> ECoord {
        self + EDIRECTION_VECTORS[direction.to_usize()]
    }
}

impl Add for ECoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for ECoord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
