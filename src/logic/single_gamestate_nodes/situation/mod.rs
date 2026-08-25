mod situation_field;

use log::debug;
use situation_field::SituationField;
use std::{fmt, ops::Deref};

use crate::logic::general::{
    direction::Direction, evaluation::Evaluation, field::BasicField, game_state::GameState, moves::Moves, snake::Snake, snakes::{SNAKES, Snakes}
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SituationMatch(Moves);

impl Deref for SituationMatch {
    type Target = Moves;
    fn deref(&self) -> &Moves {
        &self.0
    }
}

#[derive(Clone)]
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
        SituationMatch(self.0.map(|d| d.map(&f)))
    }

    // Remaps directions from label-order [A,B,C,D] to gamestate-snake-order.
    // label_ids[0..2] map labels B/C/D to their actual gamestate snake IDs.
    // A (index 0) always stays at slot 0.
    fn remap_to_gamestate(self, label_ids: &[Option<u8>; 3]) -> Self {
        let mut out = [None; SNAKES];
        out[0] = self.0[0]; // A = own snake, always slot 0
        for (label_idx, maybe_id) in label_ids.iter().enumerate() {
            if let Some(id) = maybe_id {
                out[*id as usize] = self.0[label_idx + 1];
            }
        }
        SituationMatch(out)
    }
}

impl fmt::Display for SituationMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Match({:?})", self.0)
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

#[derive(Clone)]
pub struct SituationSet {
    situations: Vec<Situation>,
}

impl SituationSet {
    pub fn new(situations: Vec<Situation>) -> Self {
        Self { situations }
    }

    pub fn check(&self, gamestate: &GameState<BasicField>) -> Option<SituationMatch> {
        self.situations.iter().find_map(|s| s.check(gamestate))
    }

    pub fn score(&self, gamestate: &GameState<BasicField>) -> Option<f32> {
        self.situations
            .iter()
            .find_map(|s| s.score(gamestate))
    }

    pub fn evaluate(
        &self,
        gamestate: &GameState<BasicField>,
        evaluation: &mut Evaluation,
    ) -> Option<Direction> {
        evaluation.new_section("Situations");
        for situation in &self.situations {
            for m in situation.check_all(gamestate) {
                if let SituationMatch([Some(direction), ..]) = m {
                    evaluation.score(direction, situation.score, situation.detail.clone());
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Situation {
    patterns: Vec<SituationPattern>,
    condition: Option<fn(Snakes) -> bool>,
    score: f32,
    detail: String,
}

impl Situation {
    pub fn recommending(str: &str, direction: Direction, score: f32, detail: impl Into<String>) -> Self {
        Self::build(
            str,
            SituationMatch([Some(direction), None, None, None]),
            score,
            detail,
        )
    }

    pub fn multi_recommending(
        str: &str,
        directions: [Option<Direction>; SNAKES],
        score: f32,
        detail: impl Into<String>,
    ) -> Self {
        Self::build(str, SituationMatch(directions), score, detail)
    }

    fn build(str: &str, result: SituationMatch, score: f32, detail: impl Into<String>) -> Self {
        Self {
            patterns: vec![SituationPattern::parse(str, result)],
            condition: None,
            score,
            detail: detail.into(),
        }
        .full_symmetry()
    }

    fn rotational(mut self) -> Self {
        let r1 = self.patterns[0].rotate_cw();
        let r2 = r1.rotate_cw();
        let r3 = r2.rotate_cw();
        self.patterns.push(r1);
        self.patterns.push(r2);
        self.patterns.push(r3);
        self.dedup()
    }

    /// Adds the mirror (left-right reflection) of each existing pattern.
    fn mirrored(mut self) -> Self {
        let mirrored: Vec<_> = self.patterns.iter().map(|p| p.mirror_x()).collect();
        self.patterns.extend(mirrored);
        self.dedup()
    }

    /// Generates all distinct symmetries: up to 8 (4 rotations × 2 mirror states, dihedral group D4).
    fn full_symmetry(self) -> Self {
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
        self.patterns.iter().find_map(|p| self.check_pattern(p, gamestate))
    }

    pub fn score(&self, gamestate: &GameState<BasicField>) -> Option<f32> {
        self.check(gamestate).map(|_| self.score)
    }

    pub fn check_all(&self, gamestate: &GameState<BasicField>) -> Vec<SituationMatch> {
        self.patterns.iter().filter_map(|p| self.check_pattern(p, gamestate)).collect()
    }

    fn check_pattern(&self, p: &SituationPattern, gamestate: &GameState<BasicField>) -> Option<SituationMatch> {
        let (result, label_ids) = p.check(gamestate)?;
        if let Some(condition) = self.condition {
            let snakes = if label_ids.iter().any(|id| id.is_some()) {
                // Slot 0 = A (own snake). Labeled snakes B/C/D go to slots 1/2/3.
                // Remaining slots are filled with unlabeled gamestate snakes so none are lost.
                let src = gamestate.snakes();
                let mut ordered = [Snake::NonExistent; SNAKES];
                ordered[0] = src.cell(0).get();
                let labeled_ids: std::collections::HashSet<u8> = label_ids.iter().filter_map(|x| *x).collect();
                for (slot, maybe_id) in label_ids.iter().enumerate() {
                    if let Some(id) = maybe_id {
                        ordered[slot + 1] = src.cell(*id).get();
                    }
                }
                let mut fill_slot = label_ids.iter().filter(|x| x.is_some()).count() + 1;
                for id in 1..SNAKES as u8 {
                    if !labeled_ids.contains(&id) {
                        if fill_slot < SNAKES {
                            ordered[fill_slot] = src.cell(id).get();
                            fill_slot += 1;
                        }
                    }
                }
                Snakes::from_label_order(ordered)
            } else {
                gamestate.snakes().clone()
            };
            if !condition(snakes) {
                return None;
            }
        }
        // Remap from label-order [A,B,C,D] to gamestate-snake-order.
        // result[0] (A = own snake) stays at slot 0.
        // result[1..] (B/C/D) move to the slot of the matched gamestate snake ID.
        Some(result.remap_to_gamestate(&label_ids))
    }

    pub fn condition(mut self, condition: fn(Snakes) -> bool) -> Self {
        self.condition = Some(condition);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Situation, SituationSet};
    use crate::{
        logic::general::{
            direction::Direction, evaluation::Evaluation, field::BasicField, game_state::GameState,
            snake::Snake, snakes::Snakes,
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
            100.0,
            "Test",
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            . . .
            . A .
            N N N
            ",
            Direction::Up,
            100.0,
            "Test",
        );
        assert!(situation.check(&state).is_none());

        let situation = Situation::recommending(
            "
            . . .
            A . .
            N . .
            ",
            Direction::Up,
            100.0,
            "Test",
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            . .
            A .
            N .
            ",
            Direction::Up,
            100.0,
            "Test",
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            N . .
            N . A
            N N N
            ",
            Direction::Up,
            100.0,
            "Test",
        );
        assert!(situation.check(&state).is_some());

        let situation = Situation::recommending(
            "
            N . .
            N . A
            N B N
            ",
            Direction::Up,
            100.0,
            "Test",
        );
        assert!(situation.check(&state).is_some());
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

        fn own_longer_than_b(snakes: Snakes) -> bool {
            match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            }
        }

        fn own_not_shorter_than_b(snakes: Snakes) -> bool {
            match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a >= b,
                _ => false,
            }
        }

        // Pattern matches but condition fails (3 > 3 is false) → no match
        let situation =
            Situation::recommending(pattern, Direction::Up, 100.0, "Test").condition(own_longer_than_b);
        assert!(situation.check(&state).is_none());

        // Pattern matches and condition passes (3 >= 3 is true) → match
        let situation =
            Situation::recommending(pattern, Direction::Up, 100.0, "Test").condition(own_not_shorter_than_b);
        assert!(situation.check(&state).is_some());
    }

    #[test]
    fn test_label_to_gamestate_remap() {
        let gamestate = read_game_state("requests/evaluate.json");
        let state = GameState::<BasicField>::from(&gamestate);
        println!("{}", state);

        let mut dirs = [None; 4];
        dirs[1] = Some(Direction::Left); // direction for label B
        let situation = Situation::multi_recommending(
            // rows parsed bottom-up: row 0 = bottom, row 2 = top.
            // A at col 2 row 2, B at col 0 row 0 => dx=-2, dy=-2.
            // Use * (Any) for intermediate cells to avoid body-cell mismatches.
            "
            * * A
            * * *
            B * *
            ",
            dirs,
            100.0,
            "Test",
        );

        let result = situation.check(&state);
        assert!(result.is_some(), "pattern should match");

        match result.unwrap() {
            super::SituationMatch(out) => {
                assert_eq!(
                    out[2],
                    Some(Direction::Left),
                    "B matched ID 2, so Up should be at slot 2, got {:?}",
                    out
                );
                assert_eq!(
                    out[1], None,
                    "slot 1 should be empty after remap, got {:?}",
                    out
                );
            }
        }
    }

    #[test]
    fn test_evaluate_uses_situation_score_and_detail() {
        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = GameState::<BasicField>::from(&gamestate);
        let situation_set = SituationSet::new(vec![Situation::recommending(
            "
            . . .
            . A .
            N N .
            ",
            Direction::Up,
            37.0,
            "Custom Detail",
        )]);
        let mut evaluation = Evaluation::new();

        situation_set.evaluate(&state, &mut evaluation);

        assert_eq!(evaluation.result(), Direction::Up);
        assert!(format!("{}", evaluation).contains("Custom Detail"));
    }
}

#[cfg(test)]
mod benchmarks {
    use std::hint::black_box;

    use super::{Situation, SituationSet};
    use crate::{
        logic::general::{
            direction::{Direction, Directions}, evaluation::Evaluation, field::BasicField, game_state::GameState, snake::Snake, snakes::Snakes
        },
        read_game_state,
    };

    fn test_states() -> Vec<GameState<BasicField>> {
        [
            "requests/test_move_request_2.json",
            "requests/example_move_request_2.json",
            "requests/example_move_request_3.json",
            "requests/failure_01.json",
            "requests/failure_02.json",
            "requests/failure_03.json",
            "requests/failure_04.json",
            "requests/failure_05.json",
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
            100.0,
            "Benchmark",
        );

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
            100.0,
            "Benchmark",
        )
        .condition(|snakes| {
            match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
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
                100.0,
                "Kill by lead",
            ),
            // Kill by follow
            Situation::recommending(
                "
                W B .
                W N A
                ",
                Direction::Up,
                100.0,
                "Kill by follow",
            )
            .condition(|snakes| {
                match (snakes.cell(0).get(), snakes.cell(1).get()) {
                    (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                    _ => false,
                }
            }),
            // Eat Food
            Situation::recommending(
                "
                X A",
                Direction::Left,
                100.0,
                "Eat Food",
            ),
            // Move away from walls
            Situation::recommending(
                "
                W A .
                ",
                Direction::Right,
                100.0,
                "Move away from walls",
            ),
        ]);

        let mut i = 0;
        b.iter(|| {
            let state = &states[i % states.len()];
            i += 1;
            let mut directions = black_box(Directions::new());
            let mut evaluation = black_box(Evaluation::new());
            black_box(situation_set.evaluate(black_box(state), &mut evaluation))
        });
    }
}
