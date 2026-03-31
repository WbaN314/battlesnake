mod situation_field;

use situation_field::SituationField;

use crate::logic::game::{field::BasicField, game_state::GameState, snake::Snake};

struct Situation {
    fields: Vec<SituationField>,
    width: usize,
    head_dx: isize,
    head_dy: isize,
}

impl Situation {
    pub fn from(str: &str) -> Self {
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
        }
    }

    pub fn check(&self, gamestate: &GameState<BasicField>) -> bool {
        let head = match gamestate.snakes().cell(0).get() {
            Snake::Alive { head, .. } => head,
            _ => return false,
        };
        let base_x = head.x as isize - self.head_dx;
        let base_y = head.y as isize - self.head_dy;
        self.fields.chunks(self.width).enumerate().all(|(dy, row)| {
            row.iter().enumerate().all(|(dx, field)| {
                let x = base_x + dx as isize;
                let y = base_y + dy as isize;
                field.check(gamestate.board().cell(x as i8, y as i8).map(|c| c.get()))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Situation;
    use crate::{
        logic::game::{field::BasicField, game_state::GameState},
        read_game_state,
    };

    #[test]
    fn test_situation_check_matches() {
        let gamestate = read_game_state("requests/test_move_request_2.json");
        let state = GameState::<BasicField>::from(&gamestate);

        println!("{}", state);
        let situation = Situation::from(
            "
            . . .
            . A .
            N N .
            ",
        );
        assert!(situation.check(&state));

        let situation = Situation::from(
            "
            . . .
            . A .
            N N N
            ",
        );
        assert!(!situation.check(&state));

        let situation = Situation::from(
            "
            . . .
            A . .
            N . .
            ",
        );
        assert!(situation.check(&state));

        let situation = Situation::from(
            "
            . .
            A .
            N .
            ",
        );
        assert!(situation.check(&state));

        let situation = Situation::from(
            "
            N . .
            N . A
            N N N
            ",
        );
        assert!(situation.check(&state));

        let situation = Situation::from(
            "
            N . .
            N . A
            N B N
            ",
        );
        assert!(situation.check(&state));
    }
}
