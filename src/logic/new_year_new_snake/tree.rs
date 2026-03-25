use std::{
    collections::{HashMap, VecDeque},
    fmt,
    time::{Duration, Instant},
};

use log::{debug, trace};

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
                debug!("Reached time limit, stopping simulation");
                break;
            }
            if node_id.depth() >= self.max_depth {
                debug!("Reached max depth for {}, skipping node", node_id);
                continue;
            }
            debug!("Simulating {}", node_id);
            let node = self.nodes.get_mut(&node_id).unwrap();
            let children = node.simulate();
            let node_id = node.id();
            let node_status = node.status();
            self.propagate_status(node_id, node_status);
            match children {
                Some(children) if children.is_empty() => {
                    debug!("{} has spawned no children", node_id);
                    // No children for this direction, reque the node itself to simulate the next direction
                    trace!("Adding {} to the front of queue", node_id);
                    self.queue.push_front(node_id);
                }
                Some(children) => {
                    debug!("{} has spawned {} children", node_id, children.len());
                    for child in children {
                        let child_id = child.id();
                        trace!("Adding child {} to the back of queue", child_id);
                        self.nodes.insert(child_id, child);
                        self.queue.push_back(child_id);
                    }
                }
                None => {
                    // All directions exhausted. Go one level up to simulate the next direction of the parent
                    debug!("{} has exhausted all directions", node_id);
                    if let Some(parent_id) = node_id.parent() {
                        trace!("Adding parent {} to the front of queue", parent_id);
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
            trace!(
                "Propagating child status {} to parent {}",
                node_status, parent_id
            );
            let parent = self.nodes.get_mut(&parent_id).unwrap();
            if parent.propagate_update_from_child(node_id, node_status) {
                node_id = parent_id;
                node_status = parent.status();
                trace!("Status for {} updated to {}", parent_id, node_status);
            } else {
                trace!("Status for {} unchanged {}", parent_id, parent.status());
                break;
            }
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Build parent -> children map and group nodes by depth
        let mut children: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut by_depth: HashMap<u8, Vec<NodeId>> = HashMap::new();
        for &id in self.nodes.keys() {
            if let Some(parent_id) = id.parent() {
                children.entry(parent_id).or_default().push(id);
            }
            by_depth.entry(id.depth()).or_default().push(id);
        }

        // Count descendants bottom-up
        let mut depths: Vec<u8> = by_depth.keys().copied().collect();
        depths.sort();
        let mut descendants: HashMap<NodeId, usize> = HashMap::new();
        for &depth in depths.iter().rev() {
            for id in &by_depth[&depth] {
                let child_count: usize = children
                    .get(id)
                    .map(|c| {
                        c.iter()
                            .map(|cid| 1 + descendants.get(cid).copied().unwrap_or(0))
                            .sum()
                    })
                    .unwrap_or(0);
                descendants.insert(*id, child_count);
            }
        }

        // Sort nodes within each depth: by status (best at bottom), then by id
        for ids in by_depth.values_mut() {
            ids.sort_by(|a, b| {
                let status_a = self.nodes[a].status();
                let status_b = self.nodes[b].status();
                status_a
                    .cmp(&status_b)
                    .then_with(|| a.to_string().cmp(&b.to_string()))
            });
        }

        // Print deepest first
        for &depth in depths.iter().rev() {
            for id in &by_depth[&depth] {
                let node = &self.nodes[id];
                let desc = descendants[id];
                writeln!(f, "{} {} {}", id, node.status(), desc)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_game_state;

    #[test]
    fn correct_tree_state_propagation() {
        let gamestate = read_game_state("requests/failure_1.json");
        let root = GameState::<BasicField>::from(&gamestate);
        let mut tree = Tree::new(root).max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(root.direction_status(0), Some(NodeStatus::DeadIn(0)));
        assert_eq!(root.direction_status(1), Some(NodeStatus::AliveFor(3)));
        assert_eq!(root.direction_status(2), Some(NodeStatus::AliveFor(3)));
        assert_eq!(root.direction_status(3), Some(NodeStatus::AliveFor(3)));

        let gamestate = read_game_state("requests/failure_3.json");
        let root = GameState::<BasicField>::from(&gamestate);
        let mut tree = Tree::new(root).max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(root.direction_status(0), Some(NodeStatus::DeadIn(3)));
        assert_eq!(root.direction_status(1), Some(NodeStatus::AliveFor(3)));
        assert_eq!(root.direction_status(2), Some(NodeStatus::DeadIn(0)));
        assert_eq!(root.direction_status(3), Some(NodeStatus::DeadIn(0)));

        let gamestate = read_game_state("requests/failure_4.json");
        let root = GameState::<BasicField>::from(&gamestate);
        let mut tree = Tree::new(root).max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(root.direction_status(0), Some(NodeStatus::DeadIn(3)));
        assert_eq!(root.direction_status(1), Some(NodeStatus::DeadIn(0)));
        assert_eq!(root.direction_status(2), Some(NodeStatus::AliveFor(3)));
        assert_eq!(root.direction_status(3), Some(NodeStatus::DeadIn(0)));
    }

    #[test]
    fn display_tree() {
        let gamestate = read_game_state("requests/failure_4.json");
        let root = GameState::<BasicField>::from(&gamestate);
        let mut tree = Tree::new(root).max_depth(4);
        tree.simulate();
        // println!("{}", tree);

        println!("{}", tree.nodes.get(&"UURU".parse().unwrap()).unwrap());
        println!("{}", tree.nodes.get(&"UURU-UUUU".parse().unwrap()).unwrap());
        println!(
            "{}",
            tree.nodes.get(&"UURU-UUUU-LURU".parse().unwrap()).unwrap()
        );
    }
}
