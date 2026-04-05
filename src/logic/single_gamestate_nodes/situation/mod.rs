mod situation_field;

use log::debug;
use situation_field::SituationField;
use std::{cell::Cell, fmt};

use crate::logic::game::{
    direction::Direction, field::BasicField, game_state::GameState, snake::Snake, snakes::Snakes,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SituationMatch {
    Recommend(Direction),
    Avoid(Direction),
}

struct SituationPattern {
    fields: Vec<SituationField>,
    width: usize,
    head_dx: isize,
    head_dy: isize,
    result: SituationMatch,
}

impl SituationPattern {
    fn parse(str: &str, result: SituationMatch) -> Self {
        let lines: Vec<&str> = str
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let width = lines.first().map_or(0, |l| l.split_whitespace().count());
        let mut fields = Vec::with_capacity(lines.len() * width);

        for line in lines.iter().rev() {
            for token in line.split_whitespace() {
                fields.push(SituationField::from(token.chars().next().unwrap()));
            }
        }

        let head_pos = fields
            .iter()
            .position(|f| matches!(f, SituationField::OwnHead))
            .expect("Situation must contain an OwnHead (A) field");

        Self {
            head_dx: (head_pos % width) as isize,
            head_dy: (head_pos / width) as isize,
            fields,
            width,
            result,
        }
    }

    // 90° clockwise rotation (y-up coordinate system).
    // Transformation: (x, y) -> new_x=y, new_y=width-1-x
    // Head: new_head_dx=head_dy, new_head_dy=width-1-head_dx
    fn rotate_cw(&self) -> Self {
        let height = self.fields.len() / self.width;
        let new_width = height;
        let mut new_fields = vec![SituationField::Any; self.fields.len()];
        for y in 0..height {
            for x in 0..self.width {
                new_fields[(self.width - 1 - x) * new_width + y] = self.fields[y * self.width + x];
            }
        }
        Self {
            fields: new_fields,
            width: new_width,
            head_dx: self.head_dy,
            head_dy: (self.width as isize) - 1 - self.head_dx,
            result: self.result.map_direction(|dir| match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }),
        }
    }

    // Mirror horizontally (flip left-right).
    // Transformation: (x, y) -> (width-1-x, y)
    // Head: new_head_dx = width-1-head_dx, head_dy unchanged
    // Direction: Left <-> Right flipped.
    fn mirror_x(&self) -> Self {
        let height = self.fields.len() / self.width;
        let mut new_fields = vec![SituationField::Any; self.fields.len()];
        for y in 0..height {
            for x in 0..self.width {
                new_fields[y * self.width + (self.width - 1 - x)] = self.fields[y * self.width + x];
            }
        }
        Self {
            fields: new_fields,
            width: self.width,
            head_dx: (self.width as isize) - 1 - self.head_dx,
            head_dy: self.head_dy,
            result: self.result.map_direction(|dir| match dir {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
            }),
        }
    }

    // Returns the match result and a [Option<u8>; 3] mapping labels B/C/D to snake IDs.
    fn check(
        &self,
        gamestate: &GameState<BasicField>,
    ) -> Option<(SituationMatch, [Option<u8>; 3])> {
        let head = match gamestate.snakes().cell(0).get() {
            Snake::Alive { head, .. } => head,
            _ => return None,
        };
        let base_x = head.x as isize - self.head_dx;
        let base_y = head.y as isize - self.head_dy;
        // label_ids[0] = B, [1] = C, [2] = D
        let mut label_ids: [Option<u8>; 3] = [None; 3];
        let matches = self.fields.chunks(self.width).enumerate().all(|(dy, row)| {
            row.iter().enumerate().all(|(dx, field)| {
                let x = base_x + dx as isize;
                let y = base_y + dy as isize;
                let cell = gamestate.board().cell(x as i8, y as i8).map(|c| c.get());
                if let SituationField::OtherHead(idx) = field {
                    if let Some(BasicField::Snake { id, next: None }) = cell {
                        if id != 0 {
                            label_ids[*idx as usize] = Some(id);
                            return true;
                        } else {
                            return false;
                        }
                    }
                    return false;
                }
                field.check(cell)
            })
        });
        if matches {
            debug!("Situation pattern matched:\n{}", self);
            Some((self.result, label_ids))
        } else {
            None
        }
    }
}

impl PartialEq for SituationPattern {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.result == other.result && self.fields == other.fields
    }
}

impl SituationMatch {
    fn map_direction(self, f: impl Fn(Direction) -> Direction) -> Self {
        match self {
            Self::Recommend(dir) => Self::Recommend(f(dir)),
            Self::Avoid(dir) => Self::Avoid(f(dir)),
        }
    }
}

impl fmt::Display for SituationMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SituationMatch::Recommend(dir) => write!(f, "Recommend({:?})", dir),
            SituationMatch::Avoid(dir) => write!(f, "Disallow({:?})", dir),
        }
    }
}

impl fmt::Display for SituationPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.fields.chunks(self.width).rev() {
            let line: String = row
                .iter()
                .map(|f| f.display_char())
                .flat_map(|c| [c, ' '])
                .collect::<String>()
                .trim_end()
                .to_string();
            writeln!(f, "{}", line)?;
        }
        write!(f, "=> {}", self.result)
    }
}

pub struct SituationSet {
    situations: Vec<Situation>,
}

impl SituationSet {
    pub fn new(situations: Vec<Situation>) -> Self {
        Self { situations }
    }

    /// Iterates through all situations and applies recommendations/avoidances.
    /// Returns `Some(Direction)` if a situation recommends an allowed direction, `None` otherwise.
    pub fn evaluate(
        &self,
        gamestate: &GameState<BasicField>,
        directions: &mut [bool; 4],
    ) -> Option<Direction> {
        for situation in &self.situations {
            match situation.check(gamestate) {
                Some(SituationMatch::Recommend(direction)) if directions[direction as usize] => {
                    return Some(direction);
                }
                Some(SituationMatch::Avoid(direction)) => {
                    directions[direction as usize] = false;
                    // If only one direction remains, return it
                    let mut remaining = directions.iter().enumerate().filter(|(_, d)| **d);
                    if let (Some((i, _)), None) = (remaining.next(), remaining.next()) {
                        return Some(i.try_into().unwrap());
                    }
                }
                _ => {}
            }
        }
        None
    }
}

pub struct Situation {
    patterns: Vec<SituationPattern>,
    condition: Option<fn([Snake; 4]) -> bool>,
}

impl Situation {
    pub fn recommending(str: &str, direction: Direction) -> Self {
        Self::build(str, SituationMatch::Recommend(direction))
    }

    pub fn disallowing(str: &str, direction: Direction) -> Self {
        Self::build(str, SituationMatch::Avoid(direction))
    }

    fn build(str: &str, result: SituationMatch) -> Self {
        Self {
            patterns: vec![SituationPattern::parse(str, result)],
            condition: None,
        }
    }

    pub fn rotational(mut self) -> Self {
        let r1 = self.patterns[0].rotate_cw();
        let r2 = r1.rotate_cw();
        let r3 = r2.rotate_cw();
        self.patterns.push(r1);
        self.patterns.push(r2);
        self.patterns.push(r3);
        self.dedup()
    }

    /// Adds the mirror (left-right reflection) of each existing pattern.
    pub fn mirrored(mut self) -> Self {
        let mirrored: Vec<_> = self.patterns.iter().map(|p| p.mirror_x()).collect();
        self.patterns.extend(mirrored);
        self.dedup()
    }

    /// Generates all distinct symmetries: up to 8 (4 rotations × 2 mirror states, dihedral group D4).
    pub fn full_symmetry(self) -> Self {
        self.rotational().mirrored()
    }

    fn dedup(mut self) -> Self {
        let mut i = 0;
        while i < self.patterns.len() {
            if self.patterns[..i].contains(&self.patterns[i]) {
                self.patterns.swap_remove(i);
            } else {
                i += 1;
            }
        }
        self
    }

    pub fn check(&self, gamestate: &GameState<BasicField>) -> Option<SituationMatch> {
        self.patterns.iter().find_map(|p| {
            let (result, label_ids) = p.check(gamestate)?;
            if let Some(condition) = self.condition {
                // Build ordered Snakes: slot 0 = own snake (A), slots 1/2/3 = B/C/D matched IDs.
                // Unmatched labels get NonExistent.
                let src = gamestate.snakes();
                let mut ordered = [Snake::NonExistent; 4];
                ordered[0] = src.cell(0).get();
                for (slot, maybe_id) in label_ids.iter().enumerate() {
                    if let Some(id) = maybe_id {
                        ordered[slot + 1] = src.cell(*id).get();
                    }
                }
                if !condition(ordered) {
                    return None;
                }
            }
            Some(result)
        })
    }

    pub fn condition(mut self, condition: fn([Snake; 4]) -> bool) -> Self {
        self.condition = Some(condition);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Situation;
    use crate::{
        logic::game::{
            direction::Direction, field::BasicField, game_state::GameState, snake::Snake,
        },
        read_game_state,
    };

    #[test]
    fn test_situation_check_matches() {
        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = GameState::<BasicField>::from(&gamestate);

        println!("{}", state);
        let situation = Situation::recommending(
            "
            . . .
            . A .
            N N .
            ",
            Direction::Up,
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            . . .
            . A .
            N N N
            ",
            Direction::Up,
        );
        assert!(situation.check(&state).is_none());

        let situation = Situation::recommending(
            "
            . . .
            A . .
            N . .
            ",
            Direction::Up,
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            . .
            A .
            N .
            ",
            Direction::Up,
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            N . .
            N . A
            N N N
            ",
            Direction::Up,
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            N . .
            N . A
            N B N
            ",
            Direction::Up,
        );
        assert!(situation.check(&state).is_some());
    }

    #[test]
    fn test_rotational_direction_rotation() {
        let situation = Situation::recommending(
            "
            . A .
            . N .
            ",
            Direction::Up,
        )
        .rotational();

        assert_eq!(situation.patterns.len(), 4);

        for (i, p) in situation.patterns.iter().enumerate() {
            println!("rotation {}:\n{}", i, p);
        }

        let dirs: Vec<Direction> = situation
            .patterns
            .iter()
            .map(|p| match p.result {
                super::SituationMatch::Recommend(d) => d,
                _ => panic!("expected Recommend"),
            })
            .collect();
        assert!(
            matches!(dirs[0], Direction::Up),
            "pattern 0:\n{}",
            situation.patterns[0]
        );
        assert!(
            matches!(dirs[1], Direction::Right),
            "pattern 1:\n{}",
            situation.patterns[1]
        );
        assert!(
            matches!(dirs[2], Direction::Down),
            "pattern 2:\n{}",
            situation.patterns[2]
        );
        assert!(
            matches!(dirs[3], Direction::Left),
            "pattern 3:\n{}",
            situation.patterns[3]
        );
    }

    #[test]
    fn test_mirror_direction() {
        let situation = Situation::recommending(
            "
            N A .
            ",
            Direction::Right,
        )
        .mirrored();

        assert_eq!(situation.patterns.len(), 2);

        for (i, p) in situation.patterns.iter().enumerate() {
            println!("mirror {}:\n{}", i, p);
        }

        let dirs: Vec<Direction> = situation
            .patterns
            .iter()
            .map(|p| match p.result {
                super::SituationMatch::Recommend(d) => d,
                _ => panic!("expected Recommend"),
            })
            .collect();
        assert!(
            matches!(dirs[0], Direction::Right),
            "pattern 0:\n{}",
            situation.patterns[0]
        );
        assert!(
            matches!(dirs[1], Direction::Left),
            "pattern 1:\n{}",
            situation.patterns[1]
        );
    }

    #[test]
    fn test_full_symmetry_count() {
        let situation = Situation::recommending(
            "
            N . .
            N A .
            . . .
            ",
            Direction::Right,
        )
        .full_symmetry();

        for (i, p) in situation.patterns.iter().enumerate() {
            println!("symmetry {}:\n{}", i, p);
        }

        assert_eq!(situation.patterns.len(), 8);
    }

    #[test]
    fn test_condition() {
        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = GameState::<BasicField>::from(&gamestate);

        // Both snakes have length 3. Pattern places B to the left of A.
        let pattern = "
            N . .
            N . A
            N B N
        ";

        fn own_longer_than_b(snakes: [Snake; 4]) -> bool {
            match (snakes[0], snakes[1]) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            }
        }

        fn own_not_shorter_than_b(snakes: [Snake; 4]) -> bool {
            match (snakes[0], snakes[1]) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a >= b,
                _ => false,
            }
        }

        // Pattern matches but condition fails (3 > 3 is false) → no match
        let situation =
            Situation::recommending(pattern, Direction::Up).condition(own_longer_than_b);
        assert!(situation.check(&state).is_none());

        // Pattern matches and condition passes (3 >= 3 is true) → match
        let situation =
            Situation::recommending(pattern, Direction::Up).condition(own_not_shorter_than_b);
        assert!(situation.check(&state).is_some());
    }
}

#[cfg(test)]
mod benchmarks {
    use std::hint::black_box;

    use super::{Situation, SituationSet};
    use crate::{
        logic::game::{
            direction::Direction, field::BasicField, game_state::GameState, snake::Snake,
        },
        read_game_state,
    };

    fn test_states() -> Vec<GameState<BasicField>> {
        [
            "requests/test_move_request_2.json",
            "requests/example_move_request_2.json",
            "requests/example_move_request_3.json",
            "requests/failure_1.json",
            "requests/failure_2.json",
            "requests/failure_3.json",
            "requests/failure_4.json",
            "requests/failure_5.json",
        ]
        .iter()
        .map(|p| GameState::<BasicField>::from(&read_game_state(p)))
        .collect()
    }

    #[bench]
    fn bench_situation_full_symmetry_evaluate(b: &mut test::Bencher) {
        let states = test_states();
        let situation = Situation::recommending(
            "
            W N *
            W B N
            W . A
            ",
            Direction::Down,
        )
        .full_symmetry();

        let mut i = 0;
        b.iter(|| {
            let state = &states[i % states.len()];
            i += 1;
            black_box(situation.check(black_box(state)))
        });
    }

    #[bench]
    fn bench_situation_full_symmetry_with_condition_evaluate(b: &mut test::Bencher) {
        let states = test_states();
        let situation = Situation::recommending(
            "
            W B .
            W N A
            ",
            Direction::Up,
        )
        .full_symmetry()
        .condition(|snakes| {
            if let [
                Snake::Alive { length: a, .. },
                Snake::Alive { length: b, .. },
                _,
                _,
            ] = snakes
            {
                a > b
            } else {
                false
            }
        });

        let mut i = 0;
        b.iter(|| {
            let state = &states[i % states.len()];
            i += 1;
            black_box(situation.check(black_box(state)))
        });
    }

    #[bench]
    fn bench_situation_set_evaluate(b: &mut test::Bencher) {
        let states = test_states();
        let situation_set = SituationSet::new(vec![
            // Kill by lead
            Situation::recommending(
                "
                W N *
                W B N
                W . A
                ",
                Direction::Down,
            )
            .full_symmetry(),
            // Kill by follow
            Situation::recommending(
                "
                W B .
                W N A
                ",
                Direction::Up,
            )
            .full_symmetry()
            .condition(|snakes| {
                if let [
                    Snake::Alive { length: a, .. },
                    Snake::Alive { length: b, .. },
                    _,
                    _,
                ] = snakes
                {
                    a > b
                } else {
                    false
                }
            }),
            // Eat Food
            Situation::recommending(
                "
                X A",
                Direction::Left,
            )
            .full_symmetry(),
            // Move away from walls
            Situation::recommending(
                "
                W A .
                ",
                Direction::Right,
            )
            .full_symmetry(),
        ]);

        let mut i = 0;
        b.iter(|| {
            let state = &states[i % states.len()];
            i += 1;
            let mut directions = black_box([true; 4]);
            black_box(situation_set.evaluate(black_box(state), &mut directions))
        });
    }
}
