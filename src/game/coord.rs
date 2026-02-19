use crate::{OriginalCoord, game::direction::Direction};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

impl Coord {
    pub fn new(x: i8, y: i8) -> Self {
        Coord { x, y }
    }

    pub fn distance_to(&self, other: Coord) -> u8 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

impl From<&OriginalCoord> for Coord {
    fn from(coord: &OriginalCoord) -> Self {
        Coord {
            x: coord.x as i8,
            y: coord.y as i8,
        }
    }
}

impl From<OriginalCoord> for Coord {
    fn from(coord: OriginalCoord) -> Self {
        Coord {
            x: coord.x as i8,
            y: coord.y as i8,
        }
    }
}

impl From<Direction> for Coord {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Coord { x: 0, y: 1 },
            Direction::Down => Coord { x: 0, y: -1 },
            Direction::Left => Coord { x: -1, y: 0 },
            Direction::Right => Coord { x: 1, y: 0 },
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Coord::from(rhs)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance_to() {
        use super::*;

        let a = Coord::new(0, 0);
        let b = Coord::new(0, 1);
        let c = Coord::new(1, 0);
        let d = Coord::new(1, 1);

        assert_eq!(a.distance_to(b), 1);
        assert_eq!(a.distance_to(c), 1);
        assert_eq!(a.distance_to(d), 2);
    }
}
