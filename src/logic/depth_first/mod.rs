use crate::logic::legacy::shared::brain::Brain;
use crate::{Direction, GameState};

mod d_board;
mod d_coord;
mod d_direction;
mod d_field;
mod d_game_state;
mod d_moves_set;
mod d_snake;
mod d_snakes;

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
