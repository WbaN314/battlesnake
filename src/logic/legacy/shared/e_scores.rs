use core::fmt;
use std::{env, fmt::Display};

use log::info;

use crate::Game;

use super::e_direction::EDirection;

pub struct Scores {
    scores: Vec<([i64; 4], String)>,
}

impl Default for Scores {
    fn default() -> Self {
        Self::new()
    }
}

impl Scores {
    pub fn new() -> Self {
        Scores { scores: Vec::new() }
    }

    pub fn push(&mut self, result: ([i64; 4], String)) {
        self.scores.push(result);
    }

    pub fn evaluate(&self) -> EDirection {
        let mut viable = [true; 4];
        for i in 0..self.scores.len() {
            let to_beat = self.scores[i]
                .0
                .iter()
                .enumerate()
                .filter(|(i, _)| viable[*i])
                .map(|x| x.1)
                .max();
            for d in 0..4 {
                if self.scores[i].0[d] < *to_beat.unwrap() {
                    viable[d] = false;
                }
            }
        }
        for d in 0..4 {
            if viable[d] {
                return EDirection::from_usize(d);
            }
        }
        panic!("No viable direction found");
    }

    pub fn print_log(&self, game: &Game, turn: &i32, result: EDirection) {
        let mut s = String::new();
        s.push_str(&format!(
            "Game {} Turn {} Result {} Scores ",
            game.id,
            turn,
            result.to_direction()
        ));
        s.push_str(&format!("{}", self));
        if env::var("MODE").unwrap_or("".to_string()) == "test" {
            println!("{}", s);
        } else {
            info!("{}", s);
        }
    }
}

impl Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|")?;
        for score in self.scores.iter() {
            write!(
                f,
                " {} {} {} {} {} |",
                score.1, score.0[0], score.0[1], score.0[2], score.0[3]
            )?;
        }
        Ok(())
    }
}
