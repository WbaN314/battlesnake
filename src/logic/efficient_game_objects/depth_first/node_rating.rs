use std::fmt::{Display, Formatter};

use super::node::Node;

#[derive(Clone)]
pub struct NodeRating {}

impl NodeRating {
    pub fn from(_state: &Node) -> Self {
        NodeRating {}
    }

    pub fn update(&mut self, other: &NodeRating) {
        // TODO
    }
}

impl Display for NodeRating {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "TODO NodeRating")
    }
}
