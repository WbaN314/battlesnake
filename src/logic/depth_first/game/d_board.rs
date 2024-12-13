use super::{d_coord::DCoord, d_direction::DDirection, d_field::DField, d_snake::DSnake};
use crate::{Battlesnake, Board};
use std::cell::Cell;

pub const HEIGHT: i8 = 11;
pub const WIDTH: i8 = 11;
pub const SIZE: i8 = HEIGHT * WIDTH;

#[derive(Clone)]
pub struct DBoard<T: DField> {
    fields: [Cell<T>; SIZE as usize],
}

impl<T: DField> DBoard<T> {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let d_board = DBoard::default();
        for food in board.food.iter() {
            d_board
                .cell(food.x as i8, food.y as i8)
                .unwrap()
                .set(T::food());
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
                let mut next = None;
                if let Some(last) = last {
                    if last == coord {
                        continue; // skip duplicate, is added to snake stack in snakes
                    }
                    next = (last - coord).try_into().ok();
                }
                d_board
                    .cell(coord.x, coord.y)
                    .unwrap()
                    .set(T::snake(id, next));
                last = Some(coord);
            }
        }
        d_board
    }

    pub fn cell(&self, x: i8, y: i8) -> Option<&Cell<T>> {
        let index = y as i16 * WIDTH as i16 + x as i16;
        if x < 0 || y < 0 {
            return None;
        }
        self.fields.get(index as usize)
    }

    pub fn remove_snake(&self, snake: DSnake) {
        match snake {
            DSnake::Alive {
                id: snake_id,
                mut tail,
                ..
            }
            | DSnake::Headless {
                id: snake_id,
                mut tail,
                ..
            } => loop {
                let field = self.cell(tail.x, tail.y).unwrap().get();
                if field.get_type() == T::SNAKE && field.get_id() == snake_id {
                    self.cell(tail.x, tail.y).unwrap().set(T::empty());
                    if let Some(next) = field.get_next() {
                        tail += next.into();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            },
            _ => panic!("Cannot remove snake {:?} from board", snake),
        }
    }
}

impl<T: DField> Default for DBoard<T> {
    fn default() -> Self {
        let fields = std::array::from_fn(|_| Cell::new(T::empty()));
        Self { fields }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::depth_first::game::{d_direction::DDirection, d_field::DSlowField},
        read_game_state,
    };

    #[test]
    fn test_basics() {
        let board = DBoard::default();
        assert_eq!(board.fields.len(), SIZE as usize);
        for field in board.fields.iter() {
            assert_eq!(field.get(), DSlowField::empty());
        }
        board.cell(0, 0).unwrap().set(DSlowField::food());
        assert_eq!(board.cell(0, 0).unwrap().get(), DSlowField::food());
    }

    #[test]
    fn test_cell_out_of_bounds() {
        let board = DBoard::<DSlowField>::default();
        assert_eq!(board.cell(-1, 0), None);
        assert_eq!(board.cell(0, -1), None);
        assert_eq!(board.cell(HEIGHT, WIDTH), None);
    }

    #[test]
    #[should_panic]
    fn test_cell_panic() {
        let board = DBoard::default();
        board.cell(HEIGHT, WIDTH).unwrap().set(DSlowField::food());
    }

    #[test]
    fn test_from_request() {
        let request = read_game_state("requests/test_game_start.json");
        let board = DBoard::from_request(&request.board, &request.you);
        assert_eq!(board.cell(0, 0).unwrap().get(), DSlowField::empty());
        assert_eq!(board.cell(0, 8).unwrap().get(), DSlowField::food());
        assert_eq!(board.cell(2, 0).unwrap().get(), DSlowField::food());
        assert_eq!(board.cell(10, 8).unwrap().get(), DSlowField::food());
        assert_eq!(board.cell(8, 0).unwrap().get(), DSlowField::food());
        assert_eq!(board.cell(5, 5).unwrap().get(), DSlowField::food());
        assert_eq!(board.cell(9, 1).unwrap().get(), DSlowField::snake(0, None));

        let mut ids = vec![0];

        match board.cell(1, 1).unwrap().get() {
            DSlowField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.cell(1, 9).unwrap().get() {
            DSlowField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.cell(9, 9).unwrap().get() {
            DSlowField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }

        for id in 0..=3 {
            assert!(ids.contains(&id), "Snake with id {} not found", id);
        }

        let request = read_game_state("requests/test_move_request.json");
        let board = DBoard::<DSlowField>::from_request(&request.board, &request.you);
        assert_eq!(
            board.cell(0, 0).unwrap().get(),
            DSlowField::snake(0, Some(DDirection::Up))
        );
        assert_eq!(
            board.cell(1, 0).unwrap().get(),
            DSlowField::snake(0, Some(DDirection::Left))
        );
        assert_eq!(
            board.cell(2, 0).unwrap().get(),
            DSlowField::empty()
        );
        assert_eq!(
            board.cell(9, 2).unwrap().get(),
            DSlowField::snake(2, Some(DDirection::Down))
        );
        assert_eq!(
            board.cell(9, 1).unwrap().get(),
            DSlowField::snake(2, Some(DDirection::Down))
        );
        assert_eq!(board.cell(9, 0).unwrap().get(), DSlowField::snake(2, None));
    }

    #[test]
    fn test_remove_snake() {
        let request = read_game_state("requests/test_move_request.json");
        let board = DBoard::<DSlowField>::from_request(&request.board, &request.you);
        let snake = DSnake::Alive {
            id: 0,
            tail: DCoord { x: 1, y: 0 },
            head: DCoord { x: 0, y: 1 },
            health: 54,
            length: 3,
            stack: 0,
        };
        board.remove_snake(snake);
        assert_eq!(board.cell(0, 1).unwrap().get(), DSlowField::empty());
        assert_eq!(board.cell(0, 0).unwrap().get(), DSlowField::empty());
        assert_eq!(board.cell(1, 0).unwrap().get(), DSlowField::empty());
    }
}
