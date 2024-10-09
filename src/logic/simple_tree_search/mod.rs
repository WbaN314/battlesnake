use crate::{Battlesnake, Board, Coord, Game};

use super::shared::{brain::Brain, direction::Direction};

fn simulate_snakes_step(board: &Board) -> Vec<Board> {
    let mut new_boards = Vec::with_capacity(board.snakes.len().pow(4));

    let mut decisions = Directions::new(board.snakes.len());

    let mut decision = Some(&decisions.v);
    while let Some(directions) = decision {
        let mut board_clone = board.clone();
        for snake_index in 0..board_clone.snakes.len() {
            let x = board_clone.snakes[snake_index].body[0].x;
            let y = board_clone.snakes[snake_index].body[0].y;
            let new_head = match directions[snake_index] {
                Direction::Up => Coord { x, y: y + 1 },
                Direction::Down => Coord { x, y: y - 1 },
                Direction::Left => Coord { x: x - 1, y },
                Direction::Right => Coord { x: x + 1, y },
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

fn evaluate_snakes_step(board: &mut Board, you: &String) -> (Direction, i32) {
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
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Up,
            (0, -1) => Direction::Down,
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
            match snake_changes[i] {
                SnakeChange::Battle(j) => {
                    if board.snakes[i].length < board.snakes[j].length {
                        battle_results[i] = SnakeChange::Die;
                    } else if board.snakes[i].length == board.snakes[j].length {
                        battle_results[i] = SnakeChange::Die;
                        battle_results[j] = SnakeChange::Die;
                    } else {
                        battle_results[j] = SnakeChange::Die;
                    }
                }
                _ => (),
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
        (Direction::Up, 0)
    }
}

struct Directions {
    v: Vec<Direction>,
}

impl Directions {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(Direction::Up)
        }
        Directions { v }
    }

    fn next(&mut self) -> Option<&Vec<Direction>> {
        let mut working_index = None;
        for i in 0..self.v.len() {
            if self.v[i] != Direction::Right {
                working_index = Some(i);
                break;
            }
        }
        if let Some(i) = working_index {
            match self.v[i] {
                Direction::Up => self.v[i] = Direction::Down,
                Direction::Down => self.v[i] = Direction::Left,
                Direction::Left => self.v[i] = Direction::Right,
                Direction::Right => unreachable!(),
            }
            for j in 0..i {
                self.v[j] = Direction::Up;
            }
            Some(&self.v)
        } else {
            None
        }
    }
}

pub struct SimpleTreeSearchSnake {}

impl SimpleTreeSearchSnake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Brain for SimpleTreeSearchSnake {
    fn logic(&self, _game: &Game, _turn: &i32, board: &Board, you: &Battlesnake) -> Direction {
        let (mut new_boards, mut move_scores) = step(board, &you.id);

        let depth = if board.snakes.len() == 2 { 3 } else { 1 };

        let new_combinations_number =
            4_i32.pow(board.snakes.len() as u32) * new_boards.len() as i32;
        for _ in 0..depth {
            for i in 0..4 {
                move_scores[i] = move_scores[i].saturating_mul(new_combinations_number);
            }
            let mut tmp_boards = Vec::with_capacity(new_combinations_number as usize);
            for board in new_boards.iter() {
                let (mut b, s) = step(board, &you.id);
                for i in 0..4 {
                    move_scores[i] = move_scores[i].saturating_add(s[i]);
                }
                tmp_boards.append(&mut b);
            }
            new_boards = tmp_boards;
        }

        debug!("{:?}", move_scores);

        let mut best_move = Direction::Up;
        let mut best_score = move_scores[0];
        for i in 1..move_scores.len() {
            if move_scores[i] > best_score {
                best_score = move_scores[i];
                best_move = match i {
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Right,
                    _ => unreachable!(),
                }
            }
        }
        best_move
    }
}

fn step(board: &Board, id: &String) -> (Vec<Board>, [i32; 4]) {
    let mut new_boards = simulate_snakes_step(board);

    let mut move_scores = [0; 4];
    for board in new_boards.iter_mut() {
        match evaluate_snakes_step(board, id) {
            (Direction::Up, score) => {
                move_scores[0] = (move_scores[0] as i32).saturating_add(score)
            }
            (Direction::Down, score) => {
                move_scores[1] = (move_scores[1] as i32).saturating_add(score)
            }
            (Direction::Left, score) => {
                move_scores[2] = (move_scores[2] as i32).saturating_add(score)
            }
            (Direction::Right, score) => {
                move_scores[3] = (move_scores[3] as i32).saturating_add(score)
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
        let directions = vec![Direction::Up, Direction::Right, Direction::Right];
        let mut directions_iter = Directions { v: directions };

        assert_eq!(
            directions_iter.next(),
            Some(&vec![Direction::Down, Direction::Right, Direction::Right])
        );
        assert_eq!(
            directions_iter.next(),
            Some(&vec![Direction::Left, Direction::Right, Direction::Right])
        );
        assert_eq!(
            directions_iter.next(),
            Some(&vec![Direction::Right, Direction::Right, Direction::Right])
        );

        assert_eq!(directions_iter.next(), None);
    }
}
