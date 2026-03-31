mod situation_field;

use situation_field::SituationField;

use crate::logic::game::{direction::Direction, field::BasicField, game_state::GameState, snake::Snake};

#[derive(Copy, Clone, Debug)]
pub enum SituationMatch {
    Recommend(Direction),
    Disallow(Direction),
}

struct Situation {
    fields: Vec<SituationField>,
    width: usize,
    head_dx: isize,
    head_dy: isize,
    result: SituationMatch,
}

impl Situation {
    pub fn recommending(str: &str, direction: Direction) -> Self {
        Self::build(str, SituationMatch::Recommend(direction))
    }

    pub fn disallowing(str: &str, direction: Direction) -> Self {
        Self::build(str, SituationMatch::Disallow(direction))
    }

    fn build(str: &str, result: SituationMatch) -> Self {
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

    pub fn check(&self, gamestate: &GameState<BasicField>) -> Option<SituationMatch> {
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
        if matches { Some(self.result) } else { None }
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
}
