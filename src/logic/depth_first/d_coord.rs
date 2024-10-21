use crate::Coord;

use super::d_direction::DDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DCoord {
    pub x: i8,
    pub y: i8,
}

impl DCoord {
    pub fn direction_to(&self, other: DCoord) -> Option<DDirection> {
        if self.x == other.x {
            if self.y == other.y + 1 {
                Some(DDirection::Down)
            } else if self.y == other.y - 1 {
                Some(DDirection::Up)
            } else {
                None
            }
        } else if self.y == other.y {
            if self.x == other.x + 1 {
                Some(DDirection::Left)
            } else if self.x == other.x - 1 {
                Some(DDirection::Right)
            } else {
                None
            }
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_to() {
        let start = DCoord { x: 1, y: 1 };
        let up = DCoord { x: 1, y: 2 };
        let down = DCoord { x: 1, y: 0 };
        let left = DCoord { x: 0, y: 1 };
        let right = DCoord { x: 2, y: 1 };
        assert_eq!(start.direction_to(up), Some(DDirection::Up));
        assert_eq!(start.direction_to(down), Some(DDirection::Down));
        assert_eq!(start.direction_to(left), Some(DDirection::Left));
        assert_eq!(start.direction_to(right), Some(DDirection::Right));
    }
}
