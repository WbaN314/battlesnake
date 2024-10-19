use crate::logic::shared::{
    e_board::{EBoard, EField},
    e_coord::ECoord,
    e_snakes::SNAKES,
};

impl EBoard {
    pub fn fill(&mut self, start: &ECoord) -> Option<EArea> {
        let mut area = EArea::new();
        let x = start.x;
        let y = start.y;
        match self.get(x, y) {
            Some(EField::Empty) | Some(EField::Food) => {
                let mut s = Vec::new();
                s.push((x, x, y, 1));
                s.push((x, x, y - 1, -1));
                while let Some((mut x1, x2, y, dy)) = s.pop() {
                    let mut x = x1;
                    match self.get(x, y) {
                        Some(EField::Empty) | Some(EField::Food) => {
                            let mut candidate = self.get(x - 1, y);
                            while candidate == Some(EField::Empty)
                                || candidate == Some(EField::Food)
                            {
                                self.set(x - 1, y, EField::Filled);
                                area.area += 1;
                                x -= 1;
                                candidate = self.get(x - 1, y);
                            }
                            if x < x1 {
                                s.push((x, x1 - 1, y - dy, -dy))
                            }
                        }
                        _ => (),
                    }
                    while x1 <= x2 {
                        let mut candidate = self.get(x1, y);
                        while candidate == Some(EField::Empty) || candidate == Some(EField::Food) {
                            self.set(x1, y, EField::Filled);
                            area.area += 1;
                            x1 += 1;
                            candidate = self.get(x1, y);
                        }
                        if x1 > x {
                            s.push((x, x1 - 1, y + dy, dy));
                        }
                        if x1 - 1 > x2 {
                            s.push((x2 + 1, x1 - 1, y - dy, -dy));
                        }
                        x1 += 1;
                        loop {
                            let candidate = self.get(x1, y);
                            if x1 > x2
                                || candidate == Some(EField::Empty)
                                || candidate == Some(EField::Food)
                            {
                                break;
                            }
                            x1 += 1;
                        }
                        x = x1;
                    }
                }
            }
            _ => return None,
        }
        Some(area)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EArea {
    pub area: u8,
    pub opening_times_by_snake: [Option<u8>; SNAKES as usize],
}

// TODO: Try to enclose enemy in areas where opening time > oponent length and size < opponent length
// Evaluate area for own and enemy snake head

impl EArea {
    pub fn new() -> Self {
        Self {
            area: 0,
            opening_times_by_snake: [None; SNAKES as usize],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::shared::{e_coord::ECoord, e_game_state::EGameState},
        read_game_state,
    };

    #[test]
    fn fill_board() {
        let game_state = read_game_state("requests/example_move_request.json");
        let mut state = EGameState::from(&game_state.board, &game_state.you);
        assert!(state.board.clone().fill(&ECoord::from(0, 0)).is_none());
        assert!(state.board.clone().fill(&ECoord::from(-1, 0)).is_none());
        assert_eq!(state.board.fill(&ECoord::from(0, 1)).unwrap().area, 114);
        println!("{state}");
    }

    #[test]
    fn fill_board_2() {
        let game_state = read_game_state("requests/example_move_request_2.json");
        let mut state = EGameState::from(&game_state.board, &game_state.you);
        assert_eq!(state.board.fill(&ECoord::from(0, 1)).unwrap().area, 20);
        println!("{state}");
    }
}
