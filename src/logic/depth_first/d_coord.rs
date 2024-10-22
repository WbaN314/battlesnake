use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Coord;

use super::d_direction::DDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DCoord {
    pub x: i8,
    pub y: i8,
}

impl DCoord {
    pub fn new(x: i8, y: i8) -> Self {
        DCoord { x, y }
    }
}

impl From<&Coord> for DCoord {
    fn from(coord: &Coord) -> Self {
        DCoord {
            x: coord.x as i8,
            y: coord.y as i8,
        }
    }
}

impl From<DDirection> for DCoord {
    fn from(direction: DDirection) -> Self {
        match direction {
            DDirection::Up => DCoord { x: 0, y: 1 },
            DDirection::Down => DCoord { x: 0, y: -1 },
            DDirection::Left => DCoord { x: -1, y: 0 },
            DDirection::Right => DCoord { x: 1, y: 0 },
        }
    }
}

impl Add<DCoord> for DCoord {
    type Output = DCoord;

    fn add(self, rhs: DCoord) -> Self::Output {
        DCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<DCoord> for DCoord {
    type Output = DCoord;

    fn sub(self, rhs: DCoord) -> Self::Output {
        DCoord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for DCoord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add<DDirection> for DCoord {
    type Output = DCoord;

    fn add(self, rhs: DDirection) -> Self::Output {
        self + DCoord::from(rhs)
    }
}

impl AddAssign for DCoord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
