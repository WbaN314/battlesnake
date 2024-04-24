use std::ops::{Add, AddAssign};

use super::e_direction::EBoolDirections;

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

    // Returns
    pub fn directions_to(&self, other: &ECoord) -> EBoolDirections {
        if self.distance(other) == 0 {
            return [true; 4];
        } else {
            let mut result = [false; 4];
            if other.x > self.x {
                result[3] = true;
            } else if other.x < self.x {
                result[2] = true;
            }
            if other.y > self.y {
                result[0] = true;
            } else if other.y < self.y {
                result[1] = true
            }
            result
        }
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
