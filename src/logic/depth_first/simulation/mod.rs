mod d_node_id;
mod d_tree;
mod node;

use std::{env, time::Duration};

use d_tree::DTree;
use node::d_full_simulation_node::DFullSimulationNode;

use super::game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState};

pub struct DSimulation {
    initial_state: DGameState<DSlowField>,
}

impl DSimulation {
    pub fn new(state: DGameState<DSlowField>) -> Self {
        Self {
            initial_state: state,
        }
    }

    pub fn run(&mut self) -> DDirection {
        let full_simulation = DFullSimulationNode::new(
            Default::default(),
            vec![self.initial_state.clone().into()],
            Default::default(),
            Default::default(),
        );
        let mut simulation_tree = DTree::default()
            .root(full_simulation)
            .time(Duration::from_millis(200));
        let simulation_status = simulation_tree.simulate();
        let simulation_result = simulation_tree.result();
        let simulation_directions = simulation_result.approved_directions();

        if env::var("MODE").is_ok_and(|value| value == "test") {
            println!("{}\n", simulation_tree);
            println!("{:?}\n", simulation_status);
            println!("{}", simulation_result);
            println!("{:?}", simulation_directions);
        }

        simulation_directions
            .iter()
            .enumerate()
            .find(|(_, b)| **b)
            .unwrap_or((0, &true))
            .0
            .try_into()
            .unwrap()
    }
}
