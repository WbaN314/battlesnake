use std::{
    fmt::{Display, Formatter},
    u8,
};

use crate::logic::efficient_game_objects::e_game_state::EGameState;

use super::state_rating::StateRating;

#[derive(Clone, Debug)]
pub struct NodeRating {
    pub initial_states_on_this_node: usize,
    pub current_states_on_this_node: usize,
    pub pruned_states_from_this_node: usize,
    pub worst_current_length: u8,
    pub most_snakes_alive: u8,
}

impl NodeRating {
    pub fn new() -> Self {
        NodeRating {
            initial_states_on_this_node: 0,
            current_states_on_this_node: 0,
            pruned_states_from_this_node: 0,
            worst_current_length: u8::MAX,
            most_snakes_alive: 0,
        }
    }

    pub fn from(states: &Vec<EGameState>) -> Self {
        let mut node = NodeRating::new();
        node.initial_states_on_this_node = states.len();
        node.current_states_on_this_node = states.len();
        for state in states {
            node.update_from_state_rating(&StateRating::from(state));
        }
        node
    }

    pub fn update_from_child_node_rating(&mut self, other: &NodeRating) {
        // TODO
    }

    pub fn update_from_state_rating(&mut self, other: &StateRating) {
        self.worst_current_length = self.worst_current_length.min(other.current_length);
        self.most_snakes_alive = self.most_snakes_alive.max(other.snakes_alive);
    }

    pub fn set_pruned_states(&mut self, pruned_states: usize) {
        self.pruned_states_from_this_node = pruned_states;
    }

    pub fn set_current_states(&mut self, current_states: usize) {
        self.current_states_on_this_node = current_states;
    }
}

impl Display for NodeRating {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Initial: {}, Pruned: {}, Current: {}",
            self.initial_states_on_this_node,
            self.pruned_states_from_this_node,
            self.current_states_on_this_node
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
        self.most_snakes_alive
            .cmp(&other.most_snakes_alive)
            .then(other.worst_current_length.cmp(&self.worst_current_length))
            .then(
                other
                    .current_states_on_this_node
                    .cmp(&self.current_states_on_this_node),
            )
    }
}
