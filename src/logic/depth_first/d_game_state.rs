use crate::{Battlesnake, Board};

use super::{d_board::DBoard, d_snakes::DSnakes};

pub struct DGameState {
    board: DBoard,
    snakes: DSnakes,
}

impl DGameState {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let mut d_board = DBoard::from_request(board, you);
        let mut snakes = DSnakes::from_request(board, you);
        DGameState {
            board: d_board,
            snakes,
        }
    }
}
