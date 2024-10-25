use crate::logic::legacy::shared::e_snakes::SNAKES;

use super::d_direction::DDirection;

pub type DMove = Option<DDirection>;
pub type DMoves = [DMove; SNAKES as usize];

#[derive(Debug)]
pub struct DMovesSet {
    moves: [[bool; 4]; SNAKES as usize],
}

impl DMovesSet {
    pub fn new(moves: [[bool; 4]; SNAKES as usize]) -> Self {
        Self { moves }
    }

    pub fn list(&self) -> Vec<DMoves> {
        let mut sums: [u16; SNAKES as usize] = [0; SNAKES as usize];
        for id in 0..SNAKES {
            sums[id as usize] = self.moves[id as usize]
                .iter()
                .map(|y| *y as u16)
                .sum::<u16>()
                .max(1);
        }

        let prod = sums.iter().product::<u16>();

        let mut list: Vec<DMoves> = vec![[None; SNAKES as usize]; prod as usize];

        // 111  digit_rep:   6 3 1 --> # total slots / # pattern_rep / # valids
        // 112  pattern_rep: 1 2 4 --> # previous unique patterns
        // 113  valids:      2 2 3 --> # valid directions
        // 121
        // 122
        // 123
        // 211
        // 212
        // 213
        // 221
        // 222
        // 223

        let mut n_previous_unique_patterns = 1;
        for id in 0..SNAKES {
            for pattern_repetition in 0..n_previous_unique_patterns {
                let mut n_current_directions = 0;
                for good_direction in 0..4 {
                    if self.moves[id as usize][good_direction as usize] {
                        let n_digit_repetitions =
                            prod / (sums[id as usize] * n_previous_unique_patterns);

                        for digit_repetition in 0..n_digit_repetitions {
                            list[(digit_repetition
                                + n_current_directions * n_digit_repetitions
                                + pattern_repetition * prod / n_previous_unique_patterns)
                                as usize][id as usize] = Some(good_direction.try_into().unwrap());
                        }
                        n_current_directions += 1;
                    }
                }
            }
            n_previous_unique_patterns *= sums[id as usize];
        }

        if list == vec![[None; SNAKES as usize]] {
            return vec![];
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use crate::{logic::depth_first::d_game_state::DGameState, read_game_state};

    use super::*;

    #[bench]
    fn bench_list(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        let moves_set = state.possible_moves();
        println!("{:#?}", moves_set);
        b.iter(|| moves_set.list());
    }

    #[test]
    fn test_list() {
        let none = [[false; 4]; SNAKES as usize];
        let no_moves_set = DMovesSet::new(none);
        let no_moves_list = no_moves_set.list();
        println!("{:#?}", no_moves_list);
        assert_eq!(no_moves_list.len(), 0);

        let all = [[true; 4]; SNAKES as usize];
        let all_moves_set = DMovesSet::new(all);
        let all_moves_list = all_moves_set.list();
        println!("{:#?}", all_moves_list);
        assert_eq!(all_moves_list.len(), 256);
        assert_eq!(
            all_moves_list[0],
            [
                Some(DDirection::Up),
                Some(DDirection::Up),
                Some(DDirection::Up),
                Some(DDirection::Up)
            ]
        );
        assert_eq!(
            all_moves_list[1],
            [
                Some(DDirection::Up),
                Some(DDirection::Up),
                Some(DDirection::Up),
                Some(DDirection::Down)
            ]
        );
        assert_eq!(
            all_moves_list[255],
            [
                Some(DDirection::Right),
                Some(DDirection::Right),
                Some(DDirection::Right),
                Some(DDirection::Right)
            ]
        );
    }
}
