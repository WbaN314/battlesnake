use core::fmt;
use std::fmt::{Display, Formatter};

use super::{
    e_board::{X_SIZE, Y_SIZE},
    e_coord::ECoord,
};

#[derive(Clone)]
pub struct EScoreBoard([f64; X_SIZE as usize * Y_SIZE as usize]);

impl EScoreBoard {
    pub fn new() -> Self {
        Self([
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
            5.0, 4.0, 3.0, 2.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 3.0,
            4.0, 5.0, 6.0, 7.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0,
            7.0, 6.0, 5.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 2.0, 3.0,
            4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0,
            1.0, 0.0,
        ])
    }

    pub fn _from(i: f64) -> Self {
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
        let mut score_board = EScoreBoard::_from(0.0);
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
