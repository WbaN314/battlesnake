use crate::OriginalGameState;

use super::game::{
    d_board::{HEIGHT, WIDTH},
    d_coord::DCoord,
    d_direction::DDirection,
};
use arrayvec::ArrayVec;
use d_scores::DScores;

mod d_scores;

pub struct DIntuition<'a> {

    state: &'a OriginalGameState,
    allowed_directions: ArrayVec<DDirection, 4>,
}

impl<'a> DIntuition<'a> {
    pub fn new(
        state: &'a OriginalGameState) -> Self {
        Self {
            state,
            allowed_directions: ArrayVec::from([
                DDirection::Up,
                DDirection::Down,
                DDirection::Left,
                DDirection::Right,
            ]),
        }
    }

    pub fn allowed_directions(mut self, directions: ArrayVec<DDirection, 4>) -> Self {
        self.allowed_directions = ArrayVec::from(directions);
        self
    }

    pub fn run(self) -> DDirection {
        let mut scores = DScores::new();

        for direction in self.allowed_directions.iter() {
            let distance_to_food = self.distance_to_food(*direction);
            scores.push(*direction, distance_to_food);

            let distance_to_middle = self.distance_to_middle(*direction);
            scores.push(*direction, distance_to_middle);
        }

        scores.evaluate()
    }

    fn distance_to_middle(&self, direction: DDirection) -> f64 {
        let mut head: DCoord = self.state.you.head.into();
        match direction {
            DDirection::Up => {
                head.y += 1;
            }
            DDirection::Down => {
                head.y -= 1;
            }
            DDirection::Left => {
                head.x -= 1;
            }
            DDirection::Right => {
                head.x += 1;
            }
        }
        let mut distance = 0.0;
        distance += (head.x as f64 - (WIDTH as f64 - 1.0) / 2.0).abs();
        distance += (head.y as f64 - (HEIGHT as f64 - 1.0) / 2.0).abs();
        -distance
    }

    fn distance_to_food(&self, direction: DDirection) -> f64 {
        let mut head: DCoord = self.state.you.head.into();
        match direction {
            DDirection::Up => {
                head.y += 1;
            }
            DDirection::Down => {
                head.y -= 1;
            }
            DDirection::Left => {
                head.x -= 1;
            }
            DDirection::Right => {
                head.x += 1;
            }
        }
        let mut distance = f64::MAX;
        for food in &self.state.board.food {
            let food_coord: DCoord = food.into();
            distance = distance.min(head.distance_to(food_coord) as f64);
        }
        -distance
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::depth_first::{
            game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
            intuition::DIntuition,
        },
        read_game_state,
    };

    #[test]
    fn test_distance_to_middle() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let intuition = DIntuition::new(&gamestate);
        assert_eq!(intuition.distance_to_middle(DDirection::Up), -8.0);
        assert_eq!(intuition.distance_to_middle(DDirection::Down), -10.0);
        assert_eq!(intuition.distance_to_middle(DDirection::Left), -10.0);
        assert_eq!(intuition.distance_to_middle(DDirection::Right), -8.0);
    }

    #[test]
    fn test_distance_to_food() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let intuition = DIntuition::new(&gamestate);
        assert_eq!(intuition.distance_to_food(DDirection::Up), -5.0);
        assert_eq!(intuition.distance_to_food(DDirection::Down), -7.0);
        assert_eq!(intuition.distance_to_food(DDirection::Left), -7.0);
        assert_eq!(intuition.distance_to_food(DDirection::Right), -5.0);
    }
}
