mod d_node_id;
mod d_tree;
mod node;

use std::{time::Duration, usize};

use arrayvec::ArrayVec;
use d_tree::{DTree, DTreeTime};
use node::{
    d_full_simulation_node::DFullSimulationNode, d_optimistic_capture_node::DOptimisticCaptureNode,
};

use super::game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState};

pub struct DSimulation {
    initial_state: DGameState<DSlowField>,
    capture: bool,
    capture_max_duration: Option<Duration>,
    simulation_max_duration: Option<Duration>,
    simulation_max_depth: Option<usize>,
    capture_max_depth: Option<usize>,
    simulation_node_max_duration: Option<Duration>,
}

impl DSimulation {
    pub fn new(state: DGameState<DSlowField>) -> Self {
        Self {
            initial_state: state,
            capture: true,
            capture_max_duration: None,
            simulation_max_duration: None,
            simulation_max_depth: None,
            simulation_node_max_duration: None,
            capture_max_depth: None,
        }
    }

    pub fn capture_max_duration(self, duration: Duration) -> Self {
        Self {
            capture_max_duration: Some(duration),
            ..self
        }
    }

    pub fn capture(self, activated: bool) -> Self {
        Self {
            capture: activated,
            ..self
        }
    }

    pub fn simulation_max_duration(self, duration: Duration) -> Self {
        Self {
            simulation_max_duration: Some(duration),
            ..self
        }
    }

    pub fn simulation_max_depth(self, depth: usize) -> Self {
        Self {
            simulation_max_depth: Some(depth),
            ..self
        }
    }

    pub fn capture_max_depth(self, depth: usize) -> Self {
        Self {
            capture_max_depth: Some(depth),
            ..self
        }
    }

    pub fn simulation_node_max_duration(self, duration: Duration) -> Self {
        Self {
            simulation_node_max_duration: Some(duration),
            ..self
        }
    }

    pub fn run(self) -> ArrayVec<DDirection, 4> {
        let mut snake_relevance_depths = [
            [true, true, true, true],
            [true, true, true, true],
            [true, true, true, true],
            [true, true, true, true],
        ];

        // Capture
        if self.capture {
            // Capture Node
            let capture_simulation = DOptimisticCaptureNode::new(
                Default::default(),
                self.initial_state.clone(),
                Default::default(),
                Default::default(),
                Default::default(),
            );

            // Capture Tree
            let mut capture_tree = DTree::default().root(capture_simulation);

            if let Some(duration) = self.capture_max_duration {
                capture_tree = capture_tree.time(duration);
            }
            if let Some(max_depth) = self.capture_max_depth {
                capture_tree = capture_tree.max_depth(max_depth);
            }
            capture_tree.simulate();

            // Capture Tree Result
            let capture_result = capture_tree.result();
            let capture_contact_turn = capture_result.capture_contact_turn();
            snake_relevance_depths = [
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
            ];
            for i in 0..4 {
                for j in 1..4 {
                    if let Some(depth) = capture_contact_turn[i][j] {
                        snake_relevance_depths[i][j] =
                            depth < self.simulation_max_depth.unwrap_or(u8::MAX as usize) as u8;
                    }
                }
            }
            println!("CAPTURE RESULT\n{}\n", capture_result);
        }

        // Simulation Node
        let simulation_node_time = if let Some(duration) = self.simulation_node_max_duration {
            DTreeTime::new(duration)
        } else {
            DTreeTime::default()
        };
        let full_simulation = DFullSimulationNode::new(
            Default::default(),
            vec![self.initial_state.clone().into()],
            simulation_node_time,
            Default::default(),
            Some(snake_relevance_depths),
        );

        // Simulation Tree
        let mut simulation_tree = DTree::default().root(full_simulation);
        if let Some(duration) = self.simulation_max_duration {
            simulation_tree = simulation_tree.time(duration);
        }
        if let Some(max_depth) = self.simulation_max_depth {
            simulation_tree = simulation_tree.max_depth(max_depth);
        }
        simulation_tree.simulate();

        // Simulation Tree Result
        let simulation_result = simulation_tree.result();
        let simulation_directions = simulation_result.approved_directions();
        println!("SIMULATION RESULT \n{}\n", simulation_result);

        // Final Result
        println!("--- Final Result ---");
        let mut result: ArrayVec<DDirection, 4> = ArrayVec::new();
        for (i, direction) in simulation_directions.into_iter().enumerate() {
            if direction {
                result.push(i.try_into().unwrap());
            }
        }
        println!("{:?}\n", result);

        result
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        logic::depth_first::{
            game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
            simulation::DSimulation,
        },
        read_game_state,
    };

    #[test]
    fn test_basic_simulation() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let directions = DSimulation::new(state)
            .capture(false)
            .simulation_max_duration(Duration::from_millis(250))
            .run();

        assert_eq!(directions.len(), 2);
        assert!(directions.contains(&DDirection::Up));
        assert!(directions.contains(&DDirection::Right));
    }
}
