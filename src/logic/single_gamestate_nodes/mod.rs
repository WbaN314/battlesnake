use std::time::Duration;
#[cfg(debug_assertions)]
use std::time::Instant;

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

        // Start with all directions and simulate
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

        // Exclude DeadIn directions
        for (index, _) in result
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, NodeStatus::DeadIn(_)))
        {
            directions[index] = false;
        }

        // If all are excluded, include all again
        if directions.iter().all(|&d| !d) {
            directions = [true; 4];
        }

        // Evaluate situations and return or avoid direction
        #[cfg(debug_assertions)]
        let time = Instant::now();
        let situations = [
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
        ];

        for situation in situations {
            match situation.check(&gamestate) {
                Some(SituationMatch::Recommend(direction)) if directions[direction as usize] => {
                    #[cfg(debug_assertions)]
                    println!("Time for Situations (match): {:?}", time.elapsed());
                    return direction.into();
                }
                Some(SituationMatch::Avoid(direction)) => directions[direction as usize] = false,
                _ => continue,
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("Time for Situations: {:?}", time.elapsed());
            println!("Directions after Situations {:?}", directions);
        }

        // Food hunting and general strategies should probably go here
        // failure_20_for_improved_area_evaluation -> NodeID length limit reached in late game
        // failure_31_going_right_leads_to_death -> better general board positioning
        // failure_43_going_down_guarantees_getting_killed -> Single Child priority queue
        // failure_46_go_for_kill -> Kill propagation in simulation

        // Take the best from the allowed directions or default
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
