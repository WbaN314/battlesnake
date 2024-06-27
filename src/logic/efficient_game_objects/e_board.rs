use std::cell::RefCell;

use super::{e_coord::ECoord, e_snakes::SNAKES};

pub const X_SIZE: i8 = 11;
pub const Y_SIZE: i8 = 11;

#[derive(Clone)]
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

    pub fn fill(&mut self, start: &ECoord) -> Option<EArea> {
        let mut area = EArea::new();
        let x = start.x;
        let y = start.y;
        match self.get(x, y) {
            Some(EField::Empty) | Some(EField::Food) => {
                let mut s = Vec::new();
                s.push((x, x, y, 1));
                s.push((x, x, y - 1, -1));
                while let Some((mut x1, x2, y, dy)) = s.pop() {
                    let mut x = x1;
                    match self.get(x, y) {
                        Some(EField::Empty) | Some(EField::Food) => {
                            let mut candidate = self.get(x - 1, y);
                            while candidate == Some(EField::Empty)
                                || candidate == Some(EField::Food)
                            {
                                self.set(x - 1, y, EField::Filled);
                                area.area += 1;
                                x -= 1;
                                candidate = self.get(x - 1, y);
                            }
                            if x < x1 {
                                s.push((x, x1 - 1, y - dy, -dy))
                            }
                        }
                        _ => (),
                    }
                    while x1 <= x2 {
                        let mut candidate = self.get(x1, y);
                        while candidate == Some(EField::Empty) || candidate == Some(EField::Food) {
                            self.set(x1, y, EField::Filled);
                            area.area += 1;
                            x1 += 1;
                            candidate = self.get(x1, y);
                        }
                        if x1 > x {
                            s.push((x, x1 - 1, y + dy, dy));
                        }
                        if x1 - 1 > x2 {
                            s.push((x2 + 1, x1 - 1, y - dy, -dy));
                        }
                        x1 += 1;
                        loop {
                            let candidate = self.get(x1, y);
                            if x1 > x2
                                || candidate == Some(EField::Empty)
                                || candidate == Some(EField::Food)
                            {
                                break;
                            }
                            x1 += 1;
                        }
                        x = x1;
                    }
                }
            }
            _ => return None,
        }
        Some(area)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

// TODO: Try to enclose enemy in areas where opening time > oponent length and size < opponent length
// Evaluate area for own and enemy snake head

impl EArea {
    pub fn new() -> Self {
        Self {
            area: 0,
            opening_times_by_snake: [None; SNAKES as usize],
        }
    }
}
