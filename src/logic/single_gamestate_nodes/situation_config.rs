use crate::logic::{
    general::{direction::Direction, snake::Snake},
    single_gamestate_nodes::{
        env_config::ENV_CONFIG,
        situation::{Situation, SituationSet},
    },
};
use std::sync::LazyLock;

pub static SIMULATION_SCORE_SITUATIONS: LazyLock<SituationSet> =
    LazyLock::new(|| SituationSet::new(vec![]));

pub static CHILD_PRIORITY_SITUATIONS: LazyLock<SituationSet> = LazyLock::new(|| {
    SituationSet::new(vec![
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
        ),
    ])
});

pub static NODE_DIRECTION_PREFERENCE_SITUATIONS: LazyLock<SituationSet> = LazyLock::new(|| {
    SituationSet::new(vec![
        Situation::recommending(
            "
            A
            .
            .
            .
            B
            W
            ",
            Direction::Down,
            0.0,
            "editor_07",
        )
        .condition(
            |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            },
        ),
        Situation::recommending(
            "
            A .
            . .
            . .
            N B
            W W
            ",
            Direction::Right,
            0.0,
            "editor_06",
        )
        .condition(
            |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            },
        ),
        Situation::recommending(
            "
            * A
            . .
            . .
            B .
            W W
            ",
            Direction::Down,
            0.0,
            "editor_05",
        )
        .condition(
            |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            },
        ),
    ])
});

pub static ROOT_EVALUATION_SITUATIONS: LazyLock<SituationSet> = LazyLock::new(|| {
    SituationSet::new(vec![
        Situation::recommending(
            "
            W . A
            ",
            Direction::Left,
            ENV_CONFIG.SCORE_AVOID_MOVING_NEXT_TO_WALL,
            "Avoid Moving Next to Wall",
        ),
        Situation::recommending(
            "
            X A
            ",
            Direction::Left,
            ENV_CONFIG.SCORE_GRAB_FOOD,
            "Grab Food",
        )
        .condition(|snakes| snakes.length_gap_to_longest_other_snake() <= 2),
        Situation::recommending(
            "
            * N A
            N B .
            W W W
            ",
            Direction::Right,
            ENV_CONFIG.SCORE_KILL_SITUATION,
            "editor_01",
        ),
        Situation::recommending(
            "
            A .
            N B
            W W
            ",
            Direction::Right,
            ENV_CONFIG.SCORE_KILL_SITUATION,
            "editor_02",
        )
        .condition(
            |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            },
        ),
        Situation::recommending(
            "
            A .
            . .
            . .
            N B
            W W
            ",
            Direction::Right,
            ENV_CONFIG.SCORE_RESTRICT_SITUATION,
            "editor_06",
        )
        .condition(
            |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a > b,
                _ => false,
            },
        ),
    ])
});
