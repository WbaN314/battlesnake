// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use core::fmt;
use log::info;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::{collections::HashMap, env};

use crate::{Battlesnake, Board, Coord, Game};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Direction::Up => serializer.serialize_str("up"),
            Direction::Down => serializer.serialize_str("down"),
            Direction::Left => serializer.serialize_str("left"),
            Direction::Right => serializer.serialize_str("right"),
        }
    }
}

trait Brain {
    fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction;
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are Move::Up, Move::Down, Move::Left, or Move::Right
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
    let brain: Box<dyn Brain> = if let Ok(value) = env::var("VARIANT") {
        debug!("{}", value);
        if value == "hungry_simple".to_string() {
            Box::new(hungry_simple_snake::HungrySimpleSnake::new())
        } else if value == "simple_tree_search" {
            Box::new(simple_tree_search_snake::SimpleTreeSearchSnake::new())
        } else {
            Box::new(hungry_simple_snake::HungrySimpleSnake::new())
        }
    } else {
        Box::new(hungry_simple_snake::HungrySimpleSnake::new())
    };
    let next_move = brain.logic(game, turn, board, you);
    info!("MOVE {}: {}", turn, next_move);
    return next_move;
}

mod hungry_simple_snake {
    use super::*;
    pub struct HungrySimpleSnake {}

    impl HungrySimpleSnake {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Brain for HungrySimpleSnake {
        fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
            let mut is_move_safe: HashMap<Direction, _> = vec![
                (Direction::Up, true),
                (Direction::Down, true),
                (Direction::Left, true),
                (Direction::Right, true),
            ]
            .into_iter()
            .collect();
            let my_head = &you.body[0];
            let board_width = board.width;
            let board_height = board.height as i32;
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
            let snakes = &board.snakes;
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
            let foods = &board.food;
            let middle = Coord {
                x: board_width / 2,
                y: board_height / 2,
            };
            let closest_food = if foods.len() == 0 {
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
            let chosen_move = if closest_food.x > my_head.x
                && *is_move_safe.get(&Direction::Right).unwrap()
            {
                Direction::Right
            } else if closest_food.x < my_head.x && *is_move_safe.get(&Direction::Left).unwrap() {
                Direction::Left
            } else if closest_food.y > my_head.y && *is_move_safe.get(&Direction::Up).unwrap() {
                Direction::Up
            } else if closest_food.y < my_head.y && *is_move_safe.get(&Direction::Down).unwrap() {
                Direction::Down
            } else {
                if *is_move_safe.get(&Direction::Right).unwrap() {
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
            };
            return chosen_move;
        }
    }
}

mod simple_tree_search_snake {
    use super::*;

    fn simulate_snakes_step(_turn: &i32, board: &Board, _you: &Battlesnake) -> Vec<Board> {
        let mut new_boards = Vec::with_capacity(board.snakes.len().pow(4));

        let mut decisions = Directions::new(board.snakes.len());

        let mut decision = Some(&decisions.v);
        while let Some(directions) = decision {
            let mut board_clone = board.clone();
            for snake_index in 0..board_clone.snakes.len() {
                let x = board_clone.snakes[snake_index].body[0].x;
                let y = board_clone.snakes[snake_index].body[0].y;
                let new_head = match directions[snake_index] {
                    Direction::Up => Coord::from(x, y + 1),
                    Direction::Down => Coord::from(x, y - 1),
                    Direction::Left => Coord::from(x - 1, y),
                    Direction::Right => Coord::from(x + 1, y),
                };
                board_clone.snakes[snake_index].head = new_head;
                // leave body untouched yet
                // food needs to be evaluated
                // save vec clone work if state will be invalid anyways
            }
            new_boards.push(board_clone);
            decision = decisions.next();
        }
        new_boards
    }

    #[derive(Copy, Clone, Debug)]
    enum SnakeChange {
        Grow,
        None,
        Die,
        Battle(usize),
    }

    fn evaluate_snakes_step(_turn: &i32, board: &mut Board, you: &Battlesnake) -> (Direction, i32) {
        // find own snake
        let mut own_snake_index = 0;
        for i in 0..board.snakes.len() {
            if board.snakes[i].id == you.id {
                own_snake_index = i;
                break;
            }
        }
        let own_snake = &board.snakes[own_snake_index];

        // Find moved direction
        let own_direction = match (
            own_snake.head.x - own_snake.body[0].x,
            own_snake.head.y - own_snake.body[0].y,
        ) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Up,
            (0, -1) => Direction::Down,
            _ => unreachable!(),
        };

        // check out of bounds of own snake
        if own_snake.head.x < 0
            || own_snake.head.x >= board.width
            || own_snake.head.y < 0
            || own_snake.head.y >= board.height as i32
        {
            debug!("{} -1", own_direction);
            return (own_direction, -1);
        };

        let mut snake_changes = vec![SnakeChange::None; board.snakes.len()];

        // check for battles
        for i in 0..board.snakes.len() {
            for j in i + 1..board.snakes.len() {
                let x1 = board.snakes[i].head.x;
                let y1 = board.snakes[i].head.y;
                let x2 = board.snakes[j].head.x;
                let y2 = board.snakes[j].head.y;
                if x1 == x2 && y1 == y2 {
                    snake_changes[i] = SnakeChange::Battle(j);
                    break;
                }
            }
        }

        // Resolve battles
        let mut battle_results = snake_changes.clone();
        for i in 0..snake_changes.len() {
            match snake_changes[i] {
                SnakeChange::Battle(j) => {
                    if board.snakes[i].length < board.snakes[j].length {
                        battle_results[i] = SnakeChange::Die;
                    } else if board.snakes[i].length == board.snakes[j].length {
                        battle_results[i] = SnakeChange::Die;
                        battle_results[j] = SnakeChange::Die;
                    } else {
                        battle_results[j] = SnakeChange::Die;
                    }
                }
                _ => (),
            }
        }
        snake_changes = battle_results;

        // Evaluate non battle deaths
        for i in 0..board.snakes.len() {
            let snake = &board.snakes[i];
            if snake.head.x >= board.width
                || snake.head.x < 0
                || snake.head.y >= board.height as i32
                || snake.head.y < 0
            {
                snake_changes[i] = SnakeChange::Die;
                continue;
            }
            for other_snake in board.snakes.iter() {
                for part in other_snake.body.iter() {
                    if part.x == snake.head.x && part.y == snake.head.y {
                        snake_changes[i] = SnakeChange::Die;
                        break;
                    }
                }
            }
        }

        // Evaluate food
        for i in 0..board.snakes.len() {
            let snake = &board.snakes[i];
            for food in board.food.iter() {
                if food.x == snake.head.x && food.y == snake.head.y {
                    snake_changes[i] = SnakeChange::Grow;
                }
            }
        }

        // Evaluate final snake changes
        let mut new_snakes = Vec::new();
        for i in 0..board.snakes.len() {
            match snake_changes[i] {
                SnakeChange::Grow => {
                    let mut new_body = board.snakes[i].body.clone();
                    new_body.push(board.snakes[i].head);
                    new_body.rotate_right(1);
                    board.snakes[i].body = new_body;
                    new_snakes.push(board.snakes[i].clone());
                }
                SnakeChange::Battle(_) | SnakeChange::None => {
                    let l = board.snakes[i].body.len() - 1;
                    board.snakes[i].body[l] = board.snakes[i].head;
                    board.snakes[i].body.rotate_right(1);
                    new_snakes.push(board.snakes[i].clone())
                }
                SnakeChange::Die => (),
            }
        }
        board.snakes = new_snakes;

        // evaluate board
        match snake_changes[own_snake_index] {
            SnakeChange::Die => {
                debug!("{} -1", own_direction);
                (own_direction, -1)
            }
            SnakeChange::Battle(_) => {
                debug!("{} 3", own_direction);
                (own_direction, 3)
            }
            SnakeChange::Grow => {
                debug!("{} 2", own_direction);
                (own_direction, 2)
            }
            SnakeChange::None => {
                debug!("{} 1", own_direction);
                (own_direction, 1)
            }
        }
    }

    struct Directions {
        v: Vec<Direction>,
    }

    impl Directions {
        fn new(n: usize) -> Self {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(Direction::Up)
            }
            Directions { v }
        }

        fn next(&mut self) -> Option<&Vec<Direction>> {
            let mut working_index = None;
            for i in 0..self.v.len() {
                if self.v[i] != Direction::Right {
                    working_index = Some(i);
                    break;
                }
            }
            if let Some(i) = working_index {
                match self.v[i] {
                    Direction::Up => self.v[i] = Direction::Down,
                    Direction::Down => self.v[i] = Direction::Left,
                    Direction::Left => self.v[i] = Direction::Right,
                    Direction::Right => unreachable!(),
                }
                for j in 0..i {
                    self.v[j] = Direction::Up;
                }
                Some(&self.v)
            } else {
                None
            }
        }
    }

    pub struct SimpleTreeSearchSnake {}

    impl SimpleTreeSearchSnake {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Brain for SimpleTreeSearchSnake {
        fn logic(&self, _game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
            let mut new_boards = simulate_snakes_step(turn, board, you);

            let mut move_scores = [0; 4];
            for board in new_boards.iter_mut() {
                match evaluate_snakes_step(turn, board, you) {
                    (Direction::Up, score) => move_scores[0] += score,
                    (Direction::Down, score) => move_scores[1] += score,
                    (Direction::Left, score) => move_scores[2] += score,
                    (Direction::Right, score) => move_scores[3] += score,
                }
            }

            debug!("{:?}", move_scores);

            let mut best_move = Direction::Up;
            let mut best_score = move_scores[0];
            for i in 1..move_scores.len() {
                if move_scores[i] > best_score {
                    best_score = move_scores[i];
                    best_move = match i {
                        1 => Direction::Down,
                        2 => Direction::Left,
                        3 => Direction::Right,
                        _ => unreachable!(),
                    }
                }
            }
            best_move
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_directions_iterator() {
            let directions = vec![Direction::Up, Direction::Right, Direction::Right];
            let mut directions_iter = Directions { v: directions };

            assert_eq!(
                directions_iter.next(),
                Some(&vec![Direction::Down, Direction::Right, Direction::Right])
            );
            assert_eq!(
                directions_iter.next(),
                Some(&vec![Direction::Left, Direction::Right, Direction::Right])
            );
            assert_eq!(
                directions_iter.next(),
                Some(&vec![Direction::Right, Direction::Right, Direction::Right])
            );

            assert_eq!(directions_iter.next(), None);
        }
    }

    #[test]
    fn test_logic() {
        let brain = SimpleTreeSearchSnake::new();

        let game = Game::new();
    }
}

mod mocks {}
