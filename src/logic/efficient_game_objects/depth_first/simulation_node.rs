#![allow(dead_code)]
use std::fmt::{Display, Formatter};

use super::{node::Node, node_rating::NodeRating, simulation_parameters::SimulationParameters};
use crate::logic::efficient_game_objects::e_game_state::EGameState;

#[derive(Clone)]
pub enum SimulationNode {
    Completed(NodeRating),
    Relevant(Node),
    NotRelevant,
    Unfinished,
}

impl SimulationNode {
    pub fn new(states: Vec<EGameState>) -> Self {
        SimulationNode::Relevant(Node::new(states, &SimulationParameters::new()))
    }

    pub fn from(node: Node) -> Self {
        SimulationNode::Relevant(node)
    }

    pub fn transform_to_completed(&mut self) {
        match self {
            SimulationNode::Relevant(node) => {
                *self = SimulationNode::Completed(node.rating.clone())
            }
            SimulationNode::Completed(_) => panic!("Is already completed"),
            SimulationNode::NotRelevant => panic!("Is not relevant"),
            SimulationNode::Unfinished => panic!("Is unfinished"),
        }
    }

    pub fn update_rating(&mut self, other_rating: &NodeRating) {
        match self {
            SimulationNode::Relevant(node) => {
                node.rating.update(other_rating);
            }
            SimulationNode::Completed(rating) => rating.update(other_rating),
            SimulationNode::NotRelevant => panic!("Is not relevant"),
            SimulationNode::Unfinished => panic!("Is unfinished"),
        }
    }

    pub fn get_rating(&self) -> Option<&NodeRating> {
        match self {
            SimulationNode::Completed(rating) => Some(rating),
            SimulationNode::Relevant(node) => Some(&node.rating),
            SimulationNode::NotRelevant => None,
            SimulationNode::Unfinished => None,
        }
    }

    pub fn calculate_children(&mut self, parameters: &SimulationParameters) -> [SimulationNode; 4] {
        match self {
            SimulationNode::Relevant(node) => {
                return node.calculate_child_simulation_nodes(parameters);
            }
            SimulationNode::Completed(_) => panic!("Children should be already calculated"),
            SimulationNode::NotRelevant => panic!("Node is not relevant"),
            SimulationNode::Unfinished => panic!("Node is unfinished"),
        }
    }

    pub fn print_states(&self) {
        match self {
            SimulationNode::Relevant(node) => {
                node.print_states();
            }
            SimulationNode::Completed(_) => {
                println!("No states to print on Completed SimulationNode")
            }
            SimulationNode::NotRelevant => {
                println!("No states to print on NotRelevant SimulationNode")
            }
            SimulationNode::Unfinished => {
                println!("No states to print on Unfinished SimulationNode")
            }
        }
    }
}

impl Display for SimulationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationNode::Relevant(node) => write!(f, "Relevant {}", node),
            SimulationNode::Completed(rating) => write!(f, "Completed {}", rating),
            SimulationNode::NotRelevant => write!(f, "NotRelevant"),
            SimulationNode::Unfinished => write!(f, "Unfinished"),
        }
    }
}
