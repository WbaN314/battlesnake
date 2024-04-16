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
        "author": "WbaN", // TODO: Your Battlesnake Username
        "color": "#f5982f", // TODO: Choose color
        "head": "fang", // TODO: Choose head
        "tail": "rattle", // TODO: Choose tail
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
        } else if value == "smart_snake" {
            Box::new(smart_snake::SmartSnake::new())
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

#[cfg(test)]
mod tests {
    use crate::logic::Direction;

    use super::get_move;

    fn read_game_state(path: &str) -> crate::GameState {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let game_state: crate::GameState = serde_json::from_reader(reader).unwrap();
        game_state
    }

    #[test]
    fn get_move_2() {
        let game_state = read_game_state("requests/example_move_request_2.json");
        let chosen_move = get_move(
            &game_state.game,
            &game_state.turn,
            &game_state.board,
            &game_state.you,
        );
        assert_eq!(chosen_move, Direction::Up);
    }

    #[test]
    fn get_move_3() {
        let game_state = read_game_state("requests/example_move_request_3.json");
        let chosen_move = get_move(
            &game_state.game,
            &game_state.turn,
            &game_state.board,
            &game_state.you,
        );
        assert_eq!(chosen_move, Direction::Down);
    }
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

    fn simulate_snakes_step(board: &Board) -> Vec<Board> {
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

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum SnakeChange {
        Grow,
        None,
        Die,
        Battle(usize),
    }

    fn evaluate_snakes_step(board: &mut Board, you: &String) -> (Direction, i32) {
        // find own snake
        let mut own_snake_index = None;
        for i in 0..board.snakes.len() {
            if board.snakes[i].id == *you {
                own_snake_index = Some(i);
                break;
            }
        }

        if let Some(own_snake_index) = own_snake_index {
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
                debug!("{} dead", own_direction);
                return (own_direction, -10);
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
                    if food.x == snake.head.x
                        && food.y == snake.head.y
                        && snake_changes[i] != SnakeChange::Die
                    {
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
                    debug!("{} dead", own_direction);
                    (own_direction, -10)
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
        } else {
            // no own snake
            (Direction::Up, 0)
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
        fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
            let (mut new_boards, mut move_scores) = step(board, &you.id);

            let depth = if board.snakes.len() == 2 { 3 } else { 1 };

            let new_combinations_number =
                4_i32.pow(board.snakes.len() as u32) * new_boards.len() as i32;
            for _ in 0..depth {
                for i in 0..4 {
                    move_scores[i] = move_scores[i].saturating_mul(new_combinations_number);
                }
                let mut tmp_boards = Vec::with_capacity(new_combinations_number as usize);
                for board in new_boards.iter() {
                    let (mut b, s) = step(board, &you.id);
                    for i in 0..4 {
                        move_scores[i] = move_scores[i].saturating_add(s[i]);
                    }
                    tmp_boards.append(&mut b);
                }
                new_boards = tmp_boards;
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

    fn step(board: &Board, id: &String) -> (Vec<Board>, [i32; 4]) {
        let mut new_boards = simulate_snakes_step(board);

        let mut move_scores = [0; 4];
        for board in new_boards.iter_mut() {
            match evaluate_snakes_step(board, id) {
                (Direction::Up, score) => {
                    move_scores[0] = (move_scores[0] as i32).saturating_add(score)
                }
                (Direction::Down, score) => {
                    move_scores[1] = (move_scores[1] as i32).saturating_add(score)
                }
                (Direction::Left, score) => {
                    move_scores[2] = (move_scores[2] as i32).saturating_add(score)
                }
                (Direction::Right, score) => {
                    move_scores[3] = (move_scores[3] as i32).saturating_add(score)
                }
            };
        }
        (new_boards, move_scores)
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
}

mod smart_snake {
    use std::{collections::VecDeque, time::Instant};

    use log::{info, warn};

    use crate::{
        logic::efficient_game_objects::{self, Area},
        Battlesnake, Board, Game,
    };

    use self::efficient_game_objects::{DirectionTree, GameState, DIRECTIONS, DIRECTION_VECTORS};

    use super::{Brain, Direction};

    pub struct SmartSnake {}

    impl SmartSnake {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Brain for SmartSnake {
        fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
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
                if evaluated_depths[i] >= best_depth - 1 {
                    areas[i] = game_state
                        .clone()
                        .fill(&(my_snake.head + DIRECTION_VECTORS[i]))
                        .unwrap()
                        .area;
                }
            }
            info!("Calculated areas: {:?}", areas);
            let largest_area = *areas.iter().max().unwrap_or(&0);

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

            match final_result {
                efficient_game_objects::Direction::Up => Direction::Up,
                efficient_game_objects::Direction::Down => Direction::Down,
                efficient_game_objects::Direction::Left => Direction::Left,
                efficient_game_objects::Direction::Right => Direction::Right,
            }
        }
    }
}

mod efficient_game_objects {
    use core::fmt;
    use core::panic;
    use std::cell::Ref;
    use std::cell::RefCell;
    use std::cell::RefMut;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use std::fmt::Display;
    use std::ops::Deref;
    use std::ops::DerefMut;
    use std::time::Duration;
    use std::time::Instant;

    use crate::Battlesnake as DefaultSnake;
    use crate::Board as DefaultBoard;
    use crate::Coord;
    use crate::Game;

    const X_SIZE: usize = 11;
    const Y_SIZE: usize = 11;
    const SNAKES: usize = 4;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Direction {
        Up = 0,
        Down = 1,
        Left = 2,
        Right = 3,
    }

    impl Direction {
        pub fn from_usize(u: usize) -> Direction {
            match u {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => panic!("Invalid usize for Direction conversion"),
            }
        }

        pub fn to_usize(self) -> usize {
            match self {
                Direction::Up => 0,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Right => 3,
            }
        }
    }

    impl Display for Direction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                &Direction::Up => write!(f, "Up"),
                &Direction::Down => write!(f, "Down"),
                &Direction::Left => write!(f, "Left"),
                &Direction::Right => write!(f, "Right"),
            }
        }
    }

    pub const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub const DIRECTION_VECTORS: [Coord; 4] = [
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: -1 },
        Coord { x: -1, y: 0 },
        Coord { x: 1, y: 0 },
    ];

    type Result<T> = std::result::Result<T, Death>;

    // Define our error types. These may be customized for our error handling cases.
    // Now we will be able to write our own errors, defer to an underlying error
    // implementation, or do something in between.
    #[derive(Debug, Clone)]
    pub struct Death;

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

    #[derive(Clone)]
    pub struct DirectionTree {
        map: BTreeMap<DirectionVec, Option<DirectionNode>>,
        current: VecDeque<DirectionVec>,
    }

    impl Display for DirectionTree {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut s = String::new();
            for (key, value) in self.map.iter() {
                s.push_str(&format!("{:?}\n", key));
                match value {
                    Some(node) => s.push_str(&node.to_string()),
                    None => s.push_str("Completed"),
                }
                s.push_str("\n\n");
            }
            write!(f, "{}", s)
        }
    }

    #[derive(Clone)]
    struct DirectionNode {
        pub states: Vec<GameState>,
        pub evaluated: [bool; 4],
    }

    impl Display for DirectionNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "# states: {} \ndirections: {:?}",
                self.states.len(),
                self.evaluated
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct DirectionVec(Vec<Direction>);

    impl DirectionVec {
        fn new() -> Self {
            Self(Vec::new())
        }

        fn from(v: Vec<Direction>) -> Self {
            Self(v)
        }
    }

    impl Deref for DirectionVec {
        type Target = Vec<Direction>;
        fn deref(&self) -> &Vec<Direction> {
            &self.0
        }
    }

    impl DerefMut for DirectionVec {
        fn deref_mut(&mut self) -> &mut Vec<Direction> {
            &mut self.0
        }
    }

    impl PartialOrd for DirectionVec {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for DirectionVec {
        fn eq(&self, other: &Self) -> bool {
            if self.cmp(other) == std::cmp::Ordering::Equal {
                true
            } else {
                false
            }
        }
    }

    impl Eq for DirectionVec {}

    impl Ord for DirectionVec {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.len() > other.len() {
                std::cmp::Ordering::Greater
            } else if self.len() < other.len() {
                std::cmp::Ordering::Less
            } else {
                let mut i = 0;
                loop {
                    if i >= self.len() {
                        break std::cmp::Ordering::Equal;
                    } else if self[i].to_usize() > other[i].to_usize() {
                        break std::cmp::Ordering::Greater;
                    } else if self[i].to_usize() < other[i].to_usize() {
                        break std::cmp::Ordering::Less;
                    }
                    i += 1;
                }
            }
        }
    }

    impl DirectionNode {
        pub fn new() -> Self {
            Self {
                states: Vec::new(),
                evaluated: [false; 4],
            }
        }

        pub fn from(valid_states: Vec<GameState>) -> DirectionNode {
            Self {
                states: valid_states,
                evaluated: [false; 4],
            }
        }

        pub fn calc_next(&mut self, to: Direction, distance: u32) -> Result<DirectionNode> {
            let mut new_valid_states = Vec::new();
            for state in self.states.iter() {
                let relevant_moves = state.relevant_moves(distance);
                for relevant_move in relevant_moves {
                    if relevant_move[0].unwrap() != to {
                        continue;
                    }
                    let mut new_state = state.clone();
                    match new_state.move_snakes(relevant_move) {
                        Ok(_) => new_valid_states.push(new_state),
                        Err(_) => return Result::Err(Death),
                    };
                }
            }
            self.evaluated[to.to_usize()] = true;
            if new_valid_states.len() == 0 {
                return Result::Err(Death);
            }
            Ok(Self::from(new_valid_states))
        }

        pub fn completely_evaluated(&self) -> bool {
            for i in self.evaluated {
                if !i {
                    return false;
                }
            }
            return true;
        }
    }

    impl DirectionTree {
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                current: VecDeque::from(Vec::new()),
            }
        }

        pub fn from(state: GameState) -> Self {
            let mut d_tree = Self::new();
            let d_node = DirectionNode::from(vec![state]);
            d_tree.map.insert(DirectionVec::new(), Some(d_node));
            d_tree.current.push_back(DirectionVec::new());
            d_tree
        }

        pub fn calc(&mut self, mut from: DirectionVec, to: Direction, distance: u32) -> Result<()> {
            let mut delete = false;
            let mut result;
            let mut calc_next_result: Option<DirectionNode>;
            match self.map.get_mut(&from) {
                Some(Some(node)) => {
                    match node.calc_next(to, distance) {
                        Ok(r) => {
                            calc_next_result = Some(r);
                            result = Result::Ok(())
                        }
                        Err(_) => {
                            calc_next_result = None;
                            result = Result::Err(Death)
                        }
                    }
                    if node.completely_evaluated() {
                        delete = true
                    }
                }
                Some(None) => {
                    calc_next_result = None;
                    result = Result::Err(Death)
                }
                _ => {
                    panic!("Invalid access")
                }
            }
            let mut fromto = from.clone();
            fromto.push(to);
            self.map.insert(fromto, calc_next_result);
            if delete {
                self.map.insert(from, None);
            }
            result
        }

        pub fn calcs(&mut self, from: DirectionVec, distance: u32) -> [bool; 4] {
            let mut results = [false; 4];
            for d in 0..4 {
                match self.calc(from.clone(), Direction::from_usize(d), distance) {
                    Ok(_) => results[d] = true,
                    Err(_) => results[d] = false,
                }
            }
            results
        }

        pub fn simulate_timed(&mut self, distance: u32, milliseconds: u64) -> [usize; 4] {
            let mut result = [0; 4];

            let timer = Instant::now();
            while timer.elapsed() < Duration::from_millis(milliseconds) {
                match self.current.pop_front() {
                    None => break,
                    Some(d_vec) => {
                        let bools = self.calcs(d_vec.clone(), distance);
                        for i in 0..4 {
                            if bools[i] {
                                let mut new = d_vec.clone();
                                new.push(Direction::from_usize(i));
                                self.current.push_back(new);
                            }
                        }
                    }
                }
            }

            let best_len = if let Some(d_vec) = self.map.keys().last() {
                d_vec.len()
            } else {
                0
            };
            for key in self.map.keys().rev() {
                if key.len() < best_len - 1 || key.len() == 0 {
                    break;
                } else if result[key[0].to_usize()] < key.len() {
                    if self.map.get(key).unwrap().is_some() {
                        result[key[0].to_usize()] = key.len();
                    }
                }
            }
            result
        }
    }

    #[derive(Clone)]
    pub struct GameState {
        pub board: Board,
        pub snakes: Snakes,
    }

    impl GameState {
        pub fn new() -> Self {
            let snakes: [Option<Snake>; SNAKES] = std::array::from_fn(|_| None);
            GameState {
                board: Board::new(),
                snakes: Snakes::new(),
            }
        }

        pub fn from(old: &DefaultBoard, you: &DefaultSnake) -> Self {
            let mut gamestate = Self::new();

            for food in old.food.iter() {
                gamestate.board.set(food.x, food.y, Field::Food);
            }

            let mut order: Vec<usize> = (0..old.snakes.len()).collect();
            for i in 0..old.snakes.len() {
                if *old.snakes[i].id == *you.id {
                    order.swap(0, i);
                    break;
                }
            }

            for i in 0..old.snakes.len() {
                for (j, snake_part) in old.snakes[order[i]].body.iter().enumerate() {
                    let next = if j == 0 {
                        None
                    } else {
                        Some(old.snakes[order[i]].body[j - 1].clone())
                    };
                    match gamestate.board.get(snake_part.x, snake_part.y) {
                        Some(Field::SnakePart {
                            snake_number,
                            stacked,
                            next,
                        }) => gamestate.board.set(
                            snake_part.x,
                            snake_part.y,
                            Field::SnakePart {
                                snake_number,
                                next,
                                stacked: stacked + 1,
                            },
                        ),
                        _ => gamestate.board.set(
                            snake_part.x,
                            snake_part.y,
                            Field::SnakePart {
                                snake_number: i,
                                next: next,
                                stacked: 1,
                            },
                        ),
                    };
                }
            }

            for i in 0..old.snakes.len() {
                gamestate
                    .snakes
                    .set(i, Some(Snake::from(&old.snakes[order[i]], i as i32)));
            }

            gamestate.validate_state();

            gamestate
        }

        pub fn fill(&mut self, start: &Coord) -> Option<Area> {
            let mut area = Area::new();
            let x = start.x;
            let y = start.y;
            match self.board.get(x, y) {
                Some(Field::Empty) | Some(Field::Food) => {
                    let mut s = Vec::new();
                    s.push((x, x, y, 1));
                    s.push((x, x, y - 1, -1));
                    while let Some((mut x1, x2, y, dy)) = s.pop() {
                        let mut x = x1;
                        match self.board.get(x, y) {
                            Some(Field::Empty) | Some(Field::Food) => {
                                let mut candidate = self.board.get(x - 1, y);
                                while candidate == Some(Field::Empty)
                                    || candidate == Some(Field::Food)
                                {
                                    self.board.set(x - 1, y, Field::Filled);
                                    area.area += 1;
                                    x -= 1;
                                    candidate = self.board.get(x - 1, y);
                                }
                                if x < x1 {
                                    s.push((x, x1 - 1, y - dy, -dy))
                                }
                            }
                            _ => (),
                        }
                        while x1 <= x2 {
                            let mut candidate = self.board.get(x1, y);
                            while candidate == Some(Field::Empty) || candidate == Some(Field::Food)
                            {
                                self.board.set(x1, y, Field::Filled);
                                area.area += 1;
                                x1 += 1;
                                candidate = self.board.get(x1, y);
                            }
                            if x1 > x {
                                s.push((x, x1 - 1, y + dy, dy));
                            }
                            if x1 - 1 > x2 {
                                s.push((x2 + 1, x1 - 1, y - dy, -dy));
                            }
                            x1 += 1;
                            loop {
                                let candidate = self.board.get(x1, y);
                                if x1 > x2
                                    || candidate == Some(Field::Empty)
                                    || candidate == Some(Field::Food)
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

        pub fn relevant_moves(&self, distance: u32) -> Vec<[Option<Direction>; 4]> {
            let mut snake_relevant = [false; SNAKES];
            if let Some(my_snake) = self.snakes.get(0).as_ref() {
                // Determine relevant snakes based on distance
                for i in 0..SNAKES {
                    if let Some(snake) = self.snakes.get(i).as_ref() {
                        if my_snake.head.distance(&snake.head) <= distance {
                            snake_relevant[i] = true;
                        }
                    }
                }
            } else {
                return Vec::new();
            }

            // Determine "dangerous" move combinations of relevant snakes where they do not do stupid stuff
            let mut dangerous_moves = [[false; 4]; SNAKES];
            for snake_index in 0..SNAKES {
                if snake_relevant[snake_index] {
                    for d in 0..4 {
                        let new_head_candidate =
                            self.snakes.get(snake_index).as_ref().unwrap().head
                                + DIRECTION_VECTORS[d];
                        match self.board.get(new_head_candidate.x, new_head_candidate.y) {
                            Some(Field::Empty) | Some(Field::Food) => {
                                dangerous_moves[snake_index][d] = true;
                            }
                            Some(Field::SnakePart { snake_number, .. }) => {
                                let tail = self.snakes.get(snake_number).as_ref().unwrap().tail;
                                if tail.x == new_head_candidate.x && tail.y == new_head_candidate.y
                                {
                                    dangerous_moves[snake_index][d] = true;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            // Get the count of actually relevant snake move combinations
            let mut relevant_count = [0; SNAKES];
            for i in 0..SNAKES {
                relevant_count[i] =
                    dangerous_moves[i]
                        .iter()
                        .fold(0, |acc, e| if *e { acc + 1 } else { acc });
            }
            let mut relevant_move_found = false;
            for count in relevant_count {
                if count != 0 {
                    relevant_move_found = true;
                }
            }
            if !relevant_move_found {
                return Vec::new();
            }
            let final_count = relevant_count.iter().fold(1, |acc, e| acc * e.max(&1));

            // Generate the relevant move combinations
            let mut move_combinations: Vec<[Option<Direction>; 4]> =
                vec![[None, None, None, None]; final_count];
            let mut pattern_repeat = 1;
            let mut move_repeat = final_count;
            for snake_index in 0..SNAKES {
                if relevant_count[snake_index] == 0 {
                    continue;
                }
                move_repeat /= relevant_count[snake_index];
                let mut move_index = 0;
                for p in 0..pattern_repeat {
                    for current_valid_move_count in 0..relevant_count[snake_index] {
                        loop {
                            if dangerous_moves[snake_index][move_index] {
                                break;
                            }
                            move_index += 1;
                        }
                        for m in 0..move_repeat {
                            let final_position = p * move_repeat * relevant_count[snake_index]
                                + move_repeat * current_valid_move_count
                                + m;
                            move_combinations[final_position][snake_index] =
                                Some(match move_index {
                                    0 => Direction::Up,
                                    1 => Direction::Down,
                                    2 => Direction::Left,
                                    3 => Direction::Right,
                                    _ => unreachable!(),
                                });
                        }
                        move_index += 1;
                    }
                    move_index = 0;
                }
                pattern_repeat *= relevant_count[snake_index];
            }

            move_combinations
        }

        fn eliminate_dead_snake(&self, snake_index: usize) {
            let mut eliminate = false;
            if let Some(snake) = self.snakes.get(snake_index).as_ref() {
                if snake.die {
                    eliminate = true;
                    let mut x = snake.tail.x;
                    let mut y = snake.tail.y;
                    loop {
                        match self.board.get(x, y) {
                            Some(Field::SnakePart {
                                next, snake_number, ..
                            }) if snake_number == snake_index => {
                                let next = next.clone();
                                self.board.set(x, y, Field::Empty);
                                match next {
                                    Some(next) => {
                                        x = next.x;
                                        y = next.y;
                                    }
                                    None => break,
                                }
                            }
                            _ => break,
                        }
                    }
                }
            }
            if eliminate {
                self.snakes.get_mut(snake_index).take();
            }
        }

        pub fn move_snakes(&mut self, moveset: [Option<Direction>; 4]) -> Result<()> {
            // Problem: When eating a new body part is appended immediately on tail such that there are two body parts in the same space
            // then next turn only one body part moves the other stays behind
            // similar happends at start when the whole snake is on one square altough it has length 3
            // TODO: Handle this

            // Hunger eliminations first, also health gain and loss due to food or no food
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                    let x = snake.head.x;
                    let y = snake.head.y;
                    match self.board.get(x, y) {
                        Some(Field::Food) => snake.health = 100,
                        _ => (),
                    }
                    snake.health -= 1;
                    if snake.health <= 0 {
                        snake.die = true;
                        if i == 0 {
                            return Result::Err(Death);
                        }
                    };
                }
            }

            // Remove snakes that died of hunger
            for i in 0..SNAKES {
                self.eliminate_dead_snake(i);
            }

            // Handle heads
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                    if let Some(mv) = moveset[i] {
                        let new_head = snake.head + DIRECTION_VECTORS[mv.to_usize()];
                        // handle old snake head field
                        match self.board.get(snake.head.x, snake.head.y) {
                            Some(Field::SnakePart {
                                snake_number,
                                stacked,
                                ..
                            }) => self.board.set(
                                snake.head.x,
                                snake.head.y,
                                Field::SnakePart {
                                    snake_number,
                                    stacked: stacked,
                                    next: Some(new_head),
                                },
                            ),
                            _ => unreachable!("Old snake head not on snake part."),
                        };
                        // handle new snake head field
                        snake.head = new_head;
                        match self.board.get(snake.head.x, snake.head.y) {
                            Some(Field::Empty) => {
                                self.board.set(
                                    snake.head.x,
                                    snake.head.y,
                                    Field::Contested {
                                        snake_number: i,
                                        food: false,
                                    },
                                );
                            }
                            Some(Field::Food) => {
                                // health is handled before, no handling here
                                // grow is set on contested field evaluation
                                self.board.set(
                                    snake.head.x,
                                    snake.head.y,
                                    Field::Contested {
                                        snake_number: i,
                                        food: true,
                                    },
                                );
                            }
                            Some(Field::Contested { snake_number, food }) => {
                                match self.snakes.get_mut(snake_number).as_mut() {
                                    Some(other_snake) => {
                                        if snake.length > other_snake.length {
                                            other_snake.die = true;
                                            self.board.set(
                                                snake.head.x,
                                                snake.head.y,
                                                Field::Contested {
                                                    snake_number: i,
                                                    food,
                                                },
                                            );
                                        } else if snake.length < other_snake.length {
                                            snake.die = true;
                                            self.board.set(
                                                snake.head.x,
                                                snake.head.y,
                                                Field::Contested { snake_number, food },
                                            );
                                        } else {
                                            snake.die = true;
                                            other_snake.die = true;
                                            self.board.set(
                                                snake.head.x,
                                                snake.head.y,
                                                Field::Contested { snake_number, food },
                                            );
                                        }
                                    }
                                    None => unreachable!("Ghost snake"),
                                }
                            }
                            Some(Field::SnakePart { .. }) => {
                                snake.die = true;
                            }
                            None => snake.die = true,
                            _ => unreachable!("Old board is broken"),
                        }
                    }
                }
            }

            // Remove snakes that died due to bad moves
            for i in 0..SNAKES {
                self.eliminate_dead_snake(i);
            }

            // Make contested fields to snakeparts again. Only winner snakes should have contested heads, losers should not have contested fields set anymore.
            // Handle tails of surviving snakes and reset grow
            for i in 0..SNAKES {
                // Handle contested fields
                if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                    match self.board.get(snake.head.x, snake.head.y) {
                        Some(Field::Contested { snake_number, food }) => {
                            if food {
                                snake.grow = true;
                            }
                            self.board.set(
                                snake.head.x,
                                snake.head.y,
                                Field::SnakePart {
                                    snake_number,
                                    stacked: 1,
                                    next: None,
                                },
                            );
                        }
                        Some(Field::SnakePart { .. }) => {
                            // might happen if snakes have no moves processed (i.e. tofar away)
                            ()
                        }
                        _ => unreachable!("Invalid board state"),
                    };

                    // Handle tail
                    match self.board.get(snake.tail.x, snake.tail.y) {
                        Some(Field::SnakePart {
                            snake_number,
                            stacked,
                            next,
                        }) => {
                            match stacked {
                                1 => {
                                    self.board.set(snake.tail.x, snake.tail.y, Field::Empty);
                                    match next {
                                        Some(next) => snake.tail = next,
                                        None => snake.die = true,
                                    }
                                }
                                2.. => {
                                    self.board.set(
                                        snake.tail.x,
                                        snake.tail.y,
                                        Field::SnakePart {
                                            snake_number,
                                            stacked: stacked - 1,
                                            next,
                                        },
                                    );
                                    ()
                                }
                                v => unreachable!("Invalid stacked value {}", v),
                            };
                        }
                        _ => unreachable!("Old tail is wrong"),
                    };

                    if snake.grow {
                        match self.board.get(snake.tail.x, snake.tail.y) {
                            Some(Field::SnakePart {
                                snake_number,
                                stacked,
                                next,
                            }) => self.board.set(
                                snake.tail.x,
                                snake.tail.y,
                                Field::SnakePart {
                                    snake_number,
                                    stacked: stacked + 1,
                                    next,
                                },
                            ),
                            _ => unreachable!("Invalid tail"),
                        };
                        snake.grow = false;
                    }
                }
            }

            for i in 0..SNAKES {
                self.eliminate_dead_snake(i);
            }

            self.validate_state();

            Result::Ok(())
        }

        pub fn middle(&self) -> Coord {
            Coord::from((X_SIZE / 2) as i32, (Y_SIZE / 2) as i32)
        }

        fn validate_state(&self) {
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get(i).as_ref() {
                    match self.board.get(snake.head.x, snake.head.y) {
                        Some(Field::SnakePart {
                            snake_number, next, ..
                        }) => {
                            if !next.is_none() || snake_number != i {
                                panic!(
                                    "Head is pointing to wrong SnakePart for snake {} \n {}",
                                    i, self
                                )
                            }
                        }
                        _ => panic!(
                            "Head is not pointing to SnakePart for snake {} \n {}",
                            i, self
                        ),
                    }
                    match self.board.get(snake.tail.x, snake.tail.y) {
                        Some(Field::SnakePart { snake_number, .. }) => {
                            if snake_number != i {
                                panic!(
                                    "Tail is pointing to wrong SnakePart for snake {} \n {}",
                                    i, self
                                );
                            }
                        }
                        _ => panic!(
                            "Tail is not pointing to SnakePart for snake {} \n {}",
                            i, self
                        ),
                    }
                }
            }
        }
    }

    impl fmt::Display for GameState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut output: String = String::with_capacity((X_SIZE + 1) * Y_SIZE);
            for y in (0..Y_SIZE).rev() {
                for x in 0..X_SIZE {
                    if let Some(state) = self.board.get(x as i32, y as i32) {
                        output.push(match state {
                            Field::Empty => '.',
                            Field::Food => 'F',
                            Field::SnakePart { snake_number, .. } => {
                                char::from_digit(snake_number as u32, 10).unwrap_or('?')
                            }
                            Field::Filled => 'X',
                            Field::Contested { .. } => 'C',
                        });
                        output.push(' ');
                    }
                }
                output.push('\n')
            }
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get(i).as_ref() {
                    let next_tail = match self.board.get(snake.tail.x, snake.tail.y) {
                        Some(Field::SnakePart { next, .. }) => {
                            next.unwrap_or(Coord { x: -1, y: -1 })
                        }
                        _ => panic!("Invalid tail state"),
                    };
                    output.push_str(&format!(
                        "Snake {} -> head: {} {} tail: {} {} next_tail: {} {} \n",
                        i,
                        snake.head.x,
                        snake.head.y,
                        snake.tail.x,
                        snake.tail.y,
                        next_tail.x,
                        next_tail.y
                    ))
                }
            }
            write!(f, "{}", output)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Snakes([RefCell<Option<Snake>>; SNAKES]);

    impl Snakes {
        fn new() -> Self {
            Self(std::array::from_fn(|_| RefCell::new(None)))
        }

        pub fn set(&self, i: usize, snake: Option<Snake>) {
            self.0[i].replace(snake);
        }

        pub fn get(&self, i: usize) -> Ref<Option<Snake>> {
            self.0[i].borrow()
        }

        fn get_mut(&self, i: usize) -> RefMut<Option<Snake>> {
            self.0[i].borrow_mut()
        }
    }

    #[derive(Clone)]
    pub struct Board([RefCell<Field>; X_SIZE * Y_SIZE]);

    impl Board {
        fn new() -> Self {
            Self(std::array::from_fn(|_| RefCell::new(Field::new())))
        }

        pub fn set(&self, x: i32, y: i32, state: Field) -> bool {
            if x < 0 || x >= X_SIZE as i32 || y < 0 || y >= Y_SIZE as i32 {
                false
            } else {
                let index = X_SIZE * y as usize + x as usize;
                self.0[index].replace(state);
                true
            }
        }

        pub fn get(&self, x: i32, y: i32) -> Option<Field> {
            if x < 0 || x >= X_SIZE as i32 || y < 0 || y >= Y_SIZE as i32 {
                None
            } else {
                let index = X_SIZE * y as usize + x as usize;
                Some(self.0[index].borrow().clone())
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Field {
        Empty,
        Food,
        SnakePart {
            snake_number: usize,
            stacked: usize,
            next: Option<Coord>,
        },
        Filled,
        Contested {
            snake_number: usize,
            food: bool,
        },
    }

    impl Field {
        fn new() -> Self {
            Self::Empty
        }
    }

    #[derive(Clone, Debug)]
    pub struct Snake {
        pub number: i32,
        pub head: Coord,
        pub tail: Coord,
        pub health: i32,
        pub length: i32,
        pub die: bool,
        pub grow: bool,
    }

    impl Snake {
        fn from(snake: &DefaultSnake, number: i32) -> Self {
            Self {
                number: number,
                head: snake.head,
                tail: snake.body.last().unwrap().clone(),
                health: snake.health,
                length: snake.length,
                die: false,
                grow: false,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Area {
        pub area: usize,
    }

    impl Area {
        pub fn new() -> Self {
            Self { area: 0 }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn read_game_state(path: &str) -> crate::GameState {
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            let game_state: crate::GameState = serde_json::from_reader(reader).unwrap();
            game_state
        }

        #[test]
        fn print_board_1() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            println!("{board}")
        }

        #[test]
        fn print_board_1_up() {
            let game_state = read_game_state("requests/example_move_request.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            board
                .move_snakes([Some(Direction::Up), Some(Direction::Up), None, None])
                .unwrap();
            println!("{board}")
        }

        #[test]
        fn print_board_1_up_up() {
            let game_state = read_game_state("requests/example_move_request.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            board
                .move_snakes([Some(Direction::Up), Some(Direction::Up), None, None])
                .unwrap();
            board
                .move_snakes([Some(Direction::Up), Some(Direction::Up), None, None])
                .unwrap();
            println!("{board}")
        }

        #[test]
        fn print_board_1_up_up_up() {
            let game_state = read_game_state("requests/example_move_request.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            board
                .move_snakes([Some(Direction::Up), Some(Direction::Up), None, None])
                .unwrap();
            board
                .move_snakes([Some(Direction::Up), Some(Direction::Up), None, None])
                .unwrap();
            board
                .move_snakes([Some(Direction::Up), Some(Direction::Up), None, None])
                .unwrap();
            println!("{board}")
        }

        #[test]
        fn print_board_2() {
            let game_state = read_game_state("requests/example_move_request_2.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            println!("{board}")
        }

        #[test]
        fn snakes_to_board() {
            let game_state = read_game_state("requests/example_move_request.json");
            let gamestate = GameState::from(&game_state.board, &game_state.you);
            assert_eq!(gamestate.snakes.get(0).as_ref().unwrap().health, 54);
            assert_eq!(gamestate.snakes.get(1).as_ref().unwrap().health, 16);
            assert!(gamestate.snakes.get(2).is_none());
            assert!(gamestate.snakes.get(3).is_none());
        }

        #[test]
        fn snakeparts_on_board() {
            let game_state = read_game_state("requests/example_move_request.json");
            let gamestate = GameState::from(&game_state.board, &game_state.you);
            assert_eq!(
                gamestate.board.get(0, 0).unwrap(),
                Field::SnakePart {
                    snake_number: 0,
                    next: None,
                    stacked: 1
                }
            );
            assert_eq!(
                gamestate.board.get(1, 0).unwrap(),
                Field::SnakePart {
                    snake_number: 0,
                    next: Some(Coord { x: 0, y: 0 }),
                    stacked: 1
                }
            );
            assert_eq!(
                gamestate.board.get(2, 0).unwrap(),
                Field::SnakePart {
                    snake_number: 0,
                    next: Some(Coord { x: 1, y: 0 }),
                    stacked: 1
                }
            );
        }

        #[test]
        fn fill_board() {
            let game_state = read_game_state("requests/example_move_request.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            assert!(board.clone().fill(&Coord::from(0, 0)).is_none());
            assert!(board.clone().fill(&Coord::from(-1, 0)).is_none());
            assert_eq!(board.fill(&Coord::from(0, 1)).unwrap().area, 114);
            println!("{board}");
        }

        #[test]
        fn fill_board_2() {
            let game_state = read_game_state("requests/example_move_request_2.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            assert_eq!(board.fill(&Coord::from(0, 1)).unwrap().area, 20);
            println!("{board}");
        }

        #[test]
        fn relevant_moves() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let movesets = board.relevant_moves(u32::MAX);
            for m in movesets {
                println!("{:?}", m);
            }
        }

        #[test]
        fn relevant_moves_2() {
            let game_state = read_game_state("requests/example_move_request_2.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let movesets = board.relevant_moves(u32::MAX);
            for m in movesets {
                println!("{:?}", m);
            }
        }

        #[test]
        fn relevant_moves_3() {
            let game_state = read_game_state("requests/example_move_request_3.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let movesets = board.relevant_moves(u32::MAX);
            for m in movesets {
                println!("{:?}", m);
            }
        }

        #[test]
        fn move_other_snakes_up() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut moved_up = board.clone();
            match moved_up.move_snakes([None, Some(Direction::Up), None, None]) {
                Ok(_) => println!("{}", moved_up),
                Err(_) => println!("Death"),
            }
        }

        #[test]
        fn move_other_snakes_left() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut moved_up = board.clone();
            match moved_up.move_snakes([Some(Direction::Left), Some(Direction::Left), None, None]) {
                Ok(_) => println!("{}", moved_up),
                Err(_) => println!("Death"),
            }
        }

        #[test]
        fn move_other_snakes_down() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut moved_up = board.clone();
            match moved_up.move_snakes([Some(Direction::Up), Some(Direction::Down), None, None]) {
                Ok(_) => println!("{}", moved_up),
                Err(_) => println!("Death"),
            }
        }

        #[test]
        fn direction_tree() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut d_tree = DirectionTree::from(board);
            d_tree.calc(DirectionVec::new(), Direction::Up, u32::MAX);
            d_tree.calc(DirectionVec::new(), Direction::Down, u32::MAX);
            d_tree.calc(DirectionVec::new(), Direction::Left, u32::MAX);
            d_tree.calc(DirectionVec::new(), Direction::Right, u32::MAX);
            d_tree.calc(
                DirectionVec::from(vec![Direction::Up]),
                Direction::Up,
                u32::MAX,
            );
            d_tree.calc(
                DirectionVec::from(vec![Direction::Up, Direction::Up]),
                Direction::Up,
                u32::MAX,
            );
            d_tree.calc(
                DirectionVec::from(vec![Direction::Up, Direction::Up, Direction::Up]),
                Direction::Up,
                u32::MAX,
            );
            d_tree.calc(
                DirectionVec::from(vec![Direction::Down]),
                Direction::Up,
                u32::MAX,
            );
            println!("{}", d_tree)
        }

        #[test]
        fn direction_tree_simulate() {
            let game_state = read_game_state("requests/example_move_request.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut d_tree = DirectionTree::from(board);
            d_tree.simulate_timed(u32::MAX, 200);
            println!("{}", d_tree)
        }

        #[test]
        fn print_board_3() {
            let game_state = read_game_state("requests/failure_1.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            println!("{board}");
        }

        #[test]
        fn print_board_3_after_move() {
            let game_state = read_game_state("requests/failure_1.json");
            let mut board = GameState::from(&game_state.board, &game_state.you);
            println!("{board}");
            board.move_snakes([
                Some(Direction::Down),
                Some(Direction::Up),
                Some(Direction::Down),
                None,
            ]);
            println!("{board}")
        }

        #[test]
        fn failure_1() {
            let game_state = read_game_state("requests/failure_1.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut d_tree = DirectionTree::from(board);
            d_tree.simulate_timed(u32::MAX, 200);
        }

        #[test]
        fn limit_distance() {
            let game_state = read_game_state("requests/failure_1.json");
            let board = GameState::from(&game_state.board, &game_state.you);
            let mut d_tree = DirectionTree::from(board);
            let mut d_tree_2 = d_tree.clone();
            let result = d_tree.simulate_timed(u32::MAX, 200);
            println!("{:?}", result);
            let result_2 = d_tree_2.simulate_timed(4, 200);
            println!("{:?}", result_2);
        }
    }
}

mod mocks {}
