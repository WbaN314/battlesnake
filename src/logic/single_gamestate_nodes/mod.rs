use crate::{
    OriginalDirection, OriginalGameState, logic::{
        general::{
            board::{HEIGHT, WIDTH},
            coord::Coord,
            direction::{DIRECTIONS, Direction},
            evaluation::Evaluation,
            field::{BasicField, FloodFillField},
            game_state::GameState,
            snake::Snake,
        }, legacy::shared::brain::Brain, single_gamestate_nodes::{
            node::NodeStatus, situation_config::SIMULATION_SCORE_SITUATIONS, tree::Tree,
        },
    },
};
use env_config::ENV_CONFIG;
use situation_config::{CHILD_PRIORITY_SITUATIONS, NODE_DIRECTION_PREFERENCE_SITUATIONS, ROOT_EVALUATION_SITUATIONS};
use log::warn;

mod env_config;
mod node;
mod situation;
mod situation_config;
mod tree;

pub struct GamestateNodesSnake;

impl GamestateNodesSnake {
    pub fn new() -> Self {
        Self
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
                        evaluation.score(
                            direction,
                            ENV_CONFIG.SCORE_FIRST_MOVES_TOWARD_CENTER,
                            "Toward Center",
                        );
                    }
                }
            }
        }
    }

    /// Builds the production-configured search tree for `gamestate`, with every
    /// tuning applied *except* the terminal bound. Callers pick the bound
    /// themselves: production uses `.max_time(...)`, benchmarks use
    /// `.max_nodes(...)` / `.max_depth(...)` so their runtime is deterministic.
    pub(crate) fn configured_tree(gamestate: GameState<BasicField>) -> Tree {
        Tree::new(gamestate)
            .all_root_directions()
            .child_priority_situations(CHILD_PRIORITY_SITUATIONS.clone())
            .node_direction_preference_situations(NODE_DIRECTION_PREFERENCE_SITUATIONS.clone())
            .score_situations(SIMULATION_SCORE_SITUATIONS.clone())
            //.similarity_pruning(|_| 6)
            .head_tail_pruning(|_| [u8::MAX, 12, 4])
            .simulate_snakes_seperately(|d| d >= 2)
    }

    fn simulation(
        gamestate: GameState<BasicField>,
        evaluation: &mut Evaluation,
    ) -> [NodeStatus; 4] {
        let mut tree =
            Self::configured_tree(gamestate.clone()).max_time(ENV_CONFIG.SIMULATION_TIME_MS);
        tree.simulate();
        let result = tree.result();

        if ENV_CONFIG.LOCAL_SIMULATION {
            tree.log_depths();
        }

        // Exclude DeadIn directions
        evaluation.new_section("Simulation");
        for (index, result) in result.into_iter().enumerate() {
            match result {
                NodeStatus::ProbablyDeadIn(n, score) => {
                    evaluation.eliminate(
                        index.try_into().unwrap(),
                        100 + n,
                        format!("Probably Dead In {}", n),
                    );
                    evaluation.score(index.try_into().unwrap(), *score as f32, "Simulation Score");
                }
                NodeStatus::DeadIn(n, score) => {
                    evaluation.eliminate(index.try_into().unwrap(), n, format!("Dead In {}", n));
                    evaluation.score(index.try_into().unwrap(), *score as f32, "Simulation Score");
                }
                NodeStatus::AliveFor(n, score) => {
                    evaluation.score(
                        index.try_into().unwrap(),
                        n as f32 * 0.1,
                        format!("Alive For {}", n),
                    );
                    evaluation.score(index.try_into().unwrap(), *score as f32, "Simulation Score");
                }
                NodeStatus::WinnerIn(n, score) => {
                    evaluation.score(
                        index.try_into().unwrap(),
                        (10 as i8 - n as i8).max(1) as f32 * 1000.0,
                        format!("Winner In {}", n),
                    );
                    evaluation.score(index.try_into().unwrap(), *score as f32, "Simulation Score");
                }
                _ => {
                    panic!("Unexpected NodeStatus: {:?}", result)
                }
            }
        }

        result
    }

    pub fn logic_with_evaluation_result(
        &self,
        gamestate: &OriginalGameState,
    ) -> (OriginalDirection, String) {
        let turn = gamestate.turn as u8;
        let id = gamestate.game.id.clone();
        let gamestate: GameState<BasicField> = gamestate.into();
        let mut evaluation = Evaluation::for_mode(ENV_CONFIG.LOCAL_SIMULATION);

        #[cfg(debug_assertions)]
        println!("{}", gamestate);

        Self::move_to_middle_first(turn, &gamestate, &mut evaluation);

        // Simulation
        Self::simulation(gamestate.clone(), &mut evaluation);

        // Situations
        ROOT_EVALUATION_SITUATIONS.evaluate(&gamestate, &mut evaluation);

        // Area
        let number_of_alive_snakes = (0..4).filter(|&id| gamestate.is_alive(id)).count();
        evaluation.new_section("Capture");
        let mut enemy_min_dist_from_center: [Option<u8>; 4] = [None; 4];
        for direction in DIRECTIONS {
            let mut state: GameState<FloodFillField> = gamestate.clone().into();
            let result = state.flood_fill(direction, true);

            if let Some(turn) = result.not_enough_area_in_turn[0] {
                let multiplier =
                    (-ENV_CONFIG.SCORE_NOT_ENOUGH_AREA_DECAY_COEFFICIENT * turn as f32).exp();
                evaluation.score(
                    direction,
                    ENV_CONFIG.SCORE_NOT_ENOUGH_AREA * multiplier,
                    format!("Not Enough Area x {:.2}", multiplier),
                );
            }

            if number_of_alive_snakes <= 3 {
                // Squeezing only if at most 3 snakes alive -> failure_61.json
                let squeezed_snakes = result.not_enough_area_in_turn[1..]
                    .iter()
                    .filter(|x| x.is_some())
                    .count() as f32;
                evaluation.score(
                    direction,
                    squeezed_snakes * ENV_CONFIG.SCORE_SQUEEZED_SNAKES,
                    "Squeezed Snakes",
                );
            }
            evaluation.score(
                direction,
                result.flooded_area[0].len() as f32,
                "Flooded Area",
            );

            if number_of_alive_snakes == 2 {
                let center = Coord::new(WIDTH / 2, HEIGHT / 2);
                enemy_min_dist_from_center[direction as usize] = result.flooded_area[1]
                    .iter()
                    .map(|&(coord, _)| coord.king_distance_to(center))
                    .min();
            }

            for &(_, distance) in &result.food[0] {
                let multiplier = (-ENV_CONFIG.SCORE_FOOD_DECAY_COEFFICIENT * distance as f32).exp();
                evaluation.score(
                    direction,
                    ENV_CONFIG.SCORE_FOOD * multiplier,
                    format!("Food x {:.1}", multiplier),
                );
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
                    evaluation.score(d, ENV_CONFIG.SCORE_ENEMY_PUSHED, "Enemy Pushed to Side");
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
                    evaluation.score(direction, ENV_CONFIG.SCORE_NEXT_TO_WALL, "Next to Wall");
                }
            }
        }

        // Away from trouble / center preference based on snake count
        evaluation.new_section("Positioning");
        if let Snake::Alive { head, .. } = gamestate.snakes().cell(0).get() {
            let center = Coord::new(WIDTH / 2, HEIGHT / 2);
            let current_dist = head.distance_to(center);
            for direction in DIRECTIONS {
                let next_head = head + direction;
                if next_head.distance_to(center) < current_dist {
                    evaluation.score(direction, ENV_CONFIG.SCORE_TOWARDS_CENTER, "Towards Center");
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
                            evaluation.score(
                                direction,
                                ENV_CONFIG.SCORE_ENEMY_MIDPOINT,
                                "Toward Enemy Midpoint",
                            );
                        }
                    }
                }
            }
        }

        let direction = evaluation.result();
        let eval_string = evaluation.to_string();

        #[cfg(debug_assertions)]
        println!("{}", eval_string);

        if evaluation.is_enabled() {
            warn!("ID {} Turn {} Evaluation -> {}", id, turn, eval_string);
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

#[cfg(test)]
pub(crate) mod bench_fixtures;

#[cfg(test)]
mod benchmarks;

#[cfg(test)]
mod tests {
    use super::GamestateNodesSnake;
    use super::bench_fixtures;
    use super::env_config::ENV_CONFIG;
    use crate::get_move_from_json_file_with_timeout;

    #[test]
    fn test_with_longer_simulation_time() {
        let timeout_ms = 10000;
        get_move_from_json_file_with_timeout("failure_81.json", timeout_ms);
    }

    /// Diagnostic reporter (not a timing bench): runs each fixture once under the
    /// production time budget and prints the emergent search metrics — resolved
    /// depth, simulated nodes, branching factor. These are only meaningful under
    /// `max_time` (a node/depth bound fixes them by construction), so this mirrors
    /// what production actually does. Ignored because it takes SIMULATION_TIME_MS
    /// per fixture; run explicitly with:
    ///   cargo test --lib report_search_stats -- --ignored --nocapture
    #[test]
    #[ignore = "slow; diagnostic — run with --ignored --nocapture"]
    fn report_search_stats() {
        use super::node::NodeStatus;
        use tabled::builder::Builder;
        use tabled::settings::{Alignment, Style, object::Columns};

        // The minmax horizon actually resolved across the root directions. Unlike
        // `max_depth_reached` (deepest node anywhere in an uneven tree), this is the
        // depth the *decision* is backed by. Preference order:
        //   1. Any AliveFor  -> min of their n (shallowest verified-survival branch,
        //      a conservative floor on how deep we resolved).
        //   2. Else any WinnerIn -> min of their n. `WinnerIn(n)` counts "steps to
        //      victory", a different quantity than survival, so it's only used as a
        //      fallback — a short winning line must not drag down the estimate when a
        //      plain AliveFor sibling gives the truer survival horizon.
        //   3. Else (all dead) -> max of DeadIn/ProbablyDeadIn n (longest we stall
        //      forced death). Pruned/NotSimulated carry no horizon and are skipped.
        fn resolved_depth(result: &[NodeStatus; 4]) -> u8 {
            let pick = |f: fn(&NodeStatus) -> Option<u8>| result.iter().filter_map(f);
            if let Some(min) = pick(|s| match s {
                NodeStatus::AliveFor(n, _) => Some(*n),
                _ => None,
            })
            .min()
            {
                min
            } else if let Some(min) = pick(|s| match s {
                NodeStatus::WinnerIn(n, _) => Some(*n),
                _ => None,
            })
            .min()
            {
                min
            } else {
                pick(|s| match s {
                    NodeStatus::DeadIn(n, _) | NodeStatus::ProbablyDeadIn(n, _) => Some(*n),
                    _ => None,
                })
                .max()
                .unwrap_or(0)
            }
        }

        let mut builder = Builder::default();
        builder.push_record(["fixture", "nodes", "depth", "branching"]);

        let mut sum_nodes = 0usize;
        let mut sum_depth = 0u32;
        let mut sum_branching = 0f32;
        let mut count = 0u32;
        for (path, state) in bench_fixtures::PATHS
            .iter()
            .zip(bench_fixtures::basic_field_states())
        {
            let mut tree =
                GamestateNodesSnake::configured_tree(state).max_time(ENV_CONFIG.SIMULATION_TIME_MS);
            tree.simulate();
            let stats = tree.stats();
            let depth = resolved_depth(&tree.result());
            sum_nodes += stats.total_nodes;
            sum_depth += depth as u32;
            sum_branching += stats.avg_branching_factor;
            count += 1;
            builder.push_record([
                path.trim_start_matches("requests/").to_string(),
                stats.total_nodes.to_string(),
                depth.to_string(),
                format!("{:.2}", stats.avg_branching_factor),
            ]);
        }
        let n = count.max(1) as f32;
        builder.push_record([
            "AVG".to_string(),
            format!("{:.0}", sum_nodes as f32 / n),
            format!("{:.1}", sum_depth as f32 / n),
            format!("{:.2}", sum_branching / n),
        ]);

        let mut table = builder.build();
        table.with(Style::rounded());
        table.modify(Columns::new(1..), Alignment::right());
        println!("\n{table}\n");
    }
}
