use std::collections::HashSet;

use crate::logic::legacy::shared::{
    e_board::{EField, X_SIZE, Y_SIZE},
    e_coord::ECoord,
    e_direction::EDIRECTION_VECTORS,
    e_game_state::EGameState,
    e_snakes::SNAKES,
};

use super::e_board_extension::EArea;

impl EGameState {
    /// calculate and add opening times to EArea
    ///
    /// fill must have been called before such that board has filled fields
    fn add_opening_times(&self, area: &mut EArea) {
        let mut border_coordinates = HashSet::new();
        let mut relevant_snakes = [false; SNAKES as usize];
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match self.board.get(x, y) {
                    Some(EField::Filled) => {
                        for d_vec in EDIRECTION_VECTORS {
                            let test_x = x + d_vec.x;
                            let test_y = y + d_vec.y;
                            match self.board.get(test_x, test_y) {
                                Some(EField::SnakePart { snake_number, .. }) => {
                                    relevant_snakes[snake_number as usize] = true;
                                    border_coordinates.insert(ECoord::from(test_x, test_y));
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        let mut opening_times: [Option<u8>; SNAKES as usize] = [None; SNAKES as usize];
        for s_index in 0..relevant_snakes.len() {
            if relevant_snakes[s_index] {
                match self.snakes.get(s_index as u8).as_ref() {
                    Some(snake) => {
                        let mut current = snake.tail;
                        let mut time_to_open = 0;
                        loop {
                            match self.board.get(current.x, current.y) {
                                Some(EField::SnakePart { stacked, next, .. }) => {
                                    if border_coordinates.contains(&current) {
                                        opening_times[s_index] = Some(time_to_open);
                                        break;
                                    } else {
                                        time_to_open += stacked as u8;
                                        current = next.unwrap(); // must hit before None as only neighboring snakes are selected
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        area.opening_times_by_snake = opening_times;
    }

    pub fn advanced_fill(&mut self, start: &ECoord) -> Option<EArea> {
        let area = self.board.fill(start);
        match area {
            Some(area) => {
                let mut new_area = area.clone();
                self.add_opening_times(&mut new_area);
                return Some(new_area);
            }
            None => return None,
        }
    }
}
