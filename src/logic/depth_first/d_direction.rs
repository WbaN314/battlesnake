use super::d_coord::DCoord;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDirection {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<DCoord> for DDirection {
    type Error = ();

    fn try_from(coord: DCoord) -> Result<Self, Self::Error> {
        match coord {
            DCoord { x: 0, y: 1 } => Ok(DDirection::Up),
            DCoord { x: 0, y: -1 } => Ok(DDirection::Down),
            DCoord { x: -1, y: 0 } => Ok(DDirection::Left),
            DCoord { x: 1, y: 0 } => Ok(DDirection::Right),
            _ => Err(()),
        }
    }
}
