use std::collections::HashMap;

use crate::{Coord, Direction, GameState};

use super::shared::brain::Brain;

pub struct SimpleHungrySnake {}

impl Default for SimpleHungrySnake {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleHungrySnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for SimpleHungrySnake {
    fn logic(&self, gamestate: &GameState) -> Direction {
        let mut is_move_safe: HashMap<Direction, _> = vec![
            (Direction::Up, true),
            (Direction::Down, true),
            (Direction::Left, true),
            (Direction::Right, true),
        ]
        .into_iter()
        .collect();
        let my_head = &gamestate.you.body[0];
        let board_width = gamestate.board.width;
        let board_height = gamestate.board.height as i32;
        if my_head.x + 1 == board_width {
            is_move_safe.insert(Direction::Right, false);
        }
        if my_head.x == 0 {
            is_move_safe.insert(Direction::Left, false);
        }
        if my_head.y + 1 == board_height {
            is_move_safe.insert(Direction::Up, false);
        }
        if my_head.y == 0 {
            is_move_safe.insert(Direction::Down, false);
        }
        let snakes = &gamestate.board.snakes;
        for s in snakes {
            for i in 0..s.body.len() {
                if s.body[i].y == my_head.y {
                    if s.body[i].x == my_head.x + 1 {
                        is_move_safe.insert(Direction::Right, false);
                    }
                    if s.body[i].x + 1 == my_head.x {
                        is_move_safe.insert(Direction::Left, false);
                    }
                }
                if s.body[i].x == my_head.x {
                    if s.body[i].y == my_head.y + 1 {
                        is_move_safe.insert(Direction::Up, false);
                    }
                    if s.body[i].y + 1 == my_head.y {
                        is_move_safe.insert(Direction::Down, false);
                    }
                }
            }
        }
        let foods = &gamestate.board.food;
        let middle = Coord {
            x: board_width / 2,
            y: board_height / 2,
        };
        let closest_food = if foods.is_empty() {
            &middle
        } else {
            let mut closest_distance = u32::MAX;
            let mut tmp = &foods[0];
            for food in foods {
                let distance = my_head.x.abs_diff(food.x) + my_head.y.abs_diff(food.y);
                if distance <= closest_distance {
                    closest_distance = distance;
                    tmp = food;
                }
            }
            tmp
        };
        
        if closest_food.x > my_head.x && *is_move_safe.get(&Direction::Right).unwrap() {
                Direction::Right
            } else if closest_food.x < my_head.x && *is_move_safe.get(&Direction::Left).unwrap() {
                Direction::Left
            } else if closest_food.y > my_head.y && *is_move_safe.get(&Direction::Up).unwrap() {
                Direction::Up
            } else if closest_food.y < my_head.y && *is_move_safe.get(&Direction::Down).unwrap() {
                Direction::Down
            } else if *is_move_safe.get(&Direction::Right).unwrap() {
                Direction::Right
            } else if *is_move_safe.get(&Direction::Left).unwrap() {
                Direction::Left
            } else if *is_move_safe.get(&Direction::Up).unwrap() {
                Direction::Up
            } else if *is_move_safe.get(&Direction::Down).unwrap() {
                Direction::Down
            } else {
                Direction::Down
            }
    }
}
