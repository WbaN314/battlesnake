#![allow(dead_code)]
use std::fmt::{Display, Formatter};

use super::{node::Node, node_rating::NodeRating};
use crate::logic::efficient_game_objects::e_game_state::EGameState;

#[derive(Clone)]
pub enum SimulationNode {
    Completed(NodeRating),
    Relevant(Node),
    NotRelevant,
}

impl SimulationNode {
    pub fn new(state: EGameState) -> Self {
        SimulationNode::Relevant(Node::new(state))
    }

    pub fn from(node: Node) -> Self {
        SimulationNode::Relevant(node)
    }

    pub fn transform_to_completed(&mut self) {
        match self {
            SimulationNode::Relevant(node) => {
                if let Some(rating) = node.get_rating() {
                    *self = SimulationNode::Completed(rating.clone())
                } else {
                    panic!("Cannot convert to completed without rating");
                }
            }
            SimulationNode::Completed(_) => panic!("Is already completed"),
            SimulationNode::NotRelevant => panic!("Is not relevant"),
        }
    }

    pub fn update_rating(&mut self, other_rating: &NodeRating) {
        match self {
            SimulationNode::Relevant(node) => {
                node.update_node_rating(other_rating);
            }
            SimulationNode::Completed(rating) => rating.update(other_rating),
            SimulationNode::NotRelevant => panic!("Is not relevant"),
        }
    }

    pub fn get_rating(&self) -> Option<&NodeRating> {
        match self {
            SimulationNode::Completed(rating) => Some(rating),
            SimulationNode::Relevant(node) => node.get_rating(),
            SimulationNode::NotRelevant => None,
        }
    }

    pub fn calculate_children(&mut self, distance: u8) -> [SimulationNode; 4] {
        match self {
            SimulationNode::Relevant(node) => {
                return node.calculate_child_simulation_nodes(distance);
            }
            SimulationNode::Completed(_) => panic!("Children should be already calculated"),
            SimulationNode::NotRelevant => panic!("Node is not relevant"),
        }
    }
}

impl Display for SimulationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationNode::Relevant(node) => write!(f, "Relevant {}", node),
            SimulationNode::Completed(rating) => write!(f, "Completed {}", rating),
            SimulationNode::NotRelevant => write!(f, "NotRelevant"),
        }
    }
}
