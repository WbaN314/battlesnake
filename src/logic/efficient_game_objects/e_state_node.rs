use core::fmt;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use super::{
    e_direction::{EBoolDirections, EDirection},
    e_game_state::{EGameState, EStateRating},
    e_snakes::{ESimulationError, Result},
};

#[derive(Clone)]
pub struct EStateNode {
    pub states: Vec<EGameState>,
    pub evaluated: EBoolDirections,
    pub rating: EStateRating,
}

impl EStateNode {
    pub fn from(valid_states: Vec<EGameState>) -> EStateNode {
        let mut s = Self {
            states: valid_states,
            evaluated: [false; 4],
            rating: EStateRating::new(),
        };
        s.rating = s.rate_states();
        s
    }

    fn rate_states(&self) -> EStateRating {
        let mut rating = EStateRating::new();
        for state in self.states.iter() {
            let current = state.rate_state();
            rating.snakes = current.snakes.max(rating.snakes);
        }
        rating
    }

    pub fn calc_next(
        &mut self,
        to: EDirection,
        distance: u8,
        start: &Instant,
        duration: &Duration,
    ) -> Result<EStateNode> {
        self.evaluated[to.to_usize()] = true;
        let mut new_valid_states = Vec::new();
        for state in self.states.iter() {
            let relevant_moves = state.relevant_moves(distance);
            let mut found_valid_move = false;
            for relevant_move in relevant_moves {
                if relevant_move[0].unwrap() != to {
                    continue;
                }
                let mut new_state = state.clone();
                if start.elapsed() > *duration {
                    return Result::Err(ESimulationError::Timer);
                }
                match new_state.move_snakes(relevant_move) {
                    Ok(_) => new_valid_states.push(new_state),
                    Err(_) => return Result::Err(ESimulationError::Death),
                };
                found_valid_move = true
            }
            if !found_valid_move {
                return Result::Err(ESimulationError::Death);
            }
        }
        Ok(Self::from(new_valid_states))
    }

    pub fn completely_evaluated(&self) -> bool {
        for i in self.evaluated {
            if !i {
                return false;
            }
        }
        return true;
    }
}

impl Display for EStateNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "# states: {} \ndirections: {:?}",
            self.states.len(),
            self.evaluated
        )
    }
}
