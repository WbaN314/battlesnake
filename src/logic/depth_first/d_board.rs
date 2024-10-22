use super::{d_coord::DCoord, d_field::DField};
use crate::{Battlesnake, Board};
use std::cell::Cell;

pub const HEIGHT: i8 = 11;
pub const WIDTH: i8 = 11;
pub const SIZE: i8 = HEIGHT * WIDTH;

pub struct DBoard {
    fields: [Cell<DField>; SIZE as usize],
}

impl DBoard {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let d_board = DBoard::default();
        for food in board.food.iter() {
            d_board
                .cell(food.x as i8, food.y as i8)
                .unwrap()
                .set(DField::Food);
        }
        let mut snake_id = 0;
        for snake in board.snakes.iter() {
            let id = if snake.id == you.id {
                0
            } else {
                snake_id += 1;
                snake_id
            };
            let mut last: Option<DCoord> = None;
            for coord in snake.body.iter() {
                let coord: DCoord = coord.into();
                let next = if let Some(last) = last {
                    coord.direction_to(last)
                } else {
                    None
                };
                match d_board.cell(coord.x, coord.y).unwrap().get() {
                    DField::Snake { id: old_id, .. } => {
                        if id != old_id {
                            panic!("Trying to set snake on other snake");
                        }
                        d_board
                            .cell(coord.x, coord.y)
                            .unwrap()
                            .set(DField::Snake { id: id, next });
                    }
                    DField::Empty => {
                        d_board
                            .cell(coord.x, coord.y)
                            .unwrap()
                            .set(DField::Snake { id: id, next });
                    }
                    _ => panic!("Trying to set snake on invalid field"),
                }
                last = Some(coord);
            }
        }
        d_board
    }

    pub fn cell(&self, x: i8, y: i8) -> Option<&Cell<DField>> {
        let index = y as i16 * WIDTH as i16 + x as i16;
        if index < 0 || index >= SIZE as i16 {
            return None;
        }
        self.fields.get(index as usize)
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
    use super::*;
    use crate::{logic::depth_first::d_direction::DDirection, read_game_state};

    #[test]
    fn test_basics() {
        let board = DBoard::default();
        assert_eq!(board.fields.len(), SIZE as usize);
        for field in board.fields.iter() {
            assert_eq!(field.get(), DField::default());
        }
        board.cell(0, 0).unwrap().set(DField::Food);
        assert_eq!(board.cell(0, 0).unwrap().get(), DField::Food);
    }

    #[test]
    fn test_cell_out_of_bounds() {
        let board = DBoard::default();
        assert_eq!(board.cell(-1, 0), None);
        assert_eq!(board.cell(0, -1), None);
        assert_eq!(board.cell(HEIGHT, WIDTH), None);
    }

    #[test]
    #[should_panic]
    fn test_cell_panic() {
        let board = DBoard::default();
        board.cell(HEIGHT, WIDTH).unwrap().set(DField::Food);
    }

    #[test]
    fn test_from_request() {
        let request = read_game_state("requests/example_game_start.json");
        let board = DBoard::from_request(&request.board, &request.you);
        assert_eq!(board.cell(0, 0).unwrap().get(), DField::Empty);
        assert_eq!(board.cell(0, 8).unwrap().get(), DField::Food);
        assert_eq!(board.cell(2, 0).unwrap().get(), DField::Food);
        assert_eq!(board.cell(10, 8).unwrap().get(), DField::Food);
        assert_eq!(board.cell(8, 0).unwrap().get(), DField::Food);
        assert_eq!(board.cell(5, 5).unwrap().get(), DField::Food);
        assert_eq!(
            board.cell(9, 1).unwrap().get(),
            DField::Snake { id: 0, next: None }
        );

        let mut ids = vec![0];

        match board.cell(1, 1).unwrap().get() {
            DField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.cell(1, 9).unwrap().get() {
            DField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.cell(9, 9).unwrap().get() {
            DField::Snake { id, .. } => {
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
            board.cell(0, 0).unwrap().get(),
            DField::Snake { id: 0, next: None }
        );
        assert_eq!(
            board.cell(1, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                next: Some(DDirection::Left)
            }
        );
        assert_eq!(
            board.cell(2, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                next: Some(DDirection::Left)
            }
        );
    }
}
