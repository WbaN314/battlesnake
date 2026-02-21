use crate::{
    OriginalBattlesnake, OriginalBoard, OriginalGameState,
    logic::game::{
        coord::Coord,
        field::{self, BasicField, Field},
        snake::Snake,
    },
};
use std::cell::Cell;

pub const HEIGHT: i8 = 11;
pub const WIDTH: i8 = 11;

#[derive(Clone)]
pub struct Board<T: Field> {
    fields: [[Cell<T>; WIDTH as usize]; HEIGHT as usize],
}

impl<T: Field> Board<T> {
    pub fn from_request(board: &OriginalBoard, you: &OriginalBattlesnake) -> Self {
        let d_board = Board::default();
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
            let mut last: Option<Coord> = None;
            for coord in snake.body.iter() {
                let coord: Coord = coord.into();
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
        self.fields
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    pub fn remove_snake(&self, snake: Snake) {
        match snake {
            Snake::Alive {
                id: snake_id,
                mut tail,
                ..
            }
            | Snake::Headless {
                id: snake_id,
                mut tail,
                ..
            } => loop {
                let field = self.cell(tail.x, tail.y).unwrap().get();
                if let BasicField::Snake { id, next } = field.value() {
                    if id == snake_id {
                        self.cell(tail.x, tail.y).unwrap().set(T::empty());
                        if let Some(next) = next {
                            tail += next.into();
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            },
            _ => panic!("Cannot remove snake {:?} from board", snake),
        }
    }
}

impl<T: Field> Default for Board<T> {
    fn default() -> Self {
        Board {
            fields: std::array::from_fn(|_| std::array::from_fn(|_| Cell::new(T::empty()))),
        }
    }
}

impl<T: Field> From<OriginalGameState> for Board<T> {
    fn from(original_game_state: OriginalGameState) -> Self {
        Board::from_request(&original_game_state.board, &original_game_state.you)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::game::{direction::Direction, field::BasicField},
        read_game_state,
    };

    #[test]
    fn test_cell_out_of_bounds() {
        let board = Board::<BasicField>::default();
        assert_eq!(board.cell(-1, 0), None);
        assert_eq!(board.cell(0, -1), None);
        assert_eq!(board.cell(HEIGHT, WIDTH), None);
        assert_eq!(board.cell(WIDTH, 0), None);
        assert_eq!(board.cell(0, HEIGHT), None);
    }

    #[test]
    #[should_panic]
    fn test_cell_panic() {
        let board = Board::default();
        board.cell(HEIGHT, WIDTH).unwrap().set(BasicField::food());
    }

    #[test]
    fn test_from_request() {
        let request = read_game_state("requests/test_game_start.json");
        let board = Board::from_request(&request.board, &request.you);
        assert_eq!(board.cell(0, 0).unwrap().get(), BasicField::empty());
        assert_eq!(board.cell(0, 8).unwrap().get(), BasicField::food());
        assert_eq!(board.cell(2, 0).unwrap().get(), BasicField::food());
        assert_eq!(board.cell(10, 8).unwrap().get(), BasicField::food());
        assert_eq!(board.cell(8, 0).unwrap().get(), BasicField::food());
        assert_eq!(board.cell(5, 5).unwrap().get(), BasicField::food());
        assert_eq!(board.cell(9, 1).unwrap().get(), BasicField::snake(0, None));

        let mut ids = vec![0];

        match board.cell(1, 1).unwrap().get() {
            BasicField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.cell(1, 9).unwrap().get() {
            BasicField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }
        match board.cell(9, 9).unwrap().get() {
            BasicField::Snake { id, .. } => {
                assert!(!ids.contains(&id));
                ids.push(id);
            }
            _ => panic!("Not a snake"),
        }

        for id in 0..=3 {
            assert!(ids.contains(&id), "Snake with id {} not found", id);
        }

        let request = read_game_state("requests/test_move_request.json");
        let board = Board::<BasicField>::from_request(&request.board, &request.you);
        assert_eq!(
            board.cell(0, 0).unwrap().get(),
            BasicField::snake(0, Some(Direction::Up))
        );
        assert_eq!(
            board.cell(1, 0).unwrap().get(),
            BasicField::snake(0, Some(Direction::Left))
        );
        assert_eq!(board.cell(2, 0).unwrap().get(), BasicField::empty());
        assert_eq!(
            board.cell(9, 2).unwrap().get(),
            BasicField::snake(2, Some(Direction::Down))
        );
        assert_eq!(
            board.cell(9, 1).unwrap().get(),
            BasicField::snake(2, Some(Direction::Down))
        );
        assert_eq!(board.cell(9, 0).unwrap().get(), BasicField::snake(2, None));
    }

    #[test]
    fn test_remove_snake() {
        let request = read_game_state("requests/test_move_request.json");
        let board = Board::<BasicField>::from_request(&request.board, &request.you);
        let snake = Snake::Alive {
            id: 0,
            tail: Coord::new(1, 0),
            head: Coord::new(0, 1),
            health: 54,
            length: 3,
            stack: 0,
        };
        board.remove_snake(snake);
        assert_eq!(board.cell(0, 1).unwrap().get(), BasicField::empty());
        assert_eq!(board.cell(0, 0).unwrap().get(), BasicField::empty());
        assert_eq!(board.cell(1, 0).unwrap().get(), BasicField::empty());
    }
}

#[cfg(test)]
mod benchmarks {
    use std::hint::black_box;

    use super::*;
    use crate::read_game_state;

    #[bench]
    fn bench_remove_snake(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/failure_34_follow_own_tail.json");
        let snake = Snake::from_request(&gamestate.board.snakes[1], 1);
        let board = Board::<BasicField>::from(gamestate);
        b.iter(|| {
            let board_clone = board.clone();
            let snake_clone = snake.clone();
            let _ = black_box(black_box(board_clone).remove_snake(black_box(snake_clone)));
        });
    }

    #[bench]
    fn bench_set_field_get_field_via_cell(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/failure_34_follow_own_tail.json");
        let board = Board::<BasicField>::from(gamestate);
        b.iter(|| {
            board
                .cell(black_box(0), black_box(0))
                .map(|cell| cell.set(black_box(BasicField::food())));
            let _ = black_box(board.cell(black_box(0), black_box(0)).unwrap().get());
        });
    }

    #[bench]
    #[ignore = "Baseline comparison, not a real benchmark"]
    fn bench_baseline_comparison(b: &mut test::Bencher) {
        let mut fields: [[BasicField; WIDTH as usize]; HEIGHT as usize] =
            std::array::from_fn(|_| std::array::from_fn(|_| BasicField::empty()));
        b.iter(|| {
            fields[black_box(0)][black_box(0)] = black_box(BasicField::food());
            let _ = black_box(fields[black_box(0)][black_box(0)]);
        });
    }
}
