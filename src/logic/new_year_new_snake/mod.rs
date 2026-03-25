use crate::{OriginalDirection, OriginalGameState, logic::legacy::shared::brain::Brain};

pub struct NewYearNewSnake;

mod node;
mod node_id;
mod tree;
mod tree_stats;

impl NewYearNewSnake {
    pub fn new() -> Self {
        Self
    }
}

impl Brain for NewYearNewSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        OriginalDirection::Up
    }
}
