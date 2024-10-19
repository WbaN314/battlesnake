use super::d_field::DField;
use std::{cell::Cell, fmt::Display};

const HEIGHT: usize = 11;
const WIDTH: usize = 11;
const SIZE: usize = HEIGHT * WIDTH;

pub struct DBoard {
    fields: [Cell<DField>; SIZE],
}

impl DBoard {
    pub fn get(&self, x: usize, y: usize) -> Option<DField> {
        let position = y * HEIGHT + x;
        if position < SIZE {
            Some(self.fields[position].get())
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, field: DField) {
        let position = y * HEIGHT + x;
        self.fields[position].set(field);
    }
}

impl Default for DBoard {
    fn default() -> Self {
        let fields = std::array::from_fn(|_| Cell::new(DField::default()));
        Self { fields }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = DBoard::default();
        assert_eq!(board.fields.len(), SIZE);
    }
}
