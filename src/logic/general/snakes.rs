use super::snake::Snake;
use crate::{OriginalBattlesnake, OriginalBoard, OriginalGameState};
use std::cell::Cell;

pub const SNAKES: usize = 4;

#[derive(Clone)]
pub struct Snakes {
    snakes: [Cell<Snake>; SNAKES],
}

impl Snakes {
    pub fn from_request(board: &OriginalBoard, you: &OriginalBattlesnake) -> Self {
        let mut d_snakes = [
            Cell::new(Snake::default()),
            Cell::new(Snake::default()),
            Cell::new(Snake::default()),
            Cell::new(Snake::default()),
        ];
        let mut snake_id = 0;
        for snake in board.snakes.iter() {
            let id = if snake.id == you.id {
                0
            } else {
                snake_id += 1;
                snake_id
            };
            d_snakes[id] = Cell::new(Snake::from_request(snake, id as u8));
        }
        Snakes { snakes: d_snakes }
    }

    pub fn from_cells(snakes: [Cell<Snake>; SNAKES]) -> Self {
        Snakes { snakes }
    }

    pub fn cell(&self, id: u8) -> &Cell<Snake> {
        &self.snakes[id as usize]
    }

    pub fn lengths(&self) -> [u8; SNAKES] {
        let mut lengths = [0; SNAKES];
        for i in 0..SNAKES {
            match self.cell(i as u8).get() {
                Snake::Alive { length, .. } => lengths[i] = length,
                Snake::Headless { length, .. } => lengths[i] = length,
                Snake::Vanished { length, .. } => lengths[i] = length,
                _ => (),
            }
        }
        lengths
    }
}

impl From<OriginalGameState> for Snakes {
    fn from(original_game_state: OriginalGameState) -> Self {
        Snakes::from_request(&original_game_state.board, &original_game_state.you)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::general::coord::Coord;

    #[test]
    fn test_memory_size() {
        assert_eq!(std::mem::size_of::<Snakes>(), 36);
    }

    #[test]
    fn test_from_request() {
        let gamestate = crate::read_game_state("requests/example_move_request.json");
        let d_snakes = Snakes::from_request(&gamestate.board, &gamestate.you);
        assert_eq!(
            d_snakes.cell(0).get(),
            Snake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: Coord::new(0, 0),
                tail: Coord::new(2, 0),
                stack: 0
            }
        );
        assert_eq!(
            d_snakes.cell(1).get(),
            Snake::Alive {
                id: 1,
                health: 16,
                length: 3,
                head: Coord::new(5, 3),
                tail: Coord::new(6, 2),
                stack: 0
            }
        );
        assert_eq!(d_snakes.cell(2).get(), Snake::NonExistent);
        assert_eq!(d_snakes.cell(3).get(), Snake::NonExistent);
    }
}
