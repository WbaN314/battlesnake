use crate::logic::legacy::shared::brain::Brain;
use crate::{Direction, GameState};

mod game;
mod simulation;

pub struct DepthFirstSnake {}

impl DepthFirstSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for DepthFirstSnake {
    fn logic(&self, _gamestate: &GameState) -> Direction {
        todo!()
    }
}
