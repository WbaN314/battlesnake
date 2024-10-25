use super::{
    d_board::{DBoard, HEIGHT, WIDTH},
    d_direction::{DDirection, D_DIRECTION_LIST},
    d_field::DField,
    d_moves_set::{DMoves, DMovesSet},
    d_snake::DSnake,
    d_snakes::DSnakes,
};
use crate::{logic::legacy::shared::e_snakes::SNAKES, Battlesnake, Board};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct DGameState {
    board: DBoard,
    snakes: DSnakes,
}

impl DGameState {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let snakes = DSnakes::from_request(board, you);
        let d_board = DBoard::from_request(board, you);
        DGameState {
            board: d_board,
            snakes,
        }
    }

    pub fn next_state(&mut self, moves: DMoves) -> &mut Self {
        // Elimination handling https://github.com/BattlesnakeOfficial/rules/blob/main/standard.go#L172
        // Eliminate starved snakes first (moving on food with 1 health in previous round is allowed, moving on non food will die now)
        // Evaluate and eliminate collisions after
        self.move_tails().move_heads(moves)
    }

    fn move_heads(&mut self, moves: DMoves) -> &mut Self {
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
                            self.board.cell(head.x, head.y).unwrap().set(DField::Snake {
                                id,
                                next: Some(direction),
                            });
                            if field.get() == DField::Food {
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
                        .set(snake.health(health - 1).to_headless().unmoved(1));
                }
                (
                    DSnake::Headless {
                        health, unmoved, ..
                    },
                    None,
                ) => {
                    self.snakes
                        .cell(id)
                        .set(snake.health(health - 1).unmoved(unmoved + 1));
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
                    match self.board.cell(head.x, head.y).unwrap().get() {
                        DField::Snake { .. } => {
                            snakes_to_remove[id as usize] = Some(snake);
                            self.snakes.cell(id).set(snake.to_dead());
                        }
                        _ => (),
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
                        .set(DField::Snake { id, next: None });
                }
                _ => (),
            }
        }

        self
    }

    fn move_tails(&mut self) -> &mut Self {
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { stack, .. } | DSnake::Headless { stack, .. } if stack > 0 => {
                    self.snakes.cell(id).set(snake.stack(stack - 1));
                }
                DSnake::Alive { tail, .. } | DSnake::Headless { tail, .. } => {
                    match self.board.cell(tail.x, tail.y).unwrap().get() {
                        DField::Snake {
                            next: Some(direction),
                            ..
                        } => {
                            self.snakes.cell(id).set(snake.tail(tail + direction));
                            self.board.cell(tail.x, tail.y).unwrap().set(DField::Empty);
                        }
                        DField::Snake { next: None, .. } => {
                            self.snakes.cell(id).set(snake.to_vanished());
                            self.board.cell(tail.x, tail.y).unwrap().set(DField::Empty);
                        }
                        _ => {
                            panic!("Snake tail is on invalid field");
                        }
                    }
                }
                _ => (),
            }
        }
        self
    }
}

impl DGameState {
    // MovesSet Generation
    pub fn possible_moves(&self) -> DMovesSet {
        let mut possible_moves = [[false; 4]; SNAKES as usize];
        for id in 0..SNAKES {
            let snake = self.snakes.cell(id).get();
            match snake {
                DSnake::Alive { head, .. } => {
                    for direction in D_DIRECTION_LIST {
                        let new_head = head + direction;
                        if let Some(field) = self.board.cell(new_head.x, new_head.y) {
                            match field.get() {
                                DField::Empty | DField::Food => {
                                    possible_moves[id as usize][direction as usize] = true;
                                }
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        DMovesSet::new(possible_moves)
    }
}

impl Display for DGameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let row = [' '; WIDTH as usize * 3];
        let mut board = [row; HEIGHT as usize * 3];

        // Write head markers before board
        for i in 0..SNAKES {
            let snake = self.snakes.cell(i).get();
            match snake {
                DSnake::Alive { head, id, .. } => {
                    let id = (id + 'A' as u8) as char;
                    let x = head.x;
                    let y = head.y;
                    board[y as usize * 3 + 1][x as usize * 3] = id;
                    board[y as usize * 3 + 1][x as usize * 3 + 2] = id;
                    board[y as usize * 3][x as usize * 3 + 1] = id;
                    board[y as usize * 3 + 2][x as usize * 3 + 1] = id;
                    board[y as usize * 3 + 1][x as usize * 3 + 1] = id;
                }
                _ => (),
            }
        }

        // Fill the board with the current state
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.board.cell(x, y).unwrap().get() {
                    DField::Empty => {
                        board[y as usize * 3 + 1][x as usize * 3 + 1] = '.';
                    }
                    DField::Food => {
                        board[y as usize * 3 + 1][x as usize * 3 + 1] = 'X';
                    }
                    DField::Snake { id, next } => {
                        let c = (id + 'a' as u8) as char;
                        board[y as usize * 3 + 1][x as usize * 3 + 1] = '*';
                        match next {
                            Some(DDirection::Up) => {
                                board[y as usize * 3 + 2][x as usize * 3 + 1] = c;
                                board[y as usize * 3 + 3][x as usize * 3 + 1] = c;
                            }
                            Some(DDirection::Down) => {
                                board[y as usize * 3][x as usize * 3 + 1] = c;
                                board[y as usize * 3 - 1][x as usize * 3 + 1] = c;
                            }
                            Some(DDirection::Left) => {
                                board[y as usize * 3 + 1][x as usize * 3] = c;
                                board[y as usize * 3 + 1][x as usize * 3 - 1] = c;
                            }
                            Some(DDirection::Right) => {
                                board[y as usize * 3 + 1][x as usize * 3 + 2] = c;
                                board[y as usize * 3 + 1][x as usize * 3 + 3] = c;
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
                    board[tail.y as usize * 3 + 1][tail.x as usize * 3 + 1] =
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
            for x in 0..board.len() {
                output.push(board[y][x]);
                output.push(' ');
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
                DSnake::Vanished { id, .. } => {
                    other_info.push_str(&format!("Snake {} (Vanished)\n", (id + 'A' as u8) as char))
                }
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
        logic::depth_first::{
            d_coord::DCoord, d_direction::DDirection, d_field::DField, d_snake::DSnake,
        },
        read_game_state,
    };

    #[test]
    fn test_display() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
    }

    #[bench]
    // Should be < 50ns
    fn bench_next_state(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
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
    fn bench_possible_moves(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
        b.iter(|| {
            let _ = state.possible_moves();
        });
    }

    #[test]
    fn test_possible_moves() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
        let moves = state.possible_moves();
        println!("{:#?}", moves);
    }

    #[test]
    fn test_next_state() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you);
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
            DField::Empty => (),
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
    fn test_move_heads_headless() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you);
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
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you);
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
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
        state.move_heads([
            Some(DDirection::Up),
            Some(DDirection::Left),
            Some(DDirection::Down),
            Some(DDirection::Down),
        ]);
        println!("{}", state);
        match state.snakes.cell(0).get() {
            DSnake::Alive { head, .. } => assert_eq!(head, DCoord { x: 0, y: 1 }),
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
            DField::Snake { id, next } => {
                assert_eq!(id, 0);
                assert_eq!(next, Some(DDirection::Up));
            }
            _ => panic!("Problem with field (0, 0)"),
        }
        match state.board.cell(0, 1).unwrap().get() {
            DField::Snake { id, next } => {
                assert_eq!(id, 0);
                assert_eq!(next, None);
            }
            _ => panic!("Problem with field (1, 0)"),
        }
        match state.board.cell(4, 4).unwrap().get() {
            DField::Snake { id, next } => {
                assert_eq!(id, 3);
                assert_eq!(next, None);
            }
            _ => panic!("Problem with field (4, 4)"),
        }
        match state.board.cell(4, 5).unwrap().get() {
            DField::Snake { id, next } => {
                assert_eq!(id, 3);
                assert_eq!(next, Some(DDirection::Down));
            }
            _ => panic!("Problem with field (4, 5)"),
        }
        match state.board.cell(5, 4).unwrap().get() {
            DField::Empty => (),
            _ => panic!("Problem with field (5, 4)"),
        }
        match state.board.cell(9, 0).unwrap().get() {
            DField::Empty => (),
            _ => panic!("Problem with field (9, 0)"),
        }
    }

    #[test]
    fn test_move_tails() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = DGameState::from_request(&gamestate.board, &gamestate.you);
        match state.snakes.cell(0).get() {
            DSnake::Alive { stack, tail, .. } => {
                assert_eq!(stack, 0);
                assert_eq!(tail, DCoord { x: 2, y: 0 });
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
            state.board.cell(2, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                next: Some(DDirection::Left)
            }
        );
        state.move_tails();
        assert_eq!(state.board.cell(2, 0).unwrap().get(), DField::Empty);
        assert_eq!(
            state.board.cell(9, 2).unwrap().get(),
            DField::Snake {
                id: 2,
                next: Some(DDirection::Down)
            }
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
                assert_eq!(stack, 0);
                assert_eq!(tail, DCoord { x: 9, y: 2 });
            }
            _ => panic!("Problem with Snake C"),
        }
        state.move_tails().move_tails();
        assert_eq!(state.board.cell(0, 0).unwrap().get(), DField::Empty);
        assert_eq!(
            state.board.cell(9, 0).unwrap().get(),
            DField::Snake { id: 2, next: None }
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
        let d_gamestate = DGameState::from_request(&gamestate.board, &gamestate.you);
        assert_eq!(
            d_gamestate.board.cell(0, 0).unwrap().get(),
            DField::Snake { id: 0, next: None }
        );
        assert_eq!(
            d_gamestate.board.cell(1, 0).unwrap().get(),
            DField::Snake {
                id: 0,
                next: Some(DDirection::Left)
            }
        );
        assert_eq!(
            d_gamestate.board.cell(5, 4).unwrap().get(),
            DField::Snake { id: 1, next: None }
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
                length: 4,
                head: DCoord { x: 5, y: 4 },
                tail: DCoord { x: 6, y: 2 },
                stack: 0
            }
        );
    }
}
