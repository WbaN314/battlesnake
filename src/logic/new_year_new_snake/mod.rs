use crate::{OriginalDirection, OriginalGameState, logic::legacy::shared::brain::Brain};

pub struct NewYearNewSnake;

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