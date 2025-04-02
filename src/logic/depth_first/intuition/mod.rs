use super::game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState};
use arrayvec::ArrayVec;
use d_scores::DScores;

mod d_scores;

pub struct DIntuition {
    game: DGameState<DSlowField>,
    allowed_directions: ArrayVec<DDirection, 4>,
}

impl DIntuition {
    pub fn new(game: DGameState<DSlowField>) -> Self {
        Self {
            game,
            allowed_directions: ArrayVec::from([
                DDirection::Up,
                DDirection::Down,
                DDirection::Left,
                DDirection::Right,
            ]),
        }
    }

    pub fn allowed_directions(mut self, directions: ArrayVec<DDirection, 4>) -> Self {
        self.allowed_directions = ArrayVec::from(directions);
        self
    }

    pub fn run(self) -> DDirection {
        let mut scores = DScores::new();

        for direction in self.allowed_directions {}

        DDirection::Up
    }
}
