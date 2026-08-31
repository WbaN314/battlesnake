use crate::{OriginalCoord, logic::general::direction::Direction};
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

    pub fn king_distance_to(&self, other: Coord) -> u8 {
        (self.x - other.x).unsigned_abs().max((self.y - other.y).unsigned_abs())
    }

    pub fn directions_to(&self, other: Coord) -> [Option<Direction>; 2] {
        let mut directions = [None, None];
        let mut i = 0;
        if self.y < other.y {
            directions[i] = Some(Direction::Up);
            i += 1;
        } else if self.y > other.y {
            directions[i] = Some(Direction::Down);
            i += 1;
        }
        if self.x < other.x {
            directions[i] = Some(Direction::Right);
        } else if self.x > other.x {
            directions[i] = Some(Direction::Left);
        }
        directions
    }

    pub fn in_triangle(&self, a: Coord, b: Coord, c: Coord) -> bool {
        let sign = |p1: Coord, p2: Coord, p3: Coord| -> i32 {
            (p1.x as i32 - p3.x as i32) * (p2.y as i32 - p3.y as i32)
                - (p2.x as i32 - p3.x as i32) * (p1.y as i32 - p3.y as i32)
        };
        let d1 = sign(*self, a, b);
        let d2 = sign(*self, b, c);
        let d3 = sign(*self, c, a);
        (d1 > 0 && d2 > 0 && d3 > 0) || (d1 < 0 && d2 < 0 && d3 < 0)
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

    #[test]
    fn test_in_triangle() {
        use super::*;

        let a = Coord::new(0, 0);
        let b = Coord::new(10, 0);
        let c = Coord::new(5, 10);

        // clearly inside
        assert!(Coord::new(5, 5).in_triangle(a, b, c));
        assert!(Coord::new(3, 2).in_triangle(a, b, c));
        // clearly outside
        assert!(!Coord::new(0, 10).in_triangle(a, b, c));
        assert!(!Coord::new(10, 10).in_triangle(a, b, c));
        assert!(!Coord::new(1, 9).in_triangle(a, b, c));
        // on a vertex
        assert!(!Coord::new(0, 0).in_triangle(a, b, c));
        assert!(!Coord::new(10, 0).in_triangle(a, b, c));
        assert!(!Coord::new(5, 10).in_triangle(a, b, c));
        // on an edge
        assert!(!Coord::new(5, 0).in_triangle(a, b, c));  // bottom edge midpoint
        assert!(!Coord::new(2, 4).in_triangle(a, b, c));  // left edge
        assert!(!Coord::new(8, 4).in_triangle(a, b, c));  // right edge
    }

    #[test]
    fn test_directions_to() {
        use super::*;

        let a = Coord::new(0, 0);
        let b = Coord::new(0, 1);
        let c = Coord::new(1, 0);
        let d = Coord::new(1, 1);

        assert_eq!(a.directions_to(a), [None, None]);
        assert_eq!(a.directions_to(b), [Some(Direction::Up), None]);
        assert_eq!(a.directions_to(c), [Some(Direction::Right), None]);
        assert_eq!(a.directions_to(d), [Some(Direction::Up), Some(Direction::Right)]);
    }
}
