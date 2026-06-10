#[cfg(debug_assertions)]
use std::time::Instant;
use std::{rc::Rc, time::Duration};

use crate::{
    OriginalDirection, OriginalGameState,
    logic::{
        general::{
            direction::{DIRECTIONS, Direction, Directions},
            evaluation::Evaluation,
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
        ]);
        #[cfg(debug_assertions)]
        {
            println!("Time for Situations construction: {:?}", time.elapsed());
        }
        situation_set
    }

    fn simulation(
        gamestate: GameState<BasicField>,
        evaluation: &mut Evaluation,
    ) -> [NodeStatus; 4] {
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
        // println!("{}", tree.stats());

        // Exclude DeadIn directions
        evaluation.new_section("Simulation");
        for (index, result) in result.into_iter().enumerate() {
            match result {
                NodeStatus::DeadIn(n) => {
                    evaluation.eliminate(index.try_into().unwrap(), n.min(4))
                }
                NodeStatus::AliveFor(n) => {
                    evaluation.score(index.try_into().unwrap(), n as i32)
                }
                _ => {}
            }
        }

        result
    }
}

impl Brain for NewYearNewSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let gamestate: GameState<BasicField> = gamestate.into();
        let mut evaluation = Evaluation::new();

        #[cfg(debug_assertions)]
        println!("{}", gamestate);

        // Simulation
        let result = NewYearNewSnake::simulation(gamestate.clone(), &mut evaluation);

        #[cfg(debug_assertions)]
        {
            println!("{:?}", result);
        }

        // Situations
        #[cfg(debug_assertions)]
        let time = Instant::now();
        let situation_set = NewYearNewSnake::special_situation_set();
        situation_set.evaluate(&gamestate, &mut evaluation);

        #[cfg(debug_assertions)]
        {
            println!("Time for Situations: {:?}", time.elapsed());
        }

        // Area
        evaluation.new_section("Capture");
        for direction in DIRECTIONS {
            let mut state: GameState<FloodFillField> = gamestate.clone().into();
            let result = state.flood_fill(direction);
            #[cfg(debug_assertions)]
            {
                println!(
                    "Flood fill for direction {:?} resulted in: {:?}",
                    direction, result
                );
                println!("{}", state);
            }
            if let Some(turn) = result.not_enough_area_in_turn[0] {
                evaluation.eliminate(direction, turn.min(16));
            }
            let squeezed_snakes = result.not_enough_area_in_turn[1..]
                .iter()
                .filter(|x|x.is_some())
                .count() as i32;
            evaluation.score(direction, squeezed_snakes * 100);
            evaluation.score(direction, result.flooded_area[0] as i32);
            evaluation.score(direction, result.food[0] as i32 * 10);
        }

        // Food hunting and general strategies should probably go here
        // failure_31_going_right_leads_to_death -> better general board positioning
        // failure_43_going_down_guarantees_getting_killed -> Single Child priority queue
        // failure_46_go_for_kill -> Kill propagation in simulation

        let direction = evaluation.result();

        #[cfg(debug_assertions)]
        {
            println!("{}", evaluation);
            println!("Selected direction: {}", direction);
        }

        direction.into()
    }
}
