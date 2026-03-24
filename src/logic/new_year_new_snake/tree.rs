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
        while let Some(node_id) = self.queue.pop_front() {
            let node = self.nodes.get_mut(&node_id).unwrap();
            let children = node.simulate();
            let node_id = node.id();
            let node_status = node.status();
            match children {
                Some(children) if children.is_empty() => {
                    // No children for this direction, reque the node itself to simulate the next direction
                    self.queue.push_front(node_id);
                    // Status of node might have changed after simulation, so we need to propagate it up the tree
                    self.propagate_status(node_id, node_status);
                }
                Some(children) => {
                    for child in children {
                        let child_id = child.id();
                        self.nodes.insert(child_id, child);
                        self.queue.push_back(child_id);
                    }
                    // Status of node might have changed after simulation, so we need to propagate it up the tree
                    self.propagate_status(node_id, node_status);
                }

                None => {
                    // All directions exhausted. Go one level up to simulate the next direction of the parent
                    if let Some(parent_id) = node_id.parent() {
                        self.queue.push_front(parent_id);
                    }
                }
            }
        }
    }

    fn propagate_status(&mut self, node_id: NodeId, node_status: NodeStatus) {
        let mut node_id = node_id;
        let mut node_status = node_status;
        while let Some(parent_id) = node_id.parent() {
            let parent = self.nodes.get_mut(&parent_id).unwrap();
            if parent.update_from_child(node_id, node_status) {
                node_id = parent_id;
                node_status = parent.status();
            } else {
                break;
            }
        }
    }
}
