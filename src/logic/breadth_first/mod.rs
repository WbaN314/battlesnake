use crate::{
    logic::{
        shared::{
            e_board::EField, e_coord::ECoord, e_direction::EDirection,
            e_direction::EDIRECTION_VECTORS, e_game_state::EGameState, e_snakes::SNAKES,
        },
        Brain, Direction,
    },
    Battlesnake, Board, Game,
};
use core::fmt;
use e_score_board::EScoreBoard;
use e_state_tree::EStateTree;
use log::info;
use std::{env, fmt::Display, time::Duration};

mod e_score_board;
mod e_state_node;
mod e_state_tree;

struct Scores {
    scores: Vec<([i64; 4], String)>,
}

impl Scores {
    fn new() -> Self {
        Scores { scores: Vec::new() }
    }

    fn push(&mut self, result: ([i64; 4], String)) {
        self.scores.push(result);
    }

    fn evaluate(&self) -> EDirection {
        let mut viable = [true; 4];
        for i in 0..self.scores.len() {
            let to_beat = self.scores[i]
                .0
                .iter()
                .enumerate()
                .filter(|(i, _)| viable[*i])
                .map(|x| x.1)
                .max();
            for d in 0..4 {
                if self.scores[i].0[d] < *to_beat.unwrap() {
                    viable[d] = false;
                }
            }
        }
        for d in 0..4 {
            if viable[d] {
                return EDirection::from_usize(d);
            }
        }
        panic!("No viable direction found");
    }

    pub fn print_log(&self, game: &Game, turn: &i32, result: EDirection) {
        let mut s = String::new();
        s.push_str(&format!(
            "Game {} Turn {} Result {} Scores ",
            game.id,
            turn,
            result.to_direction()
        ));
        s.push_str(&format!("{}", self));
        if env::var("MODE").unwrap_or("".to_string()) == "test" {
            println!("{}", s);
        } else {
            info!("{}", s);
        }
    }
}

impl Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|")?;
        for score in self.scores.iter() {
            write!(
                f,
                " {} {} {} {} {} |",
                score.1, score.0[0], score.0[1], score.0[2], score.0[3]
            )?;
        }
        Ok(())
    }
}

pub struct BreadthFirstSnake {}

impl BreadthFirstSnake {
    pub fn new() -> Self {
        Self {}
    }

    fn depth_and_alive_and_snakes_and_length(
        &self,
        game_state: &EGameState,
        distance: u8,
        duration: Duration,
    ) -> [([i64; 4], String); 4] {
        let mut d_tree = EStateTree::from(game_state.clone());
        let simulation_states = d_tree.simulate_timed(distance, duration);

        let mut results = [
            ([0; 4], "Depth".to_string()),
            ([0; 4], "Alive".to_string()),
            ([0; 4], "Snakes".to_string()),
            ([0; 4], "Length".to_string()),
        ];
        for d in 0..4 {
            results[0].0[d] = simulation_states[d].depth as i64;
            results[1].0[d] = simulation_states[d].alive as i64;
            results[2].0[d] = -1 * *(simulation_states[d].snake_count.last().unwrap_or(&10)) as i64;
            results[3].0[d] = *(simulation_states[d].my_length.last().unwrap_or(&0)) as i64;
        }
        results
    }

    fn movable(&self, game_state: &EGameState) -> ([i64; 4], String) {
        let mut result = [0; 4];
        let current_head = game_state.snakes.get(0).as_ref().unwrap().clone().head;
        let mut game_state = game_state.clone();
        game_state.move_tails();
        for d in 0..4 {
            let new_head = current_head + EDIRECTION_VECTORS[d];
            match game_state.board.get(new_head.x, new_head.y) {
                Some(EField::Empty) | Some(EField::Food) => result[d] = 1,
                _ => (),
            };
        }
        (result, "Movable".to_string())
    }

    fn areas(&self, game_state: &EGameState) -> ([i64; 4], String) {
        let mut result = [0; 4];
        let my_snake = game_state.snakes.get(0).as_ref().unwrap().clone();
        let mut game_state = game_state.clone();
        game_state.move_tails();
        for d in 0..4 {
            if let Some(area) = game_state
                .clone()
                .advanced_fill(&(my_snake.head + EDIRECTION_VECTORS[d]))
            {
                let min_to_open = area
                    .opening_times_by_snake
                    .iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .min();
                if area.area >= my_snake.length {
                    result[d] = 1;
                } else if let Some(min_to_open) = min_to_open {
                    if area.area >= min_to_open {
                        result[d] = 1;
                    }
                }
            }
        }
        (result, "Area".to_string())
    }

    fn food(&self, board: &Board, game_state: &EGameState) -> ([i64; 4], String) {
        let mut result = [0; 4];
        let my_snake = game_state.snakes.get(0).as_ref().unwrap().clone();
        // Closest food distance that can be reached first
        if board.food.len() > 0 {
            for d in 0..4 {
                let mut closest_uncontested_food_and_distance: Option<(ECoord, u8)> = None;
                let mut e_food_and_distances = Vec::new();
                let start = my_snake.head + EDIRECTION_VECTORS[d];
                for food in board.food.iter() {
                    let e_food = ECoord::from(food.x as i8, food.y as i8);
                    let distance = start.distance(&e_food);
                    e_food_and_distances.push((e_food, distance));
                }
                e_food_and_distances.sort_by(|a, b| a.1.cmp(&b.1));
                for (e_food, distance) in e_food_and_distances {
                    let mut contested = false;
                    for s in 1..SNAKES {
                        if let Some(snake) = game_state.snakes.get(s).as_ref() {
                            let other_distance = snake.head.distance(&e_food) - 1;
                            if (snake.length < my_snake.length && other_distance < distance)
                                || snake.length >= my_snake.length && other_distance <= distance
                            {
                                contested = true;
                            }
                        }
                    }
                    if !contested {
                        closest_uncontested_food_and_distance = Some((e_food, distance));
                        break;
                    }
                }
                result[d] = match closest_uncontested_food_and_distance {
                    Some((_, distance)) => -1 * distance as i64,
                    _ => -100,
                };
            }
        }
        (result, "Food".to_string())
    }

    fn captures(&self, game_state: &EGameState) -> ([i64; 4], String) {
        let mut result = [0; 4];
        let my_length = game_state.snakes.get(0).as_ref().unwrap().length;
        let capture_results = game_state.capture();
        for d in 0..4 {
            if let Some(capture_result) = capture_results[d] {
                if capture_result.fields[0] > my_length {
                    result[d] = 100.max(capture_result.fields[0] as i64);
                } else {
                    result[d] = capture_result.fields[0] as i64;
                }
            } else {
                result[d] = -1;
            }
        }
        (result, "Capture".to_string())
    }

    fn close_weights(&self, game_state: &EGameState) -> ([i64; 4], String) {
        let mut result = [0; 4];
        let my_snake = game_state.snakes.get(0).as_ref().unwrap().clone();
        let mut moved_tails = game_state.clone();
        moved_tails.move_tails();
        let mut board_weights = EScoreBoard::from(0.0);
        board_weights.board_weights(&moved_tails, false);
        board_weights = board_weights.convolution(
            &vec![
                vec![0.0, 1.0, 0.0],
                vec![1.0, 4.0, 1.0],
                vec![0.0, 1.0, 0.0],
            ],
            true,
        );
        if env::var("MODE").unwrap_or("".to_string()) == "test" {
            println!("{}", &board_weights);
        }
        for d in 0..4 {
            let candidate = my_snake.head + EDIRECTION_VECTORS[d];
            result[d] = board_weights.get(candidate.x, candidate.y).unwrap_or(0.0) as i64;
        }
        (result, "Close".to_string())
    }

    fn far_weights(&self, game_state: &EGameState) -> ([i64; 4], String) {
        let mut result = [0; 4];
        let my_snake = game_state.snakes.get(0).as_ref().unwrap().clone();
        let mut board_weights_far = EScoreBoard::new();
        board_weights_far.board_weights(&game_state, true);
        for _ in 0..3 {
            board_weights_far = board_weights_far.convolution(
                &vec![
                    vec![0.0, 0.0, 1.0, 0.0, 0.0],
                    vec![0.0, 1.0, 2.0, 1.0, 0.0],
                    vec![1.0, 2.0, 4.0, 2.0, 1.0],
                    vec![0.0, 1.0, 2.0, 1.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0, 0.0],
                ],
                true,
            );
        }
        if env::var("MODE").unwrap_or("".to_string()) == "test" {
            println!("{}", &board_weights_far);
        }
        for d in 0..4 {
            let candidate = my_snake.head + EDIRECTION_VECTORS[d];
            result[d] = board_weights_far
                .get(candidate.x, candidate.y)
                .unwrap_or(0.0) as i64;
        }

        (result, "Far".to_string())
    }
}

impl Brain for BreadthFirstSnake {
    fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
        let distance = 10;
        let simulate_duration = 200;

        // _chickens.lock().unwrap().insert("Test".to_string(), true);

        let game_state = EGameState::from(board, you);
        let mut scores = Scores::new();

        // movable
        scores.push(self.movable(&game_state));

        // depth and alive
        let depth_and_alive_results = self.depth_and_alive_and_snakes_and_length(
            &game_state,
            distance,
            Duration::from_millis(simulate_duration),
        );
        scores.push(depth_and_alive_results[0].clone()); // Depth
        scores.push(depth_and_alive_results[1].clone()); // Alive
        scores.push(depth_and_alive_results[2].clone()); // Snakes
        scores.push(depth_and_alive_results[3].clone()); // Length

        // areas
        scores.push(self.areas(&game_state));

        // captures
        scores.push(self.captures(&game_state));

        // food
        scores.push(self.food(board, &game_state));

        // close weights
        scores.push(self.close_weights(&game_state));

        // far weights
        scores.push(self.far_weights(&game_state));

        // Evaluate the results
        let result = scores.evaluate();

        // Print the results
        scores.print_log(game, turn, result);

        result.to_direction()
    }
}
