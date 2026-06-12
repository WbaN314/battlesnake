use std::{env, time::Duration};

use log::{info, warn};

use crate::{
    OriginalDirection, OriginalGameState,
    logic::{
        general::{
            direction::{DIRECTIONS, Direction},
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

pub struct GamestateNodesSnake;

mod node;
mod situation;
mod tree;

struct EnvironmentConfig {
    simulation_time: Duration,
}

impl EnvironmentConfig {
    fn read() -> Self {
        let simulation_time = Duration::from_millis(
            env::var("SIMULATION_TIME_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(200),
        );
        Self { simulation_time }
    }
}

impl GamestateNodesSnake {
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
            0,
            "Fast Track",
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
        let situation_set = SituationSet::new(vec![
            // Kill by lead
            Situation::recommending(
                "
                X A
                ",
                Direction::Left,
                60,
                "Grab Food",
            )
            .full_symmetry(),
            Situation::recommending(
                "
                W N *
                W B N
                W . A
                ",
                Direction::Down,
                100,
                "Kill by Lead",
            )
            .full_symmetry(),
            // Kill by follow
            Situation::recommending(
                "
                W B .
                W N A
                ",
                Direction::Up,
                100,
                "Kill by Follow",
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
        situation_set
    }

    fn simulation(
        gamestate: GameState<BasicField>,
        evaluation: &mut Evaluation,
        env_config: &EnvironmentConfig,
    ) -> [NodeStatus; 4] {
        let mut tree = Tree::new(gamestate.clone())
            .all_root_directions()
            .dead_ancestor_pruning()
            .similarity_pruning(|_| 6)
            .fast_track(move |node| {
                matches!(
                    GamestateNodesSnake::fast_track_trigger_situation().check(node.gamestate()),
                    Some(SituationMatch::Recommend(_))
                )
            })
            .max_time(env_config.simulation_time);
        tree.simulate();
        let result = tree.result();

        // Exclude DeadIn directions
        evaluation.new_section("Simulation");
        for (index, result) in result.into_iter().enumerate() {
            match result {
                NodeStatus::DeadIn(n) => evaluation.eliminate(index.try_into().unwrap(), n),
                NodeStatus::AliveFor(n) => {
                    evaluation.score(index.try_into().unwrap(), n as i32, "Alive For")
                }
                _ => {}
            }
        }

        result
    }

    pub fn logic_with_evaluation_result(&self, gamestate: &OriginalGameState) -> (OriginalDirection, String) {
        let env_config = EnvironmentConfig::read();
        let gamestate: GameState<BasicField> = gamestate.into();
        let mut evaluation = Evaluation::new();

        #[cfg(debug_assertions)]
        println!("{}", gamestate);

        // Simulation
        GamestateNodesSnake::simulation(gamestate.clone(), &mut evaluation, &env_config);

        // Situations
        let situation_set = GamestateNodesSnake::special_situation_set();
        situation_set.evaluate(&gamestate, &mut evaluation);

        // Area
        evaluation.new_section("Capture");
        for direction in DIRECTIONS {
            let mut state: GameState<FloodFillField> = gamestate.clone().into();
            let result = state.flood_fill(direction);
            if let Some(turn) = result.not_enough_area_in_turn[0] {
                evaluation.eliminate(direction, turn.min(16));
            }
            let squeezed_snakes = result.not_enough_area_in_turn[1..]
                .iter()
                .filter(|x| x.is_some())
                .count() as i32;
            evaluation.score(direction, squeezed_snakes * 100, "Squeezed Snakes");
            evaluation.score(direction, result.flooded_area[0] as i32, "Flooded Area");
            evaluation.score(direction, result.food[0].len() as i32 * 10, "Reachable Food");
        }

        // Food hunting and general strategies should probably go here
        // failure_31_going_right_leads_to_death -> better general board positioning
        // failure_43_going_down_guarantees_getting_killed -> Single Child priority queue
        // failure_46_go_for_kill -> Kill propagation in simulation

        let direction = evaluation.result();
        let eval_string = evaluation.to_string();
        if env::var("LOG_EVAL").is_ok() {
            warn!("{eval_string}");
        }

        (direction.into(), eval_string)
    }
}

impl Brain for GamestateNodesSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        self.logic_with_evaluation_result(gamestate).0
    }
}
