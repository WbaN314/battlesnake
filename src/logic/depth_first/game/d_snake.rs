use core::panic;

use super::d_coord::DCoord;
use crate::OriginalBattlesnake;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DSnake {
    Alive {
        id: u8,
        health: u8,
        length: u8,
        head: DCoord,
        tail: DCoord,
        stack: u8,
    },
    Dead {
        id: u8,
    },
    Headless {
        id: u8,
        health: u8,
        length: u8,
        tail: DCoord,
        stack: u8,
        last_head: DCoord,
    },
    Vanished {
        id: u8,
        length: u8,
        last_head: DCoord,
    },
    #[default]
    NonExistent,
}

impl DSnake {
    pub fn head(&self, value: DCoord) -> Self {
        let mut snake = *self;
        match snake {
            DSnake::Alive { ref mut head, .. } => {
                *head = value;
            }
            _ => panic!("Cannot set head on snake {:?}", self),
        };
        snake
    }

    pub fn tail(&self, value: DCoord) -> Self {
        let mut snake = *self;
        match snake {
            DSnake::Alive { ref mut tail, .. } | DSnake::Headless { ref mut tail, .. } => {
                *tail = value;
            }
            _ => panic!("Cannot set tail on snake {:?}", self),
        };
        snake
    }

    pub fn health(&self, value: u8) -> Self {
        let mut snake = *self;
        match snake {
            DSnake::Alive { ref mut health, .. } | DSnake::Headless { ref mut health, .. } => {
                *health = value;
            }
            _ => panic!("Cannot set health on snake {:?}", self),
        };
        snake
    }

    pub fn stack(&self, value: u8) -> Self {
        let mut snake = *self;
        match snake {
            DSnake::Alive { ref mut stack, .. } | DSnake::Headless { ref mut stack, .. } => {
                *stack = value;
            }
            _ => panic!("Cannot set stack on snake {:?}", self),
        };
        snake
    }

    pub fn length(&self, value: u8) -> Self {
        let mut snake = *self;
        match snake {
            DSnake::Alive { ref mut length, .. } | DSnake::Headless { ref mut length, .. } => {
                *length = value;
            }
            _ => panic!("Cannot set length on snake {:?}", self),
        };
        snake
    }

    pub fn to_vanished(&self) -> Self {
        match self {
            DSnake::Alive {
                id, length, head, ..
            } => DSnake::Vanished {
                id: *id,
                length: *length,
                last_head: *head,
            },
            DSnake::Headless {
                id,
                length,
                last_head,
                ..
            } => DSnake::Vanished {
                id: *id,
                length: *length,
                last_head: *last_head,
            },
            _ => panic!("Cannot vanish snake {:?}", self),
        }
    }

    pub fn to_headless(&self) -> Self {
        match self {
            DSnake::Alive {
                id,
                health,
                length,
                tail,
                stack,
                head,
                ..
            } => DSnake::Headless {
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
            DSnake::Alive { id, .. } => DSnake::Dead { id: *id },
            DSnake::Headless { id, .. } => DSnake::Dead { id: *id },
            _ => panic!("Cannot kill snake {:?}", self),
        }
    }
}

impl DSnake {
    pub fn from_request(snake: &OriginalBattlesnake, id: u8) -> Self {
        let head = DCoord::from(&snake.head);
        let tail = DCoord::from(snake.body.last().unwrap());
        let mut last = snake.body[0];
        let mut stack = 0;
        for coord in snake.body.iter().skip(1) {
            if *coord == last {
                stack += 1;
            }
            last = *coord;
        }
        DSnake::Alive {
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
    fn test_from_request() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let d_snake = DSnake::from_request(&gamestate.you, 0);
        assert_eq!(
            d_snake,
            DSnake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: DCoord { x: 0, y: 0 },
                tail: DCoord { x: 2, y: 0 },
                stack: 0
            }
        );
    }

    #[test]
    fn test_value_changers() {
        let snake = DSnake::Alive {
            id: 0,
            health: 54,
            length: 3,
            head: DCoord { x: 0, y: 0 },
            tail: DCoord { x: 3, y: 0 },
            stack: 0,
        };
        let snake = snake
            .head(DCoord::new(4, 0))
            .tail(DCoord::new(0, 0))
            .health(100)
            .stack(1)
            .length(4);
        assert_eq!(
            snake,
            DSnake::Alive {
                id: 0,
                health: 100,
                length: 4,
                head: DCoord { x: 4, y: 0 },
                tail: DCoord { x: 0, y: 0 },
                stack: 1
            }
        );
    }
}
