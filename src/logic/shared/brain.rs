use crate::{Battlesnake, Board, Game};

use super::direction::Direction;

pub trait Brain {
    fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction;
}
