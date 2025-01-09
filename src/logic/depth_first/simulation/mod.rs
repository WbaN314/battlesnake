mod d_node_id;
mod d_tree;
mod node;

use std::{env, time::Duration};

use arrayvec::ArrayVec;
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
        let capture_simulation = DOptimisticCaptureNode::new(
            Default::default(),
            self.initial_state.clone(),
            Default::default(),
            Default::default(),
            Default::default(),
        );
        let mut capture_tree = DTree::default()
            .root(capture_simulation)
            .time(Duration::from_millis(50));
        capture_tree.simulate();
        let capture_result = capture_tree.result();

        println!("{}\n", capture_result);

        let full_simulation = DFullSimulationNode::new(
            Default::default(),
            vec![self.initial_state.clone().into()],
            Default::default(),
            Default::default(),
        );
        let mut simulation_tree = DTree::default()
            .root(full_simulation)
            .time(Duration::from_millis(200));
        simulation_tree.simulate();
        let simulation_result = simulation_tree.result();
        let simulation_directions = simulation_result.approved_directions();

        println!("{}\n", simulation_result);

        let mut result: ArrayVec<DDirection, 4> = ArrayVec::new();
        for (i, direction) in simulation_directions.into_iter().enumerate() {
            if direction {
                result.push(i.try_into().unwrap());
            }
        }

        println!("--- Final Result ---");
        println!("{:?}\n", result);

        result[0]
    }
}
