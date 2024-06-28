use std::{env, time::Duration};

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

pub fn mirror_h(v: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        let mut row = Vec::with_capacity(v[0].len());
        for j in 0..v[i].len() {
            row.push(v[i][v[i].len() - j - 1]);
        }
        result.push(row);
    }
    result
}

pub fn mirror_v(v: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        let mut row = Vec::with_capacity(v[0].len());
        for j in 0..v[i].len() {
            row.push(v[v.len() - i - 1][j]);
        }
        result.push(row);
    }
    result
}

pub fn mirror_m(v: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        let mut row = Vec::with_capacity(v[0].len());
        for j in 0..v[i].len() {
            row.push(v[v.len() - i - 1][v[i].len() - j - 1]);
        }
        result.push(row);
    }
    result
}

pub struct SmartSnake {}

impl SmartSnake {
    pub fn new() -> Self {
        Self {}
    }

    fn add_food_weights(
        &self,
        mut weights: EScoreBoard,
        game_state: &EGameState,
        uncontested_food: [Option<(ECoord, u8)>; 4],
    ) -> EScoreBoard {
        // calculate length difference to longest snake
        if let Some(my_snake) = game_state.snakes.get(0).as_ref() {
            let mut length_diff = 0; // positive means mine is longest
            for s in 1..SNAKES {
                if let Some(snake) = game_state.snakes.get(s).as_ref() {
                    let diff = my_snake.length as i32 - snake.length as i32;
                    if diff < length_diff {
                        length_diff = diff;
                    }
                }
            }

            // if I am longest and have enough health, don't go for food
            if length_diff > 3 && my_snake.health > 40 {
                return weights;
            }

            // change weights
            for d in 0..4 {
                match uncontested_food[d] {
                    Some((_, distance)) => {
                        let new_head = my_snake.head + EDIRECTION_VECTORS[d];
                        let weight = (20 - distance).max(0) as f64;
                        weights.update(new_head.x, new_head.y, weight);
                    }
                    _ => (),
                }
            }
        }
        weights
    }

    fn board_weights(&self, mut weights: EScoreBoard, game_state: &EGameState) -> EScoreBoard {
        // food
        let health = game_state.snakes.get(0).as_ref().unwrap().health;
        let mut food_bonus = (100.0 - health as f64).max(0.0) + 10.0;
        if health < 15 {
            food_bonus *= 10.0
        } else if health < 10 {
            food_bonus *= 100.0
        }

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

        // Other Snake Head Proximity Weights
        let top_left = vec![
            vec![0.000, 0.000, 0.000, 0.000, 0.000],
            vec![0.000, -99.0, -50.0, 50.00, 0.000],
            vec![0.000, -50.0, 0.000, 75.00, 0.000],
            vec![0.000, 50.00, 75.00, 99.00, 0.000],
            vec![0.000, 0.000, 0.000, 50.00, 0.000],
        ];
        let top_right = mirror_h(&top_left);
        let bottom_left = mirror_v(&top_left);
        let bottom_right = mirror_m(&top_left);
        let left = vec![
            vec![-50.0, 0.0, 50.0],
            vec![-50.0, 0.0, 50.0],
            vec![-50.0, 0.0, 50.0],
            vec![-50.0, 0.0, 50.0],
            vec![-50.0, 0.0, 50.0],
        ];
        let right = mirror_h(&left);
        let bottom = vec![
            vec![50.00, 50.00, 50.00, 50.00, 50.00],
            vec![0.000, 0.000, 0.000, 0.000, 0.000],
            vec![-50.0, -50.0, -50.0, -50.0, -50.0],
        ];
        let top = mirror_v(&bottom);
        for osi in 1..SNAKES {
            match game_state.snakes.get(osi).as_ref() {
                Some(snake) => {
                    let head = snake.head;
                    if head.x <= 4 && head.y >= 6 {
                        // Top Left
                        weights.update_around(head.x, head.y, &top_left);
                    } else if head.x >= 6 && head.y >= 6 {
                        // Top Right
                        weights.update_around(head.x, head.y, &top_right);
                    } else if head.x <= 4 && head.y <= 4 {
                        // Bottom Left
                        weights.update_around(head.x, head.y, &bottom_left);
                    } else if head.x >= 6 && head.y <= 4 {
                        // Bottom Right
                        weights.update_around(head.x, head.y, &bottom_right);
                    } else if head.x <= 4 {
                        // Left
                        weights.update_around(head.x, head.y, &left);
                    } else if head.x >= 6 {
                        // Right
                        weights.update_around(head.x, head.y, &right);
                    } else if head.y <= 4 {
                        // Bottom
                        weights.update_around(head.x, head.y, &bottom);
                    } else if head.y >= 6 {
                        // Top
                        weights.update_around(head.x, head.y, &top);
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

            // space
            if s.space[0].1 {
                t.push(-1);
            } else {
                t.push(s.space.iter().map(|x| x.1 as i64).sum::<i64>());
            }

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
            t.push(f64::round(s.weight_close) as i64);

            // board weight far
            t.push(f64::round(s.weight_far * 100.0) as i64);

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
        moved_tails.move_tails();

        for d in 0..4 {
            if let Some(area) = moved_tails
                .clone()
                .advanced_fill(&(my_snake_clone.head + EDIRECTION_VECTORS[d]))
            {
                simulation_states[d].area = area;
            }
        }

        // Movable directions
        for d in 0..4 {
            if simulation_states[d].area.area > 0 {
                simulation_states[d].movable = true;
            }
        }

        // Closest food distance that can be reached first
        if board.food.len() > 0 {
            for d in 0..4 {
                let mut closest_uncontested_food_and_distance: Option<(ECoord, u8)> = None;
                let mut e_food_and_distances = Vec::new();
                let start = my_snake_clone.head + EDIRECTION_VECTORS[d];
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
                            if (snake.length < my_snake_clone.length && other_distance < distance)
                                || snake.length >= my_snake_clone.length
                                    && other_distance <= distance
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
                simulation_states[d].food = closest_uncontested_food_and_distance;
            }
        }

        // Board weights close evaluation
        let mut moved_tails = game_state.clone();
        moved_tails.move_tails();
        let mut board_weights = self.board_weights(EScoreBoard::from(0.0), &moved_tails);
        board_weights = self.add_food_weights(
            board_weights,
            &game_state,
            [
                simulation_states[0].food,
                simulation_states[1].food,
                simulation_states[2].food,
                simulation_states[3].food,
            ],
        );
        board_weights = board_weights.convolution(
            &vec![
                vec![0.0, 1.0, 0.0],
                vec![1.0, 4.0, 1.0],
                vec![0.0, 1.0, 0.0],
            ],
            true,
        );
        // println!("{}", &board_weights);
        for d in 0..4 {
            let candidate = my_snake_clone.head + EDIRECTION_VECTORS[d];
            simulation_states[d].weight_close =
                board_weights.get(candidate.x, candidate.y).unwrap_or(0.0);
        }

        //board weights far evaluation
        let mut board_weights_far = self.board_weights(EScoreBoard::new(), &game_state);
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
        for d in 0..4 {
            let candidate = my_snake_clone.head + EDIRECTION_VECTORS[d];
            simulation_states[d].weight_far = board_weights_far
                .get(candidate.x, candidate.y)
                .unwrap_or(0.0);
        }

        // Space evaluation
        for d in 0..4 {
            let mut clone_state = game_state.clone();
            let captures = clone_state.capture(EDirection::from_usize(d));
            for j in 0..SNAKES {
                simulation_states[d].space[j as usize] =
                    (captures.0[j as usize], captures.1[j as usize]);
            }
        }

        // Evaluate the results
        let result = self.evaluate_states(&mut simulation_states).to_direction();

        // Print the results
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

        if env::var("MODE").unwrap_or("".to_string()) == "test" {
            println!("{}", s);
        } else {
            info!("{}", s);
        }

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
        let score_board = smart_snake.board_weights(EScoreBoard::from(0.0), &board);
        println!("{}", &score_board);
        println!("{:?}", &score_board._center_of_gravity());
    }

    #[test]
    fn test_print_convolution() {
        let game_state = read_game_state("requests/failure_28_grab_food.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let smart_snake = SmartSnake::new();
        board.move_tails();
        let mut score_board = smart_snake.board_weights(EScoreBoard::from(0.0), &board);
        println!("{}", &score_board);
        score_board = score_board.convolution(
            &vec![
                vec![0.0, 1.0, 0.0],
                vec![1.0, 4.0, 1.0],
                vec![0.0, 1.0, 0.0],
            ],
            false,
        );
        println!("{}", &score_board);
    }

    #[test]
    fn mirrors() {
        let v = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let v_mirrored_h = mirror_h(&v);
        let v_mirrored_v = mirror_v(&v);
        println!("{:?}", v);
        println!("{:?}", v_mirrored_h);
        println!("{:?}", v_mirrored_v);
        let v_mirrored_h_mirrored_v = mirror_v(&v_mirrored_h);
        let v_mirrored_v_mirrored_h = mirror_h(&v_mirrored_v);
        let v_mirrored_m = mirror_m(&v);
        println!("{:?}", v_mirrored_m);
        assert_eq!(v_mirrored_h_mirrored_v, v_mirrored_v_mirrored_h);
        assert_eq!(v_mirrored_m, v_mirrored_h_mirrored_v);
    }
}
