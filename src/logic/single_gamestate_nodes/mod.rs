use std::time::Duration;

use crate::{
    OriginalDirection, OriginalGameState,
    logic::{
        game::{direction::Direction, field::BasicField, game_state::GameState, snake::Snake},
        legacy::shared::brain::Brain,
        single_gamestate_nodes::{
            node::NodeStatus,
            situation::{Situation, SituationMatch},
            tree::Tree,
        },
    },
};

pub struct NewYearNewSnake;

mod node;
mod situation;
mod tree;

impl NewYearNewSnake {
    pub fn new() -> Self {
        Self
    }
}

impl Brain for NewYearNewSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let gamestate: GameState<BasicField> = gamestate.into();

        #[cfg(debug_assertions)]
        println!("{}", gamestate);

        let mut directions = [true; 4];

        let mut tree = Tree::new(gamestate.clone())
            .all_root_directions()
            .dead_ancestor_pruning()
            .similarity_pruning(|_| 6)
            .max_time(Duration::from_millis(200));
        tree.simulate();
        let result = tree.result();

        #[cfg(debug_assertions)]
        println!("{}", tree.stats());

        for (index, _) in result
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, NodeStatus::DeadIn(_)))
        {
            directions[index] = false;
        }

        if directions.iter().all(|&d| !d) {
            directions = [true; 4];
        }

        let situations = [
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
            ),
            // Move away from walls
            Situation::recommending(
                "
                W A .
                ",
                Direction::Right,
            )
            .full_symmetry(),
        ];

        for situation in situations {
            match situation.check(&gamestate) {
                Some(SituationMatch::Recommend(direction)) if directions[direction as usize] => {
                    return direction.into();
                }
                Some(SituationMatch::Avoid(direction)) => directions[direction as usize] = false,
                _ => continue,
            }
        }

        directions
            .into_iter()
            .enumerate()
            .filter(|(_, allowed)| *allowed)
            .filter_map(|(index, _)| {
                if result[index].is_comparable() {
                    Some((index, result[index]))
                } else {
                    None
                }
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index.try_into().unwrap())
            .unwrap_or(Direction::Up)
            .into()
    }
}
