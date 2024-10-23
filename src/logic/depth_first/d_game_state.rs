use core::panic;
use std::fmt::{Display, Formatter};

use crate::{logic::legacy::shared::e_snakes::SNAKES, Battlesnake, Board};

use super::{
    d_board::{DBoard, HEIGHT, WIDTH},
    d_direction::DDirection,
    d_field::DField,
    d_snake::DSnake,
    d_snakes::DSnakes,
};

pub type DMove = Option<DDirection>;
pub type DMoves = [DMove; SNAKES as usize];

pub struct DGameState {
    board: DBoard,
    snakes: DSnakes,
}

impl DGameState {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let snakes = DSnakes::from_request(board, you);
        let d_board = DBoard::from_request(board, you);
        DGameState {
            board: d_board,
            snakes,
        }
    }

    pub fn next(&mut self, moves: DMoves) {
        self.move_tails();
    }

    fn move_tails(&mut self) {
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { stack, .. } | DSnake::Headless { stack, .. } if stack > 0 => {
                    self.snakes.cell(id).set(snake.stack(stack - 1));
                }
                DSnake::Alive { tail, .. } | DSnake::Headless { tail, .. } => {
                    match self.board.cell(tail.x, tail.y).unwrap().get() {
                        DField::Snake {
                            next: Some(direction),
                            ..
                        } => {
                            self.snakes.cell(id).set(snake.tail(tail + direction));
                            self.board.cell(tail.x, tail.y).unwrap().set(DField::Empty);
                        }
                        DField::Snake { next: None, .. } => {
                            self.snakes.cell(id).set(snake.to_vanished());
                            self.board.cell(tail.x, tail.y).unwrap().set(DField::Empty);
                        }
                        _ => {
                            panic!("Snake tail is on invalid field");
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

impl Display for DGameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let row = [' '; WIDTH as usize * 3];
        let mut board = [row; HEIGHT as usize * 3];

        // Write head markers before board
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                DSnake::Alive { head, id, .. } => {
                    let id = (id + 'A' as u8) as char;
                    let x = head.x;
                    let y = head.y;
                    board[y as usize * 3 + 1][x as usize * 3] = id;
                    board[y as usize * 3 + 1][x as usize * 3 + 2] = id;
                    board[y as usize * 3][x as usize * 3 + 1] = id;
                    board[y as usize * 3 + 2][x as usize * 3 + 1] = id;
                    board[y as usize * 3 + 1][x as usize * 3 + 1] = id;
                }
                _ => (),
            }
        }

        // Fill the board with the current state
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.board.cell(x, y).unwrap().get() {
                    DField::Empty => {
                        board[y as usize * 3 + 1][x as usize * 3 + 1] = '.';
                    }
                    DField::Food => {
                        board[y as usize * 3 + 1][x as usize * 3 + 1] = 'X';
                    }
                    DField::Snake { id, next } => {
                        let c = (id + 'a' as u8) as char;
                        board[y as usize * 3 + 1][x as usize * 3 + 1] = '*';
                        match next {
                            Some(DDirection::Up) => {
                                board[y as usize * 3 + 2][x as usize * 3 + 1] = c;
                                board[y as usize * 3 + 3][x as usize * 3 + 1] = c;
                            }
                            Some(DDirection::Down) => {
                                board[y as usize * 3][x as usize * 3 + 1] = c;
                                board[y as usize * 3 - 1][x as usize * 3 + 1] = c;
                            }
                            Some(DDirection::Left) => {
                                board[y as usize * 3 + 1][x as usize * 3] = c;
                                board[y as usize * 3 + 1][x as usize * 3 - 1] = c;
                            }
                            Some(DDirection::Right) => {
                                board[y as usize * 3 + 1][x as usize * 3 + 2] = c;
                                board[y as usize * 3 + 1][x as usize * 3 + 3] = c;
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        // Write tail markers over board
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                DSnake::Alive { tail, stack, .. } | DSnake::Headless { tail, stack, .. } => {
                    board[tail.y as usize * 3 + 1][tail.x as usize * 3 + 1] =
                        (stack + '0' as u8) as char;
                }
                _ => (),
            }
        }

        // Construct the final display string
        let bottom =
            "+---0-----1-----2-----3-----4-----5-----6-----7-----8-----9----10---+\n".to_string();
        let left: Vec<char> = "|0||1||2||3||4||5||6||7||8||9|01|".chars().collect();
        let mut output = bottom.clone();
        for y in (0..board.len()).rev() {
            output.push(left[y]);
            output.push(' ');
            for x in 0..board.len() {
                output.push(board[y][x]);
                output.push(' ');
            }
            output.push(left[y]);
            output.push('\n');
        }
        output.push_str(&bottom);

        let mut other_info = String::from('\n');
        for i in 0..SNAKES {
            match self.snakes.cell(i).get() {
                DSnake::Alive {
                    id, health, length, ..
                } => other_info.push_str(&format!(
                    "Snake {} (Alive) - Health: {}, Length: {}\n",
                    (id + 'A' as u8) as char,
                    health,
                    length
                )),
                DSnake::Headless {
                    id, health, length, ..
                } => other_info.push_str(&format!(
                    "Snake {} (Headless) - Health: {}, Length: {}\n",
                    (id + 'A' as u8) as char,
                    health,
                    length
                )),
                DSnake::Dead { id, .. } => {
                    other_info.push_str(&format!("Snake {} (Dead)\n", (id + 'A' as u8) as char))
                }
                DSnake::Vanished { id, .. } => {
                    other_info.push_str(&format!("Snake {} (Vanished)\n", (id + 'A' as u8) as char))
                }
                DSnake::NonExistent => (),
            }
        }
        output.push_str(&other_info);

        writeln!(f, "{}", output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::depth_first::{
            d_coord::DCoord, d_direction::DDirection, d_field::DField, d_snake::DSnake,
        },
        read_game_state,
    };

    #[test]
    fn test_display() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
    }

    #[test]
    fn test_move_tails() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you);
        state.move_tails();
        assert_eq!(
            state.snakes.cell(0).get(),
            DSnake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: DCoord { x: 0, y: 0 },
                tail: DCoord { x: 2, y: 0 },
                stack: 0
            }
        );
        assert_eq!(
            state.snakes.cell(1).get(),
            DSnake::Alive {
                id: 1,
                health: 16,
                length: 4,
                head: DCoord { x: 5, y: 4 },
                tail: DCoord { x: 6, y: 2 },
                stack: 0
            }
        );
    }

    #[test]
    fn test_from_request() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let d_gamestate = DGameState::from_request(&gamestate.board, &gamestate.you);
        assert_eq!(
            d_gamestate.board.cell(0, 0).unwrap().get(),
            DField::Snake { id: 0, next: None }
        );
        assert_eq!(
            d_gamestate.board.cell(1, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                next: Some(DDirection::Left)
            }
        );
        assert_eq!(
            d_gamestate.board.cell(5, 4).unwrap().get(),
            DField::Snake { id: 1, next: None }
        );
        assert_eq!(
            d_gamestate.snakes.cell(0).get(),
            DSnake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: DCoord { x: 0, y: 0 },
                tail: DCoord { x: 2, y: 0 },
                stack: 0
            }
        );
        assert_eq!(
            d_gamestate.snakes.cell(1).get(),
            DSnake::Alive {
                id: 1,
                health: 16,
                length: 4,
                head: DCoord { x: 5, y: 4 },
                tail: DCoord { x: 6, y: 2 },
                stack: 0
            }
        );
    }
}
