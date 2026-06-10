use super::{
    moves::{MoveMatrix, Moves},
    snake::Snake,
    snakes::Snakes,
};
use crate::{
    OriginalBattlesnake, OriginalBoard, OriginalGameState,
    logic::{
        general::{
            board::{Board, HEIGHT, WIDTH},
            coord::Coord,
            direction::{DIRECTIONS, Direction},
            field::{BasicField, Field, FloodFillField},
            moves::MoveVector,
        },
        legacy::shared::e_snakes::SNAKES,
    },
};
use arrayvec::ArrayVec;
use rustc_hash::FxHasher;
use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Clone)]
pub struct GameState<T: Field> {
    board: Board<T>,
    snakes: Snakes,
    simulated_turn: u8,
}

impl<F: Field> GameState<F> {
    pub fn board(&self) -> &Board<F> {
        &self.board
    }

    pub fn snakes(&self) -> &Snakes {
        &self.snakes
    }

    /// Convenience method to play a game with a list of moves
    /// Moves are given as a list of strings where each string represents the moves for a snake
    /// Example input: ["UDDL", "DUU", "", ""]
    pub fn play(mut self, moves_string: [&str; SNAKES as usize]) -> Self {
        for i in 0..moves_string.iter().map(|s| s.len()).max().unwrap() {
            let mut moves: Moves = [None; SNAKES as usize];
            for id in 0..SNAKES {
                if let Some(c) = moves_string[id as usize].chars().nth(i) {
                    moves[id as usize] = Some(match c {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => panic!("Invalid move character"),
                    });
                }
            }
            self.next_state(moves);
        }
        self
    }

    pub fn from_request(board: &OriginalBoard, you: &OriginalBattlesnake, _turn: &i32) -> Self {
        let snakes = Snakes::from_request(board, you);
        let d_board = Board::from_request(board, you);
        GameState {
            board: d_board,
            snakes,
            simulated_turn: 0,
        }
    }

    pub fn next_state(&mut self, moves: Moves) -> &mut Self {
        // Elimination handling https://github.com/BattlesnakeOfficial/rules/blob/main/standard.go#L172
        // Eliminate starved snakes first (moving on food with 1 health in previous round is allowed, moving on non food will die now)
        // Evaluate and eliminate collisions after
        self.move_tails().move_heads(moves)
    }

    pub fn move_heads(&mut self, moves: Moves) -> &mut Self {
        // Calculate potential new heads and handle headless snakes and non moves and food and health
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            let movement = moves[id as usize];
            match (snake, movement) {
                (
                    Snake::Alive {
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
                                .set(F::snake(id, Some(direction)));
                            if let BasicField::Food = field.get().value() {
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
                (Snake::Alive { health, .. }, None) => {
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
                Snake::Alive { health, .. } | Snake::Headless { health, .. } if health == 0 => {
                    self.board.remove_snake(snake);
                    self.snakes.cell(id).set(snake.to_dead());
                }
                _ => (),
            }
        }

        // Find head conflicts
        let mut head_conflicts: [ArrayVec<u8, 3>; SNAKES as usize] = [
            ArrayVec::new(),
            ArrayVec::new(),
            ArrayVec::new(),
            ArrayVec::new(),
        ];
        for id_1 in 0..SNAKES - 1 {
            if let Snake::Alive { head, .. } = self.snakes.cell(id_1).get() {
                for id_2 in id_1 + 1..SNAKES {
                    if let Snake::Alive {
                        head: other_head, ..
                    } = self.snakes.cell(id_2).get()
                    {
                        if head == other_head {
                            head_conflicts[id_1 as usize].push(id_2);
                        }
                    }
                }
            }
        }

        let mut snakes_to_remove: [Option<Snake>; SNAKES as usize] = [None; SNAKES as usize];
        // Handle head conflicts
        for id_1 in 0..SNAKES {
            for id_2 in head_conflicts[id_1 as usize].iter() {
                let snake_1 = self.snakes.cell(id_1).get();
                let snake_2 = self.snakes.cell(*id_2).get();
                match (snake_1, snake_2) {
                    (
                        Snake::Alive {
                            length: length_1, ..
                        },
                        Snake::Alive {
                            length: length_2, ..
                        },
                    ) => {
                        if length_1 > length_2 {
                            snakes_to_remove[*id_2 as usize] = Some(snake_2);
                        } else if length_1 < length_2 {
                            snakes_to_remove[id_1 as usize] = Some(snake_1);
                        } else {
                            snakes_to_remove[id_1 as usize] = Some(snake_1);
                            snakes_to_remove[*id_2 as usize] = Some(snake_2);
                        }
                    }
                    _ => {
                        panic!("Head conflicts can only happen between alive snakes")
                    }
                }
            }
        }

        // Head body collisions
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            if let Snake::Alive { head, .. } = snake {
                if let BasicField::Snake { .. } =
                    self.board.cell(head.x, head.y).unwrap().get().value()
                {
                    snakes_to_remove[id as usize] = Some(snake);
                }
            }
        }

        // Remove all snakes that need to be removed
        for id in 0..SNAKES {
            if let Some(snake) = snakes_to_remove[id as usize] {
                self.snakes.cell(id).set(snake.to_dead());
                self.board.remove_snake(snake);
            }
        }

        // Set the head board fields for all alive snakes
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            if let Snake::Alive { head, .. } = snake {
                self.board
                    .cell(head.x, head.y)
                    .unwrap()
                    .set(F::snake(id, None));
            }
        }

        self
    }

    pub fn move_tails(&mut self) -> &mut Self {
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                Snake::Alive { stack, .. } | Snake::Headless { stack, .. } if stack > 0 => {
                    self.snakes.cell(id).set(snake.stack(stack - 1));
                }
                Snake::Alive { tail, .. } | Snake::Headless { tail, .. } => {
                    if let BasicField::Snake {
                        next: Some(next), ..
                    } = self.board.cell(tail.x, tail.y).unwrap().get().value()
                    {
                        self.snakes.cell(id).set(snake.tail(tail + next));
                        self.board.cell(tail.x, tail.y).unwrap().set(F::empty());
                    } else {
                        self.snakes.cell(id).set(snake.to_vanished());
                        self.board.cell(tail.x, tail.y).unwrap().set(F::empty());
                    }
                }
                _ => (),
            }
        }
        self.simulated_turn += 1;
        self
    }

    pub fn local_environment_hash(&self, distance: u8) -> u64 {
        let distance = distance as i8;
        let snake = self.snakes.cell(0).get();
        let mut hasher = FxHasher::default();
        match snake {
            Snake::Alive { head, length, .. } => {
                // loop over all cells that are at most distance away from head
                // the distance is the manhattan distance, i.e. x and y distance added
                for y in -distance..=distance {
                    for x in -distance + y.abs()..=distance - y.abs() {
                        let new_x = head.x + x;
                        let new_y = head.y + y;
                        if let Some(cell) = self.board.cell(new_x, new_y) {
                            cell.get().value().hash(&mut hasher);
                        }
                    }
                }
                length.hash(&mut hasher);
            }
            _ => (),
        }
        return hasher.finish();
    }

    pub fn valid_moves(&self) -> MoveMatrix {
        let mut possible_moves = [MoveVector::default(); SNAKES as usize];
        let mut moved_tails = self.clone();
        moved_tails.move_tails();
        for id in 0..SNAKES {
            possible_moves[id as usize] = moved_tails.valid_moves_for(id);
        }
        MoveMatrix::new(possible_moves)
    }

    pub fn is_alive(&self, id: u8) -> bool {
        matches!(self.snakes.cell(id).get(), Snake::Alive { .. })
    }

    // This expects the tails to be already moved
    fn valid_moves_for(&self, id: u8) -> MoveVector {
        let snake = self.snakes.cell(id).get();
        let mut possible_moves = [false; 4];
        let head = match snake {
            Snake::Alive { head, .. } => head,
            _ => return MoveVector::new(None),
        };
        for direction in DIRECTIONS {
            let new_head = head + direction;
            if let Some(field) = self.board.cell(new_head.x, new_head.y) {
                if let BasicField::Empty | BasicField::Food = field.get().value() {
                    possible_moves[direction as usize] = true;
                }
            }
        }
        MoveVector::new(Some(possible_moves))
    }
}

impl<T: Field> From<&OriginalGameState> for GameState<T> {
    fn from(original_game_state: &OriginalGameState) -> Self {
        GameState::from_request(
            &original_game_state.board,
            &original_game_state.you,
            &original_game_state.turn,
        )
    }
}

impl<T> Display for GameState<T>
where
    T: Field,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const BUF_H: usize = 2 + HEIGHT as usize * 3;
        const BUF_W: usize = 3 + WIDTH as usize * 6;

        let mut buffer = [[' '; BUF_W]; BUF_H];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let tile = self.board.cell(x, y).unwrap().get().tile(
                    self.board.cell(x, y + 1).map(|f| f.get()),
                    self.board.cell(x, y - 1).map(|f| f.get()),
                    self.board.cell(x - 1, y).map(|f| f.get()),
                    self.board.cell(x + 1, y).map(|f| f.get()),
                    self.simulated_turn,
                    self.snakes().lengths(),
                );
                let row = (HEIGHT - 1 - y) as usize * 3 + 1;
                let col = x as usize * 6 + 2;
                for tr in 0..3_usize {
                    for tc in 0..5_usize {
                        let c = tile[tr][tc];
                        let br = row + tr;
                        let bc = col + tc;
                        if br < BUF_H && bc < BUF_W {
                            buffer[br][bc] = c;
                        }
                    }
                }
            }
        }

        // Second pass: write tail stack digit at each snake's tail center position
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                Snake::Alive { tail, stack, .. } | Snake::Headless { tail, stack, .. } => {
                    let row = (HEIGHT - 1 - tail.y) as usize * 3 + 2;
                    let col = tail.x as usize * 6 + 4;
                    let digit = (stack + b'0') as char;
                    buffer[row][col] = digit;
                }
                _ => (),
            }
        }

        // Add borders
        let bottom = "+---0-----1-----2-----3-----4-----5-----6-----7-----8-----9----10---+\n"
            .chars()
            .collect::<Vec<char>>();
        let left = "+|0||1||2||3||4||5||6||7||8||9|01|+"
            .chars()
            .collect::<Vec<char>>();
        for row in 0..BUF_H {
            buffer[row][0] = left[BUF_H - row - 1];
            buffer[row][BUF_W - 1] = left[BUF_H - row - 1];
        }
        for col in 0..BUF_W {
            buffer[0][col] = bottom[col];
            buffer[BUF_H - 1][col] = bottom[col];
        }

        let mut output = String::new();
        for row in 0..BUF_H {
            for col in 0..BUF_W {
                let c = buffer[row][col];
                output.push(c);
            }
            output.push('\n');
        }

        let mut other_info = String::from('\n');
        for i in 0..SNAKES {
            match self.snakes.cell(i).get() {
                Snake::Alive {
                    id, health, length, ..
                } => other_info.push_str(&format!(
                    "Snake {} (Alive) - Health: {}, Length: {}\n",
                    (id + b'A') as char,
                    health,
                    length
                )),
                Snake::Headless {
                    id, health, length, ..
                } => other_info.push_str(&format!(
                    "Snake {} (Headless) - Health: {}, Length: {}\n",
                    (id + b'A') as char,
                    health,
                    length
                )),
                Snake::Dead { id, .. } => {
                    other_info.push_str(&format!("Snake {} (Dead)\n", (id + b'A') as char))
                }
                Snake::Vanished { id, length, .. } => other_info.push_str(&format!(
                    "Snake {} (Vanished) - Length: {}\n",
                    (id + b'A') as char,
                    length
                )),
                Snake::NonExistent => (),
            }
        }
        output.push_str(&other_info);

        write!(f, "{}", output)?;
        Ok(())
    }
}

impl GameState<FloodFillField> {
    fn mark_tails(&mut self, turn: u8, tails: [Option<Coord>; SNAKES as usize]) {
        for id in 0..SNAKES {
            if let Some(tail) = tails[id as usize] {
                if let Some(cell) = self.board.cell_coord(tail) {
                    // Only mark if the tail cell was actually cleared by move_tails().
                    // When stack > 0, move_tails() decrements stack without advancing the
                    // tail, so the cell is still FloodFillField::Snake. Overwriting it
                    // with an Empty cell would cause the next move_tails() to vanish the
                    // snake prematurely while its body remains on the board.
                    if matches!(cell.get(), FloodFillField::Empty { turn: None }) {
                        cell.set(FloodFillField::tail(turn));
                    }
                }
            }
        }
    }

    fn prepare_flood_fill(&mut self, direction: Direction) -> FloodFillResult {
        let mut result = FloodFillResult::new();

        let tails = self.snakes().tails();
        self.move_tails();
        self.mark_tails(1, tails);

        match self.snakes().cell(0).get() {
            Snake::Alive { head, .. } => {
                let new_head = head + direction;
                match self.board.cell_coord(new_head).map(|f| f.get()) {
                    Some(field @ FloodFillField::Empty { .. }) => {
                        self.board
                            .cell_coord(new_head)
                            .unwrap()
                            .set(field.fill(0, 1));
                        result.flooded_area[0] = 1;
                    }
                    Some(field @ FloodFillField::Food { .. }) => {
                        self.board
                            .cell_coord(new_head)
                            .unwrap()
                            .set(field.fill(0, 1));
                        result.flooded_area[0] = 1;
                        result.food[0] = 1;
                    }
                    Some(FloodFillField::Snake { .. }) => {
                        result.not_enough_area_in_turn[0] = Some(0);
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        for id in 1..SNAKES {
            match self.snakes().cell(id).get() {
                Snake::Alive { head, .. } => {
                    let mut filled_one = false;
                    for d in DIRECTIONS {
                        let new_head = head + d;
                        match self.board.cell_coord(new_head).map(|f| f.get()) {
                            Some(field @ FloodFillField::Empty { .. }) => {
                                self.board
                                    .cell_coord(new_head)
                                    .unwrap()
                                    .set(field.fill(id, 1));
                                result.flooded_area[id as usize] += 1;
                                filled_one = true;
                            }
                            Some(field @ FloodFillField::Food { .. }) => {
                                self.board
                                    .cell_coord(new_head)
                                    .unwrap()
                                    .set(field.fill(id, 1));
                                result.flooded_area[id as usize] += 1;
                                result.food[id as usize] += 1;
                                filled_one = true;
                            }
                            Some(field @ FloodFillField::Filled { .. }) => {
                                let new_field = field.fill(id, 1);
                                self.board.cell_coord(new_head).unwrap().set(new_field);
                                if self.score_filled_field(&mut result, &new_field, id) {
                                    filled_one = true;
                                }
                            }
                            _ => (),
                        }
                    }
                    if !filled_one {
                        result.not_enough_area_in_turn[id as usize] = Some(0);
                    }
                }
                _ => (),
            }
        }
        result
    }

    fn score_filled_field(
        &self,
        result: &mut FloodFillResult,
        new_field: &FloodFillField,
        id: u8,
    ) -> bool {
        let lengths = self.snakes().lengths();

        match new_field {
            FloodFillField::Filled { by, .. } => {
                let mut best_length_on_same_turn = 0;
                for other_id in 0..id {
                    if by[other_id as usize] == by[id as usize] {
                        if lengths[other_id as usize] > best_length_on_same_turn {
                            best_length_on_same_turn = lengths[other_id as usize];
                        }
                    }
                }

                if lengths[id as usize] > best_length_on_same_turn {
                    result.flooded_area[id as usize] += 1;
                    if new_field.was_food() {
                        result.food[id as usize] += 1;
                    }

                    for other_id in 0..id {
                        if by[other_id as usize] == by[id as usize]
                            && lengths[other_id as usize] == best_length_on_same_turn
                        {
                            result.flooded_area[other_id as usize] -= 1;
                            if new_field.was_food() {
                                result.food[other_id as usize] -= 1;
                            }
                        }
                    }
                    return true;
                } else if lengths[id as usize] == best_length_on_same_turn {
                    result.flooded_area[id as usize] += 1;
                    if new_field.was_food() {
                        result.food[id as usize] += 1;
                    }
                    return true;
                } else {
                    return false;
                }
            }
            _ => panic!("Unexpected field type in resolve_conflict"),
        }
    }

    fn run_flood_fill(&mut self, result: &mut FloodFillResult) {
        let lengths = self.snakes().lengths();
        let mut all_flooded = false;
        let mut turn = 1;
        let mut can_ignite_filled = [false; SNAKES as usize];
        let mut tails = self.snakes().tails();
        while !all_flooded {
            turn += 1;
            all_flooded = true;
            self.move_tails();
            self.mark_tails(turn, tails);
            for id in 0..SNAKES {
                if result.flooded_area[id as usize] >= lengths[id as usize] {
                    can_ignite_filled[id as usize] = true;
                }
            }
            let read_board = self.board.clone();
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    match read_board.cell(x, y).unwrap().get() {
                        field @ FloodFillField::Empty { turn: marked_turn }
                        | field @ FloodFillField::Food { turn: marked_turn } => {
                            let mut can_fill = [false; SNAKES as usize];
                            for d in DIRECTIONS {
                                let neighbor_coord = Coord::new(x, y) + d;
                                match read_board.cell_coord(neighbor_coord).map(|f| f.get()) {
                                    Some(FloodFillField::Filled { hot, .. }) => {
                                        for id in 0..SNAKES {
                                            if let Some(hot_turn) = hot[id as usize] {
                                                if hot_turn == turn - 1 {
                                                    can_fill[id as usize] = true;
                                                }
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            let best_length_of_snakes_that_can_fill = (0..SNAKES)
                                .filter(|other_id| can_fill[*other_id as usize])
                                .map(|other_id| lengths[other_id as usize])
                                .max()
                                .unwrap_or(0);
                            let mut new_field = field;
                            for id in 0..SNAKES {
                                if can_fill[id as usize] {
                                    new_field = new_field.fill(id, turn);
                                    if lengths[id as usize] == best_length_of_snakes_that_can_fill {
                                        result.flooded_area[id as usize] += 1;
                                        if new_field.was_food() {
                                            result.food[id as usize] += 1;
                                        }
                                    }
                                }
                            }
                            if can_fill.iter().any(|can| *can)
                                && (marked_turn.is_none() || marked_turn.unwrap() < turn - 1)
                            {
                                all_flooded = false;
                            }
                            self.board.cell(x, y).unwrap().set(new_field);
                        }
                        FloodFillField::Snake { .. } => (),
                        field @ FloodFillField::Filled { .. } => {
                            let mut can_ignite = [false; SNAKES as usize];
                            for d in DIRECTIONS {
                                let neighbor_coord = Coord::new(x, y) + d;
                                match read_board.cell_coord(neighbor_coord).map(|f| f.get()) {
                                    Some(FloodFillField::Filled {
                                        hot: neighbor_hot, ..
                                    }) => {
                                        for id in 0..SNAKES {
                                            if let Some(hot_turn) = neighbor_hot[id as usize] {
                                                if hot_turn == turn - 1 {
                                                    can_ignite[id as usize] = true;
                                                }
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            let best_length_of_snakes_that_can_ignite = (0..SNAKES)
                                .filter(|other_id| can_ignite[*other_id as usize])
                                .map(|other_id| lengths[other_id as usize])
                                .max()
                                .unwrap_or(0);
                            let mut new_field = field;
                            for id in 0..SNAKES {
                                if can_ignite[id as usize]
                                    && can_ignite_filled[id as usize]
                                    && lengths[id as usize] == best_length_of_snakes_that_can_ignite
                                {
                                    new_field = new_field.ignite(id, turn);
                                }
                            }
                            self.board.cell(x, y).unwrap().set(new_field);
                        }
                    }
                }
            }

            for id in 0..SNAKES {
                if result.flooded_area[id as usize] < turn.min(lengths[id as usize])
                    && result.not_enough_area_in_turn[id as usize].is_none()
                {
                    result.not_enough_area_in_turn[id as usize] = Some(turn);
                }
            }
            tails = self.snakes().tails();
        }
    }

    pub fn flood_fill(&mut self, direction: Direction) -> FloodFillResult {
        let mut result = self.prepare_flood_fill(direction);
        self.run_flood_fill(&mut result);
        result
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloodFillResult {
    pub not_enough_area_in_turn: [Option<u8>; SNAKES as usize],
    pub flooded_area: [u8; SNAKES as usize],
    pub food: [u8; SNAKES as usize],
}

impl FloodFillResult {
    pub fn new() -> Self {
        FloodFillResult {
            not_enough_area_in_turn: [None; SNAKES as usize],
            flooded_area: [0; SNAKES as usize],
            food: [0; SNAKES as usize],
        }
    }
}

impl From<GameState<BasicField>> for GameState<FloodFillField> {
    fn from(state: GameState<BasicField>) -> Self {
        let new_board = Board::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = state.board.cell(x, y).unwrap();
                new_board
                    .cell(x, y)
                    .unwrap()
                    .set(FloodFillField::from(cell.get()));
            }
        }
        GameState {
            board: new_board,
            snakes: state.snakes,
            simulated_turn: state.simulated_turn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{logic::general::coord::Coord, read_game_state};

    #[test]
    fn test_memory_size() {
        assert_eq!(std::mem::size_of::<GameState<BasicField>>(), 279);
    }

    #[test]
    fn test_display() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
    }

    #[test]
    fn test_possible_moves() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        let moves = state.valid_moves();
        println!("{:#?}", moves);
        assert_eq!(moves.into_iter().len(), 36);

        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        let moves = state.valid_moves();
        assert_eq!(
            moves.get(0),
            MoveVector::new(Some([true, false, true, true]))
        );
        assert_eq!(
            moves.get(1),
            MoveVector::new(Some([true, false, false, false]))
        );
        assert_eq!(moves.get(2), MoveVector::new(None));
        assert_eq!(moves.get(3), MoveVector::new(None));
        let generated = moves.into_iter();
        assert_eq!(generated.len(), 3);
        for m in generated {
            assert_eq!(m[1], Some(Direction::Up));
        }

        let state = state.play(["RR", "UU", "", ""]);
        println!("{}", state);
        let moves = state.valid_moves().into_iter();
        assert_eq!(moves.len(), 6);
        println!("{:#?}", moves);

        let gamestate = read_game_state("requests/failure_9.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        let moves = state.valid_moves();
        assert_eq!(
            moves.get(0),
            MoveVector::new(Some([true, true, false, true]))
        );
    }

    #[test]
    fn test_next_state() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        let mut moves = [
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Down),
        ];
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(1).get() {
            Snake::Dead { .. } => (),
            _ => panic!("Problem with Snake B"),
        }
        moves = [None, None, Some(Direction::Left), Some(Direction::Left)];
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            Snake::Headless { .. } => (),
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(3).get() {
            Snake::Alive {
                head,
                length,
                stack,
                ..
            } => {
                assert_eq!(head, Coord::new(3, 4));
                assert_eq!(length, 6);
                assert_eq!(stack, 1);
            }
            _ => panic!("Problem with Snake D"),
        }
        match state.board.cell(4, 8).unwrap().get().value() {
            BasicField::Empty { .. } => (),
            _ => panic!("Problem with field (4, 8)"),
        }
        moves = [None, None, Some(Direction::Left), Some(Direction::Down)];
        state.next_state(moves);
        println!("{}", state);
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            Snake::Vanished { .. } => (),
            _ => panic!("Problem with Snake A"),
        }
        state.next_state(moves);
        println!("{}", state);
        moves = [None, None, Some(Direction::Left), Some(Direction::Right)];
        state.next_state(moves);
        println!("{}", state);
        state.next_state(moves);
        println!("{}", state);
        moves = [None, None, Some(Direction::Up), Some(Direction::Down)];
        state.next_state(moves);
        println!("{}", state);
        match state.snakes.cell(3).get() {
            Snake::Alive { .. } => (),
            _ => panic!("Problem with Head Tail movement order"),
        }
    }

    #[test]
    fn test_next_state_2() {
        let gamestate =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let mut state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        let moves = [
            Some(Direction::Right),
            Some(Direction::Down),
            Some(Direction::Down),
            Some(Direction::Down),
        ];
        state.next_state(moves);
        println!("{}", state);
        assert_eq!(state.snakes.cell(0).get(), Snake::Dead { id: 0 });
    }

    #[test]
    fn test_move_heads_headless() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        state.move_heads([
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Down),
            None,
        ]);
        println!("{}", state);
        match state.snakes.cell(3).get() {
            Snake::Headless { .. } => (),
            _ => panic!("Problem with Snake D"),
        }
    }

    #[test]
    fn test_move_heads_food() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        state.move_heads([
            Some(Direction::Up),
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Left),
        ]);
        println!("{}", state);
        match state.snakes.cell(1).get() {
            Snake::Alive {
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
            Some(Direction::Up),
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Left),
        ]);
        println!("Alternative 1:\n{}", state);
        match state.snakes.cell(3).get() {
            Snake::Dead { .. } => (),
            _ => panic!("Problem with Snake D"),
        }
        state2.move_heads([
            Some(Direction::Up),
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Down),
        ]);
        println!("Alternative 2:\n{}", state2);
        match state2.snakes.cell(3).get() {
            Snake::Alive {
                head,
                length,
                stack,
                ..
            } => {
                assert_eq!(head, Coord::new(3, 4));
                assert_eq!(length, 6);
                assert_eq!(stack, 1);
            }
            _ => panic!("Problem with Snake D"),
        }
    }

    #[test]
    fn test_move_heads() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::from(&gamestate);
        println!("{}", state);
        state.move_heads([
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Down),
            Some(Direction::Down),
        ]);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            Snake::Alive { head, .. } => assert_eq!(head, Coord { x: 0, y: 2 }),
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(1).get() {
            Snake::Dead { .. } => (),
            _ => panic!("Problem with Snake B"),
        }
        match state.snakes.cell(2).get() {
            Snake::Dead { .. } => (),
            _ => panic!("Problem with Snake C"),
        }
        match state.snakes.cell(3).get() {
            Snake::Alive { head, .. } => assert_eq!(head, Coord { x: 4, y: 4 }),
            _ => panic!("Problem with Snake D"),
        }
        match state.board.cell(0, 0).unwrap().get() {
            BasicField::Snake { id, next } => {
                assert_eq!(id, 0);
                assert_eq!(next, Some(Direction::Up));
            }
            _ => panic!("Problem with field (0, 0)"),
        }
        match state.board.cell(0, 2).unwrap().get() {
            BasicField::Snake { id, next } => {
                assert_eq!(id, 0);
                assert_eq!(next, None);
            }
            _ => panic!("Problem with field (1, 0)"),
        }
        match state.board.cell(4, 4).unwrap().get() {
            BasicField::Snake { id, next } => {
                assert_eq!(id, 3);
                assert_eq!(next, None);
            }
            _ => panic!("Problem with field (4, 4)"),
        }
        match state.board.cell(4, 5).unwrap().get() {
            BasicField::Snake { id, next } => {
                assert_eq!(id, 3);
                assert_eq!(next, Some(Direction::Down));
            }
            _ => panic!("Problem with field (4, 5)"),
        }
        match state.board.cell(5, 4).unwrap().get() {
            BasicField::Empty { .. } => (),
            _ => panic!("Problem with field (5, 4)"),
        }
        match state.board.cell(9, 0).unwrap().get() {
            BasicField::Empty { .. } => (),
            _ => panic!("Problem with field (9, 0)"),
        }
    }

    #[test]
    fn test_move_tails() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        match state.snakes.cell(0).get() {
            Snake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, Coord { x: 1, y: 0 });
            }
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(2).get() {
            Snake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 1);
                assert_eq!(tail, Coord { x: 9, y: 2 });
            }
            _ => panic!("Problem with Snake C"),
        }
        assert_eq!(
            state.board.cell(1, 0).unwrap().get(),
            BasicField::snake(0, Some(Direction::Left))
        );
        state.move_tails();
        assert_eq!(state.board.cell(1, 0).unwrap().get(), BasicField::empty());
        assert_eq!(
            state.board.cell(9, 2).unwrap().get(),
            BasicField::snake(2, Some(Direction::Down))
        );
        match state.snakes.cell(0).get() {
            Snake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, Coord { x: 0, y: 0 });
            }
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(2).get() {
            Snake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, Coord { x: 9, y: 2 });
            }
            _ => panic!("Problem with Snake C"),
        }
        state.move_tails().move_tails();
        assert_eq!(state.board.cell(0, 0).unwrap().get(), BasicField::empty());
        assert_eq!(
            state.board.cell(9, 0).unwrap().get(),
            BasicField::snake(2, None)
        );
        match state.snakes.cell(0).get() {
            Snake::Vanished { id, .. } => assert_eq!(id, 0),
            _ => panic!("Problem with Snake A"),
        }
        match state.snakes.cell(2).get() {
            Snake::Alive {
                stack, tail, head, ..
            } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, Coord { x: 9, y: 0 });
                assert_eq!(head, Coord { x: 9, y: 0 });
            }
            _ => panic!("Problem with Snake C"),
        }
    }

    #[test]
    fn test_from_request() {
        let gamestate = read_game_state("requests/example_move_request.json");
        let d_gamestate = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        assert_eq!(
            d_gamestate.board.cell(0, 0).unwrap().get(),
            BasicField::snake(0, None)
        );
        assert_eq!(
            d_gamestate.board.cell(1, 0).unwrap().get(),
            BasicField::snake(0, Some(Direction::Left))
        );
        assert_eq!(
            d_gamestate.snakes.cell(0).get(),
            Snake::Alive {
                id: 0,
                health: 54,
                length: 3,
                head: Coord { x: 0, y: 0 },
                tail: Coord { x: 2, y: 0 },
                stack: 0
            }
        );
        assert_eq!(
            d_gamestate.snakes.cell(1).get(),
            Snake::Alive {
                id: 1,
                health: 16,
                length: 3,
                head: Coord { x: 5, y: 3 },
                tail: Coord { x: 6, y: 2 },
                stack: 0
            }
        );
    }

    #[test]
    fn test_is_alive() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut d_gamestate = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", d_gamestate);

        assert!(matches!(
            d_gamestate.snakes.cell(0).get(),
            Snake::Alive { .. }
        ));
        d_gamestate.next_state([
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Down),
        ]);
        println!("{}", d_gamestate);
        assert!(matches!(
            d_gamestate.snakes.cell(0).get(),
            Snake::Dead { .. }
        ));
    }

    #[test]
    fn test_3_head_collision() {
        let gamestate = read_game_state("requests/test_3_head_collision.json");
        let mut state = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let moves = [
            Some(Direction::Left),
            Some(Direction::Up),
            None,
            Some(Direction::Right),
        ];

        assert!(matches!(state.snakes.cell(0).get(), Snake::Alive { .. }));
        assert!(matches!(state.snakes.cell(1).get(), Snake::Alive { .. }));
        assert!(matches!(state.snakes.cell(2).get(), Snake::Alive { .. }));
        assert!(matches!(state.snakes.cell(3).get(), Snake::Alive { .. }));

        state.next_state(moves);

        println!("{}", state);

        assert!(matches!(state.snakes.cell(0).get(), Snake::Alive { .. }));
        assert!(matches!(state.snakes.cell(1).get(), Snake::Dead { .. }));
        assert!(matches!(state.snakes.cell(2).get(), Snake::Headless { .. }));
        assert!(matches!(state.snakes.cell(3).get(), Snake::Dead { .. }));
    }

    #[test]
    fn test_quick_hash() {
        let gamestate = read_game_state("requests/test_move_request_2c.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        let gamestate_2 = read_game_state("requests/test_move_request_2b.json");
        let state_2 = GameState::<BasicField>::from_request(
            &gamestate_2.board,
            &gamestate_2.you,
            &gamestate_2.turn,
        );
        println!("{}", state_2);
        let hash = state.local_environment_hash(9);
        let hash_2 = state_2.local_environment_hash(9);
        println!("Hash: {}", hash);
        println!("Hash: {}", hash_2);
        assert_eq!(hash, hash_2);

        let hash_3 = state.local_environment_hash(10);
        let hash_4 = state_2.local_environment_hash(10);
        println!("Hash: {}", hash_3);
        println!("Hash: {}", hash_4);
        assert_ne!(hash_3, hash_4);
    }

    #[test]
    fn test_flood_fill() {
        let cases = [
            (
                "requests/failure_21_bait_into_trap_with_top_wall.json",
                Direction::Right,
            ),
            (
                "requests/failure_21_bait_into_trap_with_top_wall.json",
                Direction::Up,
            ),
        ];

        for (file, dir) in cases {
            println!("\n=== {} direction {:?} ===", file, dir);
            let gamestate = read_game_state(file);
            let state = GameState::<BasicField>::from_request(
                &gamestate.board,
                &gamestate.you,
                &gamestate.turn,
            );
            println!("{}", state);

            let mut ff_state: GameState<FloodFillField> = state.into();

            let result = ff_state.flood_fill(dir);

            println!("{}", ff_state);
            println!("{:?}", ff_state.board().cell(1, 5).unwrap().get());
            println!("{:?}", result);

            let total: u8 = result.flooded_area.iter().sum();
        }
    }

    #[test]
    fn play_state() {
        let gamestate = read_game_state("requests/failure_53_go_for_kill.json");
        let state = GameState::<BasicField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let new_state = state.play(["LLLLLDRRR", "LLLLLDDDR", "", ""]);
        println!("{}", new_state);
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use crate::read_game_state;
    use std::hint::black_box;

    #[bench]
    fn bench_move_tails(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        b.iter(|| {
            let state = state.clone();
            black_box(state).move_tails();
        });
    }

    #[bench]
    fn bench_move_heads(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        b.iter(|| {
            let state = state.clone();
            black_box(state).move_heads(black_box([
                Some(Direction::Up),
                Some(Direction::Left),
                Some(Direction::Down),
                Some(Direction::Down),
            ]));
        });
    }

    #[bench]
    fn bench_next_state(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        b.iter(|| {
            let state = state.clone();
            black_box(state).next_state(black_box([
                Some(Direction::Up),
                Some(Direction::Left),
                Some(Direction::Down),
                Some(Direction::Down),
            ]));
        });
    }

    #[bench]
    fn bench_local_environment_hash(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        b.iter(|| {
            let _ = state.local_environment_hash(black_box(5));
        });
    }

    #[bench]
    fn bench_flood_fill(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/example_move_request.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);
        b.iter(|| {
            let mut ff_state: GameState<FloodFillField> = state.clone().into();
            black_box(&mut ff_state).flood_fill(black_box(Direction::Up));
        });
    }
}
