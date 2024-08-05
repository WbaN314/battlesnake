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
