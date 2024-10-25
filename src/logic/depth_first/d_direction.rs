use super::d_coord::DCoord;

pub static D_DIRECTION_LIST: [DDirection; 4] = [
    DDirection::Up,
    DDirection::Down,
    DDirection::Left,
    DDirection::Right,
];

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

impl TryFrom<u8> for DDirection {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DDirection::Up),
            1 => Ok(DDirection::Down),
            2 => Ok(DDirection::Left),
            3 => Ok(DDirection::Right),
            _ => Err(()),
        }
    }
}
