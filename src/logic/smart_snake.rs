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

#[derive(Debug)]
pub struct Score {
    scores: Vec<u32>,
}

impl Score {
    fn new() -> Self {
        Score { scores: Vec::new() }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.scores.len() != other.scores.len() {
            panic!("Vectors have different lengths")
        } else {
            for i in 0..self.scores.len() {
                if self.scores[i] > other.scores[i] {
                    return std::cmp::Ordering::Greater;
                } else if self.scores[i] < other.scores[i] {
                    return std::cmp::Ordering::Less;
                }
            }
            return std::cmp::Ordering::Equal;
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Score {}

pub struct SmartSnake {}

impl SmartSnake {
    pub fn new() -> Self {
        Self {}
    }

    fn evaluate_states(&self, states: [ESimulationState; 4]) -> EDirection {
        info!("{:#?}", states);
        let mut scores: [Score; 4] = [Score::new(), Score::new(), Score::new(), Score::new()];

        for d in 0..4 {
            let s = &states[d];
            let t = &mut scores[d].scores;

            // movable
            t.push(s.movable as u32);

            // depth
            t.push(s.depth as u32);

            // alive
            t.push(s.alive as u32);

            // snakes
            let mut x = 0;
            for i in 0..s.snake_count.len() {
                x += SNAKES - s.snake_count[i];
            }
            t.push(x as u32);

            // area
            t.push(s.area as u32);

            // food
            if let Some(food) = s.food {
                t.push(100 - food as u32);
            } else {
                t.push(0);
            }
        }
        info!("{:?}", scores);

        EDirection::from_usize(scores.iter().enumerate().max_by_key(|x| x.1).unwrap().0)
    }
}

impl Brain for SmartSnake {
    fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
        let distance = 10;
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
