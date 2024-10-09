use std::fmt::{Display, Formatter};

use super::simulation_tree::SimulationTree;

pub struct SimulationResult {
    simulation_tree: SimulationTree,
}

impl SimulationResult {
    pub fn from(simulation_tree: SimulationTree) -> Self {
        Self { simulation_tree }
    }
}

impl Display for SimulationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simulation_tree)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::logic::{
        depth_first::simulation_parameters::SimulationParameters, json_requests::read_game_state,
        shared::e_game_state::EGameState,
    };

    #[test]
    fn test_add_all_child_parent_converts_to_result() {
        let game_state = read_game_state("requests/failure_23_go_for_kill_here.json");
        let e_game_state = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", e_game_state);
        let parameters = SimulationParameters::new().duration(Duration::from_millis(200));
        let result = SimulationTree::from(e_game_state)
            .with_parameters(parameters)
            .simulate_timed();
        result.simulation_tree.print_node_ratings();
    }
}
