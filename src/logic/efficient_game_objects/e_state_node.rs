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
pub struct ENodeRating {
    pub highest_snake_count: u8,
}

impl ENodeRating {
    pub fn new() -> Self {
        Self {
            highest_snake_count: 0,
        }
    }

    pub fn update(&mut self, state: &EStateRating) {
        self.highest_snake_count = state.snakes.max(self.highest_snake_count);
    }
}

impl Display for ENodeRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Snakes: {}", self.highest_snake_count)
    }
}

#[derive(Clone)]
pub struct EStateNode {
    pub states: Vec<EGameState>,
    pub evaluated: EBoolDirections,
    pub rating: ENodeRating,
}

impl EStateNode {
    pub fn from(valid_states: Vec<EGameState>) -> EStateNode {
        let mut s = Self {
            states: valid_states,
            evaluated: [false; 4],
            rating: ENodeRating::new(),
        };
        s.rating = s.rate_node();
        s
    }

    fn rate_node(&self) -> ENodeRating {
        let mut rating = ENodeRating::new();
        for state in self.states.iter() {
            let current = state.rate_state();
            rating.update(&current);
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
