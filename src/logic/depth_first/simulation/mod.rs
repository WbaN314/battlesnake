mod d_node_id;
mod d_tree;
mod node;

use std::time::Duration;

use arrayvec::ArrayVec;
use d_tree::{DTree, DTreeTime};
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

        let capture_contact_turn = capture_result.capture_contact_turn();
        let max_depth = 10;
        let mut snake_relevance_depths = [
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ];
        for i in 0..4 {
            for j in 1..4 {
                if let Some(depth) = capture_contact_turn[i][j] {
                    snake_relevance_depths[i][j] = depth < max_depth;
                }
            }
        }

        println!("{}\n", capture_result);

        println!("{:?}", snake_relevance_depths);

        let full_simulation = DFullSimulationNode::new(
            Default::default(),
            vec![self.initial_state.clone().into()],
            DTreeTime::new(Duration::from_millis(200)),
            Default::default(),
            Some(snake_relevance_depths),
        );
        let mut simulation_tree = DTree::default()
            .root(full_simulation)
            .time(Duration::from_millis(200))
            .max_depth(max_depth as usize);
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
