use std::{cell::Cell, fmt::Display};

use crate::{logic::legacy::shared::e_snakes::SNAKES, Battlesnake, Board};

use super::{
    d_board::{SIZE, WIDTH},
    d_coord::DCoord,
    d_direction::DDirection,
    d_moves_set::DMoves,
    d_snake::DSnake,
    d_snakes::DSnakes,
};

#[derive(Clone)]
pub struct FGameState {
    board: FBoard,
    snakes: DSnakes,
}

impl FGameState {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let snakes = DSnakes::from_request(board, you);
        let d_board = FBoard::from_request(board, you);
        FGameState {
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
                            self.board
                                .cell(head.x, head.y)
                                .unwrap()
                                .set(FField::snake(id, Some(direction)));
                            if field.get().get_type() == FField::Food {
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
                    match self.board.cell(head.x, head.y).unwrap().get().get_type() {
                        FField::Snake => {
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
                        .set(FField::snake(id, None));
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
                    match self
                        .board
                        .cell(tail.x, tail.y)
                        .unwrap()
                        .get()
                        .get_direction()
                    {
                        Some(direction) => {
                            self.snakes.cell(id).set(snake.tail(tail + direction));
                            self.board
                                .cell(tail.x, tail.y)
                                .unwrap()
                                .set(FField::empty());
                        }
                        None => {
                            self.snakes.cell(id).set(snake.to_vanished());
                            self.board
                                .cell(tail.x, tail.y)
                                .unwrap()
                                .set(FField::empty());
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

#[derive(Clone)]
pub struct FBoard {
    fields: [Cell<FField>; SIZE as usize],
}

impl FBoard {
    pub fn from_request(board: &Board, you: &Battlesnake) -> Self {
        let d_board = FBoard::default();
        for food in board.food.iter() {
            d_board
                .cell(food.x as i8, food.y as i8)
                .unwrap()
                .set(FField::food());
        }
        let mut snake_id = 0;
        for snake in board.snakes.iter() {
            let id = if snake.id == you.id {
                0
            } else {
                snake_id += 1;
                snake_id
            };
            let mut last: Option<DCoord> = None;
            for coord in snake.body.iter() {
                let coord: DCoord = coord.into();

                match last {
                    Some(last) if last == coord => continue, // skip duplicate, is added to snake stack in snakes
                    _ => (),
                }

                if d_board.cell(coord.x, coord.y).unwrap().get().get_type() == FField::Empty {
                    let next: Option<DDirection> = if let Some(last) = last {
                        (last - coord).try_into().ok()
                    } else {
                        None
                    };
                    d_board
                        .cell(coord.x, coord.y)
                        .unwrap()
                        .set(FField::snake(id, next));
                    last = Some(coord);
                }
            }
        }
        d_board
    }

    pub fn cell(&self, x: i8, y: i8) -> Option<&Cell<FField>> {
        let index = y as i16 * WIDTH as i16 + x as i16;
        if x < 0 || y < 0 {
            return None;
        }
        self.fields.get(index as usize)
    }

    pub fn remove_snake(&self, snake: DSnake) {
        match snake {
            DSnake::Alive {
                id: snake_id,
                mut tail,
                ..
            }
            | DSnake::Headless {
                id: snake_id,
                mut tail,
                ..
            } => loop {
                if self.cell(tail.x, tail.y).unwrap().get().get_type() == FField::Snake {
                    if self.cell(tail.x, tail.y).unwrap().get().get_id() == snake_id {
                        let direction = self.cell(tail.x, tail.y).unwrap().get().get_direction();
                        self.cell(tail.x, tail.y).unwrap().set(FField::empty());
                        match direction {
                            Some(direction) => {
                                tail += direction.into();
                            }
                            None => break,
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            },
            _ => panic!("Cannot remove snake {:?} from board", snake),
        }
    }
}

impl Default for FBoard {
    fn default() -> Self {
        let fields = std::array::from_fn(|_| Cell::new(FField::empty()));
        Self { fields }
    }
}

#[derive(Clone, Copy, Debug)]
struct FField(u8);

impl FField {
    const Empty: u8 = 0b0;
    const Food: u8 = 0b1;
    const Snake: u8 = 0b10;

    pub fn empty() -> Self {
        FField(FField::Empty)
    }

    pub fn food() -> Self {
        FField(FField::Food)
    }

    pub fn snake(id: u8, direction: Option<DDirection>) -> Self {
        let s = FField::Snake
            + (id << 2)
            + if let Some(direction) = direction {
                (direction as u8) << 4
            } else {
                0b100 << 4
            };
        FField(s)
    }

    pub fn get_type(&self) -> u8 {
        self.0 & 0b11
    }

    pub fn get_id(&self) -> u8 {
        (self.0 >> 2) & 0b11
    }

    pub fn get_direction(&self) -> Option<DDirection> {
        DDirection::try_from((self.0 >> 4) & 0b111).ok()
    }
}

impl Display for FGameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::depth_first::game::{
            d_board::DBoard, d_field::DField, d_game_state::DGameState, d_snake::DSnake,
            d_snakes::DSnakes,
        },
        read_game_state,
    };

    use super::*;

    #[bench]
    // Should be < 50ns
    fn bench_next_state(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = FGameState::from_request(&gamestate.board, &gamestate.you);
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

    #[test]
    fn test() {
        let field = FField::food();
        let snake = FField::snake(1, Some(DDirection::Down));

        assert_eq!(field.get_type(), FField::Food);
        assert_eq!(snake.get_type(), FField::Snake);
        assert_eq!(snake.get_id(), 1);
        assert_eq!(snake.get_direction(), Some(DDirection::Down));

        let snake2 = FField::snake(2, None);
        assert_eq!(snake2.get_type(), FField::Snake);
        assert_eq!(snake2.get_id(), 2);
        assert_eq!(snake2.get_direction(), None);

        println!("{:08b}", snake.0);

        println!("{}", std::mem::size_of::<FField>());
        println!("{}", std::mem::size_of::<DField>());

        println!("{}", std::mem::size_of::<FGameState>());
        println!("{}", std::mem::size_of::<DGameState>());
    }

    #[test]
    fn test_next_state() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let mut state = FGameState::from_request(&gamestate.board, &gamestate.you);
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
        assert_eq!(
            state.board.cell(4, 8).unwrap().get().get_type(),
            FField::Empty
        );
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
}
