use std::{env, time::Duration};

use log::{info, warn};

use crate::{
    OriginalDirection, OriginalGameState,
    logic::{
        general::{
            board::{HEIGHT, WIDTH},
            coord::Coord,
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

mod node;
mod situation;
mod tree;

pub struct GamestateNodesSnake;

struct EnvironmentConfig {
    simulation_time: Duration,
    log_eval: bool,
}

impl EnvironmentConfig {
    fn read() -> Self {
        let simulation_time = Duration::from_millis(
            env::var("SIMULATION_TIME_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(200),
        );
        let log_eval = env::var("LOG_EVAL").is_ok();
        Self {
            simulation_time,
            log_eval,
        }
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
        .condition(
            |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a <= b,
                _ => false,
            },
        )
    }

    pub fn special_situation_set() -> SituationSet {
        // Evaluate situations and return or avoid direction
        let situation_set = SituationSet::new(vec![
            Situation::recommending(
                "
                W . A
                ",
                Direction::Left,
                -20.0,
                "Avoid Moving Next to Wall",
            ),
            Situation::recommending(
                "
                X A
                ",
                Direction::Left,
                60.0,
                "Grab Food",
            )
            .condition(|snakes| snakes.length_gap_to_longest_other_snake() <= 2),
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
            .condition(
                |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                    (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                    _ => false,
                },
            ),
        ]);
        situation_set
    }

    fn move_to_middle_first(
        turn: u8,
        gamestate: &GameState<BasicField>,
        evaluation: &mut Evaluation,
    ) {
        evaluation.new_section("First Moves");
        if let Snake::Alive { head, .. } = gamestate.snakes().cell(0).get() {
            if turn < 4 && head.distance_to(Coord::new(WIDTH / 2, HEIGHT / 2)) + turn == 4 {
                let center = Coord::new(WIDTH / 2, HEIGHT / 2);
                let current_dist = head.distance_to(center);
                for direction in DIRECTIONS {
                    let next_head = head + direction;
                    if next_head.distance_to(center) < current_dist {
                        evaluation.score(direction, 200.0, "Toward Center");
                    }
                }
            }
        }
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
                Self::fast_track_trigger_situation()
                    .check(node.gamestate())
                    .is_some()
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
                    evaluation.score(index.try_into().unwrap(), n as f64 / 100.0, "Alive For")
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
        let turn = gamestate.turn as u8;
        let id = gamestate.game.id.clone();
        let gamestate: GameState<BasicField> = gamestate.into();
        let mut evaluation = Evaluation::new();
        if env_config.log_eval {
            evaluation = evaluation.one_line();
        }

        #[cfg(debug_assertions)]
        println!("{}", gamestate);

        Self::move_to_middle_first(turn, &gamestate, &mut evaluation);

        // Simulation
        Self::simulation(gamestate.clone(), &mut evaluation, &env_config);

        // Situations
        let situation_set = Self::special_situation_set();
        situation_set.evaluate(&gamestate, &mut evaluation);

        // Area
        let number_of_alive_snakes = (0..4).filter(|&id| gamestate.is_alive(id)).count();
        evaluation.new_section("Capture");
        let mut enemy_min_dist_from_center: [Option<u8>; 4] = [None; 4];
        for direction in DIRECTIONS {
            let mut state: GameState<FloodFillField> = gamestate.clone().into();
            let result = state.flood_fill(direction);

            #[cfg(debug_assertions)]
            {
                if !evaluation.is_eliminated(direction) {
                    println!("Flood Fill Board for direction {:?}:", direction);
                    println!("{}", state);
                }
            }

            if let Some(turn) = result.not_enough_area_in_turn[0] {
                evaluation.score(
                    direction,
                    0.max(10 - turn as i8) as f64 * -10.0,
                    "Not Enough Area",
                );
            }

            if number_of_alive_snakes <= 3 {
                // Squeezing only if at most 3 snakes alive -> failure_61.json
                let squeezed_snakes = result.not_enough_area_in_turn[1..]
                    .iter()
                    .filter(|x| x.is_some())
                    .count() as f64;
                evaluation.score(direction, squeezed_snakes * 100.0, "Squeezed Snakes");
            }
            let number_of_alive_snakes_multiplier = match number_of_alive_snakes {
                4 => 0.5,
                2 => 2.0,
                _ => 1.0,
            };
            evaluation.score(
                direction,
                result.flooded_area[0].len() as f64 * number_of_alive_snakes_multiplier,
                format!("Flooded Area x {}", number_of_alive_snakes_multiplier),
            );

            if number_of_alive_snakes == 2 {
                let center = Coord::new(WIDTH / 2, HEIGHT / 2);
                enemy_min_dist_from_center[direction as usize] = result.flooded_area[1]
                    .iter()
                    .map(|&(coord, _)| coord.king_distance_to(center))
                    .min();
            }

            let length_multiplier = match gamestate.snakes().length_gap_to_longest_other_snake() {
                gap if gap < 0 => 2.0,
                gap if gap == 1 => 3.0,
                gap if gap == 0 => 3.0,
                gap if gap > 8 => 0.1,
                gap if gap > 4 => 0.5,
                _ => 1.0,
            };
            let hunger_multiplier = match gamestate.snakes().cell(0).get() {
                Snake::Alive { health, .. } if health < 10 => 3.0,
                Snake::Alive { health, .. } if health < 20 => 2.0,
                _ => 1.0,
            };
            for &(coord, turn) in &result.food[0] {
                if turn == 1 {
                    evaluation.score(
                        direction,
                        60.0 * length_multiplier * hunger_multiplier,
                        format!("Food x {}", length_multiplier * hunger_multiplier),
                    );
                }
                if turn == 2 {
                    evaluation.score(
                        direction,
                        40.0 * length_multiplier * hunger_multiplier,
                        format!("Food x {}", length_multiplier * hunger_multiplier),
                    );
                }
                if turn == 3 {
                    evaluation.score(
                        direction,
                        30.0 * length_multiplier * hunger_multiplier,
                        format!("Food x {}", length_multiplier * hunger_multiplier),
                    );
                }
                if turn == 4 {
                    evaluation.score(
                        direction,
                        20.0 * length_multiplier * hunger_multiplier,
                        format!("Food x {}", length_multiplier * hunger_multiplier),
                    );
                }
                if turn == 5 {
                    evaluation.score(
                        direction,
                        10.0 * length_multiplier * hunger_multiplier,
                        format!("Food x {}", length_multiplier * hunger_multiplier),
                    );
                } else {
                    evaluation.score(
                        direction,
                        5_f64.max(15.0 - turn as f64) * length_multiplier * hunger_multiplier,
                        format!("Food x {}", length_multiplier * hunger_multiplier),
                    );
                }
            }
        }

        if number_of_alive_snakes == 2 {
            if let Some(best_dist) = enemy_min_dist_from_center.iter().filter_map(|x| *x).max() {
                let best_dirs: Vec<Direction> = DIRECTIONS
                    .iter()
                    .filter(|&&d| enemy_min_dist_from_center[d as usize] == Some(best_dist))
                    .copied()
                    .collect();
                for d in best_dirs {
                    evaluation.score(d, 100.0, "Enemy Pushed to Side");
                }
            }
        }

        // Wall avoidance
        if let Snake::Alive { head, .. } = gamestate.snakes().cell(0).get() {
            for direction in DIRECTIONS {
                let next_head = head + direction;
                if next_head.x == 0
                    || next_head.x == WIDTH - 1
                    || next_head.y == 0
                    || next_head.y == HEIGHT - 1
                {
                    evaluation.score(direction, -20.0, "Next to Wall");
                }
            }
        }

        // Away from trouble / center preference based on snake count
        evaluation.new_section("Positioning");
        if let Snake::Alive { head, .. } = gamestate.snakes().cell(0).get() {
            if number_of_alive_snakes <= 4 {
                let enemy_heads: Vec<Coord> = gamestate
                    .snakes()
                    .clone()
                    .into_iter()
                    .skip(1)
                    .filter_map(|s| {
                        if let Snake::Alive { head, .. } = s.get() {
                            Some(head)
                        } else {
                            None
                        }
                    })
                    .collect();
                let away_direction = if enemy_heads.iter().all(|e| e.x > head.x) {
                    Some(Direction::Left)
                } else if enemy_heads.iter().all(|e| e.x < head.x) {
                    Some(Direction::Right)
                } else if enemy_heads.iter().all(|e| e.y > head.y) {
                    Some(Direction::Down)
                } else if enemy_heads.iter().all(|e| e.y < head.y) {
                    Some(Direction::Up)
                } else {
                    None
                };
                if let Some(d) = away_direction {
                    evaluation.score(d, 20.0, "Away From Trouble");
                }
            }
            if number_of_alive_snakes <= 3 {
                let center = Coord::new(WIDTH / 2, HEIGHT / 2);
                let current_dist = head.distance_to(center);
                for direction in DIRECTIONS {
                    let next_head = head + direction;
                    if next_head.distance_to(center) < current_dist {
                        evaluation.score(direction, 20.0, "Toward Center");
                    }
                }
            }
            if number_of_alive_snakes <= 2 {
                if let Some(enemy_head) =
                    gamestate
                        .snakes()
                        .clone()
                        .into_iter()
                        .skip(1)
                        .find_map(|s| {
                            if let Snake::Alive { head, .. } = s.get() {
                                Some(head)
                            } else {
                                None
                            }
                        })
                {
                    let center = Coord::new(WIDTH / 2, HEIGHT / 2);
                    let target =
                        Coord::new((center.x + enemy_head.x) / 2, (center.y + enemy_head.y) / 2);
                    let current_dist = head.distance_to(target);
                    for direction in DIRECTIONS {
                        let next_head = head + direction;
                        if next_head.distance_to(target) < current_dist {
                            evaluation.score(direction, 20.0, "Toward Enemy Midpoint");
                        }
                    }
                }
            }
        }

        let direction = evaluation.result();
        let eval_string = evaluation.to_string();

        #[cfg(debug_assertions)]
        println!("{}", eval_string);

        if env_config.log_eval {
            warn!(
                "ID {} Turn {} Evaluation -> {}",
                id, turn, eval_string
            );
        }

        (direction.into(), eval_string)
    }
}

impl Brain for GamestateNodesSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let (direction, _) = self.logic_with_evaluation_result(gamestate);

        direction
    }
}
