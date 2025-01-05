use core::fmt;
use std::fmt::{Display, Formatter};

use crate::logic::legacy::shared::{
    e_board::{EField, X_SIZE, Y_SIZE},
    e_coord::ECoord,
    e_direction::EDIRECTION_VECTORS,
    e_game_state::EGameState,
    e_snakes::SNAKES,
};

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

#[derive(Clone)]
pub struct EScoreBoard([f64; X_SIZE as usize * Y_SIZE as usize]);

impl EScoreBoard {
    pub fn new() -> Self {
        Self([
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, // 10
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, // 9
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, // 8
            3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, // 7
            4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, // 6
            5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, // 5
            4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, // 4
            3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, // 3
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, // 2
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, // 1
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, // 0
        ])
    }

    pub fn from(i: f64) -> Self {
        Self([i; X_SIZE as usize * Y_SIZE as usize])
    }

    pub fn set(&mut self, x: i8, y: i8, value: f64) -> bool {
        if x < 0 || x >= X_SIZE || y < 0 || y >= Y_SIZE {
            false
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            self.0[index] = value;
            true
        }
    }

    pub fn get(&self, x: i8, y: i8) -> Option<f64> {
        if x < 0 || x >= X_SIZE || y < 0 || y >= Y_SIZE {
            None
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            Some(self.0[index])
        }
    }

    pub fn update(&mut self, x: i8, y: i8, value: f64) -> bool {
        if x < 0 || x >= X_SIZE || y < 0 || y >= Y_SIZE {
            false
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            self.0[index] += value;
            true
        }
    }

    pub fn update_around(&mut self, x: i8, y: i8, values: &Vec<Vec<f64>>) {
        if values.len() % 2 == 0 || values[0].len() % 2 == 0 {
            panic!("Values must have an odd number of rows and columns");
        }
        let half_rows = values.len() / 2;
        let half_columns = values[0].len() / 2;
        for y_i in 0..values.len() {
            for x_i in 0..values[0].len() {
                self.update(
                    x - half_columns as i8 + x_i as i8,
                    y - half_rows as i8 + y_i as i8,
                    values[values.len() - 1 - y_i][x_i],
                );
            }
        }
    }

    pub fn _center_of_gravity(&self) -> ECoord {
        let mut sum_x: i64 = 0;
        let mut sum_y: i64 = 0;
        let mut sum: i64 = 0;

        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                let value = self.get(x, y).unwrap() as i64;

                sum_x += (x - X_SIZE / 2) as i64 * value;
                sum_y += (y - Y_SIZE / 2) as i64 * value;
                sum += value;
            }
        }
        ECoord::from(
            f64::round(sum_x as f64 / sum as f64) as i8 + (X_SIZE / 2),
            f64::round(sum_y as f64 / sum as f64) as i8 + (Y_SIZE / 2),
        )
    }

    pub fn convolution(&self, kernel: &Vec<Vec<f64>>, normalize: bool) -> Self {
        let mut new_score_board = (*self).clone();
        if kernel.len() % 2 == 0 || kernel[0].len() % 2 == 0 {
            panic!("Kernel must have an odd number of rows and columns");
        }
        let half_rows = kernel.len() / 2;
        let half_columns = kernel[0].len() / 2;
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                let mut new_value = 0.0;
                let mut sum = 0.0;
                for y_i in 0..kernel.len() {
                    for x_i in 0..kernel[0].len() {
                        if let Some(value) = self.get(
                            x - half_columns as i8 + x_i as i8,
                            y - half_rows as i8 + y_i as i8,
                        ) {
                            if normalize {
                                new_value += value as f64 * (kernel[kernel.len() - 1 - y_i][x_i]);
                                sum += kernel[kernel.len() - 1 - y_i][x_i];
                            } else {
                                new_value += value as f64 * kernel[kernel.len() - 1 - y_i][x_i];
                            }
                        }
                    }
                }
                new_score_board.set(
                    x,
                    y,
                    if normalize {
                        new_value / sum
                    } else {
                        new_value
                    },
                );
            }
        }
        new_score_board
    }

    pub fn board_weights(&mut self, game_state: &EGameState, far: bool) {
        let my_snake = game_state.snakes.get(0).as_ref().unwrap().clone();
        let amount_of_alive_snakes = game_state.snakes.count_alive();
        let trajectories = game_state.trajectories();

        let (food_bonus, snake_malus, empty_bonus) = if far {
            // far
            let mut food_bonus = (200.0 - my_snake.health as f64).max(0.0) + 10.0;
            if my_snake.health < 15 {
                food_bonus *= 10.0
            } else if my_snake.health < 10 {
                food_bonus *= 100.0
            }
            let snake_malus = -10.0;
            let empty_bonus = 10.0;
            (food_bonus, snake_malus, empty_bonus)
        } else {
            // close
            let mut food_bonus = (200.0 - my_snake.health as f64).max(0.0) + 10.0;
            if my_snake.health < 15 {
                food_bonus *= 10.0
            } else if my_snake.health < 10 {
                food_bonus *= 100.0
            }
            let snake_malus = 0.0;
            let empty_bonus = 0.0;
            (food_bonus, snake_malus, empty_bonus)
        };

        // Update with food, snake and empty bonuses
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match game_state.board.get(x, y) {
                    Some(EField::Food) => {
                        self.update(x, y, food_bonus);
                    }
                    Some(EField::SnakePart { .. }) => {
                        self.update(x, y, snake_malus);
                    }
                    Some(EField::Empty) => {
                        self.update(x, y, empty_bonus);
                    }
                    _ => (),
                }
            }
        }

        // Other Snake Head Proximity Weights
        if !far {
            // close
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
                        let l = 2;
                        let h = 8;
                        if head.x <= l && head.y >= h {
                            // Top Left
                            self.update_around(head.x, head.y, &top_left);
                        } else if head.x >= h && head.y >= h {
                            // Top Right
                            self.update_around(head.x, head.y, &top_right);
                        } else if head.x <= l && head.y <= l {
                            // Bottom Left
                            self.update_around(head.x, head.y, &bottom_left);
                        } else if head.x >= h && head.y <= l {
                            // Bottom Right
                            self.update_around(head.x, head.y, &bottom_right);
                        } else if head.x <= l {
                            // Left
                            self.update_around(head.x, head.y, &left);
                        } else if head.x >= h {
                            // Right
                            self.update_around(head.x, head.y, &right);
                        } else if head.y <= l {
                            // Bottom
                            self.update_around(head.x, head.y, &bottom);
                        } else if head.y >= h {
                            // Top
                            self.update_around(head.x, head.y, &top);
                        }
                    }
                    _ => (),
                }
            }
        } else {
            // far
            for osi in 1..SNAKES {
                if let Some(other_snake) = game_state.snakes.get(osi).as_ref() {
                    if other_snake.length >= my_snake.length {
                        self.update(other_snake.head.x, other_snake.head.y, -100.0);
                    }
                }
            }
        }

        // avoid trajectory endpoints if multi opponent
        if far && amount_of_alive_snakes > 2 {
            for i in 1..SNAKES {
                if let Some(other_snake) = game_state.snakes.get(i).as_ref() {
                    if let Some(trajectory) = trajectories[i as usize] {
                        let target = game_state.collision_point(other_snake.head, trajectory);
                        self.update(target.x, target.y, -100.0);
                    }
                }
            }
        }

        // avoid edges
        if far {
            self.update(5, 5, 100.0);
            //update edges with -10.0
            for x in 0..X_SIZE {
                self.update(x, 0, -10.0);
                self.update(x, Y_SIZE - 1, -10.0);
            }
            for y in 0..Y_SIZE {
                self.update(0, y, -10.0);
                self.update(X_SIZE - 1, y, -10.0);
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_food_weights(
        &mut self,
        game_state: &EGameState,
        uncontested_food: [Option<(ECoord, u8)>; 4],
    ) {
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
            if length_diff > 1 && my_snake.health > 40 {
                return;
            }

            // compared distance weights
            let mut d = uncontested_food.iter().enumerate().collect::<Vec<_>>();
            d.sort_by(|a, b| {
                a.1.unwrap_or((ECoord::from(0, 0), u8::MAX))
                    .1
                    .cmp(&b.1.unwrap_or((ECoord::from(0, 0), u8::MAX)).1)
            });
            let mut d2 = [0, 0, 0, 0];
            for i in 1..4 {
                d2[d[i].0] = d[i].1.unwrap_or((ECoord::from(0, 0), u8::MAX)).1
                    - d[0].1.unwrap_or((ECoord::from(0, 0), u8::MAX)).1;
            }
            let relative_distance_weight = d2.map(|x| {
                if x == 0 {
                    1.0
                } else if x == 1 {
                    0.5
                } else {
                    0.0
                }
            });

            // change weights
            for d in 0..4 {
                match uncontested_food[d] {
                    Some((_, distance)) => {
                        let new_head = my_snake.head + EDIRECTION_VECTORS[d];
                        let mut weight = (100.0 - my_snake.health as f64).max(0.0)
                            + (25.0 - distance as f64).max(0.0)
                            + (25.0 - my_snake.length as f64).max(0.0);
                        weight *= relative_distance_weight[d];
                        self.update(new_head.x, new_head.y, weight);
                    }
                    _ => (),
                }
            }
        }
    }
}

impl Display for EScoreBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in (0..Y_SIZE).rev() {
            for x in 0..X_SIZE {
                write!(f, "{:<6.1}", self.0[(y * X_SIZE + x) as usize])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_gravity() {
        let mut score_board = EScoreBoard::new();
        println!("{}", &score_board);
        println!("{:?}", score_board._center_of_gravity());
        score_board.update(1, 1, 1000.0);
        println!("{}", &score_board);
        println!("{:?}", score_board._center_of_gravity());
        score_board.update_around(
            1,
            1,
            &vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0],
            ],
        );
        println!("{}", &score_board);
    }

    #[test]
    fn test_print_convolution() {
        let mut score_board = EScoreBoard::from(0.0);
        score_board.set(0, 0, 10.0);
        println!("{}", &score_board);
        score_board = score_board.convolution(
            &vec![
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                vec![1.0, 2.0, 4.0, 2.0, 1.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
            ],
            true,
        );
        println!("{}", &score_board);
        score_board = score_board.convolution(
            &vec![
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                vec![1.0, 2.0, 4.0, 2.0, 1.0],
                vec![0.0, 1.0, 2.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
            ],
            true,
        );
        println!("{}", &score_board);
    }
}
