use std::cell::Cell;

use crate::{Battlesnake, Board};

use super::d_snake::DSnake;

const SNAKES: usize = 4;

pub struct DSnakes {
    snakes: [Cell<DSnake>; SNAKES],
}

impl DSnakes {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        todo!()
    }
}
