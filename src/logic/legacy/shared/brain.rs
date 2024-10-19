use crate::{Battlesnake, Board, Direction, Game};

pub trait Brain {
    fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction;
}
