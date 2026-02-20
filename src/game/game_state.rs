use arrayvec::ArrayVec;

use super::{
    moves::{Moves, MovesSet},
    snake::Snake,
    snakes::Snakes,
};
use crate::{
    OriginalBattlesnake, OriginalBoard, OriginalGameState,
    game::{
        board::{Board, HEIGHT, WIDTH},
        coord::Coord,
        direction::{DIRECTION_LIST, Direction},
        field::{BasicField, Field},
    },
    logic::legacy::shared::e_snakes::SNAKES,
};
use std::{
    fmt::{Display, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Clone)]
pub struct GameState<T: Field> {
    board: Board<T>,
    snakes: Snakes,
    turn: i32,
}

impl<T: Field> GameState<T> {
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

    pub fn from_request(board: &OriginalBoard, you: &OriginalBattlesnake, turn: &i32) -> Self {
        let snakes = Snakes::from_request(board, you);
        let d_board = Board::from_request(board, you);
        GameState {
            board: d_board,
            snakes,
            turn: *turn,
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
                                .set(T::snake(id, Some(direction)));
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
                    .set(T::snake(id, None));
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

    pub fn possible_moves(&self, consider: [bool; SNAKES as usize]) -> MovesSet {
        let mut possible_moves = [[false; 4]; SNAKES as usize];
        let mut moved_tails = self.clone();
        moved_tails.move_tails();
        for id in 0..SNAKES {
            if consider[id as usize] {
                possible_moves[id as usize] = moved_tails.possible_moves_for(id);
            }
        }
        MovesSet::new(possible_moves)
    }

    pub fn possible_moves_for(&self, id: u8) -> [bool; 4] {
        let snake = self.snakes.cell(id).get();
        let mut possible_moves = [false; 4];
        let head = match snake {
            Snake::Alive { head, .. } => head,
            _ => return possible_moves,
        };
        for direction in DIRECTION_LIST {
            let new_head = head + direction;
            if let Some(field) = self.board.cell(new_head.x, new_head.y) {
                if let BasicField::Empty | BasicField::Food = field.get().value() {
                    possible_moves[direction as usize] = true;
                }
            }
        }
        // At least one move must be possible
        // If no move is possible, set the first one to true
        if possible_moves.iter().all(|&x| !x) {
            possible_moves[0] = true;
        }
        possible_moves
    }

    pub fn get_alive(&self) -> [bool; SNAKES as usize] {
        let mut alive = [false; SNAKES as usize];
        for i in 0..SNAKES {
            alive[i as usize] = match self.snakes.cell(i).get() {
                Snake::Alive { .. } => true,
                Snake::Headless { .. } => true,
                Snake::Vanished { .. } => true,
                _ => false,
            }
        }
        alive
    }

    pub fn get_length(&self) -> Option<usize> {
        let snake = self.snakes.cell(0).get();
        match snake {
            Snake::Alive { length, .. } => Some(length as usize),
            _ => None,
        }
    }

    pub fn quick_hash(&self, distance: u8) -> u64 {
        let distance = distance as i8;
        let snake = self.snakes.cell(0).get();
        let mut hasher = DefaultHasher::new();
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
        self.get_alive().hash(&mut hasher);
        return hasher.finish();
    }

    pub fn get_heads(&self) -> [Option<Coord>; SNAKES as usize] {
        let mut heads = [None; SNAKES as usize];
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                Snake::Alive { head, .. } => heads[i as usize] = Some(head),
                _ => (),
            }
        }
        heads
    }
}

impl<T: Field> From<OriginalGameState> for GameState<T> {
    fn from(original_game_state: OriginalGameState) -> Self {
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
        let row = [' '; WIDTH as usize * 3 * 2];
        let mut board = [row; HEIGHT as usize * 3];

        // Write head markers before board
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            if let Snake::Alive { head, id, .. } = snake {
                let id = (id + b'A') as char;
                let x = head.x;
                let y = head.y;
                board[y as usize * 3 + 1][x as usize * 3 * 2] = id;
                board[y as usize * 3 + 1][x as usize * 3 * 2 + 2 * 2] = id;
                board[y as usize * 3][x as usize * 3 * 2 + 2] = id;
                board[y as usize * 3 + 2][x as usize * 3 * 2 + 2] = id;
                board[y as usize * 3 + 1][x as usize * 3 * 2 + 2] = id;
            }
        }

        // Fill the board with the current state
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.board.cell(x, y).unwrap().get().value() {
                    BasicField::Empty { .. } => {
                        board[y as usize * 3 + 1][x as usize * 3 * 2 + 2] = '.';
                    }
                    BasicField::Food { .. } => {
                        board[y as usize * 3 + 1][x as usize * 3 * 2 + 2] = 'X';
                    }
                    BasicField::Snake { id, next } => {
                        let c = (id + b'a') as char;
                        board[y as usize * 3 + 1][x as usize * 3 * 2 + 2] = '*';
                        match next {
                            Some(Direction::Up) => {
                                board[y as usize * 3 + 2][x as usize * 3 * 2 + 2] = c;
                                board[y as usize * 3 + 3][x as usize * 3 * 2 + 2] = c;
                            }
                            Some(Direction::Down) => {
                                board[y as usize * 3][x as usize * 3 * 2 + 2] = c;
                                board[y as usize * 3 - 1][x as usize * 3 * 2 + 2] = c;
                            }
                            Some(Direction::Left) => {
                                board[y as usize * 3 + 1][x as usize * 3 * 2] = c;
                                board[y as usize * 3 + 1][x as usize * 3 * 2 - 2] = c;
                            }
                            Some(Direction::Right) => {
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
                Snake::Alive { tail, stack, .. } | Snake::Headless { tail, stack, .. } => {
                    board[tail.y as usize * 3 + 1][tail.x as usize * 3 * 2 + 2] =
                        (stack + b'0') as char;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_game_state;

    #[test]
    fn test_memory_size() {
        assert_eq!(std::mem::size_of::<GameState<BasicField>>(), 284);
    }

    #[test]
    fn test_display() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
    }

    #[test]
    fn test_possible_moves() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = state.possible_moves([true, true, true, true]);
        println!("{:#?}", moves);
        assert_eq!(moves.generate().len(), 36);

        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = state.possible_moves([true, true, true, true]);
        assert_eq!(moves.get(0), [true, false, true, true]);
        assert_eq!(moves.get(1), [true, false, false, false]);
        assert_eq!(moves.get(2), [false, false, false, false]);
        assert_eq!(moves.get(3), [false, false, false, false]);
        let generated = moves.generate();
        assert_eq!(generated.len(), 3);
        for m in generated {
            assert_eq!(m[1], Some(Direction::Up));
        }

        let state = state.play(["RR", "UU", "", ""]);
        println!("{}", state);
        let moves = state.possible_moves([true, true, true, true]).generate();
        assert_eq!(moves.len(), 6);
        println!("{:#?}", moves);

        let gamestate = read_game_state("requests/failure_9.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = state.possible_moves([true, true, true, true]);
        assert_eq!(moves.get(0), [true, true, false, true]);

        let gamestate = read_game_state("requests/failure_2.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = state.possible_moves([true, false, true, false]);
        assert_eq!(moves.get(1), [false, false, false, false]);
    }

    #[test]
    fn test_next_state() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::<BasicField>::from(gamestate);
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
        let mut state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = [
            Some(Direction::Right),
            Some(Direction::Down),
            Some(Direction::Down),
            Some(Direction::Down),
        ];
        state.next_state(moves);
        println!("{}", state);
        assert!(!state.get_alive()[0]);
    }

    #[test]
    fn test_move_heads_headless() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = GameState::<BasicField>::from(gamestate);
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
        let mut state = GameState::<BasicField>::from(gamestate);
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
        let mut state = GameState::from(gamestate);
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
        assert!(d_gamestate.get_alive()[0]);
        d_gamestate.next_state([
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Down),
        ]);
        println!("{}", d_gamestate);
        assert!(!d_gamestate.get_alive()[0]);
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

        assert_eq!(state.get_alive(), [true, true, true, true]);

        state.next_state(moves);

        println!("{}", state);

        assert_eq!(state.get_alive(), [true, false, true, false]);
    }

    #[test]
    fn test_quick_hash() {
        let gamestate = read_game_state("requests/test_move_request_2c.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let gamestate_2 = read_game_state("requests/test_move_request_2b.json");
        let state_2 = GameState::<BasicField>::from_request(
            &gamestate_2.board,
            &gamestate_2.you,
            &gamestate_2.turn,
        );
        println!("{}", state_2);
        let hash = state.quick_hash(9);
        let hash_2 = state_2.quick_hash(9);
        println!("Hash: {}", hash);
        println!("Hash: {}", hash_2);
        assert_eq!(hash, hash_2);

        let hash_3 = state.quick_hash(10);
        let hash_4 = state_2.quick_hash(10);
        println!("Hash: {}", hash_3);
        println!("Hash: {}", hash_4);
        assert_ne!(hash_3, hash_4);
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

    #[bench]
    fn bench_next_state(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        let moves = [
            Some(Direction::Up),
            Some(Direction::Left),
            Some(Direction::Left),
            Some(Direction::Down),
        ];
        b.iter(|| {
            let mut state = state.clone();
            state.next_state(moves);
        });
    }

    #[bench]
    fn bench_possible_moves(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = GameState::<BasicField>::from(gamestate);
        println!("{}", state);
        b.iter(|| {
            let _ = state.possible_moves([true, true, true, true]);
        });
    }
}
