use core::fmt;
use std::cell::{Ref, RefCell, RefMut};

use crate::Battlesnake;

use super::e_coord::ECoord;

pub const SNAKES: u8 = 4;

#[derive(Clone, Debug)]
pub struct ESnake {
    pub number: u8,
    pub head: ECoord,
    pub tail: ECoord,
    pub health: u8,
    pub length: u8,
    pub die: bool,
    pub far_away: bool,
}

impl ESnake {
    pub fn from(snake: &Battlesnake, number: i32) -> Self {
        Self {
            number: number as u8,
            head: ECoord::from(snake.head.x as i8, snake.head.y as i8),
            tail: ECoord::from(
                snake.body.last().unwrap().x as i8,
                snake.body.last().unwrap().y as i8,
            ),
            health: snake.health as u8,
            length: snake.length as u8,
            die: false,
            far_away: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ESnakes([RefCell<Option<ESnake>>; SNAKES as usize]);

impl ESnakes {
    pub fn new() -> Self {
        Self(std::array::from_fn(|_| RefCell::new(None)))
    }

    pub fn set(&self, i: u8, snake: Option<ESnake>) {
        self.0[i as usize].replace(snake);
    }

    pub fn get(&self, i: u8) -> Ref<Option<ESnake>> {
        self.0[i as usize].borrow()
    }

    pub fn get_mut(&self, i: u8) -> RefMut<Option<ESnake>> {
        self.0[i as usize].borrow_mut()
    }

    pub fn count_alive(&self) -> u8 {
        self.0.iter().filter(|x| x.borrow().is_some()).count() as u8
    }
}

pub type Result<T> = std::result::Result<T, ESimulationError>;

#[derive(Debug, Clone)]
pub enum ESimulationError {
    Death,
    Timer,
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct Death;

#[derive(Debug, Clone)]
pub struct Timer;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Death {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "We die.")
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Calculation aborted due to timer.")
    }
}
