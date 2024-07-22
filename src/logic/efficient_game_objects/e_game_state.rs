use core::{fmt, panic};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    result::Result as StdResult,
    time::{Duration, Instant},
};

use env_logger::init;

use crate::{Battlesnake, Board};

use super::{
    e_board::{EArea, EBoard, EField, X_SIZE, Y_SIZE},
    e_coord::ECoord,
    e_direction::{EDirection, EDIRECTION_VECTORS},
    e_snakes::{ESimulationError, ESnake, ESnakes, Result, SNAKES},
};

#[derive(Clone, Copy)]
pub struct EStateRating {
    pub snakes: u8,
}

impl EStateRating {
    pub fn new() -> Self {
        Self { snakes: u8::MAX }
    }

    pub fn from(state: &EGameState) -> Self {
        let mut rating = Self::new();
        rating.snakes = 0;
        for i in 0..SNAKES {
            if state.snakes.get(i).as_ref().is_some() {
                rating.snakes += 1;
            }
        }
        rating
    }
}

#[derive(Clone)]
pub struct EGameState {
    pub board: EBoard,
    pub snakes: ESnakes,
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
                    Some(old.snakes[order[i]].body[j - 1].clone())
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
                            next: if let Some(coord) = next {
                                Some(ECoord::from(coord.x as i8, coord.y as i8))
                            } else {
                                None
                            },
                            stacked: 1,
                        },
                    ),
                };
            }
        }

        for i in 0u8..old.snakes.len() as u8 {
            gamestate.snakes.set(
                i,
                Some(ESnake::from(&old.snakes[order[i as usize]], i as i32)),
            );
        }

        gamestate.validate_state();

        gamestate
    }

    pub fn rate_state(&self) -> EStateRating {
        EStateRating::from(self)
    }

    pub fn advanced_fill(&mut self, start: &ECoord) -> Option<EArea> {
        let area = self.board.fill(start);
        match area {
            Some(area) => {
                let mut new_area = area.clone();
                self.add_opening_times(&mut new_area);
                return Some(new_area);
            }
            None => return None,
        }
    }

    /// calculate and add opening times to EArea
    ///
    /// fill must have been called before such that board has filled fields
    fn add_opening_times(&self, area: &mut EArea) {
        let mut border_coordinates = HashSet::new();
        let mut relevant_snakes = [false; SNAKES as usize];
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match self.board.get(x, y) {
                    Some(EField::Filled) => {
                        for d_vec in EDIRECTION_VECTORS {
                            let test_x = x + d_vec.x;
                            let test_y = y + d_vec.y;
                            match self.board.get(test_x, test_y) {
                                Some(EField::SnakePart { snake_number, .. }) => {
                                    relevant_snakes[snake_number as usize] = true;
                                    border_coordinates.insert(ECoord::from(test_x, test_y));
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        let mut opening_times: [Option<u8>; SNAKES as usize] = [None; SNAKES as usize];
        for s_index in 0..relevant_snakes.len() {
            if relevant_snakes[s_index] {
                match self.snakes.get(s_index as u8).as_ref() {
                    Some(snake) => {
                        let mut current = snake.tail;
                        let mut time_to_open = 0;
                        loop {
                            match self.board.get(current.x, current.y) {
                                Some(EField::SnakePart { stacked, next, .. }) => {
                                    if border_coordinates.contains(&current) {
                                        opening_times[s_index] = Some(time_to_open);
                                        break;
                                    } else {
                                        time_to_open += stacked as u8;
                                        current = next.unwrap(); // must hit before None as only neighboring snakes are selected
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        area.opening_times_by_snake = opening_times;
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
            if snake.die && !snake.far_away {
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
                            let next = next.clone();
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
                if far_away && my_head.distance(&snake.head) > distance {
                    snake.far_away = true;
                } else {
                    snake.far_away = false;
                }
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
                    match self.board.get(new_head.x, new_head.y) {
                        Some(EField::Food) => {
                            snake.health = 100;
                        }
                        _ => (),
                    }
                }
                if snake.health <= 0 {
                    snake.die = true;
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
                match tail_field {
                    Some(EField::SnakePart {
                        stacked,
                        next,
                        snake_number,
                    }) => {
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
                    _ => (),
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
                        _ => snake.die = true,
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
                            snake.die = true;
                        }
                        Some(EField::Contested { snake_number, food }) => {
                            if let Some(other_snake) = self.snakes.get_mut(snake_number).as_mut() {
                                if snake.length > other_snake.length {
                                    other_snake.die = true;
                                    self.board.set(
                                        new_head.x,
                                        new_head.y,
                                        EField::Contested {
                                            snake_number: i,
                                            food,
                                        },
                                    );
                                } else if snake.length < other_snake.length {
                                    snake.die = true;
                                } else {
                                    snake.die = true;
                                    other_snake.die = true;
                                }
                            }
                        }
                        Some(EField::Capture { .. }) => {
                            snake.die = true;
                        }
                        None => snake.die = true,
                        _ => panic!("Invalid state while moving heads"),
                    }
                    snake.head = new_head;
                }
            }
        }

        self.eliminate_dead_snakes()?;

        for i in 0..SNAKES {
            if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                if !snake.far_away {
                    match self.board.get(snake.head.x, snake.head.y) {
                        Some(EField::Contested { snake_number, food }) => {
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
                        _ => (),
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
                return EDirection::from_coords(snake_part_coords[l - 2], snake_part_coords[l - 1]);
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn timed_capture(&self, duration: Duration) -> [(usize, usize); 4] {
        let start = Instant::now();
        let mut work_queue: VecDeque<Vec<EDirection>> = VecDeque::new();
        work_queue.push_back(Vec::new());
        let mut done_map: HashMap<Vec<EDirection>, EGameState> = HashMap::new();
        let mut others_initialized_board = self.clone();
        others_initialized_board.initialize_other_captures(true);
        done_map.insert(Vec::new(), others_initialized_board);
        let mut done_scores: HashMap<Vec<EDirection>, Vec<u8>> = HashMap::new();
        done_scores.insert(Vec::new(), Vec::new());

        let mut finished_round = 0;

        while start.elapsed() < duration {
            match work_queue.pop_front() {
                Some(moves) => {
                    let old_state = done_map.get(&moves).unwrap().clone();
                    finished_round = 0.max(moves.len() as isize - 1);
                    for d in 0..4 {
                        let direction = EDirection::from_usize(d);
                        let mut new_state = old_state.clone();
                        let mut new_state_to_store = old_state.clone();
                        new_state.move_tails();
                        let initialize_result = new_state.initialize_own_capture(direction, true);
                        let capture_result = new_state.capture();
                        if initialize_result.is_ok()
                            && new_state_to_store
                                .move_heads(&[Some(direction), None, None, None])
                                .is_ok()
                        {
                            new_state_to_store.capture_iteration();
                            let mut new_moves = moves.clone();
                            new_moves.push(direction);
                            // println!("{:?}", &new_moves);
                            // println!("{}", &new_state_to_store);
                            done_map.insert(new_moves.clone(), new_state_to_store);
                            work_queue.push_back(new_moves.clone());
                            let mut new_scores = done_scores.get(&moves).unwrap().clone();
                            new_scores.push(capture_result[0]);
                            done_scores.insert(new_moves, new_scores);
                        } else {
                            let mut new_moves = moves.clone();
                            new_moves.push(direction);
                            // println!("Invalid move {:?}", &new_moves);
                        }
                    }
                }
                None => break,
            }
        }
        //sort done scores by value array
        let mut done_scores_vec: Vec<(&Vec<EDirection>, &Vec<u8>)> = done_scores
            .iter()
            .filter(|x| x.0.len() <= finished_round as usize)
            .collect();
        done_scores_vec.sort_by(|a, b| {
            if a.1.len() != b.1.len() {
                return b.1.len().cmp(&a.1.len());
            }
            let a_sum = a.1.iter().map(|x| *x as usize).sum::<usize>();
            let b_sum = b.1.iter().map(|x| *x as usize).sum::<usize>();
            b_sum.cmp(&a_sum)
        });

        let mut result = [(0, 0); 4];
        for (k, v) in done_scores_vec.iter() {
            if k.len() > 0 {
                let d = k[0].to_usize();
                if result[d].0 == 0 {
                    result[d] = (k.len(), v.iter().map(|x| *x as usize).sum::<usize>());
                }
            }
        }

        // println!("{:?}", done_scores_vec);

        result
    }

    pub fn capture(&mut self) -> [u8; SNAKES as usize + 1] {
        let number_of_fields: u8 = (Y_SIZE * X_SIZE) as u8;
        let mut captures = self.count_captures();
        // println!("Capture call");
        // println!("{}", self);
        while captures.iter().sum::<u8>() < number_of_fields {
            self.capture_iteration();
            let new_captures = self.count_captures();
            captures = new_captures;
            // println!("{}", &self);
            // println!("{:?}", &captures);
        }
        captures
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
                            match self.board.get(neighbor.x, neighbor.y) {
                                Some(EField::Capture {
                                    snake_number,
                                    length,
                                    ..
                                }) => {
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
                                _ => (),
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
    fn count_captures(&self) -> [u8; SNAKES as usize + 1] {
        let mut snake_captures = [0; SNAKES as usize + 1];
        for y in 0..Y_SIZE {
            for x in 0..X_SIZE {
                match self.board.get(x, y) {
                    Some(EField::Capture { snake_number, .. }) => {
                        if let Some(n) = snake_number {
                            snake_captures[n as usize] += 1;
                        } else {
                            snake_captures[SNAKES as usize] += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
        snake_captures
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
                match self.board.get(x, y) {
                    Some(EField::Capture {
                        snake_number,
                        length,
                        ..
                    }) => {
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
                    _ => (),
                }
            }
        }
    }
}

impl fmt::Display for EGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = String::with_capacity((X_SIZE + 1) as usize * Y_SIZE as usize);
        for y in (0..Y_SIZE).rev() {
            for x in 0..X_SIZE {
                if let Some(state) = self.board.get(x as i8, y as i8) {
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

#[cfg(test)]
mod tests {
    use crate::logic::{
        efficient_game_objects::e_game_state::EGameState, json_requests::read_game_state,
    };

    use super::*;

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
        let result = board.timed_capture(Duration::from_millis(50));
        println!("{:?}", result);
    }
}
