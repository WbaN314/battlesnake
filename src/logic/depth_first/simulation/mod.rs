mod d_node_id;
mod d_tree;
mod node;

use std::{env, time::Duration};

use d_tree::DTree;
use node::{
    d_full_simulation_node::DFullSimulationNode, d_optimistic_capture_node::DOptimisticCaptureNode,
};

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
        let optimistic_capture = DOptimisticCaptureNode::new(
            Default::default(),
            self.initial_state.clone(),
            Default::default(),
            Default::default(),
        );
        let mut capture_tree = DTree::default()
            .root(optimistic_capture)
            .time(Duration::from_millis(50));
        let capture_status = capture_tree.simulate();
        let capture_result = capture_tree.result();

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
        let mut simulation_result = simulation_tree.result();
        let direction = simulation_result.direction();

        if env::var("MODE").is_ok_and(|value| value == "test") {
            println!("{}", simulation_tree);
            println!("{:?}\n", simulation_status);
            println!("{}", simulation_result);
            println!("{}", direction);
        }

        direction
    }
}
