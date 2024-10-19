use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
    u8,
};

use crate::logic::shared::e_game_state::{EGameState, EStateRating};

#[derive(Clone, Debug)]
pub struct Running;
pub struct Finished;

#[derive(Clone, Debug)]
pub struct NodeRating<T> {
    pub initial_states_on_this_node: usize,
    pub current_states_on_this_node: usize,
    pub pruned_states_from_this_node: usize,
    pub lowest_current_length: u8, // always the worst case of all state ratings
    pub highest_snakes_alive: u8,
    pub highest_food_distance: u8,
    pub highest_middle_distance: u8,
    simulation_state: PhantomData<T>,
}

impl NodeRating<Running> {
    pub fn from(states: &Vec<EGameState>, number_initial_states: usize) -> Self {
        let mut node = Self {
            initial_states_on_this_node: number_initial_states,
            current_states_on_this_node: states.len(),
            pruned_states_from_this_node: number_initial_states - states.len(),
            lowest_current_length: u8::MAX,
            highest_snakes_alive: u8::MIN,
            highest_food_distance: u8::MIN,
            highest_middle_distance: u8::MIN,
            simulation_state: PhantomData,
        };
        for state in states {
            node.update_from_state_rating(&state.rate_state());
        }
        node
    }

    pub fn update_from_child_node_rating(&mut self, _other: &NodeRating<Running>) {
        // TODO
    }

    pub fn update_from_state_rating(&mut self, other: &EStateRating) {
        self.lowest_current_length = self.lowest_current_length.min(other.current_length);
        self.highest_snakes_alive = self.highest_snakes_alive.max(other.snakes_alive);
        self.highest_food_distance = self.highest_food_distance.max(other.food_distance);
        self.highest_middle_distance = self.highest_middle_distance.max(other.middle_distance);
    }
}

impl Display for NodeRating<Running> {
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

impl Display for NodeRating<Finished> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Snakes Alive: {}, Length: {}, Food: {}, Middle: {}",
            self.highest_snakes_alive,
            self.lowest_current_length,
            self.highest_food_distance,
            self.highest_middle_distance
        )
    }
}
impl PartialEq for NodeRating<Running> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for NodeRating<Running> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for NodeRating<Running> {}

impl Ord for NodeRating<Running> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
            .then(other.highest_snakes_alive.cmp(&self.highest_snakes_alive))
            .then(
                if self
                    .current_states_on_this_node
                    .min(other.current_states_on_this_node)
                    < 32
                {
                    other
                        .current_states_on_this_node
                        .cmp(&self.current_states_on_this_node)
                } else {
                    std::cmp::Ordering::Equal
                },
            )
            .then(self.lowest_current_length.cmp(&other.lowest_current_length))
            .then(other.highest_food_distance.cmp(&self.highest_food_distance))
            .then(
                other
                    .highest_middle_distance
                    .cmp(&self.highest_middle_distance),
            )
            .then(
                other
                    .current_states_on_this_node
                    .cmp(&self.current_states_on_this_node),
            )
    }
}

impl From<NodeRating<Running>> for NodeRating<Finished> {
    fn from(rating: NodeRating<Running>) -> Self {
        NodeRating {
            simulation_state: PhantomData,
            ..rating
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_rating() {
        let rating = NodeRating::<Running> {
            initial_states_on_this_node: 150,
            current_states_on_this_node: 100,
            pruned_states_from_this_node: 50,
            lowest_current_length: 10,
            highest_snakes_alive: 3,
            highest_food_distance: 4,
            highest_middle_distance: 6,
            simulation_state: PhantomData,
        };
        let mut rating_2 = rating.clone();
        assert_eq!(rating, rating_2);
        rating_2.highest_snakes_alive = 2;
        assert_eq!(rating.cmp(&rating_2), std::cmp::Ordering::Less);
        rating_2.highest_snakes_alive = 4;
        assert_eq!(rating.cmp(&rating_2), std::cmp::Ordering::Greater);
        rating_2.current_states_on_this_node = 30;
        assert_eq!(rating.cmp(&rating_2), std::cmp::Ordering::Greater);
        rating_2.highest_snakes_alive = 3;
        assert_eq!(rating.cmp(&rating_2), std::cmp::Ordering::Less);
    }
}
