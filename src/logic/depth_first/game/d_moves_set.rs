use crate::logic::legacy::shared::e_snakes::SNAKES;

use super::{
    d_coord::DCoord,
    d_direction::{DDirection, D_DIRECTION_LIST},
};

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

    pub fn get(&self, index: usize) -> [bool; 4] {
        self.moves[index]
    }

    pub fn generate_sparse(
        &self,
        heads: [Option<DCoord>; SNAKES as usize],
        distance: u8,
    ) -> Vec<DMoves> {
        let mut moves_list = Vec::new();
        for d in 0..4 {
            if self.moves[0][d] {
                let prioritized_moves_matrix =
                    self.prioritized_moves_matrix(heads, d.try_into().unwrap(), distance);
                moves_list.append(&mut self.generate_from_moves(&prioritized_moves_matrix));
            }
        }
        return moves_list;
    }

    pub fn generate(&self) -> Vec<DMoves> {
        self.generate_from_moves(&self.moves)
    }

    fn prioritized_moves_matrix(
        &self,
        heads: [Option<DCoord>; SNAKES as usize],
        direction: DDirection,
        distance: u8,
    ) -> [[bool; 4]; SNAKES as usize] {
        let old_head = heads[0].unwrap();
        let moved_old_head = old_head + direction;
        let mut prioritized_moves_matrix = self.moves;
        prioritized_moves_matrix[0] = [false; 4];
        prioritized_moves_matrix[0][direction as usize] = true;
        for i in 1..4 {
            if let Some(head) = heads[i] {
                if head.distance_to(old_head) > distance {
                    // Sparse matrix adaption
                    let mut priorities = [2, 2, 2, 2];
                    let difference = head - moved_old_head;
                    if difference.x.abs() > difference.y.abs() {
                        if difference.x > 0 {
                            priorities[2] = 0;
                            priorities[3] = 3;
                        } else if difference.x < 0 {
                            priorities[2] = 3;
                            priorities[3] = 0;
                        }
                        if difference.y > 0 {
                            priorities[0] = 2;
                            priorities[1] = 1;
                        } else if difference.y < 0 {
                            priorities[0] = 1;
                            priorities[1] = 2;
                        } else if difference.y == 0 {
                            if direction == DDirection::Up {
                                priorities[0] = 1;
                                priorities[1] = 2;
                            } else if direction == DDirection::Down {
                                priorities[0] = 2;
                                priorities[1] = 1;
                            } else {
                                if rand::random::<bool>() {
                                    priorities[0] = 1;
                                    priorities[1] = 2;
                                } else {
                                    priorities[0] = 2;
                                    priorities[1] = 1;
                                }
                            }
                        }
                    } else if difference.y.abs() > difference.x.abs() {
                        if difference.y > 0 {
                            priorities[0] = 3;
                            priorities[1] = 0;
                        } else if difference.y < 0 {
                            priorities[0] = 0;
                            priorities[1] = 3;
                        }
                        if difference.x > 0 {
                            priorities[2] = 1;
                            priorities[3] = 2;
                        } else if difference.x < 0 {
                            priorities[2] = 2;
                            priorities[3] = 1;
                        } else if difference.x == 0 {
                            if direction == DDirection::Left {
                                priorities[2] = 1;
                                priorities[3] = 2;
                            } else if direction == DDirection::Right {
                                priorities[2] = 2;
                                priorities[3] = 1;
                            } else {
                                if rand::random::<bool>() {
                                    priorities[2] = 1;
                                    priorities[3] = 2;
                                } else {
                                    priorities[2] = 2;
                                    priorities[3] = 1;
                                }
                            }
                        }
                    } else if difference.x.abs() == difference.y.abs() {
                        if direction == DDirection::Up || direction == DDirection::Down {
                            if difference.x > 0 {
                                priorities[0] = 3;
                                priorities[1] = 0;
                            } else {
                                priorities[0] = 0;
                                priorities[1] = 3;
                            }
                            if difference.y > 0 {
                                priorities[2] = 1;
                                priorities[3] = 2;
                            } else {
                                priorities[2] = 2;
                                priorities[3] = 1;
                            }
                        } else {
                            if difference.y > 0 {
                                priorities[0] = 3;
                                priorities[1] = 0;
                            } else {
                                priorities[0] = 0;
                                priorities[1] = 3;
                            }
                            if difference.x > 0 {
                                priorities[2] = 1;
                                priorities[3] = 2;
                            } else {
                                priorities[2] = 2;
                                priorities[3] = 1;
                            }
                        }
                    }
                    let mut priority_direction_order = [DDirection::Up; 4];
                    for i in 0..4 {
                        priority_direction_order[priorities[i]] = D_DIRECTION_LIST[i];
                    }
                    let mut found = false;
                    for d in priority_direction_order {
                        if found {
                            prioritized_moves_matrix[i][d as usize] = false;
                        }
                        if prioritized_moves_matrix[i][d as usize] {
                            found = true;
                        }
                    }
                }
            }
        }

        prioritized_moves_matrix
    }

    fn generate_from_moves(&self, moves: &[[bool; 4]; SNAKES as usize]) -> Vec<DMoves> {
        let prod = moves
            .iter()
            .map(|x| x.iter().filter(|y| **y).count().max(1))
            .product::<usize>();

        let mut list: Vec<DMoves> = Vec::with_capacity(prod);

        let mut b_end = 1;
        let mut c_end = 1;
        let mut d_end = 1;
        for i in 0..4 {
            if moves[1][i] {
                b_end = 4;
            }
            if moves[2][i] {
                c_end = 4;
            }
            if moves[3][i] {
                d_end = 4;
            }
        }

        for a in 0..4 {
            for b in 0..b_end {
                for c in 0..c_end {
                    for d in 0..d_end {
                        if moves[0][a as usize]
                            && (moves[1][b as usize] || b_end == 1)
                            && (moves[2][c as usize] || c_end == 1)
                            && (moves[3][d as usize] || d_end == 1)
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
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::depth_first::game::{d_field::DSlowField, d_game_state::DGameState},
        read_game_state,
    };

    use super::*;

    #[bench]
    #[ignore = "Not actively maintained anymore"]
    fn bench_possible_moves_generate(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{:#?}", state.possible_moves([true, true, true, true]));
        b.iter(|| {
            state.possible_moves([true, true, true, true]).generate();
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
            all_moves_list[4 * 4 + 2 * 4 + 4 - 1],
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
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        let moves_set = state.possible_moves([true, true, true, true]);
        let moves_list = moves_set.generate();

        assert_eq!(moves_list.len(), 36);

        let one_with_no_moves = DMovesSet::new([
            [true, true, false, true],
            [false, false, false, false],
            [true, false, true, false],
            [true, true, false, true],
        ]);
        let moves_list = one_with_no_moves.generate();
        assert_eq!(moves_list.len(), 3 * 2 * 3);
    }

    #[test]
    fn test_generate_sparse() {
        let one_with_no_moves = DMovesSet::new([
            [true, true, false, true],
            [false, false, false, false],
            [true, false, true, false],
            [true, true, false, true],
        ]);

        let move_sparse = one_with_no_moves.generate_sparse(
            [
                Some(DCoord::new(2, 2)),
                Some(DCoord::new(8, 2)),
                Some(DCoord::new(2, 8)),
                Some(DCoord::new(8, 8)),
            ],
            4,
        );
        let moves = one_with_no_moves.generate();

        assert_eq!(moves.len(), 3 * 2 * 3);
        assert_eq!(move_sparse.len(), 3);

        let move_sparse = one_with_no_moves.generate_sparse(
            [
                Some(DCoord::new(2, 2)),
                Some(DCoord::new(8, 2)),
                Some(DCoord::new(3, 6)),
                Some(DCoord::new(5, 3)),
            ],
            4,
        );

        assert_eq!(move_sparse.len(), 3 * 3);
    }
}
