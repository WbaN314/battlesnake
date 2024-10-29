use core::panic;

use crate::logic::legacy::shared::e_snakes::SNAKES;

use super::d_direction::DDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DField {
    Empty { reachable: [u8; SNAKES as usize] },
    Food { reachable: [u8; SNAKES as usize] },
    Snake { id: u8, next: Option<DDirection> },
}

impl DField {
    #[allow(non_snake_case)]
    pub fn Empty() -> Self {
        DField::Empty {
            reachable: [0; SNAKES as usize],
        }
    }

    #[allow(non_snake_case)]
    pub fn Food() -> Self {
        DField::Food {
            reachable: [0; SNAKES as usize],
        }
    }

    #[allow(non_snake_case)]
    pub fn Snake(id: u8, next: Option<DDirection>) -> Self {
        DField::Snake { id, next }
    }

    pub fn reachable(&self, values: [u8; SNAKES as usize]) -> Self {
        match self {
            DField::Empty { .. } => DField::Empty { reachable: values },
            DField::Food { .. } => DField::Food { reachable: values },
            DField::Snake { .. } => panic!("Trying to set reachable on snake field"),
        }
    }
}
