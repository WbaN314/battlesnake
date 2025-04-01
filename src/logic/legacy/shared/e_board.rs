use super::{e_coord::ECoord, e_snakes::SNAKES};
use std::cell::RefCell;

pub const X_SIZE: i8 = 11;
pub const Y_SIZE: i8 = 11;

#[derive(Clone, Debug)]
pub struct EBoard([RefCell<EField>; X_SIZE as usize * Y_SIZE as usize]);

impl Default for EBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl EBoard {
    pub fn new() -> Self {
        Self(std::array::from_fn(|_| RefCell::new(EField::new())))
    }

    pub fn set(&self, x: i8, y: i8, state: EField) -> bool {
        if !(0..X_SIZE).contains(&x) || !(0..Y_SIZE).contains(&y) {
            false
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            self.0[index].replace(state);
            true
        }
    }

    pub fn get(&self, x: i8, y: i8) -> Option<EField> {
        if !(0..X_SIZE).contains(&x) || !(0..Y_SIZE).contains(&y) {
            None
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            Some(*self.0[index].borrow())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum EField {
    Empty,
    Food,
    SnakePart {
        snake_number: u8,
        stacked: u8,
        next: Option<ECoord>,
    },
    Filled,
    Contested {
        snake_number: u8,
        food: bool,
    },
    Capture {
        snake_number: Option<u8>,
        length: u8,
        changeable: bool,
    },
}

impl EField {
    fn new() -> Self {
        Self::Empty
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EArea {
    pub area: u8,
    pub opening_times_by_snake: [Option<u8>; SNAKES as usize],
}

impl Default for EArea {
    fn default() -> Self {
        Self::new()
    }
}

impl EArea {
    pub fn new() -> Self {
        Self {
            area: 0,
            opening_times_by_snake: [None; SNAKES as usize],
        }
    }
}
