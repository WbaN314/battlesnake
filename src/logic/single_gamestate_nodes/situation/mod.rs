mod situation_field;

use log::debug;
use situation_field::SituationField;
use std::fmt;

use crate::logic::game::{
    direction::Direction, field::BasicField, game_state::GameState, snake::Snake,
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

    fn check(&self, gamestate: &GameState<BasicField>) -> Option<SituationMatch> {
        let head = match gamestate.snakes().cell(0).get() {
            Snake::Alive { head, .. } => head,
            _ => return None,
        };
        let base_x = head.x as isize - self.head_dx;
        let base_y = head.y as isize - self.head_dy;
        let matches = self.fields.chunks(self.width).enumerate().all(|(dy, row)| {
            row.iter().enumerate().all(|(dx, field)| {
                let x = base_x + dx as isize;
                let y = base_y + dy as isize;
                field.check(gamestate.board().cell(x as i8, y as i8).map(|c| c.get()))
            })
        });
        if matches {
            debug!("Situation pattern matched:\n{}", self);
            Some(self.result)
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

pub struct Situation {
    patterns: Vec<SituationPattern>,
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
        self.patterns.iter().find_map(|p| p.check(gamestate))
    }
}

#[cfg(test)]
mod tests {
    use super::Situation;
    use crate::{
        logic::game::{direction::Direction, field::BasicField, game_state::GameState},
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
}
