use crate::{OriginalBattlesnake, logic::game::coord::Coord};
use core::panic;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Snake {
    Alive {
        id: u8,
        health: u8,
        length: u8,
        head: Coord,
        tail: Coord,
        stack: u8,
    },
    Dead {
        id: u8,
    },
    Headless {
        id: u8,
        health: u8,
        length: u8,
        tail: Coord,
        stack: u8,
        last_head: Coord,
    },
    Vanished {
        id: u8,
        length: u8,
        last_head: Coord,
    },
    #[default]
    NonExistent,
}

impl Snake {
    pub fn head(&self, value: Coord) -> Self {
        let mut snake = *self;
        match snake {
            Snake::Alive { ref mut head, .. } => {
                *head = value;
            }
            _ => panic!("Cannot set head on snake {:?}", self),
        };
        snake
    }

    pub fn tail(&self, value: Coord) -> Self {
        let mut snake = *self;
        match snake {
            Snake::Alive { ref mut tail, .. } | Snake::Headless { ref mut tail, .. } => {
                *tail = value;
            }
            _ => panic!("Cannot set tail on snake {:?}", self),
        };
        snake
    }

    pub fn health(&self, value: u8) -> Self {
        let mut snake = *self;
        match snake {
            Snake::Alive { ref mut health, .. } | Snake::Headless { ref mut health, .. } => {
                *health = value;
            }
            _ => panic!("Cannot set health on snake {:?}", self),
        };
        snake
    }

    pub fn stack(&self, value: u8) -> Self {
        let mut snake = *self;
        match snake {
            Snake::Alive { ref mut stack, .. } | Snake::Headless { ref mut stack, .. } => {
                *stack = value;
            }
            _ => panic!("Cannot set stack on snake {:?}", self),
        };
        snake
    }

    pub fn length(&self, value: u8) -> Self {
        let mut snake = *self;
        match snake {
            Snake::Alive { ref mut length, .. } | Snake::Headless { ref mut length, .. } => {
                *length = value;
            }
            _ => panic!("Cannot set length on snake {:?}", self),
        };
        snake
    }

    pub fn to_vanished(&self) -> Self {
        match self {
            Snake::Alive {
                id, length, head, ..
            } => Snake::Vanished {
                id: *id,
                length: *length,
                last_head: *head,
            },
            Snake::Headless {
                id,
                length,
                last_head,
                ..
            } => Snake::Vanished {
                id: *id,
                length: *length,
                last_head: *last_head,
            },
            _ => panic!("Cannot vanish snake {:?}", self),
        }
    }

    pub fn to_headless(&self) -> Self {
        match self {
            Snake::Alive {
                id,
                health,
                length,
                tail,
                stack,
                head,
                ..
            } => Snake::Headless {
                id: *id,
                health: *health,
                length: *length,
                tail: *tail,
                stack: *stack,
                last_head: *head,
            },
            _ => panic!("Cannot make snake headless {:?}", self),
        }
    }

    pub fn to_dead(&self) -> Self {
        match self {
            Snake::Alive { id, .. } => Snake::Dead { id: *id },
            Snake::Headless { id, .. } => Snake::Dead { id: *id },
            _ => panic!("Cannot kill snake {:?}", self),
        }
    }
}

impl Snake {
    pub fn from_request(snake: &OriginalBattlesnake, id: u8) -> Self {
        let head = Coord::from(&snake.head);
        let tail = Coord::from(snake.body.last().unwrap());
        let mut last = snake.body[0];
        let mut stack = 0;
        for coord in snake.body.iter().skip(1) {
            if *coord == last {
                stack += 1;
            }
            last = *coord;
        }
        Snake::Alive {
            id,
            health: snake.health as u8,
            length: snake.body.len() as u8,
            head,
            tail,
            stack,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_game_state;

    #[test]
    fn test_memory_size() {
        assert_eq!(std::mem::size_of::<Snake>(), 9);
    }

    #[test]
    fn test_from_request() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let d_snake = Snake::from_request(&gamestate.you, 0);
        assert_eq!(
            d_snake,
            Snake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: Coord::new(0, 0),
                tail: Coord::new(2, 0),
                stack: 0
            }
        );
    }

    #[test]
    fn test_value_changers() {
        let snake = Snake::Alive {
            id: 0,
            health: 54,
            length: 3,
            head: Coord { x: 0, y: 0 },
            tail: Coord { x: 3, y: 0 },
            stack: 0,
        };
        let snake = snake
            .head(Coord::new(4, 0))
            .tail(Coord::new(0, 0))
            .health(100)
            .stack(1)
            .length(4);
        assert_eq!(
            snake,
            Snake::Alive {
                id: 0,
                health: 100,
                length: 4,
                head: Coord { x: 4, y: 0 },
                tail: Coord { x: 0, y: 0 },
                stack: 1
            }
        );
    }
}
