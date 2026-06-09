#[cfg(debug_assertions)]
use std::time::Instant;
use std::{rc::Rc, time::Duration};

use crate::{
    OriginalDirection, OriginalGameState,
    logic::{
        general::{
            direction::{Direction, Directions},
            field::{BasicField, FloodFillField},
            game_state::GameState,
            snake::Snake,
        },
        legacy::shared::brain::Brain,
        single_gamestate_nodes::{
            node::NodeStatus,
            situation::{Situation, SituationMatch, SituationSet},
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

    pub fn fast_track_trigger_situation() -> Situation {
        Situation::multi_recommending(
            "
                W . .
                W A .
                W N B
                ",
            [Some(Direction::Up), Some(Direction::Up), None, None],
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
                a <= b
            } else {
                false
            }
        })
    }

    pub fn special_situation_set() -> SituationSet {
        // Evaluate situations and return or avoid direction
        #[cfg(debug_assertions)]
        let time = Instant::now();
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
        #[cfg(debug_assertions)]
        {
            println!("Time for Situations construction: {:?}", time.elapsed());
        }
        situation_set
    }

    fn simulation(gamestate: GameState<BasicField>, directions: &mut Directions) -> [NodeStatus; 4] {
        let mut tree = Tree::new(gamestate.clone())
            .all_root_directions()
            .dead_ancestor_pruning()
            .similarity_pruning(|_| 6)
            .fast_track(move |node| {
                matches!(
                    NewYearNewSnake::fast_track_trigger_situation().check(node.gamestate()),
                    Some(SituationMatch::Recommend(_))
                )
            })
            .max_time(Duration::from_millis(200));
        tree.simulate();
        let result = tree.result();

        #[cfg(debug_assertions)]
        println!("{}", tree.stats());

        directions.set_checkpoint();
        // Exclude DeadIn directions
        for (index, _) in result
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, NodeStatus::DeadIn(_)))
        {
            directions.set_index(index, false);
        }

        if directions.reset_if_exhausted() {
            // Exclude DeadIn(0) directions at least
            for (index, _) in result
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, NodeStatus::DeadIn(0)))
            {
                directions.set_index(index, false);
            }
        }

        result
    }
}

impl Brain for NewYearNewSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let gamestate: GameState<BasicField> = gamestate.into();

        #[cfg(debug_assertions)]
        println!("{}", gamestate);

        let mut directions = Directions::new();

        // Simulation
        let result= NewYearNewSnake::simulation(gamestate.clone(), &mut directions);
        directions.reset_if_exhausted();

        // Situations
        #[cfg(debug_assertions)]
        let time = Instant::now();
        let situation_set = NewYearNewSnake::special_situation_set();
        if let Some(direction) = situation_set.evaluate(&gamestate, &mut directions) {
            #[cfg(debug_assertions)]
            println!("Time for Situations (match): {:?}", time.elapsed());
            return direction.into();
        }
        #[cfg(debug_assertions)]
        {
            println!("Time for Situations: {:?}", time.elapsed());
            println!("Directions after Situations {}", directions);
        }
 
        // Area
        directions.set_checkpoint();
        for direction in directions.iter() {
            let mut state: GameState<FloodFillField> = gamestate.clone().into();
            let result = state.flood_fill(direction);
             #[cfg(debug_assertions)]
             {
                println!("Flood fill for direction {:?} resulted in: {:?}", direction, result);
                println!("{}", state);
             }
        }
        directions.reset_if_exhausted();


        
        // Food hunting and general strategies should probably go here
        // failure_31_going_right_leads_to_death -> better general board positioning
        // failure_43_going_down_guarantees_getting_killed -> Single Child priority queue
        // failure_46_go_for_kill -> Kill propagation in simulation

        // Take the best from the allowed directions or default
        directions
            .into_iter()
            .filter_map(|dir| {
                let index: usize = dir.into();
                if result[index].is_comparable() {
                    Some((dir, result[index]))
                } else {
                    None
                }
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(dir, _)| dir)
            .unwrap_or(Direction::Up)
            .into()
    }
}
