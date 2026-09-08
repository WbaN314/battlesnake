use std::ops::Deref;

use crate::logic::general::{
    direction::Direction, field::Field, game_state::GameState, snake::Snake, snakes::SNAKES,
};

pub type Moves = [Option<Direction>; SNAKES as usize];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveVector(Option<[bool; 4]>);

impl MoveVector {
    /// Creates a MoveVector. `None` means the snake is dead/absent.
    /// `Some([...])` must have at least one `true` — a trapped snake defaults to Up.
    pub fn new(moves: Option<[bool; 4]>) -> Self {
        match moves {
            Some(arr) if arr.iter().all(|&b| !b) => MoveVector(Some([true, false, false, false])),
            other => MoveVector(other),
        }
    }

    pub fn is_valid(&self, direction: Direction) -> bool {
        self.0.map_or(false, |arr| arr[direction as usize])
    }

    pub fn count_valid(&self, if_none: usize) -> usize {
        self.0
            .map_or(if_none, |arr| arr.iter().filter(|&&b| b).count())
    }
}

impl Deref for MoveVector {
    type Target = Option<[bool; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MoveVector {
    fn default() -> Self {
        MoveVector(None)
    }
}

impl From<Direction> for MoveVector {
    fn from(direction: Direction) -> Self {
        let mut arr = [false; 4];
        arr[direction as usize] = true;
        MoveVector(Some(arr))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MoveMatrix {
    moves: [MoveVector; SNAKES as usize],
}

impl MoveMatrix {
    pub fn new() -> Self {
        Self {
            moves: [MoveVector::default(); SNAKES as usize],
        }
    }

    pub fn from(moves: [MoveVector; SNAKES as usize]) -> Self {
        Self { moves }
    }

    pub fn get(&self, index: usize) -> MoveVector {
        self.moves[index]
    }

    pub fn set(&mut self, index: usize, moves: MoveVector) {
        self.moves[index] = moves;
    }

    pub fn len(&self) -> usize {
        self.moves.iter().map(|&mv| mv.count_valid(1)).product()
    }

    pub fn prune_head_tail<F: Field>(mut self, gamestate: &GameState<F>, distance: u8) -> Self {
        let own_head = match gamestate.snakes().cell(0).get() {
            Snake::Alive { head, .. } => head,
            _ => return self,
        };

        for id in 1..SNAKES {
            if let Snake::Alive { head, tail, .. } = gamestate.snakes().cell(id as u8).get() {
                if own_head.distance_to(head) > distance && own_head.distance_to(tail) > distance {
                    self.moves[id] = MoveVector::new(None);
                }
            }
        }

        self
    }

    pub fn pregenerate_for(&self, direction: Direction) -> Vec<Moves> {
        let mut moves = self.moves;
        moves[0] = MoveVector::from(direction);
        MoveMatrix::from(moves).pregenerate()
    }

    pub fn pregenerate(&self) -> Vec<Moves> {
        fn pregenerate_iterations_row(row: MoveVector) -> [Option<Option<Direction>>; 4] {
            if let Some(row) = *row {
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
}

impl From<Moves> for MoveMatrix {
    fn from(moves: Moves) -> Self {
        MoveMatrix::from(moves.map(|mv| {
            if let Some(dir) = mv {
                MoveVector::from(dir)
            } else {
                MoveVector::new(None)
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::general::{field::BasicField, game_state::GameState},
        read_game_state,
    };

    #[test]
    fn test_pregenerate() {
        let no_moves = [MoveVector::new(Some([false; 4])); SNAKES as usize];
        let no_moves_set = MoveMatrix::from(no_moves);
        let no_moves_list = no_moves_set.pregenerate();
        // All trapped snakes default to Up
        assert_eq!(no_moves_list.len(), 1);
        assert_eq!(
            no_moves_list[0],
            [
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up),
                Some(Direction::Up)
            ]
        );

        let none = [MoveVector::new(None); SNAKES as usize];
        let no_moves_set = MoveMatrix::from(none);
        let no_moves_list: Vec<Moves> = no_moves_set.pregenerate();
        assert_eq!(no_moves_list.len(), 1);
        assert_eq!(no_moves_list[0], [None, None, None, None]);

        let all = [MoveVector::new(Some([true; 4])); SNAKES as usize];
        let all_moves_set = MoveMatrix::from(all);
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
        let moves_set = state.valid_moves();
        let moves_list = moves_set.pregenerate();
        assert_eq!(moves_list.len(), 36);

        let one_with_no_moves = MoveMatrix::from([
            MoveVector::new(Some([true, true, false, true])),
            MoveVector::new(Some([false, false, false, false])),
            MoveVector::new(Some([true, false, true, false])),
            MoveVector::new(Some([true, true, false, true])),
        ]);
        let moves_list = one_with_no_moves.pregenerate();
        // Trapped snake defaults to Up, so 3 * 1 * 2 * 3 = 18
        assert_eq!(moves_list.len(), 3 * 1 * 2 * 3);

        let one_with_none = MoveMatrix::from([
            MoveVector::new(Some([true, true, false, true])),
            MoveVector::new(None),
            MoveVector::new(Some([true, false, true, false])),
            MoveVector::new(Some([true, true, false, true])),
        ]);
        let moves_list = one_with_none.pregenerate();
        assert_eq!(moves_list.len(), 3 * 2 * 3);
    }

        #[test]
    fn test_prune_head_tail() {
        let gamestate = read_game_state("requests/test_game_start.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);

        let unpruned = state.valid_moves();

        let pruned = unpruned.clone().prune_head_tail(&state, u8::MAX);
        assert_eq!(pruned.get(0), unpruned.get(0));
        assert_eq!(pruned.get(1), unpruned.get(1));
        assert_eq!(pruned.get(2), unpruned.get(2));
        assert_eq!(pruned.get(3), unpruned.get(3));

        let pruned = unpruned.clone().prune_head_tail(&state, 8);
        assert_eq!(pruned.get(0), unpruned.get(0));
        assert_eq!(pruned.get(2), unpruned.get(1));
        assert_eq!(pruned.get(3), unpruned.get(2));
        assert_eq!(pruned.get(1), MoveVector::new(None));
    }
}

#[cfg(test)]
mod benchmarks {
    use std::hint::black_box;

    use crate::{
        logic::general::{field::BasicField, game_state::GameState},
        read_game_state,
    };

    #[bench]
    fn bench_pregenerate_and_iterate(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{:#?}", state.valid_moves());
        b.iter(|| {
            let moves = state.valid_moves().pregenerate();
            for m in moves {
                black_box(m);
            }
        });
    }
}
