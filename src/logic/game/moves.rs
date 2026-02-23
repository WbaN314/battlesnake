use rocket::tokio::time::Instant;

use crate::logic::game::{direction::Direction, snakes::SNAKES};

pub type Moves = [Option<Direction>; SNAKES as usize];

#[derive(Debug)]
pub struct MoveMatrix {
    moves: [Option<[bool; 4]>; SNAKES as usize],
}

impl MoveMatrix {
    pub fn new(moves: [Option<[bool; 4]>; SNAKES as usize]) -> Self {
        Self { moves }
    }

    pub fn get(&self, index: usize) -> Option<[bool; 4]> {
        self.moves[index]
    }

    fn len(&self) -> usize {
        self.moves
            .iter()
            .map(|&opt| opt.map_or(1, |arr| arr.iter().filter(|&&b| b).count()))
            .product()
    }

    #[allow(dead_code, reason = "Accessed only via IntoIterator")]
    fn pregenerate(&self) -> Vec<Moves> {
        fn pregenerate_iterations_row(row: Option<[bool; 4]>) -> [Option<Option<Direction>>; 4] {
            if let Some(row) = row {
                let mut template = [None; 4];
                let mut count = 0;
                for (i, &b) in row.iter().enumerate() {
                    if b {
                        template[count] = Some(Some(i.try_into().unwrap()));
                        count += 1;
                    }
                }
                template
            } else {
                [Some(None), None, None, None]
            }
        }
        let mut list: Vec<Moves> = Vec::with_capacity(self.len());

        let iterations = [
            pregenerate_iterations_row(self.moves[0]),
            pregenerate_iterations_row(self.moves[1]),
            pregenerate_iterations_row(self.moves[2]),
            pregenerate_iterations_row(self.moves[3]),
        ];

        let mut template: [Option<Direction>; SNAKES as usize] = Default::default();
        for a in iterations[0] {
            if let Some(a) = a {
                template[0] = a;
            } else {
                break;
            }
            for b in iterations[1] {
                if let Some(b) = b {
                    template[1] = b;
                } else {
                    break;
                }
                for c in iterations[2] {
                    if let Some(c) = c {
                        template[2] = c;
                    } else {
                        break;
                    }
                    for d in iterations[3] {
                        if let Some(d) = d {
                            template[3] = d;
                        } else {
                            break;
                        }
                        list.push(template);
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
    was_none: [bool; SNAKES as usize],
}

impl MoveMatrixIter {
    fn new(moves: [Option<[bool; 4]>; SNAKES as usize]) -> Self {
        let mut was_none = [false; SNAKES as usize];
        let mut moves_array = [[true, false, false, false]; SNAKES as usize];
        for (i, opt) in moves.iter().enumerate() {
            if let &Some(row) = opt {
                moves_array[i] = row;
            } else {
                was_none[i] = true;
            }
        }
        Self {
            moves: moves_array,
            index: 0,
            was_none,
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

            if self.moves[0][a as usize]
                && self.moves[1][b as usize]
                && self.moves[2][c as usize]
                && self.moves[3][d as usize]
            {
                return Some([
                    if self.was_none[0] {
                        None
                    } else {
                        Some(a.try_into().unwrap())
                    },
                    if self.was_none[1] {
                        None
                    } else {
                        Some(b.try_into().unwrap())
                    },
                    if self.was_none[2] {
                        None
                    } else {
                        Some(c.try_into().unwrap())
                    },
                    if self.was_none[3] {
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
        let no_moves = [Some([false; 4]); SNAKES as usize];
        let no_moves_set = MoveMatrix::new(no_moves);
        let no_moves_list = no_moves_set.pregenerate();
        assert_eq!(no_moves_list.len(), 0);

        let none = [None; SNAKES as usize];
        let no_moves_set = MoveMatrix::new(none);
        let no_moves_list: Vec<Moves> = no_moves_set.pregenerate();
        assert_eq!(no_moves_list.len(), 1);
        assert_eq!(no_moves_list[0], [None, None, None, None]);

        let all = [Some([true; 4]); SNAKES as usize];
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
            Some([true, true, false, true]),
            Some([false, false, false, false]),
            Some([true, false, true, false]),
            Some([true, true, false, true]),
        ]);
        let moves_list = one_with_no_moves.pregenerate();
        assert_eq!(moves_list.len(), 0);

        let one_with_none = MoveMatrix::new([
            Some([true, true, false, true]),
            None,
            Some([true, false, true, false]),
            Some([true, true, false, true]),
        ]);
        let moves_list = one_with_none.pregenerate();
        assert_eq!(moves_list.len(), 3 * 2 * 3);
    }

    #[test]
    fn test_generate() {
        let no_moves = [Some([false; 4]); SNAKES as usize];
        let no_moves_set = MoveMatrix::new(no_moves);
        let no_moves_list: Vec<Moves> = no_moves_set.generate().collect();
        assert_eq!(no_moves_list.len(), 0);

        let none = [None; SNAKES as usize];
        let no_moves_set = MoveMatrix::new(none);
        let no_moves_list: Vec<Moves> = no_moves_set.generate().collect();
        assert_eq!(no_moves_list.len(), 1);
        assert_eq!(no_moves_list[0], [None, None, None, None]);

        let all = [Some([true; 4]); SNAKES as usize];
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
            Some([true, true, false, true]),
            Some([false, false, false, false]),
            Some([true, false, true, false]),
            Some([true, true, false, true]),
        ]);
        let moves_list: Vec<Moves> = one_with_no_moves.generate().collect();
        assert_eq!(moves_list.len(), 0);

        let one_with_none = MoveMatrix::new([
            Some([true, true, false, true]),
            None,
            Some([true, false, true, false]),
            Some([true, true, false, true]),
        ]);
        let moves_list: Vec<Moves> = one_with_none.generate().collect();
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
