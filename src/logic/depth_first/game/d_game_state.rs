use arrayvec::ArrayVec;
use itertools::Itertools;

use super::{
    d_board::{DBoard, HEIGHT, WIDTH},
    d_coord::DCoord,
    d_direction::{DDirection, D_DIRECTION_LIST},
    d_field::{DFastField, DField, DReached, DSlowField},
    d_moves_set::{DMoves, DMovesSet},
    d_snake::DSnake,
    d_snakes::DSnakes,
};
use crate::{logic::legacy::shared::e_snakes::SNAKES, Battlesnake, Board};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct DGameState<T: DField> {
    board: DBoard<T>,
    snakes: DSnakes,
    turn: i32,
}

impl<T: DField> DGameState<T> {
    /// Convenience method to play a game with a list of moves
    /// Moves are given as a list of strings where each string represents the moves for a snake
    /// Example input: ["UDDL", "DUU", "", ""]
    pub fn play(mut self, moves_string: [&str; SNAKES as usize]) -> Self {
        for i in 0..moves_string.iter().map(|s| s.len()).max().unwrap() {
            let mut moves: DMoves = [None; SNAKES as usize];
            for id in 0..SNAKES {
                if let Some(c) = moves_string[id as usize].chars().nth(i) {
                    moves[id as usize] = Some(match c {
                        'U' => DDirection::Up,
                        'D' => DDirection::Down,
                        'L' => DDirection::Left,
                        'R' => DDirection::Right,
                        _ => panic!("Invalid move character"),
                    });
                }
            }
            self.next_state(moves);
        }
        self
    }

    pub fn from_request(board: &Board, you: &Battlesnake, turn: &i32) -> Self {
        let snakes = DSnakes::from_request(board, you);
        let d_board = DBoard::from_request(board, you);
        DGameState {
            board: d_board,
            snakes,
            turn: *turn,
        }
    }

    pub fn next_state(&mut self, moves: DMoves) -> &mut Self {
        // Elimination handling https://github.com/BattlesnakeOfficial/rules/blob/main/standard.go#L172
        // Eliminate starved snakes first (moving on food with 1 health in previous round is allowed, moving on non food will die now)
        // Evaluate and eliminate collisions after
        self.move_tails().move_heads(moves)
    }

    pub fn move_heads(&mut self, moves: DMoves) -> &mut Self {
        // Calculate potential new heads and handle headless snakes and non moves and food and health
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            let movement = moves[id as usize];
            match (snake, movement) {
                (
                    DSnake::Alive {
                        head,
                        health,
                        length,
                        stack,
                        ..
                    },
                    Some(direction),
                ) => {
                    let new_head = head + direction;
                    match self.board.cell(new_head.x, new_head.y) {
                        None => {
                            self.board.remove_snake(snake);
                            self.snakes.cell(id).set(snake.to_dead()); // Eliminate moved out of bounds directly
                        }
                        Some(field) => {
                            self.board
                                .cell(head.x, head.y)
                                .unwrap()
                                .set(T::snake(id, Some(direction)));
                            if field.get().get_type() == T::FOOD {
                                self.snakes.cell(id).set(
                                    snake
                                        .health(100)
                                        .length(length + 1)
                                        .stack(stack + 1)
                                        .head(new_head),
                                );
                            } else {
                                self.snakes
                                    .cell(id)
                                    .set(snake.health(health - 1).head(new_head));
                            }
                        }
                    }
                }
                (DSnake::Alive { health, .. }, None) => {
                    self.snakes
                        .cell(id)
                        .set(snake.health(health - 1).to_headless());
                }
                (_, None) => (),
                _ => panic!(
                    "Can only move head of alive snakes but moved {:?} {:?}",
                    snake, movement
                ),
            }
        }

        // Remove starved snakes
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { health, .. } | DSnake::Headless { health, .. } if health == 0 => {
                    self.board.remove_snake(snake);
                    self.snakes.cell(id).set(snake.to_dead());
                }
                _ => (),
            }
        }

        // Find head conflicts
        let mut head_conflicts = [None; SNAKES as usize];
        for id_1 in 0..SNAKES - 1 {
            if let DSnake::Alive { head, .. } = self.snakes.cell(id_1).get() {
                for id_2 in id_1 + 1..SNAKES {
                    if let DSnake::Alive {
                        head: other_head, ..
                    } = self.snakes.cell(id_2).get()
                    {
                        if head == other_head {
                            head_conflicts[id_1 as usize] = Some(id_2);
                        }
                    }
                }
            }
        }

        let mut snakes_to_remove: [Option<DSnake>; SNAKES as usize] = [None; SNAKES as usize];

        // Handle head conflicts
        for id_1 in 0..SNAKES {
            if let Some(id_2) = head_conflicts[id_1 as usize] {
                let snake_1 = self.snakes.cell(id_1).get();
                let snake_2 = self.snakes.cell(id_2).get();
                match (snake_1, snake_2) {
                    (
                        DSnake::Alive {
                            length: length_1, ..
                        },
                        DSnake::Alive {
                            length: length_2, ..
                        },
                    ) => {
                        if length_1 > length_2 {
                            snakes_to_remove[id_2 as usize] = Some(snake_2);
                            self.snakes.cell(id_2).set(snake_2.to_dead());
                        } else if length_1 < length_2 {
                            snakes_to_remove[id_1 as usize] = Some(snake_1);
                            self.snakes.cell(id_1).set(snake_1.to_dead());
                        } else {
                            snakes_to_remove[id_1 as usize] = Some(snake_1);
                            snakes_to_remove[id_2 as usize] = Some(snake_2);
                            self.snakes.cell(id_1).set(snake_1.to_dead());
                            self.snakes.cell(id_2).set(snake_2.to_dead());
                        }
                    }
                    _ => panic!("Head conflicts can only happen between alive snakes"),
                }
            }
        }

        // Head body collisions
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { head, .. } => {
                    if self.board.cell(head.x, head.y).unwrap().get().get_type() == T::SNAKE {
                        snakes_to_remove[id as usize] = Some(snake);
                        self.snakes.cell(id).set(snake.to_dead());
                    }
                }
                _ => (),
            }
        }

        // Remove all snakes that need to be removed
        for id in 0..SNAKES {
            if let Some(snake) = snakes_to_remove[id as usize] {
                self.board.remove_snake(snake);
            }
        }

        // Set the head board fields for all alive snakes
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { head, .. } => {
                    self.board
                        .cell(head.x, head.y)
                        .unwrap()
                        .set(T::snake(id, None));
                }
                _ => (),
            }
        }

        self
    }

    pub fn move_tails(&mut self) -> &mut Self {
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { stack, .. } | DSnake::Headless { stack, .. } if stack > 0 => {
                    self.snakes.cell(id).set(snake.stack(stack - 1));
                }
                DSnake::Alive { tail, .. } | DSnake::Headless { tail, .. } => {
                    if let Some(next) = self.board.cell(tail.x, tail.y).unwrap().get().get_next() {
                        self.snakes.cell(id).set(snake.tail(tail + next));
                        self.board.cell(tail.x, tail.y).unwrap().set(T::empty());
                    } else {
                        self.snakes.cell(id).set(snake.to_vanished());
                        self.board.cell(tail.x, tail.y).unwrap().set(T::empty());
                    }
                }
                _ => (),
            }
        }
        self
    }

    pub fn possible_moves(&self) -> DMovesSet {
        let mut possible_moves = [[false; 4]; SNAKES as usize];
        let mut moved_tails = self.clone();
        moved_tails.move_tails();
        for id in 0..SNAKES {
            possible_moves[id as usize] = moved_tails.possible_moves_for(id)
        }
        DMovesSet::new(possible_moves)
    }

    pub fn possible_moves_for(&self, id: u8) -> [bool; 4] {
        let snake = self.snakes.cell(id).get();
        let mut possible_moves = [false; 4];
        let head = match snake {
            DSnake::Alive { head, .. } => head,
            _ => return possible_moves,
        };
        for direction in D_DIRECTION_LIST {
            let new_head = head + direction;
            if let Some(field) = self.board.cell(new_head.x, new_head.y) {
                if field.get().get_type() <= 1 {
                    possible_moves[direction as usize] = true;
                }
            }
        }
        possible_moves
    }

    pub fn get_alive(&self) -> [bool; SNAKES as usize] {
        let mut alive = [false; SNAKES as usize];
        for i in 0..SNAKES as usize {
            alive[i] = match self.snakes.cell(0).get() {
                DSnake::Alive { .. } => true,
                _ => false,
            }
        }
        alive
    }

    pub fn get_length(&self) -> Option<usize> {
        let snake = self.snakes.cell(0).get();
        match snake {
            DSnake::Alive { length, .. } => Some(length as usize),
            _ => None,
        }
    }
}

impl DGameState<DSlowField> {
    pub fn move_reachable(&mut self, moves: DMoves, turn: u8) -> &mut Self {
        let mut reachable_board =
            [[[DReached::default(); SNAKES as usize]; WIDTH as usize]; HEIGHT as usize];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.board.cell(x, y).unwrap().get() {
                    DSlowField::Empty {
                        reachable: reachable_original,
                    }
                    | DSlowField::Food {
                        reachable: reachable_original,
                    } => {
                        reachable_board[y as usize][x as usize] = reachable_original;
                        for d in 0..4 {
                            let neighbor = DCoord::new(x, y) + D_DIRECTION_LIST[d];
                            if let Some(cell) = self.board.cell(neighbor.x, neighbor.y) {
                                match cell.get() {
                                    DSlowField::Empty {
                                        reachable: reachable_other,
                                        ..
                                    }
                                    | DSlowField::Food {
                                        reachable: reachable_other,
                                        ..
                                    } => {
                                        for i in 0..SNAKES {
                                            if !reachable_original[i as usize].is_set()
                                                && reachable_other[i as usize].is_set()
                                                && ((x + y) % 2) as u8
                                                    == ((turn as i32 + self.turn) % 2) as u8
                                            {
                                                reachable_board[y as usize][x as usize]
                                                    [i as usize] = DReached::new(turn);
                                            }
                                        }
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

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.board.cell(x, y).unwrap().get() {
                    field @ DSlowField::Empty { .. } | field @ DSlowField::Food { .. } => {
                        self.board
                            .cell(x, y)
                            .unwrap()
                            .set(field.reachable(reachable_board[y as usize][x as usize]));
                    }
                    _ => (),
                }
            }
        }

        // Create reachable fields for snakes that have no movement
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            let movement = moves[id as usize];
            match (snake, movement) {
                (
                    DSnake::Headless {
                        last_head: head, ..
                    }
                    | DSnake::Alive { head, .. },
                    None,
                ) => {
                    for d in D_DIRECTION_LIST {
                        let to_reach = head + d;
                        if let Some(cell) = self.board.cell(to_reach.x, to_reach.y) {
                            match cell.get() {
                                DSlowField::Empty { mut reachable }
                                | DSlowField::Food { mut reachable } => {
                                    if !reachable[id as usize].is_set() {
                                        reachable[id as usize] = DReached::new(1);
                                        cell.set(DSlowField::empty().reachable(reachable));
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        self
    }

    /// Checks if the current reachable configuration gives a valid board where no next move is possible
    /// Is optimistic such that if not all moves can be blocked, all will be viable
    pub fn scope_moves_optimistic(&self, turn: u8) -> ArrayVec<DDirection, 4> {
        // Observation: Snakes can reach a fixed point with the head only every second move

        let mut gamestate = self.clone();
        gamestate.move_tails();

        // Check if there are any problematic snakes
        let mut problematic_snakes = [false; SNAKES as usize];
        let mut movable_fields = ArrayVec::<_, 4>::new();
        let mut movable_fields_list = [false; 4];
        let my_head = match gamestate.snakes.cell(0).get() {
            DSnake::Alive { head, .. } => head,
            _ => panic!("Dead snake can't be checked for dead end"),
        };
        for i in 0..4 {
            let neighbor_coord = my_head + D_DIRECTION_LIST[i];
            let neighbor_field = gamestate.board.cell(neighbor_coord.x, neighbor_coord.y);
            if let Some(field) = neighbor_field {
                match field.get() {
                    DSlowField::Empty { reachable, .. } | DSlowField::Food { reachable, .. } => {
                        movable_fields.push((neighbor_coord, reachable));
                        movable_fields_list[i] = true;
                        for id in 1..SNAKES {
                            if reachable[id as usize].is_set() {
                                problematic_snakes[id as usize] = true;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        // No movable fields available
        if movable_fields.len() == 0 {
            return ArrayVec::new();
        }

        // No problematic snakes
        if problematic_snakes.iter().all(|&e| e == false) {
            let mut movable_directions = ArrayVec::new();
            for d in D_DIRECTION_LIST {
                if movable_fields_list[d as usize] {
                    movable_directions.push(d);
                }
            }
            return movable_directions;
        }

        // Get distribution of required points to problematic snakes
        // 1  12   13
        // -> 1 1 1
        let mut snakes_for_fields = ArrayVec::<_, 4>::new();
        for (_, reachable) in movable_fields.iter() {
            let mut allowed_snakes = ArrayVec::<_, 3>::new();
            for id in 1..SNAKES {
                if reachable[id as usize].is_set() {
                    allowed_snakes.push(id);
                }
            }
            snakes_for_fields.push(allowed_snakes);
        }
        let fields_to_snakes = snakes_for_fields.into_iter().multi_cartesian_product();

        // Get distribution of required DCoords to problematic snakes
        let mut snake_coord_distributions = Vec::new();
        for field_distribution in fields_to_snakes {
            let mut required_coords = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            for id in 1..SNAKES {
                for coord_id in 0..field_distribution.len() {
                    if field_distribution[coord_id] == id {
                        required_coords[id as usize].push(movable_fields[coord_id].0);
                    }
                }
            }
            snake_coord_distributions.push(required_coords);
        }

        'snake_coords: for snake_coords in snake_coord_distributions {
            'snakes: for id in 1..SNAKES {
                if snake_coords[id as usize].len() > 0 {
                    for coords in snake_coords[id as usize]
                        .iter()
                        .permutations(snake_coords[id as usize].len())
                    {
                        let total_distance = coords
                            .iter()
                            .zip(coords.iter().skip(1))
                            .fold(0, |acc, (a, &&b)| acc + a.distance_to(b));

                        match gamestate.snakes.cell(id).get() {
                            DSnake::Headless { last_head, .. } => {
                                if total_distance + coords.last().unwrap().distance_to(last_head)
                                    <= turn
                                {
                                    continue 'snakes;
                                }
                            }
                            DSnake::Vanished {
                                last_head, length, ..
                            } => {
                                if total_distance + coords.last().unwrap().distance_to(last_head)
                                    <= turn + turn - length + 1
                                {
                                    continue 'snakes;
                                }
                            }
                            _ => panic!("Invalid Snake for dead end check"),
                        }
                    }
                    break 'snake_coords;
                }
            }
            return ArrayVec::new();
        }

        let mut movable_directions = ArrayVec::new();
        for d in D_DIRECTION_LIST {
            if movable_fields_list[d as usize] {
                movable_directions.push(d);
            }
        }
        movable_directions
    }

    /// Checks if the current reachable configuration gives a valid board where no next move is possible
    /// Is pessimistic such that any move that can be blocked will be blocked
    pub fn scope_moves_pessimistic(&self) -> ArrayVec<DDirection, 4> {
        let mut movable_fields = ArrayVec::<DDirection, 4>::new();

        let mut gamestate = self.clone();
        gamestate.move_tails();

        let head = match gamestate.snakes.cell(0).get() {
            DSnake::Alive { head, .. } => head,
            _ => panic!("Dead snake can't be checked for dead end"),
        };
        'direction: for d in D_DIRECTION_LIST {
            let neighbor = head + d;
            if let Some(cell) = gamestate.board.cell(neighbor.x, neighbor.y) {
                match cell.get() {
                    DSlowField::Empty { reachable, .. } | DSlowField::Food { reachable, .. } => {
                        for id in 1..SNAKES {
                            if reachable[id as usize].is_set() {
                                continue 'direction;
                            }
                        }
                        movable_fields.push(d);
                    }
                    _ => (),
                }
            }
        }
        movable_fields
    }

    pub fn relevant_snakes(
        &self,
        movement: DDirection,
        turn: u8,
    ) -> [DRelevanceState; SNAKES as usize] {
        let mut relevant_snakes = [DRelevanceState::None; SNAKES as usize];
        let my_head = match self.snakes.cell(0).get() {
            DSnake::Alive { head, .. } => head,
            _ => panic!("Dead snake can't be checked for relevant snakes"),
        };
        let new_head = my_head + movement;
        if let Some(cell) = self.board.cell(new_head.x, new_head.y) {
            match cell.get() {
                DSlowField::Empty { reachable, .. } | DSlowField::Food { reachable, .. } => {
                    for id in 1..SNAKES {
                        if reachable[id as usize].turn() == turn {
                            relevant_snakes[id as usize] = DRelevanceState::Head;
                        } else if reachable[id as usize].is_set() {
                            relevant_snakes[id as usize] = DRelevanceState::Body;
                        }
                    }
                }
                _ => (),
            }
        }
        relevant_snakes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DRelevanceState {
    None,
    Body,
    Head,
}

impl From<DGameState<DSlowField>> for DGameState<DFastField> {
    fn from(value: DGameState<DSlowField>) -> Self {
        let new_board = DBoard::<DFastField>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let old_cell = value.board.cell(x, y).unwrap().get();
                new_board.cell(x, y).unwrap().set(old_cell.into());
            }
        }
        DGameState {
            board: new_board,
            snakes: value.snakes,
            turn: value.turn,
        }
    }
}

impl From<DGameState<DFastField>> for DGameState<DSlowField> {
    fn from(value: DGameState<DFastField>) -> Self {
        let new_board = DBoard::<DSlowField>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let old_cell = value.board.cell(x, y).unwrap().get();
                new_board.cell(x, y).unwrap().set(old_cell.into());
            }
        }
        DGameState {
            board: new_board,
            snakes: value.snakes,
            turn: value.turn,
        }
    }
}

impl<T> Display for DGameState<T>
where
    T: DField,
    DGameState<DSlowField>: From<DGameState<T>>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let this: DGameState<DSlowField> = self.clone().into();

        let row = [' '; WIDTH as usize * 3 * 2];
        let mut board = [row; HEIGHT as usize * 3];

        // Write head markers before board
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                DSnake::Alive { head, id, .. } => {
                    let id = (id + 'A' as u8) as char;
                    let x = head.x;
                    let y = head.y;
                    board[y as usize * 3 + 1][x as usize * 3 * 2] = id;
                    board[y as usize * 3 + 1][x as usize * 3 * 2 + 2 * 2] = id;
                    board[y as usize * 3][x as usize * 3 * 2 + 1 * 2] = id;
                    board[y as usize * 3 + 2][x as usize * 3 * 2 + 1 * 2] = id;
                    board[y as usize * 3 + 1][x as usize * 3 * 2 + 1 * 2] = id;
                }
                _ => (),
            }
        }

        // Handle reachable
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match this.board.cell(x, y).unwrap().get() {
                    DSlowField::Empty { reachable, .. } | DSlowField::Food { reachable, .. } => {
                        if reachable.iter().any(|&r| r.is_set()) {
                            let best = reachable.iter().filter(|x| x.is_set()).min().unwrap();
                            if reachable.iter().filter(|x| x.is_set()).count() == 1 {
                                let snake = reachable
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, x)| x.is_set())
                                    .next()
                                    .unwrap()
                                    .0;
                                let c = (snake as u8 + b'A') as char;
                                board[y as usize * 3 + 1][x as usize * 3 * 2 + 1] = c;
                                board[y as usize * 3 + 1][x as usize * 3 * 2 + 3] =
                                    (best.turn() + '0' as u8) as char;
                            } else {
                                board[y as usize * 3 + 1][x as usize * 3 * 2 + 1] =
                                    (reachable.iter().filter(|x| x.is_set()).count() as u8
                                        + '0' as u8) as char;
                                board[y as usize * 3 + 1][x as usize * 3 * 2 + 3] =
                                    (best.turn() + '0' as u8) as char;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        // Fill the board with the current state
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match this.board.cell(x, y).unwrap().get() {
                    DSlowField::Empty { .. } => {
                        board[y as usize * 3 + 1][x as usize * 3 * 2 + 1 * 2] = '.';
                    }
                    DSlowField::Food { .. } => {
                        board[y as usize * 3 + 1][x as usize * 3 * 2 + 1 * 2] = 'X';
                    }
                    DSlowField::Snake { id, next } => {
                        let c = (id + 'a' as u8) as char;
                        board[y as usize * 3 + 1][x as usize * 3 * 2 + 1 * 2] = '*';
                        match next {
                            Some(DDirection::Up) => {
                                board[y as usize * 3 + 2][x as usize * 3 * 2 + 1 * 2] = c;
                                board[y as usize * 3 + 3][x as usize * 3 * 2 + 1 * 2] = c;
                            }
                            Some(DDirection::Down) => {
                                board[y as usize * 3][x as usize * 3 * 2 + 1 * 2] = c;
                                board[y as usize * 3 - 1][x as usize * 3 * 2 + 1 * 2] = c;
                            }
                            Some(DDirection::Left) => {
                                board[y as usize * 3 + 1][x as usize * 3 * 2] = c;
                                board[y as usize * 3 + 1][x as usize * 3 * 2 - 1 * 2] = c;
                            }
                            Some(DDirection::Right) => {
                                board[y as usize * 3 + 1][x as usize * 3 * 2 + 2 * 2] = c;
                                board[y as usize * 3 + 1][x as usize * 3 * 2 + 3 * 2] = c;
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        // Write tail markers over board
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                DSnake::Alive { tail, stack, .. } | DSnake::Headless { tail, stack, .. } => {
                    board[tail.y as usize * 3 + 1][tail.x as usize * 3 * 2 + 1 * 2] =
                        (stack + '0' as u8) as char;
                }
                _ => (),
            }
        }

        // Construct the final display string
        let bottom =
            "+---0-----1-----2-----3-----4-----5-----6-----7-----8-----9----10---+\n".to_string();
        let left: Vec<char> = "|0||1||2||3||4||5||6||7||8||9|01|".chars().collect();
        let mut output = bottom.clone();
        for y in (0..board.len()).rev() {
            output.push(left[y]);
            output.push(' ');
            for x in 0..board[0].len() {
                output.push(board[y][x]);
            }
            output.push(left[y]);
            output.push('\n');
        }
        output.push_str(&bottom);

        let mut other_info = String::from('\n');
        for i in 0..SNAKES {
            match self.snakes.cell(i).get() {
                DSnake::Alive {
                    id, health, length, ..
                } => other_info.push_str(&format!(
                    "Snake {} (Alive) - Health: {}, Length: {}\n",
                    (id + 'A' as u8) as char,
                    health,
                    length
                )),
                DSnake::Headless {
                    id, health, length, ..
                } => other_info.push_str(&format!(
                    "Snake {} (Headless) - Health: {}, Length: {}\n",
                    (id + 'A' as u8) as char,
                    health,
                    length
                )),
                DSnake::Dead { id, .. } => {
                    other_info.push_str(&format!("Snake {} (Dead)\n", (id + 'A' as u8) as char))
                }
                DSnake::Vanished { id, length, .. } => other_info.push_str(&format!(
                    "Snake {} (Vanished) - Length: {}\n",
                    (id + 'A' as u8) as char,
                    length
                )),
                DSnake::NonExistent => (),
            }
        }
        output.push_str(&other_info);

        writeln!(f, "{}", output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::depth_first::game::{
            d_coord::DCoord,
            d_direction::DDirection,
            d_field::{DFastField, DSlowField},
            d_snake::DSnake,
        },
        read_game_state,
    };

    #[test]
    fn test_display() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
    }

    #[bench]
    // Should be < 50ns
    fn bench_next_state_slow(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let moves = [
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Left),
            Some(DDirection::Down),
        ];
        b.iter(|| {
            let mut state = state.clone();
            state.next_state(moves);
        });
    }

    #[bench]
    // Should be < 50ns
    fn bench_next_state_fast(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let moves = [
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Left),
            Some(DDirection::Down),
        ];
        b.iter(|| {
            let mut state = state.clone();
            state.next_state(moves);
        });
    }

    #[bench]
    // Should be < 10ns
    fn bench_possible_moves(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        b.iter(|| {
            let _ = state.possible_moves();
        });
    }

    #[bench]
    // Should be < 760ns
    fn bench_move_reachable(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let moves = [Some(DDirection::Up), None, Some(DDirection::Left), None];
        b.iter(|| {
            let mut state = state.clone();
            state.move_reachable(moves, 1);
        });
    }

    #[bench]
    fn bench_scope_moves(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        let moves = [Some(DDirection::Up), None, None, None];
        state.next_state(moves).move_reachable(moves, 1);
        state.next_state(moves).move_reachable(moves, 2);
        state.next_state(moves).move_reachable(moves, 3);
        state.next_state(moves).move_reachable(moves, 4);
        b.iter(|| {
            let _ = state.scope_moves_optimistic(4);
        });
    }

    #[test]
    fn test_scope_moves_optimistic() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let moves = [Some(DDirection::Up), None, None, None];
        state.next_state(moves).move_reachable(moves, 1);
        println!("{}", state);
        state.next_state(moves).move_reachable(moves, 2);
        println!("{}", state);
        state.next_state(moves).move_reachable(moves, 3);
        println!("{}", state);
        let result = state.scope_moves_optimistic(3);
        assert!(result.contains(&DDirection::Up));
        assert!(result.contains(&DDirection::Right));
        let moves = [Some(DDirection::Right), None, None, None];
        state.next_state(moves).move_reachable(moves, 4);
        println!("{}", state);
        let result = state.scope_moves_optimistic(4);
        assert!(result.contains(&DDirection::Up));
        assert!(result.contains(&DDirection::Down));
        assert!(result.contains(&DDirection::Right));
        state.next_state(moves).move_reachable(moves, 5);
        println!("{}", state);
        let result = state.scope_moves_optimistic(5);
        assert!(!result.contains(&DDirection::Up));
        assert!(!result.contains(&DDirection::Down));
        assert!(!result.contains(&DDirection::Left));
        assert!(!result.contains(&DDirection::Right));

        let gamestate = read_game_state("requests/failure_9.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let result = state.scope_moves_optimistic(1);
        assert!(result.contains(&DDirection::Up));
    }

    #[test]
    fn test_scope_moves_pessimistic() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let moves = [Some(DDirection::Up), None, None, None];
        state.next_state(moves).move_reachable(moves, 1);
        println!("{}", state);
        state.next_state(moves).move_reachable(moves, 2);
        println!("{}", state);
        state.next_state(moves).move_reachable(moves, 3);
        println!("{}", state);
        let result = state.scope_moves_pessimistic();
        assert!(result.contains(&DDirection::Up));
        assert!(!result.contains(&DDirection::Down));
        assert!(!result.contains(&DDirection::Left));
        assert!(result.contains(&DDirection::Right));
        state.next_state(moves).move_reachable(moves, 4);
        println!("{}", state);
        let result = state.scope_moves_pessimistic();
        assert!(result.contains(&DDirection::Up));
        assert!(!result.contains(&DDirection::Down));
        assert!(!result.contains(&DDirection::Left));
        assert!(!result.contains(&DDirection::Right));
        state
            .next_state([Some(DDirection::Right), None, None, None])
            .move_reachable(moves, 5);
        println!("{}", state);
        let result = state.scope_moves_pessimistic();
        assert!(result.len() == 0);

        let gamestate = read_game_state("requests/failure_9.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let result = state.scope_moves_pessimistic();
        assert!(result.contains(&DDirection::Up));
    }

    #[test]
    fn test_next_state_with_move_reachable() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let moves = [Some(DDirection::Up), None, None, None];
        state.next_state(moves).move_reachable(moves, 1);
        println!("{}", state);
        match state.board.cell(4, 4).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 1]);
            }
            _ => panic!("Problem with field (4, 4)"),
        }
        state.next_state(moves).move_reachable(moves, 2);
        println!("{}", state);
        state.next_state(moves).move_reachable(moves, 3);
        println!("{}", state);
        state.next_state(moves).move_reachable(moves, 4);
        println!("{}", state);
        match state.board.cell(0, 1).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 0, 0, 0]);
            }
            _ => panic!("Problem with field (0, 1)"),
        }
        match state.board.cell(6, 4).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 3]);
            }
            _ => panic!("Problem with field (6, 4)"),
        }
        state.next_state(moves).move_reachable(moves, 5);
        println!("{}", state);
        match state.board.cell(4, 5).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 0, 0, 0]);
            }
            _ => panic!("Problem with field (4, 5)"),
        }
        state.next_state(moves).move_reachable(moves, 6);
        println!("{}", state);
        match state.board.cell(4, 5).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 6, 0, 6]);
            }
            _ => panic!("Problem with field (4, 5)"),
        }
        match state.board.cell(5, 0).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 6, 4, 6]);
            }
            _ => panic!("Problem with field (5, 0)"),
        }
    }

    #[test]
    fn test_move_reachable() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let moves = [Some(DDirection::Up), None, Some(DDirection::Left), None];
        state.move_heads(moves).move_reachable(moves, 1);
        println!("{}", state);
        match state.board.cell(4, 4).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 1]);
            }
            _ => panic!("Problem with field (4, 4)"),
        }
        match state.board.cell(6, 4).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 0]);
            }
            _ => panic!("Problem with field (6, 4)"),
        }
        state.move_reachable(moves, 2);
        println!("{}", state);
        match state.board.cell(3, 4).unwrap().get() {
            DSlowField::Food { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 2, 0, 2]);
            }
            _ => panic!("Problem with field (3, 4)"),
        }
        match state.board.cell(2, 5).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 0, 0, 2]);
            }
            _ => panic!("Problem with field (2, 5)"),
        }
        match state.board.cell(4, 4).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 1]);
            }
            _ => panic!("Problem with field (4, 4)"),
        }
        match state.board.cell(6, 4).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 0]);
            }
            _ => panic!("Problem with field (6, 4)"),
        }
        match state.board.cell(1, 5).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 0, 0, 0]);
            }
            _ => panic!("Problem with field (1, 5)"),
        }
    }

    #[test]
    fn test_possible_moves() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let moves = state.possible_moves();
        println!("{:#?}", moves);
        assert_eq!(moves.generate().len(), 36);

        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let moves = state.possible_moves();
        assert_eq!(moves.get(0), [true, false, true, true]);
        assert_eq!(moves.get(1), [true, false, false, false]);
        assert_eq!(moves.get(2), [false, false, false, false]);
        assert_eq!(moves.get(3), [false, false, false, false]);
        let generated = moves.generate();
        assert_eq!(generated.len(), 3);
        for m in generated {
            assert_eq!(m[1], Some(DDirection::Up));
        }

        let state = state.play(["RR", "UU", "", ""]);
        println!("{}", state);
        let moves = state.possible_moves().generate();
        assert_eq!(moves.len(), 6);
        println!("{:#?}", moves);

        let gamestate = read_game_state("requests/failure_9.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let moves = state.possible_moves();
        assert_eq!(moves.get(0), [true, true, false, true]);
    }

    #[test]
    fn test_next_state() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let mut moves = [
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Left),
            Some(DDirection::Down),
        ];
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(1).get() {
            DSnake::Dead { .. } => (),
            _ => panic!("Problem with Snake B"),
        }
        moves = [None, None, Some(DDirection::Left), Some(DDirection::Left)];
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            DSnake::Headless { .. } => (),
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(3).get() {
            DSnake::Alive {
                head,
                length,
                stack,
                ..
            } => {
                assert_eq!(head, DCoord::new(3, 4));
                assert_eq!(length, 6);
                assert_eq!(stack, 1);
            }
            _ => panic!("Problem with Snake D"),
        }
        match state.board.cell(4, 8).unwrap().get() {
            DSlowField::Empty { .. } => (),
            _ => panic!("Problem with field (4, 8)"),
        }
        moves = [None, None, Some(DDirection::Left), Some(DDirection::Down)];
        state.next_state(moves);
        println!("{}", state);
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            DSnake::Vanished { .. } => (),
            _ => panic!("Problem with Snake A"),
        }
        state.next_state(moves);
        println!("{}", state);
        moves = [None, None, Some(DDirection::Left), Some(DDirection::Right)];
        state.next_state(moves);
        println!("{}", state);
        state.next_state(moves);
        println!("{}", state);
        moves = [None, None, Some(DDirection::Up), Some(DDirection::Down)];
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(3).get() {
            DSnake::Alive { .. } => (),
            _ => panic!("Problem with Head Tail movement order"),
        }
    }

    #[test]
    fn test_next_state_2() {
        let gamestate =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let moves = [
            Some(DDirection::Right),
            Some(DDirection::Down),
            Some(DDirection::Down),
            Some(DDirection::Down),
        ];
        state.next_state(moves);
        println!("{}", state);
        assert!(!state.get_alive()[0]);
    }

    #[test]
    fn test_move_heads_headless() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        state.move_heads([
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Down),
            None,
        ]);
        println!("{}", state);
        match state.snakes.cell(3).get() {
            DSnake::Headless { .. } => (),
            _ => panic!("Problem with Snake D"),
        }
    }

    #[test]
    fn test_move_heads_food() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        state.move_heads([
            Some(DDirection::Up),
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Left),
        ]);
        println!("{}", state);
        match state.snakes.cell(1).get() {
            DSnake::Alive {
                length,
                stack,
                health,
                ..
            } => {
                assert_eq!(health, 100);
                assert_eq!(length, 5);
                assert_eq!(stack, 1)
            }
            _ => panic!("Problem with Snake B"),
        }
        let mut state2 = state.clone();
        state.move_heads([
            Some(DDirection::Up),
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Left),
        ]);
        println!("Alternative 1:\n{}", state);
        match state.snakes.cell(3).get() {
            DSnake::Dead { .. } => (),
            _ => panic!("Problem with Snake D"),
        }
        state2.move_heads([
            Some(DDirection::Up),
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Down),
        ]);
        println!("Alternative 2:\n{}", state2);
        match state2.snakes.cell(3).get() {
            DSnake::Alive {
                head,
                length,
                stack,
                ..
            } => {
                assert_eq!(head, DCoord::new(3, 4));
                assert_eq!(length, 6);
                assert_eq!(stack, 1);
            }
            _ => panic!("Problem with Snake D"),
        }
    }

    #[test]
    fn test_move_heads() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        state.move_heads([
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Down),
            Some(DDirection::Down),
        ]);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            DSnake::Alive { head, .. } => assert_eq!(head, DCoord { x: 0, y: 2 }),
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(1).get() {
            DSnake::Dead { .. } => (),
            _ => panic!("Problem with Snake B"),
        }
        match state.snakes.cell(2).get() {
            DSnake::Dead { .. } => (),
            _ => panic!("Problem with Snake C"),
        }
        match state.snakes.cell(3).get() {
            DSnake::Alive { head, .. } => assert_eq!(head, DCoord { x: 4, y: 4 }),
            _ => panic!("Problem with Snake D"),
        }
        match state.board.cell(0, 0).unwrap().get() {
            DSlowField::Snake { id, next } => {
                assert_eq!(id, 0);
                assert_eq!(next, Some(DDirection::Up));
            }
            _ => panic!("Problem with field (0, 0)"),
        }
        match state.board.cell(0, 2).unwrap().get() {
            DSlowField::Snake { id, next } => {
                assert_eq!(id, 0);
                assert_eq!(next, None);
            }
            _ => panic!("Problem with field (1, 0)"),
        }
        match state.board.cell(4, 4).unwrap().get() {
            DSlowField::Snake { id, next } => {
                assert_eq!(id, 3);
                assert_eq!(next, None);
            }
            _ => panic!("Problem with field (4, 4)"),
        }
        match state.board.cell(4, 5).unwrap().get() {
            DSlowField::Snake { id, next } => {
                assert_eq!(id, 3);
                assert_eq!(next, Some(DDirection::Down));
            }
            _ => panic!("Problem with field (4, 5)"),
        }
        match state.board.cell(5, 4).unwrap().get() {
            DSlowField::Empty { .. } => (),
            _ => panic!("Problem with field (5, 4)"),
        }
        match state.board.cell(9, 0).unwrap().get() {
            DSlowField::Empty { .. } => (),
            _ => panic!("Problem with field (9, 0)"),
        }
    }

    #[test]
    fn test_move_tails() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        match state.snakes.cell(0).get() {
            DSnake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, DCoord { x: 1, y: 0 });
            }
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(2).get() {
            DSnake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 1);
                assert_eq!(tail, DCoord { x: 9, y: 2 });
            }
            _ => panic!("Problem with Snake C"),
        }
        assert_eq!(
            state.board.cell(1, 0).unwrap().get(),
            DSlowField::snake(0, Some(DDirection::Left))
        );
        state.move_tails();
        assert_eq!(state.board.cell(1, 0).unwrap().get(), DSlowField::empty());
        assert_eq!(
            state.board.cell(9, 2).unwrap().get(),
            DSlowField::snake(2, Some(DDirection::Down))
        );
        match state.snakes.cell(0).get() {
            DSnake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, DCoord { x: 0, y: 0 });
            }
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(2).get() {
            DSnake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, DCoord { x: 9, y: 2 });
            }
            _ => panic!("Problem with Snake C"),
        }
        state.move_tails().move_tails();
        assert_eq!(state.board.cell(0, 0).unwrap().get(), DSlowField::empty());
        assert_eq!(
            state.board.cell(9, 0).unwrap().get(),
            DSlowField::snake(2, None)
        );
        match state.snakes.cell(0).get() {
            DSnake::Vanished { id, .. } => assert_eq!(id, 0),
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(2).get() {
            DSnake::Alive {
                stack, tail, head, ..
            } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, DCoord { x: 9, y: 0 });
                assert_eq!(head, DCoord { x: 9, y: 0 });
            }
            _ => panic!("Problem with Snake C"),
        }
    }

    #[test]
    fn test_from_request() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let d_gamestate = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        assert_eq!(
            d_gamestate.board.cell(0, 0).unwrap().get(),
            DSlowField::snake(0, None)
        );
        assert_eq!(
            d_gamestate.board.cell(1, 0).unwrap().get(),
            DSlowField::snake(0, Some(DDirection::Left))
        );
        assert_eq!(
            d_gamestate.snakes.cell(0).get(),
            DSnake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: DCoord { x: 0, y: 0 },
                tail: DCoord { x: 2, y: 0 },
                stack: 0
            }
        );
        assert_eq!(
            d_gamestate.snakes.cell(1).get(),
            DSnake::Alive {
                id: 1,
                health: 16,
                length: 3,
                head: DCoord { x: 5, y: 3 },
                tail: DCoord { x: 6, y: 2 },
                stack: 0
            }
        );
    }

    #[test]
    fn test_is_alive() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut d_gamestate = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", d_gamestate);
        assert_eq!(d_gamestate.get_alive()[0], true);
        d_gamestate.next_state([
            Some(DDirection::Left),
            Some(DDirection::Left),
            Some(DDirection::Left),
            Some(DDirection::Down),
        ]);
        println!("{}", d_gamestate);
        assert_eq!(d_gamestate.get_alive()[0], false);
    }

    #[test]
    fn test_better_capture_propagation_order() {
        let gamestate = read_game_state("requests/failure_6.json");
        let mut state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let mut state_2 = state.clone();

        let moves = [Some(DDirection::Right), None, None, None];
        state.next_state(moves).move_reachable(moves, 1);
        state_2
            .move_tails()
            .move_reachable(moves, 1)
            .move_heads(moves);

        println!("{}", state);
        println!("{}", state_2);

        match state.board.cell(4, 9).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 0]);
            }
            _ => panic!("Problem with field (4, 9)"),
        }

        match state_2.board.cell(4, 9).unwrap().get() {
            DSlowField::Empty { reachable } => {
                assert_eq!(reachable.map(|x| x.turn()), [0, 1, 0, 0]);
            }
            _ => panic!("Problem with field (4, 9)"),
        }

        state_2.move_tails().move_reachable(moves, 2);

        println!("{}", state_2);

        let pessimistic_moves = state_2.scope_moves_pessimistic();
        assert_eq!(pessimistic_moves.len(), 0);

        let optimistic_moves = state_2.scope_moves_optimistic(2);
        assert_eq!(optimistic_moves.len(), 2);
    }

    #[test]
    fn play_state() {
        let gamestate = read_game_state("requests/failure_9.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let new_state = state.play(["DRURDRUUUUU", "R", "", ""]);
        println!("{}", new_state);
    }
}
