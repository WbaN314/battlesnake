use log::info;

use crate::{
    logic::efficient_game_objects::{self},
    Battlesnake, Board, Coord, Game,
};

use self::efficient_game_objects::{DirectionTree, DIRECTIONS, DIRECTION_VECTORS};

use super::{Brain, Direction};

pub struct SmartSnake {}

impl SmartSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for SmartSnake {
    fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
        let game_state = efficient_game_objects::GameState::from(board, you);
        let my_snake = game_state.snakes.get(0).clone().unwrap();

        // Simulate future
        let mut d_tree = DirectionTree::from(game_state.clone());
        let evaluated_depths = d_tree.simulate_timed(10, 200);
        info!("Evaluated depths: {:?}", evaluated_depths);
        let best_depth = evaluated_depths.iter().max().unwrap();

        // Check for areas
        let mut areas = [0; 4];
        for i in 0..4 {
            if evaluated_depths[i] + 1 >= *best_depth {
                areas[i] = game_state
                    .clone()
                    .fill(&(my_snake.head + DIRECTION_VECTORS[i]))
                    .unwrap()
                    .area;
            }
        }
        info!("Calculated areas: {:?}", areas);
        let largest_area = *areas.iter().max().unwrap_or(&0);

        let mut closest_food_distance = 5;
        let mut closest_food: Option<Coord> = None;
        for food in board.food.iter() {
            if you.head.distance(food) < closest_food_distance {
                closest_food = Some(*food);
                closest_food_distance = you.head.distance(food);
            }
        }
        let food_directions = if let Some(closest_food) = closest_food {
            you.head.directions_to(&closest_food)
        } else {
            [true; 4]
        };

        // Select general best direction that remains
        let mut final_result = efficient_game_objects::Direction::Up;
        let mut distance_to_optimum = i32::MIN;
        let middle = game_state.middle();
        for i in 0..4 {
            if areas[i] == largest_area {
                match i {
                    0 => {
                        let d = middle.y - my_snake.head.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    1 => {
                        let d = my_snake.head.y - middle.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    2 => {
                        let d = my_snake.head.x - middle.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    3 => {
                        let d = middle.x - my_snake.head.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    _ => unreachable!("Non existing direction"),
                }
            }
        }

        distance_to_optimum = i32::MIN;
        for i in 0..4 {
            if areas[i] == largest_area && food_directions[i] {
                match i {
                    0 => {
                        let d = middle.y - my_snake.head.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    1 => {
                        let d = my_snake.head.y - middle.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    2 => {
                        let d = my_snake.head.x - middle.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    3 => {
                        let d = middle.x - my_snake.head.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = DIRECTIONS[i];
                        }
                    }
                    _ => unreachable!("Non existing direction"),
                }
            }
        }

        // Build Killer instinct

        match final_result {
            efficient_game_objects::Direction::Up => Direction::Up,
            efficient_game_objects::Direction::Down => Direction::Down,
            efficient_game_objects::Direction::Left => Direction::Left,
            efficient_game_objects::Direction::Right => Direction::Right,
        }
    }
}
