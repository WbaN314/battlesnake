use crate::logic::legacy::shared::brain::Brain;
use crate::{Battlesnake, Board, Direction, Game};

mod d_board;
mod d_coord;
mod d_field;
mod d_game_state;
mod d_snake;
mod d_snakes;

pub struct DepthFirstSnake {}

impl DepthFirstSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for DepthFirstSnake {
    fn logic(&self, _game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) -> Direction {
        todo!()
    }
}
