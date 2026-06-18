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
            situation::{Situation, SituationSet},
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
            0.0,
            "Fast Track",
        )
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
            Situation::recommending(
                "
                W . A
                ",
                Direction::Left,
                -50.0,
                "Avoid Next to Wall",
            ),
            Situation::recommending(
                "
                X A
                ",
                Direction::Left,
                60.0,
                "Grab Food",
            ),
            Situation::recommending(
                "
                W N *
                W B N
                W . A
                ",
                Direction::Down,
                100.0,
                "Kill by Lead",
            ),
            // Kill by follow
            Situation::recommending(
                "
                W B .
                W N A
                ",
                Direction::Up,
                100.0,
                "Kill by Follow",
            )
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
                Self::fast_track_trigger_situation().check(node.gamestate()).is_some()
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
                    evaluation.score(index.try_into().unwrap(), n as f64, "Alive For")
                }
                _ => {}
            }
        }

        result
    }

    pub fn logic_with_evaluation_result(
        &self,
        gamestate: &OriginalGameState,
    ) -> (OriginalDirection, String) {
        let env_config = EnvironmentConfig::read();
        let gamestate: GameState<BasicField> = gamestate.into();
        let mut evaluation = Evaluation::new();
        if env::var("LOG_EVAL").is_ok() {
            evaluation = evaluation.one_line();
        }

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
                .count() as f64;
            evaluation.score(direction, squeezed_snakes * 100.0, "Squeezed Snakes");

            let number_of_alive_snakes = gamestate.snakes().clone().into_iter().filter(|s| matches!(s.get(), Snake::Alive { .. })).count();
            let number_of_alive_snakes_multiplier = match number_of_alive_snakes {
                4 => 0.5,
                2 => 2.0,
                _ => 1.0,
            };
            evaluation.score(direction, result.flooded_area[0] as f64 * number_of_alive_snakes_multiplier, format!("Flooded Area x {}", number_of_alive_snakes_multiplier));

            let length_multiplier=match gamestate.snakes().length_gap_to_longest_other_snake() {
                gap if gap < 0 => 2.0,
                gap if gap == 0 => 1.5,
                gap if gap > 2 => 0.5,
                _ => 1.0,
            };
            for &(coord, turn) in &result.food[0] {
                if turn == 1 {
                    evaluation.score(direction, 60.0 * length_multiplier, format!("Food x {}", length_multiplier));
                }
                if turn == 2 {
                    evaluation.score(direction, 40.0 * length_multiplier, format!("Food x {}", length_multiplier));
                }
                if turn == 3 {
                    evaluation.score(direction, 30.0 * length_multiplier, format!("Food x {}", length_multiplier));
                }
                if turn == 4 {
                    evaluation.score(direction, 20.0 * length_multiplier, format!("Food x {}", length_multiplier));
                }
                if turn == 5 {
                    evaluation.score(direction, 10.0 * length_multiplier, format!("Food x {}", length_multiplier));
                } else {
                    evaluation.score(direction, 5_f64.max(15.0 - turn as f64) * length_multiplier, format!("Food x {}", length_multiplier));
                }
            }
        }

        let direction = evaluation.result();
        let eval_string = evaluation.to_string();

        #[cfg(debug_assertions)]
        println!("{}", eval_string);

        (direction.into(), eval_string)
    }
}

impl Brain for GamestateNodesSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let (direction, eval_string) = self.logic_with_evaluation_result(gamestate);
        if env::var("LOG_EVAL").is_ok() {
            warn!(
                "ID {} Turn {} Evaluation -> {}",
                gamestate.game.id, gamestate.turn, eval_string
            );
        }
        direction
    }
}
