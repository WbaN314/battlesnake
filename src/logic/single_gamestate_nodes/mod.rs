use std::time::Duration;

use crate::{
    OriginalDirection, OriginalGameState,
    logic::{
        game::direction::Direction,
        legacy::shared::brain::Brain,
        single_gamestate_nodes::{node::NodeStatus, tree::Tree},
    },
};

pub struct NewYearNewSnake;

mod node;
mod tree;

impl NewYearNewSnake {
    pub fn new() -> Self {
        Self
    }
}

impl Brain for NewYearNewSnake {
    fn logic(&self, gamestate: &OriginalGameState) -> OriginalDirection {
        let mut tree = Tree::new(gamestate.into())
            .all_root_directions()
            .dead_ancestor_pruning()
            .max_time(Duration::from_millis(200));
        tree.simulate();
        let result = tree.result();

        if let Some((i, _)) = result
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, NodeStatus::AliveFor(_)))
            .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        {
            Direction::try_from(i).unwrap().into()
        } else if let Some((i, _)) = result
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, NodeStatus::DeadIn(_)))
            .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        {
            Direction::try_from(i).unwrap().into()
        } else {
            Direction::Up.into()
        }
    }
}
