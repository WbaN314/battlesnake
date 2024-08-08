use std::collections::HashMap;
use std::fmt::Display;

use super::node_rating::NodeRating;
use super::simulation_node::SimulationNode;
use super::simulation_parameters::SimulationParameters;
use super::simulation_state::SimulationState;
use crate::logic::efficient_game_objects::e_direction::EDirectionVec;
use crate::logic::efficient_game_objects::e_game_state::EGameState;
use crate::logic::efficient_game_objects::e_snakes::ESimulationError;

#[derive(Clone)]
pub struct Node {
    pub states: Vec<EGameState>,
    pub rating: NodeRating,
}

impl Node {
    pub fn new(states: Vec<EGameState>, parameters: &SimulationParameters) -> Self {
        let mut node = Node {
            rating: NodeRating::from(&states),
            states,
        };
        node.prune_states(parameters);
        node
    }

    fn calculate_relevant_states_after_move(
        &self,
        parameters: &SimulationParameters,
    ) -> [SimulationState<Vec<EGameState>>; 4] {
        let mut states_by_direction = [
            SimulationState::Alive(Vec::new()),
            SimulationState::Alive(Vec::new()),
            SimulationState::Alive(Vec::new()),
            SimulationState::Alive(Vec::new()),
        ];
        let mut still_relevant = [true, true, true, true];
        for state in self.states.iter() {
            if parameters.is_time_up() {
                return [
                    SimulationState::TimedOut,
                    SimulationState::TimedOut,
                    SimulationState::TimedOut,
                    SimulationState::TimedOut,
                ];
            }
            let state_result = state.calculate_relevant_states_after_move(
                parameters.move_snake_head_distance,
                still_relevant,
            );
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

    pub fn calculate_child_simulation_nodes(
        &self,
        parameters: &SimulationParameters,
    ) -> [SimulationNode; 4] {
        let state_vec = self.calculate_relevant_states_after_move(parameters);
        let mut result = [
            SimulationNode::NotRelevant,
            SimulationNode::NotRelevant,
            SimulationNode::NotRelevant,
            SimulationNode::NotRelevant,
        ];
        for i in 0..4 {
            match state_vec[i] {
                SimulationState::Alive(ref states) => {
                    let node = Node::new(states.clone(), parameters);
                    let simulation_node = SimulationNode::from(node);
                    result[i] = simulation_node;
                }
                SimulationState::Dead => {
                    result[i] = SimulationNode::NotRelevant;
                }
                SimulationState::ChickenAlive(_) => panic!("Not implemented"),
                SimulationState::TimedOut => {
                    result[i] = SimulationNode::Unfinished;
                }
            }
        }
        result
    }

    pub fn compare_including_id(
        a_id: &EDirectionVec,
        b_id: &EDirectionVec,
        a_node: &Node,
        b_node: &Node,
    ) -> std::cmp::Ordering {
        if a_id.len() < b_id.len() {
            std::cmp::Ordering::Less
        } else if a_id.len() > b_id.len() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }

    pub fn print_states(&self) {
        for state in self.states.iter() {
            println!("{}", state);
        }
    }

    pub fn prune_states(&mut self, parameters: &SimulationParameters) {
        if parameters.board_state_prune_distance.is_none() {
            return;
        }
        let initial_states = self.states.len();
        let mut pruned_states: HashMap<u64, Vec<EGameState>> = HashMap::new();
        for state in self.states.drain(..) {
            let hash = state.hash_for_pruning(parameters);
            if let Some(states) = pruned_states.get_mut(&hash) {
                states.push(state);
            } else {
                pruned_states.insert(hash, vec![state]);
            }
        }
        for (_, mut states) in pruned_states.drain() {
            self.states.push(states.pop().unwrap());
        }
        self.rating
            .set_pruned_states(initial_states - self.states.len());
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rating)?;
        Ok(())
    }
}
