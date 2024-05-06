use log::info;

use crate::{
    logic::efficient_game_objects::{
        e_coord::ECoord,
        e_direction::{EDirection, EDIRECTIONS, EDIRECTION_VECTORS},
        e_game_state::EGameState,
        e_state_tree::EStateTree,
    },
    Battlesnake, Board, Game,
};

use super::{Brain, Direction};

pub struct SmartSnake {}

impl SmartSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for SmartSnake {
    fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
        let game_state = EGameState::from(board, you);
        let my_snake = game_state.snakes.get(0).clone().unwrap();

        // Simulate future
        let mut d_tree = EStateTree::from(game_state.clone());
        let evaluated_depths = d_tree.simulate_timed(10, 200);
        info!("Evaluated depths: {:?}", evaluated_depths);
        let best_depth = evaluated_depths.iter().max().unwrap();

        // Check for areas
        let mut areas = [0; 4];
        for i in 0..4 {
            if evaluated_depths[i] >= *best_depth {
                if let Some(area) = game_state
                    .board
                    .clone()
                    .fill(&(my_snake.head + EDIRECTION_VECTORS[i]))
                {
                    areas[i] = area.area;
                }
            }
        }
        info!("Calculated areas: {:?}", areas);
        let largest_area = *areas.iter().max().unwrap_or(&0);

        let mut closest_food_distance = 5;
        let mut closest_food: Option<ECoord> = None;
        let head = ECoord::from(you.head.x as i8, you.head.y as i8);
        for food in board.food.iter() {
            let food = &ECoord::from(food.x as i8, food.y as i8);
            if head.distance(food) < closest_food_distance {
                closest_food = Some(*food);
                closest_food_distance = head.distance(food);
            }
        }
        let food_directions = if let Some(closest_food) = closest_food {
            head.directions_to(&closest_food)
        } else {
            [true; 4]
        };

        // Select general best direction that remains
        let mut final_result = EDirection::Up;
        let mut distance_to_optimum = i8::MIN;
        let middle = game_state.board.middle();
        for i in 0..4 {
            if areas[i] == largest_area {
                match i {
                    0 => {
                        let d = middle.y - my_snake.head.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    1 => {
                        let d = my_snake.head.y - middle.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    2 => {
                        let d = my_snake.head.x - middle.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    3 => {
                        let d = middle.x - my_snake.head.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    _ => unreachable!("Non existing direction"),
                }
            }
        }

        distance_to_optimum = i8::MIN;
        for i in 0..4 {
            if areas[i] == largest_area && food_directions[i] {
                match i {
                    0 => {
                        let d = middle.y - my_snake.head.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    1 => {
                        let d = my_snake.head.y - middle.y;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    2 => {
                        let d = my_snake.head.x - middle.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    3 => {
                        let d = middle.x - my_snake.head.x;
                        if d > distance_to_optimum {
                            distance_to_optimum = d;
                            final_result = EDIRECTIONS[i];
                        }
                    }
                    _ => unreachable!("Non existing direction"),
                }
            }
        }

        // Build Killer instinct

        final_result.to_direction()
    }
}
