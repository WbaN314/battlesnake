use core::fmt;

use crate::{Battlesnake, Board};

use super::{
    e_board::{EBoard, EField, X_SIZE, Y_SIZE},
    e_coord::ECoord,
    e_direction::{EDirection, EDIRECTION_VECTORS},
    e_snakes::{Death, ESnake, ESnakes, Result, SNAKES},
};

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

    fn eliminate_dead_snake(&self, snake_index: u8) {
        let mut eliminate = false;
        if let Some(snake) = self.snakes.get(snake_index).as_ref() {
            if snake.die {
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
    }

    pub fn move_snakes(&mut self, moveset: [Option<EDirection>; 4]) -> Result<()> {
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
                    Some(EField::Food) => snake.health = 100,
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
                if let Some(mv) = moveset[i as usize] {
                    let new_head = snake.head + EDIRECTION_VECTORS[mv.to_usize()];
                    // handle old snake head EField
                    match self.board.get(snake.head.x, snake.head.y) {
                        Some(EField::SnakePart {
                            snake_number,
                            stacked,
                            ..
                        }) => self.board.set(
                            snake.head.x,
                            snake.head.y,
                            EField::SnakePart {
                                snake_number,
                                stacked: stacked,
                                next: Some(new_head),
                            },
                        ),
                        _ => unreachable!("Old snake head not on snake part."),
                    };
                    // handle new snake head EField
                    snake.head = new_head;
                    match self.board.get(snake.head.x, snake.head.y) {
                        Some(EField::Empty) => {
                            self.board.set(
                                snake.head.x,
                                snake.head.y,
                                EField::Contested {
                                    snake_number: i,
                                    food: false,
                                },
                            );
                        }
                        Some(EField::Food) => {
                            // health is handled before, no handling here
                            // grow is set on contested EField evaluation
                            self.board.set(
                                snake.head.x,
                                snake.head.y,
                                EField::Contested {
                                    snake_number: i,
                                    food: true,
                                },
                            );
                        }
                        Some(EField::Contested { snake_number, food }) => {
                            match self.snakes.get_mut(snake_number).as_mut() {
                                Some(other_snake) => {
                                    if snake.length > other_snake.length {
                                        other_snake.die = true;
                                        self.board.set(
                                            snake.head.x,
                                            snake.head.y,
                                            EField::Contested {
                                                snake_number: i,
                                                food,
                                            },
                                        );
                                    } else if snake.length < other_snake.length {
                                        snake.die = true;
                                        self.board.set(
                                            snake.head.x,
                                            snake.head.y,
                                            EField::Contested { snake_number, food },
                                        );
                                    } else {
                                        snake.die = true;
                                        other_snake.die = true;
                                        self.board.set(
                                            snake.head.x,
                                            snake.head.y,
                                            EField::Contested { snake_number, food },
                                        );
                                    }
                                }
                                None => unreachable!("Ghost snake"),
                            }
                        }
                        Some(EField::SnakePart { .. }) => {
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

        // Make contested EFields to snakeparts again. Only winner snakes should have contested heads, losers should not have contested EFields set anymore.
        // Handle tails of surviving snakes and reset grow
        for i in 0..SNAKES {
            // Handle contested EFields
            if let Some(snake) = self.snakes.get_mut(i).as_mut() {
                match self.board.get(snake.head.x, snake.head.y) {
                    Some(EField::Contested { snake_number, food }) => {
                        if food {
                            snake.grow = true;
                        }
                        self.board.set(
                            snake.head.x,
                            snake.head.y,
                            EField::SnakePart {
                                snake_number,
                                stacked: 1,
                                next: None,
                            },
                        );
                    }
                    Some(EField::SnakePart { .. }) => {
                        // might happen if snakes have no moves processed (i.e. tofar away)
                        ()
                    }
                    _ => unreachable!("Invalid board state"),
                };

                // Handle tail
                match self.board.get(snake.tail.x, snake.tail.y) {
                    Some(EField::SnakePart {
                        snake_number,
                        stacked,
                        next,
                    }) => {
                        match stacked {
                            1 => {
                                self.board.set(snake.tail.x, snake.tail.y, EField::Empty);
                                match next {
                                    Some(next) => snake.tail = next,
                                    None => snake.die = true,
                                }
                            }
                            2.. => {
                                self.board.set(
                                    snake.tail.x,
                                    snake.tail.y,
                                    EField::SnakePart {
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
}

impl fmt::Display for EGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = String::with_capacity((X_SIZE + 1) as usize * Y_SIZE as usize);
        for y in (0..Y_SIZE).rev() {
            for x in 0..X_SIZE {
                if let Some(state) = self.board.get(x as i8, y as i8) {
                    output.push(match state {
                        EField::Empty => '.',
                        EField::Food => 'F',
                        EField::SnakePart { snake_number, .. } => {
                            char::from_digit(snake_number as u32, 10).unwrap_or('?')
                        }
                        EField::Filled => 'X',
                        EField::Contested { .. } => 'C',
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
