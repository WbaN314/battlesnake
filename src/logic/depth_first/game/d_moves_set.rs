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

    pub fn generate(&self) -> Vec<DMoves> {
        let prod = self
            .moves
            .iter()
            .map(|x| x.iter().filter(|y| **y).count().max(1))
            .product::<usize>();

        let mut list: Vec<DMoves> = Vec::with_capacity(prod);

        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        if self.moves[0][a as usize]
                            && self.moves[1][b as usize]
                            && self.moves[2][c as usize]
                            && self.moves[3][d as usize]
                        {
                            list.push([
                                Some(a.try_into().unwrap()),
                                Some(b.try_into().unwrap()),
                                Some(c.try_into().unwrap()),
                                Some(d.try_into().unwrap()),
                            ]);
                        }
                    }
                }
            }
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use crate::{logic::depth_first::game::d_game_state::DGameState, read_game_state};

    use super::*;

    #[bench]
    // Should be < 45ns
    fn bench_generate(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{:#?}", state.possible_moves());
        b.iter(|| {
            state.possible_moves().generate();
        });
    }

    #[test]
    fn test_generate() {
        let none = [[false; 4]; SNAKES as usize];
        let no_moves_set = DMovesSet::new(none);
        let no_moves_list = no_moves_set.generate();
        assert_eq!(no_moves_list.len(), 0);

        let all = [[true; 4]; SNAKES as usize];
        let all_moves_set = DMovesSet::new(all);
        let all_moves_list = all_moves_set.generate();
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
            all_moves_list[1 * 4 * 4 + 2 * 4 + 4 - 1],
            [
                Some(DDirection::Up),
                Some(DDirection::Down),
                Some(DDirection::Left),
                Some(DDirection::Right)
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

        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        let moves_set = state.possible_moves();
        let moves_list = moves_set.generate();

        assert_eq!(moves_list.len(), 18);
    }
}
