use crate::logic::general::snakes::SNAKES;

use super::direction::Direction;

pub trait Field: Copy {
    fn empty() -> Self;
    fn food() -> Self;
    fn snake(id: u8, next: Option<Direction>) -> Self;
    fn value(&self) -> BasicField;
    fn tile(&self) -> [[char; 9]; 5] {
        self.value().tile()
    }
    fn tile_with_lengths(&self, _lengths: &[u8; SNAKES as usize]) -> [[char; 9]; 5] {
        self.tile()
    }
    fn char_priority() -> &'static [char] {
        &[
            'a', 'b', 'c', 'd', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C',
            'D', 'X', '.', '+', ' ',
        ]
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
                                t[2][2] = lc;
                                t[2][0] = lc;
                            }
                            Direction::Right => {
                                t[2][6] = lc;
                                t[2][8] = lc;
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

#[derive(Clone, Copy, Debug)]
pub enum FloodFillField {
    Empty,
    Food,
    Snake {
        id: u8,
        next: Option<Direction>,
    },
    Filled {
        by: [Option<u8>; SNAKES],
        was_food: bool,
        hot: [bool; SNAKES],
    },
}

impl FloodFillField {
    pub fn fill(self, id: u8, turn: u8) -> Self {
        match self {
            Self::Empty => {
                let mut by = [None; SNAKES];
                by[id as usize] = Some(turn);
                let mut hot = [false; SNAKES];
                hot[id as usize] = true;
                Self::Filled {
                    by,
                    was_food: false,
                    hot,
                }
            }
            Self::Food => {
                let mut by = [None; SNAKES];
                by[id as usize] = Some(turn);
                let mut hot = [false; SNAKES];
                hot[id as usize] = true;
                Self::Filled {
                    by,
                    was_food: true,
                    hot,
                }
            }
            Self::Filled { by, was_food, hot } => {
                let mut new_by = by;
                new_by[id as usize] = Some(turn);
                let mut new_hot = hot;
                new_hot[id as usize] = true;
                Self::Filled {
                    by: new_by,
                    was_food,
                    hot: new_hot,
                }
            }
            _ => panic!("Cannot fill a cell that is already occupied by a snake"),
        }
    }

    pub fn was_food(&self) -> bool {
        match self {
            Self::Filled { was_food, .. } => *was_food,
            _ => panic!("was_food can only be called on Filled fields"),
        }
    }

    pub fn ignite(self, id: u8) -> Self {
        match self {
            Self::Filled { by, was_food, hot } => {
                let mut new_hot = hot;
                new_hot[id as usize] = true;
                Self::Filled { by, was_food, hot: new_hot }
            }
            _ => panic!("Cannot ignite a cell that is not filled"),
        }
    }

    pub fn cool(self) -> Self {
        match self {
            Self::Filled { by, was_food, .. } => {
                let new_hot = [false; SNAKES];
                Self::Filled { by, was_food, hot: new_hot }
            }
            _ => panic!("Cannot cool a cell that is not filled"),
        }
    }
}

impl Field for FloodFillField {
    fn empty() -> Self {
        FloodFillField::Empty
    }

    fn food() -> Self {
        FloodFillField::Food
    }

    fn snake(id: u8, next: Option<Direction>) -> Self {
        FloodFillField::Snake { id, next }
    }

    fn value(&self) -> BasicField {
        match self {
            FloodFillField::Empty => BasicField::Empty,
            FloodFillField::Food => BasicField::Food,
            FloodFillField::Snake { id, next } => BasicField::Snake {
                id: *id,
                next: *next,
            },
            FloodFillField::Filled { .. } => BasicField::Empty,
        }
    }

    fn char_priority() -> &'static [char] {
        &[
            ' ', 'a', 'b', 'c', 'd', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B',
            'C', 'D', 'X', '.', '+',
        ]
    }

    fn tile_with_lengths(&self, lengths: &[u8; SNAKES as usize]) -> [[char; 9]; 5] {
        match self {
            FloodFillField::Empty => {
                let t = BasicField::Empty.tile();
                t
            }
            FloodFillField::Food => {
                let t = BasicField::Food.tile();
                t
            }
            FloodFillField::Snake { id, next } => {
                let mut t = BasicField::Snake {
                    id: *id,
                    next: *next,
                }
                .tile();
                if t[1][4] == ' ' {
                    t[1][4] = '?';
                }
                if t[3][4] == ' ' {
                    t[3][4] = '?';
                }
                if t[0][4] == ' ' {
                    t[0][4] = '?';
                }
                if t[4][4] == ' ' {
                    t[4][4] = '?';
                }
                if t[2][0] == ' ' {
                    t[2][0] = '?';
                }
                if t[2][8] == ' ' {
                    t[2][8] = '?';
                }
                if t[2][2] == ' ' {
                    t[2][2] = '?';
                }
                if t[2][6] == ' ' {
                    t[2][6] = '?';
                }
                t
            }
            FloodFillField::Filled { by, .. } => {
                // Find the snake with the minimum distance (Some(n)), ignoring None slots.
                // Display its letter, '+' if tied, ' ' if all None.
                let mut min_val: Option<u8> = None;
                let mut min_id: u8 = 0;
                let mut count_min: u8 = 0;
                let mut max_len_min_id = None;
                let mut max_len_min_id_count = 0;
                for (i, &v) in by.iter().enumerate() {
                    match (v, min_val) {
                        (Some(val), None) => {
                            min_val = Some(val);
                            min_id = i as u8;
                            count_min = 1;
                            max_len_min_id = Some(i as u8);
                            max_len_min_id_count = 1;
                        }
                        (Some(val), Some(cur)) => {
                            if val < cur {
                                min_val = Some(val);
                                min_id = i as u8;
                                count_min = 1;
                                max_len_min_id = Some(i as u8);
                                max_len_min_id_count = 1;
                            } else if val == cur {
                                count_min += 1;
                                if let Some(current) = max_len_min_id {
                                    if lengths[i] > lengths[current as usize] {
                                        max_len_min_id = Some(i as u8);
                                        max_len_min_id_count = 1;
                                    } else if lengths[i] == lengths[current as usize] {
                                        max_len_min_id_count += 1;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                match count_min {
                    0 => BasicField::Empty.tile(),
                    1 => {
                        let c = (b'a' + min_id) as char;
                        let mut t = [[' '; 9]; 5];
                        t[0][4] = c;
                        t[1][4] = c;
                        t[2][4] = c;
                        t[3][4] = c;
                        t[4][4] = c;
                        t[2][0] = c;
                        t[2][2] = c;
                        t[2][6] = c;
                        t[2][8] = c;
                        t
                    }
                    _ => {
                        let mut t = [[' '; 9]; 5];
                        if max_len_min_id_count == 1 {
                            let c = (b'a' + max_len_min_id.unwrap()) as char;
                            t[2][4] = c;
                        } else {
                            t[2][4] = '+';
                        }
                        t
                    }
                }
            }
        }
    }
}

impl From<BasicField> for FloodFillField {
    fn from(field: BasicField) -> Self {
        match field {
            BasicField::Empty => FloodFillField::Empty,
            BasicField::Food => FloodFillField::Food,
            BasicField::Snake { id, next } => FloodFillField::Snake { id, next },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::general::direction::DIRECTIONS;

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
    use crate::logic::general::direction::Direction;
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
