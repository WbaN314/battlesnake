use core::fmt;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use super::{
    e_direction::{EBoolDirections, EDirection},
    e_game_state::EGameState,
    e_snakes::{ESimulationError, Result},
};

#[derive(Clone)]
pub struct EStateNode {
    pub states: Vec<EGameState>,
    pub evaluated: EBoolDirections,
}

impl EStateNode {
    pub fn from(valid_states: Vec<EGameState>) -> EStateNode {
        Self {
            states: valid_states,
            evaluated: [false; 4],
        }
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
            if relevant_moves.len() == 0 {
                return Result::Err(ESimulationError::Death);
            }
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
            }
        }
        if new_valid_states.len() == 0 {
            return Result::Err(ESimulationError::Death);
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
