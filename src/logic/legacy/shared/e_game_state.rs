use super::{
    e_board::{EBoard, EField, X_SIZE, Y_SIZE},
    e_coord::ECoord,
    e_direction::{EDirection, EDIRECTION_VECTORS},
    e_snakes::{ESimulationError, ESnake, ESnakes, Result, SNAKES},
};
use crate::{Battlesnake, Board};
use core::{fmt, panic};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    u8,
};

#[derive(Clone, Copy)]
pub struct EStateRating {
    pub snakes_alive: u8,
    pub current_length: u8,
    pub food_distance: u8,
    pub middle_distance: u8,
}

impl EStateRating {
    pub fn from(state: &EGameState) -> Self {
        let snakes_alive = state.snakes.count_alive();
        let current_length = state.snakes.get(0).as_ref().unwrap().length;
        let food_distance = state.food_distance().unwrap_or(u8::MAX);
        let middle_distance = state.middle_distance();
        Self {
            snakes_alive,
            current_length,
            food_distance,
            middle_distance,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EGameState {
    pub board: EBoard,
    pub snakes: ESnakes,
}

impl Default for EGameState {
    fn default() -> Self {
        Self::new()
    }
}

impl EGameState {
    pub fn new() -> Self {
        EGameState {
            board: EBoard::new(),
            snakes: ESnakes::new(),
        }
    }

    pub fn from(old: &Board, you: &Battlesnake) -> Self {
        let gamestate = Self::new();

        for food in old.food.iter() {
            gamestate
                .board
                .set(food.x as i8, food.y as i8, EField::Food);
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
                    Some(old.snakes[order[i]].body[j - 1])
                };
                match gamestate.board.get(snake_part.x as i8, snake_part.y as i8) {
                    Some(EField::SnakePart {
                        snake_number,
                        stacked,
                        next,
                    }) => gamestate.board.set(
                        snake_part.x as i8,
                        snake_part.y as i8,
                        EField::SnakePart {
                            snake_number,
                            next,
                            stacked: stacked + 1,
                        },
                    ),
                    _ => gamestate.board.set(
                        snake_part.x as i8,
                        snake_part.y as i8,
                        EField::SnakePart {
                            snake_number: i as u8,
                            next: next.map(|coord| ECoord::from(coord.x as i8, coord.y as i8)),
                            stacked: 1,
                        },
                    ),
                };
            }
        }

        for i in 0u8..old.snakes.len() as u8 {
            gamestate
                .snakes
                .set(i, Some(ESnake::from(&old.snakes[order[i as usize]])));
        }

        gamestate.validate_state();

        gamestate
    }

    pub fn rate_state(&self) -> EStateRating {
        EStateRating::from(self)
    }

    pub fn relevant_moves(&self, distance: u8) -> Vec<[Option<EDirection>; SNAKES as usize]> {
        let mut snake_relevant = [false; SNAKES as usize];
        if let Some(my_snake) = self.snakes.get(0).as_ref() {
            // Determine relevant snakes based on distance
            for i in 0..SNAKES {
                if let Some(snake) = self.snakes.get(i).as_ref() {
                    if my_snake.head.distance(&snake.head) <= distance {
                        snake_relevant[i as usize] = true;
                    }
                }
            }
        } else {
            return Vec::new();
        }

        // Determine "dangerous" move combinations of relevant snakes where they do not do stupid stuff
        let mut dangerous_moves = [[false; 4]; SNAKES as usize];
        for snake_index in 0..SNAKES {
            if snake_relevant[snake_index as usize] {
                for d in 0..4 {
                    let new_head_candidate =
                        self.snakes.get(snake_index).as_ref().unwrap().head + EDIRECTION_VECTORS[d];
                    match self.board.get(new_head_candidate.x, new_head_candidate.y) {
                        Some(EField::Empty) | Some(EField::Food) => {
                            dangerous_moves[snake_index as usize][d] = true;
                        }
                        Some(EField::SnakePart { snake_number, .. }) => {
                            let tail = self.snakes.get(snake_number).as_ref().unwrap().tail;
                            if tail.x == new_head_candidate.x && tail.y == new_head_candidate.y {
                                dangerous_moves[snake_index as usize][d] = true;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        // Set at least one move to dangerous for relevant snakes if they do not have any
        // This guarantees that the snake dies in the simulations
        for i in 0..SNAKES {
            if snake_relevant[i as usize] {
                let mut has_valid_move = false;
                for mve in dangerous_moves[i as usize] {
                    if mve {
                        has_valid_move = true;
                    }
                }
                if !has_valid_move {
                    dangerous_moves[i as usize][0] = true;
                }
            }
        }

        // Get the count of actually relevant snake move combinations
        let mut relevant_count = [0; SNAKES as usize];
        for i in 0..SNAKES {
            relevant_count[i as usize] =
                dangerous_moves[i as usize]
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
        let mut move_combinations: Vec<[Option<EDirection>; SNAKES as usize]> =
            vec![[None, None, None, None]; final_count];
        let mut pattern_repeat = 1;
        let mut move_repeat = final_count;
        for snake_index in 0..SNAKES {
            if relevant_count[snake_index as usize] == 0 {
                continue;
            }
            move_repeat /= relevant_count[snake_index as usize];
            let mut move_index = 0;
            for p in 0..pattern_repeat {
                for current_valid_move_count in 0..relevant_count[snake_index as usize] {
                    loop {
                        if dangerous_moves[snake_index as usize][move_index] {
                            break;
                        }
                        move_index += 1;
                    }
                    for m in 0..move_repeat {
                        let final_position = p * move_repeat * relevant_count[snake_index as usize]
                            + move_repeat * current_valid_move_count
                            + m;
                        move_combinations[final_position][snake_index as usize] =
                            Some(match move_index {
                                0 => EDirection::Up,
                                1 => EDirection::Down,
                                2 => EDirection::Left,
                                3 => EDirection::Right,
                                _ => unreachable!(),
                            });
                    }
                    move_index += 1;
                }
                move_index = 0;
            }
            pattern_repeat *= relevant_count[snake_index as usize];
        }

        move_combinations
    }

    fn eliminate_dead_snakes(&self) -> Result<()> {
        for i in 0..SNAKES {
            self.eliminate_dead_snake(i)?
        }
        Ok(())
    }

    fn eliminate_dead_snake(&self, snake_index: u8) -> Result<()> {
        let mut eliminate = false;
        if let Some(snake) = self.snakes.get(snake_index).as_ref() {
            if snake._die && !snake._far_away {
                if snake_index == 0 {
                    return Err(ESimulationError::Death);
                }
                eliminate = true;
                let mut x = snake.tail.x;
                let mut y = snake.tail.y;
                loop {
                    match self.board.get(x, y) {
                        Some(EField::SnakePart {
                            next, snake_number, ..
                        }) if snake_number == snake_index => {
                            self.board.set(x, y, EField::Empty);
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
        Ok(())
    }

    /// Grows the snake
    /// No health handling
    fn grow_snake(&self, snake: &mut ESnake) {
        snake.length += 1;
        match self.board.get(snake.tail.x, snake.tail.y) {
            Some(EField::SnakePart {
                snake_number,
                stacked,
                next,
            }) => self.board.set(
                snake.tail.x,
                snake.tail.y,
                EField::SnakePart {
                    snake_number,
                    stacked: stacked + 1,
                    next,
                },
            ),
            _ => unreachable!("Invalid tail"),
        };
    }

    pub fn move_snakes(
        &mut self,
        moveset: [Option<EDirection>; SNAKES as usize],
        distance: u8,
        hunger: bool,
    ) -> Result<()> {
        self.set_snakes_far_away(distance, true);
        if hunger {
            self.handle_hunger(&moveset)?;
        }
        self.move_tails();
        self.move_heads(&moveset)?;
        self.set_snakes_far_away(distance, false);
        Ok(())
    }

    fn set_snakes_far_away(&mut self, distance: u8, far_away: bool) {
        let my_head = self.snakes.get(0).as_ref().unwrap().head;

        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                snake._far_away = far_away && my_head.distance(&snake.head) > distance;
            }
        }
    }

    /// handle only hunger eliminations
    /// growth is not handled
    /// board afterwards contains snakes as they were before, but snakes that died of hunger are eliminated
    fn handle_hunger(&mut self, moveset: &[Option<EDirection>; SNAKES as usize]) -> Result<()> {
        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                if snake.health >= 1 {
                    snake.health -= 1;
                }
                if let Some(movement) = moveset[i as usize] {
                    let new_head = snake.head.move_in_direction(movement);
                    if let Some(EField::Food) = self.board.get(new_head.x, new_head.y) {
                        snake.health = 100;
                    }
                }
                if snake.health == 0 {
                    snake._die = true;
                }
            }
        }
        self.eliminate_dead_snakes()?;
        Ok(())
    }

    /// move tails
    /// tail movement is independent of food consumption in the same round
    /// food consumption will add a stack to new tail later on
    pub fn move_tails(&mut self) {
        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                let tail_field = self.board.get(snake.tail.x, snake.tail.y);
                if let Some(EField::SnakePart {
                    stacked,
                    next,
                    snake_number,
                }) = tail_field
                {
                    if stacked > 1 {
                        self.board.set(
                            snake.tail.x,
                            snake.tail.y,
                            EField::SnakePart {
                                snake_number,
                                stacked: stacked - 1,
                                next,
                            },
                        );
                    } else {
                        self.board.set(snake.tail.x, snake.tail.y, EField::Empty);
                        if let Some(next) = next {
                            snake.tail = next;
                        }
                    }
                }
            }
        }
    }

    /// responsible for moving heads
    /// tails and hunger eliminations should be done before
    /// responsible for handling growth by stacking (already new) tail
    fn move_heads(&mut self, moveset: &[Option<EDirection>; SNAKES as usize]) -> Result<()> {
        for i in 0..SNAKES {
            if let Some(direction) = moveset[i as usize] {
                if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                    let new_head = snake.head.move_in_direction(direction);
                    // update old head field's next field
                    // if it points to weird stuff, kill the snake
                    match self.board.get(snake.head.x, snake.head.y) {
                        Some(EField::SnakePart {
                            snake_number,
                            stacked,
                            ..
                        }) => {
                            self.board.set(
                                snake.head.x,
                                snake.head.y,
                                EField::SnakePart {
                                    snake_number,
                                    stacked,
                                    next: Some(new_head),
                                },
                            );
                        }
                        _ => snake._die = true,
                    }
                    // handle new snake head
                    // set contested
                    match self.board.get(new_head.x, new_head.y) {
                        Some(EField::Empty) => {
                            self.board.set(
                                new_head.x,
                                new_head.y,
                                EField::Contested {
                                    snake_number: i,
                                    food: false,
                                },
                            );
                        }
                        Some(EField::Food) => {
                            self.board.set(
                                new_head.x,
                                new_head.y,
                                EField::Contested {
                                    snake_number: i,
                                    food: true,
                                },
                            );
                        }
                        Some(EField::SnakePart { .. }) => {
                            snake._die = true;
                        }
                        Some(EField::Contested { snake_number, food }) => {
                            if let Some(other_snake) = self.snakes.get_mut(snake_number).as_mut() {
                                if snake.length > other_snake.length {
                                    other_snake._die = true;
                                    self.board.set(
                                        new_head.x,
                                        new_head.y,
                                        EField::Contested {
                                            snake_number: i,
                                            food,
                                        },
                                    );
                                } else if snake.length < other_snake.length {
                                    snake._die = true;
                                } else {
                                    snake._die = true;
                                    other_snake._die = true;
                                }
                            }
                        }
                        Some(EField::Capture { .. }) => {
                            snake._die = true;
                        }
                        None => snake._die = true,
                        _ => panic!("Invalid state while moving heads"),
                    }
                    snake.head = new_head;
                }
            }
        }

        self.eliminate_dead_snakes()?;

        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                if !snake._far_away {
                    if let Some(EField::Contested { snake_number, food }) =
                        self.board.get(snake.head.x, snake.head.y)
                    {
                        if snake_number == i {
                            if food {
                                self.grow_snake(snake);
                            }
                            self.board.set(
                                snake.head.x,
                                snake.head.y,
                                EField::SnakePart {
                                    snake_number: i,
                                    stacked: 1,
                                    next: None,
                                },
                            );
                        } else {
                            panic!("Contested field does not match the snake")
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_state(&self) {
        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get(i).as_ref() {
                match self.board.get(snake.head.x, snake.head.y) {
                    Some(EField::SnakePart {
                        snake_number, next, ..
                    }) => {
                        if next.is_some() || snake_number != i {
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
                    Some(EField::SnakePart { snake_number, .. }) => {
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

    pub fn collision_point(&self, start: ECoord, direction: EDirection) -> ECoord {
        let mut current = start;
        loop {
            let next = current + EDIRECTION_VECTORS[direction.to_usize()];
            match self.board.get(next.x, next.y) {
                Some(EField::Empty) | Some(EField::Food) => {
                    current = next;
                }
                _ => break,
            }
        }
        current
    }

    pub fn trajectories(&self) -> [Option<EDirection>; SNAKES as usize] {
        let mut trajectories = [None; SNAKES as usize];
        for i in 0..SNAKES {
            trajectories[i as usize] = self.trajectory(i);
        }
        trajectories
    }

    pub fn trajectory(&self, snake_index: u8) -> Option<EDirection> {
        if let Some(snake) = self.snakes.get(snake_index).as_ref() {
            let tail = snake.tail;

            let mut snake_part_coords = Vec::with_capacity(snake.length as usize);
            let mut current = tail;
            loop {
                snake_part_coords.push(current);
                match self.board.get(current.x, current.y) {
                    Some(EField::SnakePart { next, .. }) => {
                        if let Some(next) = next {
                            current = next;
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }

            let mut direction_counts = [0; 4];
            let l = snake_part_coords.len();
            for i in ((l as i32 - 3).max(1) as usize..l).rev() {
                let d = EDirection::from_coords(snake_part_coords[i - 1], snake_part_coords[i]);
                if let Some(d) = d {
                    direction_counts[d.to_usize()] += 1;
                }
            }
            for i in 0..4 {
                if direction_counts[i] >= 2 {
                    return Some(EDirection::from_usize(i));
                }
            }
            if l >= 2 {
                EDirection::from_coords(snake_part_coords[l - 2], snake_part_coords[l - 1])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn execute_capture(&mut self) -> CaptureResult {
        let mut count_result = self.count_captures();
        let mut iterations = 1;
        loop {
            self.capture_iteration();
            let new_count_result = self.count_captures();
            if count_result.uncontested == new_count_result.uncontested {
                break;
            }
            count_result = new_count_result;
            iterations += 1;
        }
        CaptureResult {
            fields: count_result.captures,
            iterations,
        }
    }

    fn capture_in_direction(&mut self, direction: EDirection) -> Option<CaptureResult> {
        match self.initialize_capture(direction) {
            Ok(_) => Some(self.execute_capture()),
            Err(_) => None,
        }
    }

    pub fn capture(&self) -> [Option<CaptureResult>; 4] {
        let mut results = [None; 4];
        for i in 0..4 {
            let mut state = self.clone();
            results[i] = state.capture_in_direction(EDirection::from_usize(i));
        }
        results
    }

    /// One capture iteration including tail movement at the beginning
    fn capture_iteration(&mut self) {
        self.move_tails();
        let new_board = self.board.clone();
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match self.board.get(x, y) {
                    Some(EField::Empty) | Some(EField::Food) => {
                        for d in 0..4 {
                            let neighbor = ECoord::from(x, y) + EDIRECTION_VECTORS[d];
                            if let Some(EField::Capture {
                                snake_number,
                                length,
                                ..
                            }) = self.board.get(neighbor.x, neighbor.y)
                            {
                                match new_board.get(x, y) {
                                    Some(EField::Capture {
                                        length: already_captured_length,
                                        snake_number: already_captured_snake_number,
                                        changeable: true,
                                    }) => {
                                        if already_captured_snake_number != snake_number {
                                            if already_captured_length < length {
                                                new_board.set(
                                                    x,
                                                    y,
                                                    EField::Capture {
                                                        snake_number,
                                                        length,
                                                        changeable: true,
                                                    },
                                                );
                                            } else if already_captured_length == length {
                                                new_board.set(
                                                    x,
                                                    y,
                                                    EField::Capture {
                                                        snake_number: None,
                                                        length,
                                                        changeable: true,
                                                    },
                                                );
                                            } // else: no change
                                        }
                                    }
                                    Some(EField::Empty) | Some(EField::Food) => {
                                        new_board.set(
                                            x,
                                            y,
                                            EField::Capture {
                                                snake_number,
                                                length,
                                                changeable: true,
                                            },
                                        );
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        self.board = new_board;
        self.set_changeable_for_captures(false);
    }

    /// Counts the number of captures for each snake and the blocked fields
    fn count_captures(&self) -> CaptureCount {
        let mut capture_count = CaptureCount {
            captures: [0; SNAKES as usize],
            contested: 0,
            uncontested: 0,
            snakeparts: 0,
        };
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match self.board.get(x, y) {
                    Some(EField::Capture { snake_number, .. }) => {
                        if let Some(n) = snake_number {
                            capture_count.captures[n as usize] += 1;
                        } else {
                            capture_count.contested += 1;
                        }
                    }
                    Some(EField::Empty) | Some(EField::Food) => capture_count.uncontested += 1,
                    Some(EField::SnakePart { .. }) => capture_count.snakeparts += 1,
                    _ => (),
                }
            }
        }
        capture_count
    }

    /// Sets initial capture fields for own and other snakes
    fn initialize_other_captures(&mut self, changeable: bool) {
        for i in 1..SNAKES {
            if let Some(snake) = self.snakes.get(i).as_ref() {
                for d in 0..4 {
                    let start = snake.head + EDIRECTION_VECTORS[d];
                    match self.board.get(start.x, start.y) {
                        Some(EField::Empty) | Some(EField::Food) => {
                            self.board.set(
                                start.x,
                                start.y,
                                EField::Capture {
                                    snake_number: Some(i),
                                    length: snake.length,
                                    changeable,
                                },
                            );
                        }
                        Some(EField::Capture {
                            length,
                            changeable: true,
                            ..
                        }) => {
                            if length < snake.length {
                                self.board.set(
                                    start.x,
                                    start.y,
                                    EField::Capture {
                                        snake_number: Some(i),
                                        length: snake.length,
                                        changeable,
                                    },
                                );
                            } else if length == snake.length {
                                self.board.set(
                                    start.x,
                                    start.y,
                                    EField::Capture {
                                        snake_number: None,
                                        length,
                                        changeable,
                                    },
                                );
                            } // else: no change
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    /// Sets initial capture fields for own and other snakes
    fn initialize_capture(&mut self, direction: EDirection) -> Result<()> {
        self.move_tails();
        self.initialize_own_capture(direction, true)?;
        self.initialize_other_captures(true);
        Ok(())
    }

    /// Sets initial capture fields for own snake
    fn initialize_own_capture(&mut self, direction: EDirection, changeable: bool) -> Result<()> {
        match self.snakes.get_mut(0).as_mut() {
            Some(own_snake) => {
                let start = own_snake.head + EDIRECTION_VECTORS[direction.to_usize()];
                match self.board.get(start.x, start.y) {
                    Some(EField::Empty) => {
                        self.board.set(
                            start.x,
                            start.y,
                            EField::Capture {
                                snake_number: Some(0),
                                length: own_snake.length,
                                changeable,
                            },
                        );
                    }
                    Some(EField::Food) => {
                        self.board.set(
                            start.x,
                            start.y,
                            EField::Capture {
                                snake_number: Some(0),
                                length: own_snake.length,
                                changeable,
                            },
                        );
                    }
                    _ => return Err(ESimulationError::Death), // Invalid start field
                }
            }
            _ => panic!("No own snake found"),
        }
        Ok(())
    }

    /// Set changeable for the existing captures
    fn set_changeable_for_captures(&mut self, changeable: bool) {
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                if let Some(EField::Capture {
                    snake_number,
                    length,
                    ..
                }) = self.board.get(x, y)
                {
                    self.board.set(
                        x,
                        y,
                        EField::Capture {
                            snake_number,
                            length,
                            changeable,
                        },
                    );
                }
            }
        }
    }

    fn relevant_states_after_move_in_direction(
        &self,
        direction: EDirection,
        distance: u8,
        possible_move_sets: &Vec<[Option<EDirection>; SNAKES as usize]>,
    ) -> Result<Vec<Self>> {
        let mut new_relevant_states = Vec::new();
        let relevant_move_sets = possible_move_sets
            .iter()
            .filter(|x| x[0].unwrap() == direction);
        let mut relevant_move_set_exists = false;
        for current_move_set in relevant_move_sets {
            relevant_move_set_exists = true;
            let mut new_state = self.clone();
            new_state.move_snakes(*current_move_set, distance, true)?;
            new_relevant_states.push(new_state);
        }
        if !relevant_move_set_exists {
            Err(ESimulationError::Death)
        } else {
            Ok(new_relevant_states)
        }
    }

    pub fn calculate_relevant_states_after_move(
        &self,
        distance: u8,
        evaluate_direction: [bool; 4],
    ) -> [Result<Vec<Self>>; 4] {
        let mut results = [
            Err(ESimulationError::NotEvaluated),
            Err(ESimulationError::NotEvaluated),
            Err(ESimulationError::NotEvaluated),
            Err(ESimulationError::NotEvaluated),
        ];
        let possible_move_sets = self.relevant_moves(distance);
        for i in 0..4 {
            if evaluate_direction[i] {
                results[i] = self.relevant_states_after_move_in_direction(
                    EDirection::from_usize(i),
                    distance,
                    &possible_move_sets,
                );
            }
        }
        results
    }

    pub fn hash_for_pruning(&self, distance: u8) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.snakes.hash(&mut hasher);
        let my_head = self.snakes.get(0).as_ref().unwrap().head;
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                if my_head.distance(&ECoord::from(x, y)) <= distance {
                    self.board.get(x, y).hash(&mut hasher);
                }
            }
        }
        hasher.finish()
    }

    pub fn food_distance(&self) -> Option<u8> {
        let my_head = self.snakes.get(0).as_ref().unwrap().head;
        let mut min_distance = None;
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                if let Some(EField::Food) = self.board.get(x, y) {
                    let distance = my_head.distance(&ECoord::from(x, y));
                    if distance < min_distance.unwrap_or(u8::MAX) {
                        min_distance = Some(distance);
                    }
                }
            }
        }
        min_distance
    }

    pub fn middle_distance(&self) -> u8 {
        let my_head = self.snakes.get(0).as_ref().unwrap().head;
        let middle = ECoord::from(X_SIZE / 2, Y_SIZE / 2);
        my_head.distance(&middle)
    }
}

impl fmt::Display for EGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = String::with_capacity((X_SIZE + 1) as usize * Y_SIZE as usize);
        for y in (0..Y_SIZE).rev() {
            for x in 0..X_SIZE {
                if let Some(state) = self.board.get(x, y) {
                    output.push(match state {
                        EField::Empty => '.',
                        EField::Food => '+',
                        EField::SnakePart {
                            snake_number, next, ..
                        } => {
                            if next.is_some() {
                                char::from_u32(snake_number as u32 + 97).unwrap_or('?')
                            } else {
                                char::from_u32(snake_number as u32 + 65).unwrap_or('?')
                            }
                        }
                        EField::Filled => 'X',
                        EField::Contested { .. } => '&',
                        EField::Capture { snake_number, .. } => {
                            if snake_number.is_some() {
                                char::from_digit(snake_number.unwrap() as u32, 10).unwrap_or('?')
                            } else {
                                'X'
                            }
                        }
                    });
                    output.push(' ');
                }
            }
            output.push('\n')
        }
        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get(i).as_ref() {
                let next_tail = match self.board.get(snake.tail.x, snake.tail.y) {
                    Some(EField::SnakePart { next, .. }) => next.unwrap_or(ECoord { x: -1, y: -1 }),
                    _ => ECoord { x: -1, y: -1 },
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

#[derive(Clone, Copy, Debug)]
pub struct CaptureResult {
    pub fields: [u8; SNAKES as usize],
    #[allow(dead_code)]
    pub iterations: u8,
}

#[derive(Clone, Copy, Debug)]
struct CaptureCount {
    pub captures: [u8; SNAKES as usize],
    pub contested: u8,
    pub uncontested: u8,
    pub snakeparts: u8,
}

#[cfg(test)]
mod tests {

    use crate::read_game_state;

    use super::*;

    #[bench]
    fn bench_next_state(b: &mut test::Bencher) {
        let game_state = read_game_state("requests/test_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let moves = [
            Some(EDirection::Up),
            Some(EDirection::Left),
            Some(EDirection::Left),
            Some(EDirection::Down),
        ];
        b.iter(|| {
            let mut new_board = board.clone();
            new_board.move_snakes(moves, u8::MAX, true).unwrap();
        });
    }

    #[bench]
    fn bench_possible_moves(b: &mut test::Bencher) {
        let game_state = read_game_state("requests/test_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        b.iter(|| {
            let _ = board.relevant_moves(u8::MAX);
        });
    }

    #[test]
    fn test_print_capture_iteration() {
        let game_state = read_game_state("requests/failure_21_bait_into_trap_with_top_wall.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        board.initialize_capture(EDirection::Up).unwrap();
        println!("{}", &board);
        board.capture_iteration();
        println!("{}", &board);
        board.capture_iteration();
        println!("{}", &board);
    }

    #[test]
    fn test_print_capture() {
        let game_state = read_game_state("requests/failure_21_bait_into_trap_with_top_wall.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let result = board.capture();
        println!("{:?}", result);
    }

    #[test]
    fn test_print_capture_in_direction() {
        let game_state =
            read_game_state("requests/failure_42_going_right_enables_getting_killed.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let result = board.capture_in_direction(EDirection::Right);
        println!("{}", &board);
        println!("{:?}", result);
    }
}
