use core::fmt;
use core::panic;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;
use std::time::Instant;

use crate::Battlesnake as DefaultSnake;
use crate::Board as DefaultBoard;
use crate::Coord;

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
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[allow(dead_code)]
    pub fn from(v: Vec<Direction>) -> Self {
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
    pub fn from(valid_states: Vec<GameState>) -> DirectionNode {
        Self {
            states: valid_states,
            evaluated: [false; 4],
        }
    }

    pub fn calc_next(&mut self, to: Direction, distance: u32) -> Result<DirectionNode> {
        self.evaluated[to.to_usize()] = true;
        let mut new_valid_states = Vec::new();
        for state in self.states.iter() {
            let relevant_moves = state.relevant_moves(distance);
            if relevant_moves.len() == 0 {
                return Result::Err(Death);
            }
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

    pub fn calc(&mut self, from: DirectionVec, to: Direction, distance: u32) -> Result<()> {
        let mut delete = false;
        let result;
        let calc_next_result: Option<DirectionNode>;
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
                    let bools = self.calcs(
                        d_vec.clone(),
                        0.max(distance as i32 - d_vec.len() as i32) as u32,
                    );
                    // println!("{:?} {:?}", &d_vec, &bools);
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

        for key in self.map.keys().rev() {
            // if self.map.get(key).unwrap().is_some() {
            //    println!("{:?} {}", key, self.map.get(key).unwrap().clone().unwrap())
            // }
            if key.len() == 0 {
                break;
            } else if result[key[0].to_usize()] < key.len() {
                result[key[0].to_usize()] = key.len();
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
        GameState {
            board: Board::new(),
            snakes: Snakes::new(),
        }
    }

    pub fn from(old: &DefaultBoard, you: &DefaultSnake) -> Self {
        let gamestate = Self::new();

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
                            while candidate == Some(Field::Empty) || candidate == Some(Field::Food)
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
                        while candidate == Some(Field::Empty) || candidate == Some(Field::Food) {
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
                        self.snakes.get(snake_index).as_ref().unwrap().head + DIRECTION_VECTORS[d];
                    match self.board.get(new_head_candidate.x, new_head_candidate.y) {
                        Some(Field::Empty) | Some(Field::Food) => {
                            dangerous_moves[snake_index][d] = true;
                        }
                        Some(Field::SnakePart { snake_number, .. }) => {
                            let tail = self.snakes.get(snake_number).as_ref().unwrap().tail;
                            if tail.x == new_head_candidate.x && tail.y == new_head_candidate.y {
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
            relevant_count[i] = dangerous_moves[i]
                .iter()
                .fold(0, |acc, e| if *e { acc + 1 } else { acc });
        }
        let mut relevant_move_found = false;
        for count in relevant_count {
            if count != 0 {
                relevant_move_found = true;
            }
        }
        if !relevant_move_found || relevant_count[0] == 0 {
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
                        move_combinations[final_position][snake_index] = Some(match move_index {
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
                    Some(Field::SnakePart { next, .. }) => next.unwrap_or(Coord { x: -1, y: -1 }),
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
        let board = GameState::from(&game_state.board, &game_state.you);
        println!("{board}");
    }

    #[test]
    fn print_board_3_after_move() {
        let game_state = read_game_state("requests/failure_1.json");
        let mut board = GameState::from(&game_state.board, &game_state.you);
        println!("{board}");
        board
            .move_snakes([
                Some(Direction::Down),
                Some(Direction::Up),
                Some(Direction::Down),
                None,
            ])
            .unwrap();
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
    fn failure_2() {
        let game_state = read_game_state("requests/failure_2.json");
        let board = GameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let mut d_tree = DirectionTree::from(board);
        let result = d_tree.simulate_timed(u32::MAX, 200);
        println!("{:?}", result);
    }

    #[test]
    fn failure_4() {
        let game_state = read_game_state("requests/failure_4.json");
        let board = GameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let mut d_tree = DirectionTree::from(board);
        let result = d_tree.simulate_timed(u32::MAX, 200);
        println!("{:?}", result);
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
