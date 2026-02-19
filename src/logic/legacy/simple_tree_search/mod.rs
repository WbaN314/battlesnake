use log::debug;

use crate::{OriginalBoard, OriginalCoord, OriginalDirection, OriginalGameState};

use super::shared::brain::Brain;

fn simulate_snakes_step(board: &OriginalBoard) -> Vec<OriginalBoard> {
    let mut new_boards = Vec::with_capacity(board.snakes.len().pow(4));

    let mut decisions = Directions::new(board.snakes.len());

    let mut decision = Some(&decisions.v);
    while let Some(directions) = decision {
        let mut board_clone = board.clone();
        for snake_index in 0..board_clone.snakes.len() {
            let x = board_clone.snakes[snake_index].body[0].x;
            let y = board_clone.snakes[snake_index].body[0].y;
            let new_head = match directions[snake_index] {
                OriginalDirection::Up => OriginalCoord { x, y: y + 1 },
                OriginalDirection::Down => OriginalCoord { x, y: y - 1 },
                OriginalDirection::Left => OriginalCoord { x: x - 1, y },
                OriginalDirection::Right => OriginalCoord { x: x + 1, y },
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

fn evaluate_snakes_step(board: &mut OriginalBoard, you: &String) -> (OriginalDirection, i32) {
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
            (1, 0) => OriginalDirection::Right,
            (-1, 0) => OriginalDirection::Left,
            (0, 1) => OriginalDirection::Up,
            (0, -1) => OriginalDirection::Down,
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
            if let SnakeChange::Battle(j) = snake_changes[i] {
                if board.snakes[i].length < board.snakes[j].length {
                    battle_results[i] = SnakeChange::Die;
                } else if board.snakes[i].length == board.snakes[j].length {
                    battle_results[i] = SnakeChange::Die;
                    battle_results[j] = SnakeChange::Die;
                } else {
                    battle_results[j] = SnakeChange::Die;
                }
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
        (OriginalDirection::Up, 0)
    }
}

struct Directions {
    v: Vec<OriginalDirection>,
}

impl Directions {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(OriginalDirection::Up)
        }
        Directions { v }
    }

    fn next(&mut self) -> Option<&Vec<OriginalDirection>> {
        let mut working_index = None;
        for i in 0..self.v.len() {
            if self.v[i] != OriginalDirection::Right {
                working_index = Some(i);
                break;
            }
        }
        if let Some(i) = working_index {
            match self.v[i] {
                OriginalDirection::Up => self.v[i] = OriginalDirection::Down,
                OriginalDirection::Down => self.v[i] = OriginalDirection::Left,
                OriginalDirection::Left => self.v[i] = OriginalDirection::Right,
                OriginalDirection::Right => unreachable!(),
            }
            for j in 0..i {
                self.v[j] = OriginalDirection::Up;
            }
            Some(&self.v)
        } else {
            None
        }
    }
}

pub struct SimpleTreeSearchSnake {}

impl Default for SimpleTreeSearchSnake {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleTreeSearchSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for SimpleTreeSearchSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let (mut new_boards, mut move_scores) = step(&gamestate.board, &gamestate.you.id);

        let depth = if gamestate.board.snakes.len() == 2 {
            3
        } else {
            1
        };

        let new_combinations_number =
            4_i32.pow(gamestate.board.snakes.len() as u32) * new_boards.len() as i32;
        for _ in 0..depth {
            for i in 0..4 {
                move_scores[i] = move_scores[i].saturating_mul(new_combinations_number);
            }
            let mut tmp_boards = Vec::with_capacity(new_combinations_number as usize);
            for board in new_boards.iter() {
                let (mut b, s) = step(board, &gamestate.you.id);
                for i in 0..4 {
                    move_scores[i] = move_scores[i].saturating_add(s[i]);
                }
                tmp_boards.append(&mut b);
            }
            new_boards = tmp_boards;
        }

        debug!("{:?}", move_scores);

        let mut best_move = OriginalDirection::Up;
        let mut best_score = move_scores[0];
        for i in 1..move_scores.len() {
            if move_scores[i] > best_score {
                best_score = move_scores[i];
                best_move = match i {
                    1 => OriginalDirection::Down,
                    2 => OriginalDirection::Left,
                    3 => OriginalDirection::Right,
                    _ => unreachable!(),
                }
            }
        }
        best_move
    }
}

fn step(board: &OriginalBoard, id: &String) -> (Vec<OriginalBoard>, [i32; 4]) {
    let mut new_boards = simulate_snakes_step(board);

    let mut move_scores: [i32; 4] = [0; 4];
    for board in new_boards.iter_mut() {
        match evaluate_snakes_step(board, id) {
            (OriginalDirection::Up, score) => move_scores[0] = move_scores[0].saturating_add(score),
            (OriginalDirection::Down, score) => {
                move_scores[1] = move_scores[1].saturating_add(score)
            }
            (OriginalDirection::Left, score) => {
                move_scores[2] = move_scores[2].saturating_add(score)
            }
            (OriginalDirection::Right, score) => {
                move_scores[3] = move_scores[3].saturating_add(score)
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
        let directions = vec![
            OriginalDirection::Up,
            OriginalDirection::Right,
            OriginalDirection::Right,
        ];
        let mut directions_iter = Directions { v: directions };

        assert_eq!(
            directions_iter.next(),
            Some(&vec![
                OriginalDirection::Down,
                OriginalDirection::Right,
                OriginalDirection::Right
            ])
        );
        assert_eq!(
            directions_iter.next(),
            Some(&vec![
                OriginalDirection::Left,
                OriginalDirection::Right,
                OriginalDirection::Right
            ])
        );
        assert_eq!(
            directions_iter.next(),
            Some(&vec![
                OriginalDirection::Right,
                OriginalDirection::Right,
                OriginalDirection::Right
            ])
        );

        assert_eq!(directions_iter.next(), None);
    }
}
