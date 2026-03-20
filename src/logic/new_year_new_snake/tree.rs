use std::collections::{HashMap, VecDeque};

use crate::logic::{
    game::{field::BasicField, game_state::GameState},
    new_year_new_snake::{
        node::{Node, NodeStatus},
        node_id::NodeId,
    },
};

struct Tree {
    nodes: HashMap<NodeId, Node>,
    queue: VecDeque<NodeId>,
}

impl Tree {
    pub fn new(root: GameState<BasicField>) -> Self {
        let node = Node::new(NodeId::new(), root);
        let queue = VecDeque::from([node.id()]);
        let nodes = HashMap::from([(node.id(), node)]);
        Self { nodes, queue }
    }

    pub fn simulate(&mut self) {
        // Get next node to simulate
        // Simulate the node and get its children
        // If status is AliveFor(1), add all children to the tree and queue
        //  call child_result on the parent node to update the status
        // If status is DeadIn(n), all children or grandchildren of this node die, so the parent node should simulate again
        //  call child_result on the parent node to update the status
        //  Add parent to front of the queue
        // If status is AliveFor(2+), something went wrong
    }
}
