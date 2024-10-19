use super::e_coord::ECoord;
use std::cell::RefCell;

pub const X_SIZE: i8 = 11;
pub const Y_SIZE: i8 = 11;

#[derive(Clone, Debug)]
pub struct EBoard([RefCell<EField>; X_SIZE as usize * Y_SIZE as usize]);

impl EBoard {
    pub fn new() -> Self {
        Self(std::array::from_fn(|_| RefCell::new(EField::new())))
    }

    pub fn set(&self, x: i8, y: i8, state: EField) -> bool {
        if x < 0 || x >= X_SIZE || y < 0 || y >= Y_SIZE {
            false
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            self.0[index].replace(state);
            true
        }
    }

    pub fn get(&self, x: i8, y: i8) -> Option<EField> {
        if x < 0 || x >= X_SIZE || y < 0 || y >= Y_SIZE {
            None
        } else {
            let index = X_SIZE as usize * y as usize + x as usize;
            Some(self.0[index].borrow().clone())
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
