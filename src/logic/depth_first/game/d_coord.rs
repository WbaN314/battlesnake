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

    pub fn distance_to(&self, other: DCoord) -> u8 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
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

impl From<Coord> for DCoord {
    fn from(coord: Coord) -> Self {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance_to() {
        use super::*;

        let a = DCoord::new(0, 0);
        let b = DCoord::new(0, 1);
        let c = DCoord::new(1, 0);
        let d = DCoord::new(1, 1);

        assert_eq!(a.distance_to(b), 1);
        assert_eq!(a.distance_to(c), 1);
        assert_eq!(a.distance_to(d), 2);
    }
}
