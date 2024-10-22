use crate::{Direction, GameState};

pub trait Brain {
    fn logic(&self, gamestate: &GameState) -> Direction;
}
