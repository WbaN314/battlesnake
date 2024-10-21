use super::d_coord::DCoord;

pub enum DSnake {
    Alive {
        health: u8,
        length: u8,
        head: DCoord,
        tail: DCoord,
    },
    Dead,
    Headless {
        health: u8,
        length: u8,
        tail: DCoord,
    },
}

impl DSnake {
    pub fn from_request() -> Self {
        todo!()
    }
}
