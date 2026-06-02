use super::direction::Direction;

pub trait Field: Copy {
    fn empty() -> Self;
    fn food() -> Self;
    fn snake(id: u8, next: Option<Direction>) -> Self;
    fn value(&self) -> BasicField;
    fn tile(&self) -> [[char; 9]; 5] {
        self.value().tile()
    }
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

impl BasicField {
    pub fn tile(&self) -> [[char; 9]; 5] {
        let mut t = [[' '; 9]; 5];
        match *self {
            BasicField::Empty => {
                t[2][4] = '.';
            }
            BasicField::Food => {
                t[2][4] = 'X';
            }
            BasicField::Snake { id, next } => {
                let lc = (b'a' + id) as char;
                let uc = (b'A' + id) as char;
                match next {
                    None => {
                        // head: uppercase letter at center + 4 cardinal neighbors
                        t[2][2] = uc;
                        t[2][4] = uc;
                        t[2][6] = uc;
                        t[1][4] = uc;
                        t[3][4] = uc;
                    }
                    Some(dir) => {
                        t[2][4] = '+';
                        match dir {
                            Direction::Up => {
                                t[0][4] = lc;
                                t[1][4] = lc;
                            }
                            Direction::Down => {
                                t[3][4] = lc;
                                t[4][4] = lc;
                            }
                            Direction::Left => {
                                t[2][3] = lc;
                                t[2][2] = lc;
                            }
                            Direction::Right => {
                                t[2][5] = lc;
                                t[2][6] = lc;
                            }
                        }
                    }
                }
            }
        }
        t
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
    use crate::logic::game::direction::DIRECTIONS;

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

            for direction in DIRECTIONS {
                let field = BitField::snake(id, Some(direction));
                assert_eq!(field.value(), BasicField::snake(id, Some(direction)));
            }
        }
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use crate::logic::game::direction::Direction;
    use std::hint::black_box;

    #[bench]
    fn bench_basic_field(b: &mut test::Bencher) {
        b.iter(|| {
            let field = black_box(BasicField::empty());
            let _ = black_box(field.value());
            let field = black_box(BasicField::food());
            let _ = black_box(field.value());
            let field = black_box(BasicField::snake(black_box(0), black_box(None)));
            let _ = black_box(field.value());
            let field = black_box(BasicField::snake(
                black_box(1),
                black_box(Some(Direction::Up)),
            ));
            let _ = black_box(field.value());
        });
    }

    #[bench]
    fn bench_bit_field(b: &mut test::Bencher) {
        b.iter(|| {
            let field = black_box(BitField::empty());
            let _ = black_box(field.value());
            let field = black_box(BitField::food());
            let _ = black_box(field.value());
            let field = black_box(BitField::snake(black_box(0), black_box(None)));
            let _ = black_box(field.value());
            let field = black_box(BitField::snake(
                black_box(1),
                black_box(Some(Direction::Up)),
            ));
            let _ = black_box(field.value());
        });
    }

    #[bench]
    #[ignore = "Baseline comparison, not a real benchmark"]
    fn bench_baseline_comparison(b: &mut test::Bencher) {
        b.iter(|| {
            let field = black_box(0_u16);
            let _ = black_box(field);
            let field = black_box(1_u16);
            let _ = black_box(field);
            let field = black_box(2_u16);
            let _ = black_box(field);
            let field = black_box(3_u16);
            let _ = black_box(field);
        });
    }
}
