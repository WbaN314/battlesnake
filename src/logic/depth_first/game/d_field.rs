use super::d_direction::DDirection;
use crate::logic::legacy::shared::e_snakes::SNAKES;
use core::panic;

pub trait DField: Copy {
    const EMPTY: u8 = 0;
    const FOOD: u8 = 1;
    const SNAKE: u8 = 2;

    fn empty() -> Self;
    fn food() -> Self;
    fn snake(id: u8, next: Option<DDirection>) -> Self;
    fn get_id(&self) -> u8;
    fn get_next(&self) -> Option<DDirection>;
    fn get_type(&self) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSlowField {
    Empty {
        reachable: [DReached; SNAKES as usize],
    },
    Snake {
        id: u8,
        next: Option<DDirection>,
    },
    Food {
        reachable: [DReached; SNAKES as usize],
    },
}

impl DField for DSlowField {
    fn empty() -> Self {
        DSlowField::Empty {
            reachable: [DReached::default(); SNAKES as usize],
        }
    }

    fn food() -> Self {
        DSlowField::Food {
            reachable: [DReached::default(); SNAKES as usize],
        }
    }

    fn snake(id: u8, next: Option<DDirection>) -> Self {
        DSlowField::Snake { id, next }
    }

    fn get_id(&self) -> u8 {
        match self {
            DSlowField::Snake { id, .. } => *id,
            _ => panic!("Trying to get id from non-snake field"),
        }
    }

    fn get_next(&self) -> Option<DDirection> {
        match self {
            DSlowField::Snake { next, .. } => *next,
            _ => panic!("Trying to get next from non-snake field"),
        }
    }

    fn get_type(&self) -> u8 {
        match self {
            DSlowField::Empty { .. } => DSlowField::EMPTY,
            DSlowField::Food { .. } => DSlowField::FOOD,
            DSlowField::Snake { .. } => DSlowField::SNAKE,
        }
    }
}

impl DSlowField {
    pub fn reachable(&self, values: [DReached; SNAKES as usize]) -> Self {
        match self {
            DSlowField::Empty { .. } => DSlowField::Empty { reachable: values },
            DSlowField::Food { .. } => DSlowField::Food { reachable: values },
            DSlowField::Snake { .. } => panic!("Trying to set reachable on snake field"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DReached {
    before: Option<DDirection>,
    turn: u8,
}

impl DReached {
    pub fn new(before: Option<DDirection>, turn: u8) -> Self {
        DReached { before, turn }
    }

    pub fn is_set(&self) -> bool {
        self.turn > 0
    }

    pub fn turn(&self) -> u8 {
        self.turn
    }

    pub fn before(&self) -> Option<DDirection> {
        self.before
    }
}

impl Default for DReached {
    fn default() -> Self {
        DReached::new(None, 0)
    }
}

impl Ord for DReached {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.turn.cmp(&other.turn)
    }
}

impl PartialOrd for DReached {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for DReached {}

impl PartialEq for DReached {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_size() {
        assert_eq!(std::mem::size_of::<DSlowField>(), 9);
    }
}
