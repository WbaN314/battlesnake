use std::fmt::Display;

use super::node_rating::NodeRating;
use super::simulation_node::SimulationNode;
use super::simulation_state::SimulationState;
use crate::logic::efficient_game_objects::e_game_state::EGameState;
use crate::logic::efficient_game_objects::e_snakes::{ESimulationError, Result};

#[derive(Clone)]
pub struct Node {
    states: Vec<EGameState>,
    rating: Option<NodeRating>,
}

impl Node {
    pub fn new(state: EGameState) -> Self {
        Node {
            states: vec![state],
            rating: None,
        }
    }

    pub fn from(states: Vec<EGameState>) -> Self {
        let mut node = Node {
            states,
            rating: None,
        };
        node.calculate_node_rating();
        node
    }

    pub fn calculate_node_rating(&mut self) {
        let rating = NodeRating::from(self);
        self.rating = Some(rating);
    }

    pub fn update_node_rating(&mut self, other_rating: &NodeRating) {
        match self.rating.as_mut() {
            Some(rating) => rating.update(other_rating),
            None => self.rating = Some(other_rating.clone()),
        };
    }

    pub fn get_rating(&self) -> Option<&NodeRating> {
        self.rating.as_ref()
    }

    fn calculate_relevant_states_after_move(
        &self,
        distance: u8,
    ) -> [SimulationState<Vec<EGameState>>; 4] {
        let mut states_by_direction = [
            SimulationState::Alive(Vec::new()),
            SimulationState::Alive(Vec::new()),
            SimulationState::Alive(Vec::new()),
            SimulationState::Alive(Vec::new()),
        ];
        let mut still_relevant = [true, true, true, true];
        for state in self.states.iter() {
            let state_result = state.calculate_relevant_states_after_move(distance, still_relevant);
            for i in 0..4 {
                match state_result[i].to_owned() {
                    Ok(mut states) => {
                        states_by_direction[i].append(&mut states);
                    }
                    Err(ESimulationError::Death) => {
                        states_by_direction[i] = SimulationState::Dead;
                        still_relevant[i] = false;
                    }
                    Err(_) => (),
                }
            }
        }
        states_by_direction
    }

    pub fn calculate_child_simulation_nodes(&self, distance: u8) -> [SimulationNode; 4] {
        let state_vec = self.calculate_relevant_states_after_move(distance);
        let mut result = [
            SimulationNode::NotRelevant,
            SimulationNode::NotRelevant,
            SimulationNode::NotRelevant,
            SimulationNode::NotRelevant,
        ];
        for i in 0..4 {
            match state_vec[i] {
                SimulationState::Alive(ref states) => {
                    let mut node = Node::from(states.clone());
                    node.calculate_node_rating();
                    let simulation_node = SimulationNode::from(node);
                    result[i] = simulation_node;
                }
                SimulationState::Dead => {
                    result[i] = SimulationNode::NotRelevant;
                }
                SimulationState::ChickenAlive(_) => panic!("Not implemented"),
                SimulationState::TimedOut => panic!("Not implemented"),
            }
        }
        result
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rating {
            Some(ref rating) => {
                write!(f, "Rating: {} States: {}", rating, self.states.len())?;
            }
            None => {
                write!(f, "Unrated States: {}", self.states.len())?;
            }
        }
        Ok(())
    }
}
