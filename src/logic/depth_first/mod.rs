use std::time::Duration;

use game::d_direction::DDirection;
use game::d_field::DSlowField;
use game::d_game_state::DGameState;
use intuition::DIntuition;
use log::warn;
use simulation::DSimulation;

use crate::logic::legacy::shared::brain::Brain;
use crate::{Direction, GameState};

pub mod game;
mod intuition;
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
        let d_state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        let simulation = DSimulation::new(d_state.clone());
        let simulation_result = simulation
            .capture(true)
            .capture_max_duration(Duration::from_millis(50))
            .capture_max_depth(20)
            .simulation_max_duration(Duration::from_millis(200))
            .simulation_node_max_duration(Duration::from_millis(20))
            .simulation_max_depth(10)
            .sparse_simulation_distance(6)
            .run();
        let intuition = DIntuition::new(d_state, gamestate);
        let intuition_result = intuition
            .allowed_directions(simulation_result.clone())
            .run();
        warn!(
            "Intuition selected {:?} from simulation result {:?}",
            intuition_result, simulation_result
        );
        match intuition_result {
            DDirection::Up => Direction::Up,
            DDirection::Down => Direction::Down,
            DDirection::Left => Direction::Left,
            DDirection::Right => Direction::Right,
        }
    }
}
