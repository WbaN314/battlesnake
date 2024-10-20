use std::time::Duration;

use simulation_parameters::SimulationParameters;
use simulation_tree::SimulationTree;

use crate::Direction;

use super::{
    shared::{e_direction::EDirection, e_game_state::EGameState},
    Brain,
};

mod direction_rating;
mod node;
mod node_rating;
mod simulation_node;
mod simulation_parameters;
mod simulation_result;
mod simulation_state;
mod simulation_tree;

pub struct DepthFirstSnake {}

impl DepthFirstSnake {
    pub fn new() -> Self {
        Self {}
    }

    fn depth_first_simulation(&self, game_state: &EGameState) -> EDirection {
        let parameters = SimulationParameters::new()
            .prune_hash_radius(6)
            .move_snake_heads_radius(10)
            .simulation_duration(Duration::from_millis(200));
        let result = SimulationTree::from(game_state.clone())
            .parameters(parameters)
            .simulate_timed();
        result.get_best_direction()
    }
}

impl Brain for DepthFirstSnake {
    fn logic(
        &self,
        _game: &crate::Game,
        _turn: &i32,
        board: &crate::Board,
        you: &crate::Battlesnake,
    ) -> Direction {
        let game_state = EGameState::from(board, you);
        self.depth_first_simulation(&game_state).to_direction()
    }
}
