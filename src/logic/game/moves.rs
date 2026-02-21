use crate::logic::game::{direction::Direction, snakes::SNAKES};

pub type Moves = [Option<Direction>; SNAKES as usize];

#[derive(Debug)]
pub struct MoveMatrix {
    moves: [[bool; 4]; SNAKES as usize],
}

impl MoveMatrix {
    pub fn new(moves: [[bool; 4]; SNAKES as usize]) -> Self {
        Self { moves }
    }

    pub fn get(&self, index: usize) -> [bool; 4] {
        self.moves[index]
    }

    fn len(&self) -> usize {
        self.moves
            .iter()
            .map(|x| x.iter().filter(|y| **y).count())
            .product()
    }

    #[allow(dead_code, reason = "Accessed only via IntoIterator")]
    fn pregenerate(&self) -> Vec<Moves> {
        let mut list: Vec<Moves> = Vec::with_capacity(self.len());

        let mut b_end = 1;
        let mut c_end = 1;
        let mut d_end = 1;
        for i in 0..4 {
            if self.moves[1][i] {
                b_end = 4;
            }
            if self.moves[2][i] {
                c_end = 4;
            }
            if self.moves[3][i] {
                d_end = 4;
            }
        }

        for a in 0..4 {
            for b in 0..b_end {
                for c in 0..c_end {
                    for d in 0..d_end {
                        if self.moves[0][a as usize]
                            && (self.moves[1][b as usize] || b_end == 1)
                            && (self.moves[2][c as usize] || c_end == 1)
                            && (self.moves[3][d as usize] || d_end == 1)
                        {
                            let b_val = if b_end == 1 {
                                None
                            } else {
                                Some(b.try_into().unwrap())
                            };
                            let c_val = if c_end == 1 {
                                None
                            } else {
                                Some(c.try_into().unwrap())
                            };
                            let d_val = if d_end == 1 {
                                None
                            } else {
                                Some(d.try_into().unwrap())
                            };
                            list.push([Some(a.try_into().unwrap()), b_val, c_val, d_val]);
                        }
                    }
                }
            }
        }
        list
    }

    #[allow(dead_code, reason = "Accessed only via IntoIterator")]
    fn generate(&self) -> MoveMatrixIter {
        MoveMatrixIter::new(self.moves)
    }
}

pub struct MoveMatrixIter {
    moves: [[bool; 4]; SNAKES as usize],
    index: usize,
    no_moves: [bool; SNAKES as usize],
}

impl MoveMatrixIter {
    fn new(mut moves: [[bool; 4]; SNAKES as usize]) -> Self {
        let no_moves = [
            false,
            !moves[1].iter().any(|x| *x),
            !moves[2].iter().any(|x| *x),
            !moves[3].iter().any(|x| *x),
        ];

        // if no_moves set first move to true to allow for easier generation
        for i in 0..4 {
            if no_moves[i] {
                moves[i][0] = true;
            }
        }

        Self {
            moves,
            index: 0,
            no_moves,
        }
    }
}

impl Iterator for MoveMatrixIter {
    type Item = Moves;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= u8::MAX as usize {
            let a = (self.index >> 6) & 0b11;
            let b = (self.index >> 4) & 0b11;
            let c = (self.index >> 2) & 0b11;
            let d = (self.index >> 0) & 0b11;
            self.index += 1;

            if (self.moves[0][a as usize])
                && (self.moves[1][b as usize])
                && (self.moves[2][c as usize])
                && (self.moves[3][d as usize])
            {
                return Some([
                    Some(a.try_into().unwrap()),
                    if self.no_moves[1] {
                        None
                    } else {
                        Some(b.try_into().unwrap())
                    },
                    if self.no_moves[2] {
                        None
                    } else {
                        Some(c.try_into().unwrap())
                    },
                    if self.no_moves[3] {
                        None
                    } else {
                        Some(d.try_into().unwrap())
                    },
                ]);
            }
        }
        None
    }
}

impl IntoIterator for MoveMatrix {
    type Item = Moves;
    type IntoIter = std::vec::IntoIter<Moves>;

    fn into_iter(self) -> Self::IntoIter {
        self.pregenerate().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::game::{field::BasicField, game_state::GameState},
        read_game_state,
    };

    #[test]
    fn test_pregenerate() {
        let none = [[false; 4]; SNAKES as usize];
        let no_moves_set = MoveMatrix::new(none);
        let no_moves_list = no_moves_set.pregenerate();
        assert_eq!(no_moves_list.len(), 0);

        let all = [[true; 4]; SNAKES as usize];
        let all_moves_set = MoveMatrix::new(all);
        let all_moves_list = all_moves_set.pregenerate();
        assert_eq!(all_moves_list.len(), 256);
        assert_eq!(
            all_moves_list[0],
            [
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up)
            ]
        );
        assert_eq!(
            all_moves_list[1],
            [
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Down)
            ]
        );
        assert_eq!(
            all_moves_list[4 * 4 + 2 * 4 + 4 - 1],
            [
                Some(Direction::Up),
                Some(Direction::Down),
                Some(Direction::Left),
                Some(Direction::Right)
            ]
        );
        assert_eq!(
            all_moves_list[255],
            [
                Some(Direction::Right),
                Some(Direction::Right),
                Some(Direction::Right),
                Some(Direction::Right)
            ]
        );

        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        let moves_set = state.possible_moves([true, true, true, true]);
        let moves_list = moves_set.pregenerate();

        assert_eq!(moves_list.len(), 36);

        let one_with_no_moves = MoveMatrix::new([
            [true, true, false, true],
            [false, false, false, false],
            [true, false, true, false],
            [true, true, false, true],
        ]);
        let moves_list = one_with_no_moves.pregenerate();
        assert_eq!(moves_list.len(), 3 * 2 * 3);
    }

    #[test]
    fn test_generate() {
        let none = [[false; 4]; SNAKES as usize];
        let no_moves_set = MoveMatrix::new(none);
        let no_moves_list: Vec<Moves> = no_moves_set.generate().collect();
        assert_eq!(no_moves_list.len(), 0);

        let all = [[true; 4]; SNAKES as usize];
        let all_moves_set = MoveMatrix::new(all);
        let all_moves_list: Vec<Moves> = all_moves_set.generate().collect();
        assert_eq!(all_moves_list.len(), 256);
        assert_eq!(
            all_moves_list[0],
            [
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up)
            ]
        );
        assert_eq!(
            all_moves_list[1],
            [
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Down)
            ]
        );
        assert_eq!(
            all_moves_list[4 * 4 + 2 * 4 + 4 - 1],
            [
                Some(Direction::Up),
                Some(Direction::Down),
                Some(Direction::Left),
                Some(Direction::Right)
            ]
        );
        assert_eq!(
            all_moves_list[255],
            [
                Some(Direction::Right),
                Some(Direction::Right),
                Some(Direction::Right),
                Some(Direction::Right)
            ]
        );

        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        let moves_set = state.possible_moves([true, true, true, true]);
        let moves_list: Vec<Moves> = moves_set.generate().collect();

        assert_eq!(moves_list.len(), 36);

        let one_with_no_moves = MoveMatrix::new([
            [true, true, false, true],
            [false, false, false, false],
            [true, false, true, false],
            [true, true, false, true],
        ]);
        let moves_list: Vec<Moves> = one_with_no_moves.generate().collect();
        assert_eq!(moves_list.len(), 3 * 2 * 3);
    }
}

#[cfg(test)]
mod benchmarks {
    use std::hint::black_box;

    use crate::{
        logic::game::{field::BasicField, game_state::GameState},
        read_game_state,
    };

    #[bench]
    fn bench_pregenerate_and_iterate(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{:#?}", state.possible_moves([true, true, true, true]));
        b.iter(|| {
            let moves = state
                .possible_moves(black_box([true, true, true, true]))
                .pregenerate();
            for m in moves {
                black_box(m);
            }
        });
    }

    #[bench]
    fn bench_generate_and_iterate(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{:#?}", state.possible_moves([true, true, true, true]));
        b.iter(|| {
            let moves = state
                .possible_moves(black_box([true, true, true, true]))
                .generate();
            for m in moves {
                black_box(m);
            }
        });
    }
}
