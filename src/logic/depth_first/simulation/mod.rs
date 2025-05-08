mod d_node_id;
mod d_tree;
mod node;

use std::{time::Duration, usize};

use arrayvec::ArrayVec;
use d_tree::{DTree, DTreeTime};
use log::info;
use node::d_full_simulation_node::DFullSimulationNode;

use super::game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState};

pub struct DSimulation {
    initial_state: DGameState<DSlowField>,

    simulation_max_duration: Option<Duration>,
    simulation_max_depth: Option<usize>,
    simulation_node_max_duration: Option<Duration>,
    sparse_simulation_distance: Option<u8>,
}

impl DSimulation {
    pub fn new(state: DGameState<DSlowField>) -> Self {
        Self {
            initial_state: state,
            simulation_max_duration: None,
            simulation_max_depth: None,
            simulation_node_max_duration: None,
            sparse_simulation_distance: None,
        }
    }

    pub fn sparse_simulation_distance(self, distance: u8) -> Self {
        Self {
            sparse_simulation_distance: Some(distance),
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

    pub fn simulation_node_max_duration(self, duration: Duration) -> Self {
        Self {
            simulation_node_max_duration: Some(duration),
            ..self
        }
    }

    pub fn run(self) -> ArrayVec<DDirection, 4> {
        let snake_relevance_depths = [
            [true, true, true, true],
            [true, true, true, true],
            [true, true, true, true],
            [true, true, true, true],
        ];

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
            self.simulation_max_depth.map(|d| 2 * d as u8),
            self.sparse_simulation_distance,
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

        info!("SIMULATION RESULT\n{}", simulation_result);

        // Final Result
        let mut result: ArrayVec<DDirection, 4> = ArrayVec::new();
        for (i, direction) in simulation_directions.into_iter().enumerate() {
            if direction {
                result.push(i.try_into().unwrap());
            }
        }

        info!("APPROVED SIMULATION DIRECTIONS\n{:?}", result);

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
            .simulation_max_duration(Duration::from_millis(250))
            .run();

        assert_eq!(directions.len(), 2);
        assert!(directions.contains(&DDirection::Up));
        assert!(directions.contains(&DDirection::Right));
    }

    #[test]
    fn test_max_depth() {
        let gamestate = read_game_state("requests/failure_49.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let directions = DSimulation::new(state)
            .simulation_max_depth(10)
            .sparse_simulation_distance(6)
            .run();

        println!("{:?}", directions)
    }

    #[test]
    fn test_max_depth_2() {
        let gamestate = read_game_state("requests/failure_50.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let directions = DSimulation::new(state)
            .simulation_max_depth(10)
            .sparse_simulation_distance(6)
            .run();

        assert_eq!(directions.len(), 1);
        assert_eq!(directions[0], DDirection::Left)
    }
}
