use crate::logic::general::snakes::SNAKES;

use super::direction::Direction;

pub trait Field: Copy {
    fn empty() -> Self;
    fn food() -> Self;
    fn snake(id: u8, next: Option<Direction>) -> Self;
    fn value(&self) -> BasicField;
    fn tile(
        &self,
        up: Option<Self>,
        down: Option<Self>,
        left: Option<Self>,
        right: Option<Self>,
        turn: u8,
    ) -> [[char; 5]; 3] {
        self.value().tile(
            up.map(|f| f.value()),
            down.map(|f| f.value()),
            left.map(|f| f.value()),
            right.map(|f| f.value()),
            turn,
        )
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
    pub fn tile(
        &self,
        up: Option<Self>,
        down: Option<Self>,
        left: Option<Self>,
        right: Option<Self>,
        turn: u8,
    ) -> [[char; 5]; 3] {
        let mut t = [[' '; 5]; 3];
        match *self {
            BasicField::Empty => {
                t[1][2] = '.';
            }
            BasicField::Food => {
                t[1][2] = 'X';
            }
            BasicField::Snake { id, next } => {
                let lc = (b'a' + id) as char;
                let uc = (b'A' + id) as char;
                match next {
                    None => {
                        // head: uppercase letter at center + 4 cardinal neighbors
                        t[1][0] = uc;
                        t[1][2] = uc;
                        t[1][4] = uc;
                        t[0][2] = uc;
                        t[2][2] = uc;
                    }
                    Some(dir) => {
                        t[1][2] = '+';
                        match dir {
                            Direction::Up => {
                                t[0][2] = lc;
                            }
                            Direction::Down => {
                                t[2][2] = lc;
                            }
                            Direction::Left => {
                                t[1][0] = lc;
                            }
                            Direction::Right => {
                                t[1][4] = lc;
                            }
                        }
                    }
                }
                if let Some(neighbor) = up {
                    if matches!(
                        neighbor.value(),
                        BasicField::Snake {
                            next: Some(Direction::Down),
                            ..
                        }
                    ) {
                        t[0][2] = lc;
                    }
                }
                if let Some(neighbor) = down {
                    if matches!(
                        neighbor.value(),
                        BasicField::Snake {
                            next: Some(Direction::Up),
                            ..
                        }
                    ) {
                        t[2][2] = lc;
                    }
                }
                if let Some(neighbor) = left {
                    if matches!(
                        neighbor.value(),
                        BasicField::Snake {
                            next: Some(Direction::Right),
                            ..
                        }
                    ) {
                        t[1][0] = lc;
                    }
                }
                if let Some(neighbor) = right {
                    if matches!(
                        neighbor.value(),
                        BasicField::Snake {
                            next: Some(Direction::Left),
                            ..
                        }
                    ) {
                        t[1][4] = lc;
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
    Empty {
        turn: Option<u8>,
    },
    Food {
        turn: Option<u8>,
    },
    Snake {
        id: u8,
        next: Option<Direction>,
    },
    Filled {
        by: [Option<u8>; SNAKES],
        was_food: bool,
        hot: [Option<u8>; SNAKES],
    },
}

impl FloodFillField {
    pub fn fill(self, id: u8, turn: u8) -> Self {
        match self {
            Self::Empty { .. } => {
                let mut by = [None; SNAKES];
                by[id as usize] = Some(turn);
                let mut hot = [None; SNAKES];
                hot[id as usize] = Some(turn);
                Self::Filled {
                    by,
                    was_food: false,
                    hot,
                }
            }
            Self::Food { .. } => {
                let mut by = [None; SNAKES];
                by[id as usize] = Some(turn);
                let mut hot = [None; SNAKES];
                hot[id as usize] = Some(turn);
                Self::Filled {
                    by,
                    was_food: true,
                    hot,
                }
            }
            field @ Self::Filled { .. } => field.ignite(id, turn),
            _ => panic!("Cannot fill a cell that is already occupied by a snake"),
        }
    }

    pub fn was_food(&self) -> bool {
        match self {
            Self::Filled { was_food, .. } => *was_food,
            _ => panic!("was_food can only be called on Filled fields"),
        }
    }

    pub fn ignite(self, id: u8, turn: u8) -> Self {
        match self {
            Self::Filled { by, was_food, hot } => {
                if let Some(existing_turn) = hot[id as usize] {
                    if turn < existing_turn + 4 || (turn - existing_turn) % 2 != 0 {
                        return self;
                    }
                }

                let mut new_hot = hot;
                new_hot[id as usize] = Some(turn);
                Self::Filled {
                    by,
                    was_food,
                    hot: new_hot,
                }
            }
            _ => panic!("Cannot ignite a cell that is not filled"),
        }
    }

    pub fn tail(turn: u8) -> Self {
        FloodFillField::Empty { turn: Some(turn) }
    }
}

impl Field for FloodFillField {
    fn empty() -> Self {
        FloodFillField::Empty { turn: None }
    }

    fn food() -> Self {
        FloodFillField::Food { turn: None }
    }

    fn snake(id: u8, next: Option<Direction>) -> Self {
        FloodFillField::Snake { id, next }
    }

    fn value(&self) -> BasicField {
        match self {
            FloodFillField::Empty { .. } => BasicField::Empty,
            FloodFillField::Food { .. } => BasicField::Food,
            FloodFillField::Snake { id, next } => BasicField::Snake {
                id: *id,
                next: *next,
            },
            FloodFillField::Filled { .. } => BasicField::Empty,
        }
    }

    fn tile(
        &self,
        up: Option<Self>,
        down: Option<Self>,
        left: Option<Self>,
        right: Option<Self>,
        turn: u8,
    ) -> [[char; 5]; 3] {
        match self {
            FloodFillField::Empty { turn: stored_turn } => {
                if *stored_turn == Some(turn) {
                    let mut t = [[' '; 5]; 3];
                    t[1][2] = 'T';
                    t
                } else {
                    let t = BasicField::Empty.tile(
                        up.map(|f| f.value()),
                        down.map(|f| f.value()),
                        left.map(|f| f.value()),
                        right.map(|f| f.value()),
                        turn,
                    );
                    t
                }    
            }
            FloodFillField::Food { turn: stored_turn } => {
                if *stored_turn == Some(turn) {
                    let mut t = [[' '; 5]; 3];
                    t[1][2] = 'T';
                    t
                } else {
                    let t = BasicField::Food.tile(
                        up.map(|f| f.value()),
                        down.map(|f| f.value()),
                        left.map(|f| f.value()),
                        right.map(|f| f.value()),
                        turn,
                    );
                    t
                } 
            }
            FloodFillField::Snake { id, next } => BasicField::Snake {
                id: *id,
                next: *next,
            }
            .tile(
                up.map(|f| f.value()),
                down.map(|f| f.value()),
                left.map(|f| f.value()),
                right.map(|f| f.value()),
                turn,
            ),
            FloodFillField::Filled { by, hot, .. } => {
                let mut tile = [[' '; 5]; 3];
                let lowest = by.iter().filter_map(|&x| x).min().unwrap();
                let count = by
                    .iter()
                    .filter_map(|&x| x)
                    .filter(|&x| x == lowest)
                    .count();

                if count > 1 {
                    tile[1][2] = '+';
                } else {
                    let id = by.iter().position(|&x| x == Some(lowest)).unwrap() as u8;
                    let lc = (b'a' + id) as char;
                    let uc = (b'A' + id) as char;

                    if hot[id as usize] == Some(turn) {
                        tile[1][2] = uc;
                    } else {
                        tile[1][2] = lc;
                    }

                    if let Some(FloodFillField::Filled { by, .. }) = up {
                        let lowest = by.iter().filter_map(|&x| x).min().unwrap();
                        let count = by
                            .iter()
                            .filter_map(|&x| x)
                            .filter(|&x| x == lowest)
                            .count();
                        let this_id = by.iter().position(|&x| x == Some(lowest)).unwrap() as u8;
                        if count == 1 && this_id == id {
                            tile[0][2] = lc;
                        }
                    }
                    if let Some(FloodFillField::Filled { by, .. }) = down {
                        let lowest = by.iter().filter_map(|&x| x).min().unwrap();
                        let count = by
                            .iter()
                            .filter_map(|&x| x)
                            .filter(|&x| x == lowest)
                            .count();
                        let this_id = by.iter().position(|&x| x == Some(lowest)).unwrap() as u8;
                        if count == 1 && this_id == id {
                            tile[2][2] = lc;
                        }
                    }
                    if let Some(FloodFillField::Filled { by, .. }) = left {
                        let lowest = by.iter().filter_map(|&x| x).min().unwrap();
                        let count = by
                            .iter()
                            .filter_map(|&x| x)
                            .filter(|&x| x == lowest)
                            .count();
                        let this_id = by.iter().position(|&x| x == Some(lowest)).unwrap() as u8;
                        if count == 1 && this_id == id {
                            tile[1][0] = lc;
                        }
                    }
                    if let Some(FloodFillField::Filled { by, .. }) = right {
                        let lowest = by.iter().filter_map(|&x| x).min().unwrap();
                        let count = by
                            .iter()
                            .filter_map(|&x| x)
                            .filter(|&x| x == lowest)
                            .count();
                        let this_id = by.iter().position(|&x| x == Some(lowest)).unwrap() as u8;
                        if count == 1 && this_id == id {
                            tile[1][4] = lc;
                        }
                    }
                }
                tile
            }
        }
    }
}

impl From<BasicField> for FloodFillField {
    fn from(field: BasicField) -> Self {
        match field {
            BasicField::Empty => FloodFillField::Empty { turn: None },
            BasicField::Food => FloodFillField::Food { turn: None },
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
