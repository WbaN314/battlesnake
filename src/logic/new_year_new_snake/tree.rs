use std::{
    collections::{HashMap, VecDeque},
    fmt,
    time::{Duration, Instant},
};

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
    max_depth: u8,
    max_time: Option<Duration>,
}

impl Tree {
    pub fn new(root: GameState<BasicField>) -> Self {
        let node = Node::new(NodeId::new(), root);
        let queue = VecDeque::from([node.id()]);
        let nodes = HashMap::from([(node.id(), node)]);
        Self {
            nodes,
            queue,
            max_depth: u8::MAX,
            max_time: None,
        }
    }

    pub fn max_depth(mut self, max_depth: u8) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn max_time(mut self, max_time: Duration) -> Self {
        self.max_time = Some(max_time);
        self
    }

    pub fn simulate(&mut self) {
        let deadline = self.max_time.map(|d| Instant::now() + d);
        // Get next node to simulate
        while let Some(node_id) = self.queue.pop_front() {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                break;
            }
            if node_id.depth() >= self.max_depth {
                continue;
            }
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

    fn fmt_node(
        &self,
        f: &mut fmt::Formatter<'_>,
        node_id: &NodeId,
        children: &HashMap<NodeId, Vec<NodeId>>,
        indent: usize,
    ) -> fmt::Result {
        let prefix = "  ".repeat(indent);
        let node = &self.nodes[node_id];
        writeln!(f, "{}{} {}", prefix, node_id, node.status())?;
        if let Some(child_ids) = children.get(node_id) {
            for child_id in child_ids {
                self.fmt_node(f, child_id, children, indent + 1)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Build parent -> children map
        let mut children: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut root_id = None;
        for &id in self.nodes.keys() {
            match id.parent() {
                Some(parent_id) => children.entry(parent_id).or_default().push(id),
                None => root_id = Some(id),
            }
        }
        // Sort children for deterministic output
        for ids in children.values_mut() {
            ids.sort_by_cached_key(|id| id.to_string());
        }
        if let Some(root_id) = root_id {
            self.fmt_node(f, &root_id, &children, 0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_game_state;

    #[test]
    fn display_tree() {
        let gamestate = read_game_state("requests/test_game_start.json");
        let root = GameState::<BasicField>::from(&gamestate);
        let mut tree = Tree::new(root).max_time(Duration::from_millis(200));
        tree.simulate();
        println!("{}", tree);
    }
}
