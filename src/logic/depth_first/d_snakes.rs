use super::d_snake::DSnake;
use std::cell::RefCell;

const SNAKES: usize = 4;

pub struct DSnakes {
    snakes: [RefCell<DSnake>; SNAKES],
}

impl DSnakes {}
