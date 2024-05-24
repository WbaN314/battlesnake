use std::time::Duration;

use log::info;

use crate::{
    logic::efficient_game_objects::{
        e_coord::ECoord, e_direction::EDIRECTION_VECTORS, e_game_state::EGameState,
        e_snakes::SNAKES, e_state_tree::EStateTree,
    },
    Battlesnake, Board, Game,
};

use super::{
    efficient_game_objects::{e_direction::EDirection, e_state_tree::ESimulationState},
    Brain, Direction,
};

pub struct SmartSnake {}

impl SmartSnake {
    pub fn new() -> Self {
        Self {}
    }

    fn evaluate_states(&self, states: [ESimulationState; 4]) -> EDirection {
        info!("{:#?}", states);
        let mut scores: [u64; 4] = [0; 4];

        for d in 0..4 {
            let s = &states[d];
            let mut v: u64 = 0;

            // movable
            if s.movable {
                v += 1_000_000_000;
            }

            // depth
            v += 10_000_000 * s.depth as u64;

            // alive
            if s.alive {
                v += 1_000_000;
            }

            // snakes
            v += 100_000 * (SNAKES - s.snake_count.last().unwrap_or(&SNAKES)) as u64;

            // area
            v += 100 * s.area as u64; // area < 1000 -> 100 <= v < 100_000

            // food
            if let Some(food) = s.food {
                v += 99 - food as u64; // 10 < v < 100
            }
            scores[d] = v;
        }
        info!("{:?}", scores);

        EDirection::from_usize(scores.iter().enumerate().max_by_key(|x| x.1).unwrap().0)
    }
}

impl Brain for SmartSnake {
    fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
        let distance = 8;
        let duration = 300;

        let game_state = EGameState::from(board, you);
        let my_snake_clone = game_state.snakes.get(0).clone().unwrap();

        // Simulate future
        let mut d_tree = EStateTree::from(game_state.clone());
        let mut simulation_states =
            d_tree.simulate_timed(distance, Duration::from_millis(duration));

        // Check for areas
        let mut moved_tails = game_state.clone();
        match moved_tails.move_tails() {
            Ok(_) => {
                for d in 0..4 {
                    if let Some(area) = moved_tails
                        .board
                        .clone()
                        .fill(&(my_snake_clone.head + EDIRECTION_VECTORS[d]))
                    {
                        simulation_states[d].area = area.area;
                    }
                }
            }
            Err(_) => (),
        }

        // Movable directions
        for d in 0..4 {
            if simulation_states[d].area > 0 {
                simulation_states[d].movable = true;
            }
        }

        // Closest food distance
        if board.food.len() > 0 {
            for food in board.food.iter() {
                for d in 0..4 {
                    let head_candidate = my_snake_clone.head + EDIRECTION_VECTORS[d];
                    let distance =
                        head_candidate.distance(&ECoord::from(food.x as i8, food.y as i8));
                    if let Some(old_food) = simulation_states[d].food {
                        simulation_states[d].food = Some(distance.min(old_food));
                    } else {
                        simulation_states[d].food = Some(distance);
                    }
                }
            }
        }

        // Evaluate the results
        self.evaluate_states(simulation_states).to_direction()
    }
}
