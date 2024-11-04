use core::panic;

use crate::logic::legacy::shared::e_snakes::SNAKES;

use super::d_direction::DDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DField {
    Empty {
        reachable: [DReached; SNAKES as usize],
    },
    Food {
        reachable: [DReached; SNAKES as usize],
    },
    Snake {
        id: u8,
        next: Option<DDirection>,
    },
}

impl DField {
    #[allow(non_snake_case)]
    pub fn Empty() -> Self {
        DField::Empty {
            reachable: [DReached::default(); SNAKES as usize],
        }
    }

    #[allow(non_snake_case)]
    pub fn Food() -> Self {
        DField::Food {
            reachable: [DReached::default(); SNAKES as usize],
        }
    }

    #[allow(non_snake_case)]
    pub fn Snake(id: u8, next: Option<DDirection>) -> Self {
        DField::Snake { id, next }
    }

    pub fn reachable(&self, values: [DReached; SNAKES as usize]) -> Self {
        match self {
            DField::Empty { .. } => DField::Empty { reachable: values },
            DField::Food { .. } => DField::Food { reachable: values },
            DField::Snake { .. } => panic!("Trying to set reachable on snake field"),
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
