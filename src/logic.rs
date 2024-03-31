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
    use super::*;

    pub struct SmartSnake {}

    impl SmartSnake {
        fn new() -> Self {
            Self {}
        }
    }

    impl Brain for SmartSnake {
        fn logic(&self, game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
            let board = efficient_game_objects::GameState::from(board, you);

            // TODO: Create a relevant board state generator
            let relevant_moves = board.relevant_moves(u32::MAX);

            for moveset in relevant_moves {
                let mut current_board = board.clone();
                current_board.move_snakes(moveset);
            }

            Direction::Down
        }
    }
}
mod efficient_game_objects {
    use core::fmt;
    use std::cell::Ref;
    use std::cell::RefCell;
    use std::cell::RefMut;

    use crate::Battlesnake as DefaultSnake;
    use crate::Board as DefaultBoard;
    use crate::Coord;

    const X_SIZE: usize = 11;
    const Y_SIZE: usize = 11;
    const SNAKES: usize = 4;

    #[derive(Clone, Copy, Debug)]
    pub enum Direction {
        Up = 0,
        Down = 1,
        Left = 2,
        Right = 3,
    }

    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    const DIRECTION_VECTORS: [Coord; 4] = [
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
    pub struct GameState {
        board: Board,
        snakes: Snakes,
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
                    gamestate.board.set(
                        snake_part.x,
                        snake_part.y,
                        Field::SnakePart {
                            snake_number: i,
                            next: next,
                        },
                    );
                }
            }

            for i in 0..old.snakes.len() {
                gamestate
                    .snakes
                    .set(i, Some(Snake::from(&old.snakes[order[i]], i as i32)));
            }

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
            let my_head = self.snakes.get(0).as_ref().unwrap().head;

            // Determine relevant snakes based on distance
            let mut snake_relevant = [false; SNAKES];
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get(i).as_ref() {
                    if my_head.distance(&snake.head) <= distance {
                        snake_relevant[i] = true;
                    }
                }
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
                            Some(Field::SnakePart { next, .. }) => {
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
            // Hunger eliminations first
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

            // Move snake heads to set contested fields
            // Set grow on snakes that are potentially on food
            // Set die on snakes that lose on contested fields
            for (i, movement) in moveset.iter().enumerate() {
                if let Some(movement) = movement {
                    if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                        self.board.set(
                            snake.head.x,
                            snake.head.y,
                            Field::SnakePart {
                                snake_number: i,
                                next: Some(snake.head + DIRECTION_VECTORS[*movement as usize]),
                            },
                        );
                        snake.head += DIRECTION_VECTORS[*movement as usize];
                        let x = snake.head.x;
                        let y = snake.head.y;
                        match self.board.get(x, y) {
                            Some(Field::Empty) => {
                                self.board.set(
                                    x,
                                    y,
                                    Field::Contested {
                                        snake_number: i,
                                        food: false,
                                    },
                                );
                            }
                            Some(Field::Food) => {
                                snake.grow = true;
                                self.board.set(
                                    x,
                                    y,
                                    Field::Contested {
                                        snake_number: i,
                                        food: true,
                                    },
                                );
                            }
                            Some(Field::Contested { snake_number, food }) => {
                                if food {
                                    snake.grow = true;
                                }
                                if i != snake_number {
                                    if snake.length
                                        > self.snakes.get(snake_number).as_ref().unwrap().length
                                    {
                                        self.snakes.get_mut(snake_number).as_mut().unwrap().die =
                                            true;
                                        self.board.set(
                                            x,
                                            y,
                                            Field::Contested {
                                                snake_number: i,
                                                food: food,
                                            },
                                        );
                                    } else if snake.length
                                        < self.snakes.get(snake_number).as_ref().unwrap().length
                                    {
                                        snake.die = true;
                                        if i == 0 {
                                            return Result::Err(Death);
                                        }
                                        self.board.set(
                                            x,
                                            y,
                                            Field::Contested {
                                                snake_number: snake_number,
                                                food: food,
                                            },
                                        );
                                    } else {
                                        snake.die = true;
                                        if i == 0 {
                                            return Result::Err(Death);
                                        }
                                        self.snakes.get_mut(snake_number).as_mut().unwrap().die =
                                            true;
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            // Contested fields resulted in die flags
            // Contest needs to be set to snakepart of winner
            // Grow needs to be evaluated for winner and tail handled for all
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                    let x = snake.head.x;
                    let y = snake.head.y;
                    let next_tail = match self.board.get(snake.tail.x, snake.tail.y) {
                        Some(Field::SnakePart { next, .. }) => next.unwrap(),
                        _ => unreachable!(),
                    };
                    match self.board.get(x, y) {
                        Some(Field::Contested { food, snake_number }) => {
                            let food = food.clone();
                            if i != snake_number {
                                self.board.set(snake.tail.x, snake.tail.y, Field::Empty);
                            } else {
                                self.board.set(
                                    x,
                                    y,
                                    Field::SnakePart {
                                        snake_number: snake_number,
                                        next: None,
                                    },
                                );
                                if !food {
                                    self.board.set(snake.tail.x, snake.tail.y, Field::Empty);
                                }
                            }
                        }
                        _ => {
                            self.board.set(snake.tail.x, snake.tail.y, Field::Empty);
                        }
                    }
                    if !snake.grow {
                        snake.tail = next_tail
                    }
                }
            }

            // SnakePart collisions need to be resolved now after tails have been handled
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                    let x = snake.head.x;
                    let y = snake.head.y;
                    match self.board.get(x, y) {
                        Some(Field::SnakePart { snake_number, next }) => {
                            if snake_number != i || next.is_some() {
                                snake.die = true;
                                if i == 0 {
                                    return Result::Err(Death);
                                }
                            }
                        }
                        None => {
                            snake.die = true;
                            if i == 0 {
                                return Result::Err(Death);
                            }
                        }
                        _ => (),
                    }
                }
            }

            // Remove snakes again
            for i in 0..SNAKES {
                self.eliminate_dead_snake(i);
            }

            Result::Ok(())
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
            write!(f, "{}", output)
        }
    }

    #[derive(Clone)]
    struct Snakes([RefCell<Option<Snake>>; SNAKES]);

    impl Snakes {
        fn new() -> Self {
            Self(std::array::from_fn(|_| RefCell::new(None)))
        }

        fn set(&self, i: usize, snake: Option<Snake>) {
            self.0[i].replace(snake);
        }

        fn get(&self, i: usize) -> Ref<Option<Snake>> {
            self.0[i].borrow()
        }

        fn get_mut(&self, i: usize) -> RefMut<Option<Snake>> {
            self.0[i].borrow_mut()
        }
    }

    #[derive(Clone)]
    struct Board([RefCell<Field>; X_SIZE * Y_SIZE]);

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

    #[derive(Clone)]
    struct Snake {
        number: i32,
        head: Coord,
        tail: Coord,
        health: i32,
        length: i32,
        die: bool,
        grow: bool,
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

    pub struct Area {
        area: usize,
    }

    impl Area {
        fn new() -> Self {
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
                }
            );
            assert_eq!(
                gamestate.board.get(1, 0).unwrap(),
                Field::SnakePart {
                    snake_number: 0,
                    next: Some(Coord { x: 0, y: 0 })
                }
            );
            assert_eq!(
                gamestate.board.get(2, 0).unwrap(),
                Field::SnakePart {
                    snake_number: 0,
                    next: Some(Coord { x: 1, y: 0 })
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
    }
}

mod mocks {}
