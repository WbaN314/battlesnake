use std::time::Duration;

use crate::{
    logic::efficient_game_objects::{
        e_coord::ECoord, e_direction::EDIRECTION_VECTORS, e_game_state::EGameState,
        e_snakes::SNAKES, e_state_tree::EStateTree,
    },
    Battlesnake, Board, Game,
};

use log::info;

use super::{
    efficient_game_objects::{
        e_board::{EField, X_SIZE, Y_SIZE},
        e_direction::EDirection,
        e_score_board::EScoreBoard,
        e_state_tree::ESimulationState,
    },
    Brain, Direction,
};

#[derive(Debug)]
pub struct Score {
    scores: Vec<i64>,
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

    fn board_weights(&self, game_state: &EGameState) -> EScoreBoard {
        let mut weights = EScoreBoard::new();

        // food
        let food_bonus =
            (100.0 - game_state.snakes.get(0).as_ref().unwrap().health as f64).max(0.0) + 1.0;

        // snake
        let snake_malus = -1.0;

        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match game_state.board.get(x, y) {
                    Some(EField::Food) => {
                        weights.update(x, y, food_bonus);
                    }
                    Some(EField::SnakePart { .. }) => {
                        weights.update(x, y, snake_malus);
                    }
                    _ => (),
                }
            }
        }

        for osi in 1..SNAKES {
            match game_state.snakes.get(osi).as_ref() {
                Some(snake) => {
                    let head = snake.head;
                    if head.x <= 4 && head.y >= 6 {
                        // Top Left
                        weights.update_around(
                            head.x,
                            head.y,
                            vec![
                                vec![-100.0, -50.0, 0.0],
                                vec![-50.0, 0.0, 50.0],
                                vec![0.0, 50.0, 100.0],
                            ],
                        );
                    } else if head.x >= 6 && head.y >= 6 {
                        // Top Right
                        weights.update_around(
                            head.x,
                            head.y,
                            vec![
                                vec![0.0, -50.0, -100.0],
                                vec![50.0, 0.0, -50.0],
                                vec![100.0, 50.0, 0.0],
                            ],
                        );
                    } else if head.x <= 4 && head.y <= 4 {
                        // Bottom Left
                        weights.update_around(
                            head.x,
                            head.y,
                            vec![
                                vec![0.0, 50.0, 100.0],
                                vec![-50.0, 0.0, 50.0],
                                vec![-100.0, -50.0, 0.0],
                            ],
                        );
                    } else if head.x >= 6 && head.y <= 4 {
                        // Bottom Right
                        weights.update_around(
                            head.x,
                            head.y,
                            vec![
                                vec![100.0, 50.0, 0.0],
                                vec![50.0, 0.0, -50.0],
                                vec![0.0, -50.0, -100.0],
                            ],
                        );
                    }
                }
                _ => (),
            }
        }

        weights
    }

    fn evaluate_states(&self, states: &mut [ESimulationState; 4]) -> EDirection {
        // info!("{:#?}", states);
        let mut scores: [Score; 4] = [Score::new(), Score::new(), Score::new(), Score::new()];

        for d in 0..4 {
            let s = &states[d];
            let t = &mut scores[d].scores;

            // movable
            t.push(s.movable as i64);

            // depth
            t.push(s.depth as i64);

            // alive
            t.push(s.alive as i64);

            // snakes
            let mut x = 0;
            for i in 0..s.snake_count.len() {
                x += SNAKES - s.snake_count[i];
            }
            t.push(x as i64);

            // area
            let mut area_score: usize = s.area.area as usize;
            for x in 1..s.area.opening_times_by_snake.len() {
                match s.area.opening_times_by_snake[x] {
                    Some(opening_time) => {
                        area_score += 3 * (10 - opening_time as isize).max(0) as usize;
                        // Rate areas that open up higher to focus on tail chasing
                        // opening time is 0 if tail is in area, otherwise +1 for each step it takes for tail to reach area
                    }
                    _ => (),
                }
            }
            t.push(area_score as i64);

            // board weight close
            t.push(f64::round(s.weight_close * 10.0) as i64);

            // board weight far
            t.push(f64::round(s.weight_far * 10.0) as i64);

            // food
            if let Some(food) = s.food {
                t.push(100 - food as i64);
            } else {
                t.push(0);
            }

            states[d].scores = scores[d].scores.clone();
        }

        EDirection::from_usize(scores.iter().enumerate().max_by_key(|x| x.1).unwrap().0)
    }
}

impl Brain for SmartSnake {
    fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
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
                        .clone()
                        .advanced_fill(&(my_snake_clone.head + EDIRECTION_VECTORS[d]))
                    {
                        simulation_states[d].area = area;
                    }
                }
            }
            Err(_) => (),
        }

        // Movable directions
        for d in 0..4 {
            if simulation_states[d].area.area > 0 {
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

        // Board weights close evaluation
        let mut moved_tails_again = game_state.clone();
        moved_tails_again.move_tails().unwrap();
        let mut board_weights = self.board_weights(&moved_tails_again);
        board_weights = board_weights.convolution(
            vec![
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                vec![1.0, 2.0, 4.0, 2.0, 1.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
            ],
            true,
        );
        for d in 0..4 {
            let candidate = my_snake_clone.head + EDIRECTION_VECTORS[d];
            simulation_states[d].weight_close =
                board_weights.get(candidate.x, candidate.y).unwrap_or(0.0);
        }

        //board weights far evaluation
        for _ in 0..3 {
            board_weights = board_weights.convolution(
                vec![
                    vec![0.0, 0.0, 1.0, 0.0, 0.0],
                    vec![0.0, 1.0, 2.0, 1.0, 0.0],
                    vec![1.0, 2.0, 4.0, 2.0, 1.0],
                    vec![0.0, 1.0, 2.0, 1.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0, 0.0],
                ],
                true,
            );
        }
        for d in 0..4 {
            let candidate = my_snake_clone.head + EDIRECTION_VECTORS[d];
            simulation_states[d].weight_far =
                board_weights.get(candidate.x, candidate.y).unwrap_or(0.0);
        }

        // Evaluate the results
        let result = self.evaluate_states(&mut simulation_states).to_direction();

        let mut s = String::new();
        s.push_str(&format!(
            "Game {} Turn {} Result {} Scores ",
            game.id, turn, result
        ));
        for i in 0..simulation_states[0].scores.len() {
            for d in 0..4 {
                s.push_str(&format!("{} ", simulation_states[d].scores[i]));
            }
            if i < simulation_states[0].scores.len() - 1 {
                s.push_str(format!("| ").as_str());
            }
        }
        info!("{}", s);

        result
    }
}
#[cfg(test)]
mod tests {
    use crate::logic::{
        efficient_game_objects::e_game_state::EGameState, json_requests::read_game_state,
    };

    use super::*;

    #[test]
    fn test_print_gravity() {
        let game_state = read_game_state("requests/failure_17.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let smart_snake = SmartSnake::new();
        let score_board = smart_snake.board_weights(&board);
        println!("{}", &score_board);
        println!("{:?}", &score_board._center_of_gravity());
    }

    #[test]
    fn test_print_convolution() {
        let game_state = read_game_state("requests/failure_9.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let smart_snake = SmartSnake::new();
        board.move_tails().unwrap();
        let score_board = smart_snake.board_weights(&board);
        println!("{}", &score_board);
    }
}
