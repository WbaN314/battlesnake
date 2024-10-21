use crate::{Battlesnake, Board};

use super::{d_board::DBoard, d_snakes::DSnakes};

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
    fn test_from_request() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let d_gamestate = DGameState::from_request(&gamestate.board, &gamestate.you);
        assert_eq!(
            d_gamestate.board.cell(0, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                stack: 1,
                next: None
            }
        );
        assert_eq!(
            d_gamestate.board.cell(1, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                stack: 1,
                next: Some(DDirection::Left)
            }
        );
        assert_eq!(
            d_gamestate.board.cell(5, 4).unwrap().get(),
            DField::Snake {
                id: 1,
                stack: 1,
                next: None
            }
        );
        assert_eq!(
            d_gamestate.snakes.cell(0).get(),
            DSnake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: DCoord { x: 0, y: 0 },
                tail: DCoord { x: 2, y: 0 },
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
            }
        );
    }
}
