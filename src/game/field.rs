use super::direction::Direction;
use crate::game::snakes::SNAKES;

pub trait Field: Copy {
    fn empty() -> Self;
    fn food() -> Self;
    fn snake(id: u8, next: Option<Direction>) -> Self;
    fn value(&self) -> BasicField;
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BasicField {
    Empty,
    Food,
    Snake { id: u8, next: Option<Direction> },
}

impl Field for BasicField {
    fn empty() -> Self {
        BasicField::Empty
    }

    fn food() -> Self {
        BasicField::Food
    }

    fn snake(id: u8, next: Option<Direction>) -> Self {
        BasicField::Snake { id, next }
    }

    fn value(&self) -> BasicField {
        *self
    }
}

#[derive(Clone, Copy)]
pub struct BitField(u8);

impl Field for BitField {
    fn empty() -> Self {
        BitField(0b0)
    }

    fn food() -> Self {
        BitField(0b1)
    }

    fn snake(id: u8, next: Option<Direction>) -> Self {
        let mut value = match id {
            0..=3 => 0b10 | (id << 2),
            _ => panic!("Snake id must be between 0 and 3 for BitField"),
        };
        value = match next {
            Some(Direction::Up) => value | 0b0_0000,
            Some(Direction::Down) => value | 0b1_0000,
            Some(Direction::Left) => value | 0b10_0000,
            Some(Direction::Right) => value | 0b11_0000,
            None => value | 0b11,
        };
        BitField(value)
    }

    fn value(&self) -> BasicField {
        match self.0 {
            0b0 => BasicField::Empty,
            0b1 => BasicField::Food,
            0b0011 => BasicField::Snake { id: 0, next: None },
            0b0111 => BasicField::Snake { id: 1, next: None },
            0b1011 => BasicField::Snake { id: 2, next: None },
            0b1111 => BasicField::Snake { id: 3, next: None },
            v => BasicField::Snake {
                id: (v & 0b1100) >> 2,
                next: match v >> 4 {
                    0 => Some(Direction::Up),
                    1 => Some(Direction::Down),
                    2 => Some(Direction::Left),
                    3 => Some(Direction::Right),
                    _ => unreachable!(),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{direction::DIRECTION_LIST};

    use super::*;

    #[test]
    fn test_memory_size_basic_field() {
        assert_eq!(std::mem::size_of::<BasicField>(), 2);
    }

    #[test]
    fn test_memory_size_bit_field() {
        assert_eq!(std::mem::size_of::<BitField>(), 1);
    }

    #[test]
    fn test_bitfield_conversion() {
        let field = BitField::food();
        assert_eq!(field.value(), BasicField::Food);

        let field = BitField::empty();
        assert_eq!(field.value(), BasicField::Empty);

        for id in 0..4 {
            let field = BitField::snake(id, None);
            assert_eq!(field.value(), BasicField::snake(id, None));

            for direction in DIRECTION_LIST {
                let field = BitField::snake(id, Some(direction));
                assert_eq!(
                    field.value(),
                    BasicField::snake(
                        id,
                        Some(direction)
                    )
                );
            }
        }
    }
}

#[cfg(test)]
mod benchmarks {
    use crate::{
        game::{direction::Direction, field::{BasicField, BitField}, game_state::GameState},
        read_game_state,
    };

    #[bench]
    fn bench_next_state_with_basic_field(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = [
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Down),
        ];
        b.iter(|| {
            let mut state = state.clone();
            state.next_state(moves);
        });
    }

    #[bench]
    fn bench_next_state_with_bit_field(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BitField>::from(gamestate);
        println!("{}", state);
        let moves = [
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Down),
        ];
        b.iter(|| {
            let mut state = state.clone();
            state.next_state(moves);
        });
    }
}
