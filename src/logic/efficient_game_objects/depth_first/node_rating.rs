use std::fmt::{Display, Formatter};

use crate::logic::efficient_game_objects::e_game_state::EGameState;

#[derive(Clone)]
pub struct NodeRating {
    pub states_on_this_node: usize,
    pub pruned_states_from_this_node: usize,
}

impl NodeRating {
    pub fn new() -> Self {
        NodeRating {
            states_on_this_node: 0,
            pruned_states_from_this_node: 0,
        }
    }

    pub fn from(states: &Vec<EGameState>) -> Self {
        let mut node = NodeRating::new();
        node.states_on_this_node = states.len();
        node
    }

    pub fn update(&mut self, other: &NodeRating) {
        // TODO
    }
}

impl Display for NodeRating {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "States: {}, Pruned: {}",
            self.states_on_this_node, self.pruned_states_from_this_node
        )
    }
}

impl PartialEq for NodeRating {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for NodeRating {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NodeRating {}

impl Ord for NodeRating {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}
