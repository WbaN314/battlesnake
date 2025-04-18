use std::time::Duration;

use game::d_direction::DDirection;
use game::d_field::DSlowField;
use game::d_game_state::DGameState;
use simulation::DSimulation;

use crate::logic::legacy::shared::brain::Brain;
use crate::{Direction, GameState};

pub mod game;
mod simulation;

pub struct DepthFirstSnake {}

impl Default for DepthFirstSnake {
    fn default() -> Self {
        Self::new()
    }
}

impl DepthFirstSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for DepthFirstSnake {
    fn logic(&self, gamestate: &GameState) -> Direction {
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        let simulation = DSimulation::new(state);
        let simulation_result = simulation
            .capture(true)
            .capture_max_duration(Duration::from_millis(50))
            .capture_max_depth(16)
            .simulation_max_duration(Duration::from_millis(200))
            .simulation_node_max_duration(Duration::from_millis(20))
            .simulation_max_depth(8)
            .run();
        match simulation_result.get(0).unwrap_or(&DDirection::Up) {
            DDirection::Up => Direction::Up,
            DDirection::Down => Direction::Down,
            DDirection::Left => Direction::Left,
            DDirection::Right => Direction::Right,
        }
    }
}
