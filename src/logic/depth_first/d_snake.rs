use super::d_coord::DCoord;
use crate::Battlesnake;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    },
    NonExistent,
}

impl Default for DSnake {
    fn default() -> Self {
        DSnake::NonExistent
    }
}

impl DSnake {
    pub fn from_request(snake: &Battlesnake, id: u8) -> Self {
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
}
