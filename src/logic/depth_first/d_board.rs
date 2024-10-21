use crate::{Battlesnake, Board};

use super::{d_coord::DCoord, d_field::DField};
use std::cell::Cell;

const HEIGHT: i8 = 11;
const WIDTH: i8 = 11;
const SIZE: i8 = HEIGHT * WIDTH;

pub struct DBoard {
    fields: [Cell<DField>; SIZE as usize],
}

impl DBoard {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let mut d_board = DBoard::default();

        for food in board.food.iter() {
            d_board.set(food.x as i8, food.y as i8, DField::Food);
        }

        let mut snake_id = 1;
        for snake in board.snakes.iter() {
            let mut id = snake_id;
            if snake.id == you.id {
                id = 0;
            }
            let mut last: Option<DCoord> = None;
            for coord in snake.body.iter() {
                let coord: DCoord = coord.into();
                let next = if let Some(last) = last {
                    coord.direction_to(last)
                } else {
                    None
                };
                match d_board.get(coord.x, coord.y) {
                    Some(DField::Snake {
                        id: old_id,
                        stack: old_stack,
                        ..
                    }) => {
                        if id != old_id {
                            panic!("Trying to set snake on other snake");
                        }
                        d_board.set(
                            coord.x,
                            coord.y,
                            DField::Snake {
                                id: id,
                                stack: old_stack + 1,
                                next,
                            },
                        );
                    }
                    Some(DField::Empty) => {
                        d_board.set(
                            coord.x,
                            coord.y,
                            DField::Snake {
                                id: id,
                                stack: 1,
                                next,
                            },
                        );
                    }
                    _ => panic!("Trying to set snake on invalid field"),
                }

                last = Some(coord);
            }
            snake_id += 1;
        }

        d_board
    }

    pub fn get(&self, x: i8, y: i8) -> Option<DField> {
        let position = y as i16 * HEIGHT as i16 + x as i16;
        if position < 0 || position >= SIZE as i16 {
            None
        } else {
            Some(self.fields[position as usize].get())
        }
    }

    pub fn set(&mut self, x: i8, y: i8, field: DField) {
        let position = y * HEIGHT + x;
        self.fields[position as usize].set(field);
    }
}

impl Default for DBoard {
    fn default() -> Self {
        let fields = std::array::from_fn(|_| Cell::new(DField::default()));
        Self { fields }
    }
}

#[cfg(test)]
mod tests {
    use crate::{logic::depth_first::d_direction::DDirection, read_game_state};

    use super::*;

    #[test]
    fn test_board_basics() {
        let mut board = DBoard::default();
        assert_eq!(board.fields.len(), SIZE as usize);
        for field in board.fields.iter() {
            assert_eq!(field.get(), DField::default());
        }
        board.set(0, 0, DField::Food);
        assert_eq!(board.get(0, 0), Some(DField::Food));
    }

    #[test]
    fn test_board_out_of_bounds() {
        let board = DBoard::default();
        assert_eq!(board.get(-1, 0), None);
        assert_eq!(board.get(0, -1), None);
        assert_eq!(board.get(HEIGHT, WIDTH), None);
    }

    #[test]
    #[should_panic]
    fn test_board_panic() {
        let mut board = DBoard::default();
        board.set(HEIGHT, WIDTH, DField::Food);
    }

    #[test]
    fn test_board_from_request() {
        let request = read_game_state("requests/example_game_start.json");
        let board = DBoard::from_request(&request.board, &request.you);
        assert_eq!(board.get(0, 0), Some(DField::Empty));
        assert_eq!(board.get(0, 8), Some(DField::Food));
        assert_eq!(board.get(2, 0), Some(DField::Food));
        assert_eq!(board.get(10, 8), Some(DField::Food));
        assert_eq!(board.get(8, 0), Some(DField::Food));
        assert_eq!(board.get(5, 5), Some(DField::Food));
        assert_eq!(
            board.get(9, 1),
            Some(DField::Snake {
                id: 0,
                stack: 3,
                next: None
            })
        );

        let mut ids = vec![0];

        match board.get(1, 1) {
            Some(DField::Snake { id, .. }) => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.get(1, 9) {
            Some(DField::Snake { id, .. }) => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.get(9, 9) {
            Some(DField::Snake { id, .. }) => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }

        for id in 0..=3 {
            assert!(ids.contains(&id), "Snake with id {} not found", id);
        }

        let request = read_game_state("requests/example_move_request.json");
        let board = DBoard::from_request(&request.board, &request.you);
        assert_eq!(
            board.get(0, 0),
            Some(DField::Snake {
                id: 0,
                stack: 1,
                next: None
            })
        );
        assert_eq!(
            board.get(1, 0),
            Some(DField::Snake {
                id: 0,
                stack: 1,
                next: Some(DDirection::Left)
            })
        );
        assert_eq!(
            board.get(2, 0),
            Some(DField::Snake {
                id: 0,
                stack: 1,
                next: Some(DDirection::Left)
            })
        );
    }
}
