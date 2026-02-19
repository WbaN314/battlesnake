use crate::{OriginalDirection, OriginalGameState};

pub trait Brain {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection;
}
