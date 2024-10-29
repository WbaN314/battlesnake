use super::d_snake::DSnake;
use crate::{Battlesnake, Board};
use std::cell::Cell;

const SNAKES: usize = 4;

#[derive(Clone)]
pub struct DSnakes {
    snakes: [Cell<DSnake>; SNAKES],
}

impl DSnakes {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let mut d_snakes = [
            Cell::new(DSnake::default()),
            Cell::new(DSnake::default()),
            Cell::new(DSnake::default()),
            Cell::new(DSnake::default()),
        ];
        let mut snake_id = 0;
        for snake in board.snakes.iter() {
            let id = if snake.id == you.id {
                0
            } else {
                snake_id += 1;
                snake_id
            };
            d_snakes[id] = Cell::new(DSnake::from_request(snake, id as u8));
        }
        DSnakes { snakes: d_snakes }
    }

    pub fn cell(&self, id: u8) -> &Cell<DSnake> {
        &self.snakes[id as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::depth_first::game::d_coord::DCoord;

    #[test]
    fn test_from_request() {
        let gamestate = crate::read_game_state("requests/example_move_request.json");
        let d_snakes = DSnakes::from_request(&gamestate.board, &gamestate.you);
        assert_eq!(
            d_snakes.cell(0).get(),
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
            d_snakes.cell(1).get(),
            DSnake::Alive {
                id: 1,
                health: 16,
                length: 4,
                head: DCoord { x: 5, y: 4 },
                tail: DCoord { x: 6, y: 2 },
                stack: 0
            }
        );
        assert_eq!(d_snakes.cell(2).get(), DSnake::NonExistent);
        assert_eq!(d_snakes.cell(3).get(), DSnake::NonExistent);
    }
}
