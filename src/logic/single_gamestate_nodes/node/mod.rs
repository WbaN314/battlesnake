use core::panic;
use std::{collections::HashSet, fmt::Display};

use crate::logic::{
    game::{
        direction::{DIRECTIONS, Direction},
        field::BasicField,
        game_state::GameState,
        moves::{MoveMatrix, MoveVector},
        snakes::SNAKES,
    },
    single_gamestate_nodes::node::node_id::NodeId,
};

pub mod node_id;
mod node_stats;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QueueStatus {
    Normal,
    FastTrack,
    ChildOfFastTrack,
}

#[derive(Copy, Clone, Debug, Hash)]
pub enum NodeStatus {
    AliveFor(u8),        // Number of steps where we have checked with guaranteed survival
    DeadIn(u8),          // Number of steps until inevitable death (if opponents play optimally)
    NotSimulated,        // Status not yet determined as this direction has not been simulated
    PrunedDeadAncestor,  // Node was skipped: an ancestor direction is dead
    PrunedMaxDepth,      // Node was skipped: max depth reached
    PrunedForSimilarity, // Node was skipped: a similar gamestate is already in the node
}

impl NodeStatus {
    pub fn increment(self) -> NodeStatus {
        match self {
            NodeStatus::AliveFor(n) => NodeStatus::AliveFor(n + 1),
            NodeStatus::DeadIn(n) => NodeStatus::DeadIn(n + 1),
            _ => panic!("Cannot increment status: {}", self),
        }
    }

    pub fn is_comparable(self) -> bool {
        matches!(self, NodeStatus::AliveFor(_) | NodeStatus::DeadIn(_))
    }

    pub fn for_comparison(self) -> Option<NodeStatus> {
        if self.is_comparable() {
            Some(self)
        } else {
            None
        }
    }
}

impl Eq for NodeStatus {}

impl PartialEq for NodeStatus {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for NodeStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (NodeStatus::AliveFor(n), NodeStatus::AliveFor(m)) => n.partial_cmp(m),
            (NodeStatus::DeadIn(n), NodeStatus::DeadIn(m)) => n.partial_cmp(m),
            (NodeStatus::AliveFor(_), NodeStatus::DeadIn(_)) => Some(std::cmp::Ordering::Greater),
            (NodeStatus::DeadIn(_), NodeStatus::AliveFor(_)) => Some(std::cmp::Ordering::Less),
            // NotSimulated is not comparable to AliveFor or DeadIn, but two NotSimulated are considered equal (required for partial_eq to be consistent with partial_cmp)
            (NodeStatus::NotSimulated, NodeStatus::NotSimulated) => Some(std::cmp::Ordering::Equal),
            (NodeStatus::PrunedDeadAncestor, NodeStatus::PrunedDeadAncestor) => {
                Some(std::cmp::Ordering::Equal)
            }
            (NodeStatus::PrunedMaxDepth, NodeStatus::PrunedMaxDepth) => {
                Some(std::cmp::Ordering::Equal)
            }
            (NodeStatus::PrunedForSimilarity, NodeStatus::PrunedForSimilarity) => {
                Some(std::cmp::Ordering::Equal)
            }
            _ => None,
        }
    }
}

impl Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::AliveFor(n) => write!(f, "AliveFor({})", n),
            NodeStatus::DeadIn(n) => write!(f, "DeadIn({})", n),
            NodeStatus::NotSimulated => write!(f, "NotSimulated"),
            NodeStatus::PrunedDeadAncestor => write!(f, "PrunedDeadAncestor"),
            NodeStatus::PrunedMaxDepth => write!(f, "PrunedMaxDepth"),
            NodeStatus::PrunedForSimilarity => write!(f, "PrunedForSimilarity"),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    id: NodeId,
    gamestate: GameState<BasicField>,
    children: [Option<Vec<(NodeId, NodeStatus)>>; 4],
    pinned_status: Option<NodeStatus>,
    queue_status: QueueStatus,
}

impl Node {
    pub fn new(id: NodeId, gamestate: GameState<BasicField>) -> Self {
        Self {
            id,
            gamestate,
            children: [None, None, None, None],
            pinned_status: None,
            queue_status: QueueStatus::Normal,
        }
    }

    pub fn pin_status(&mut self, status: NodeStatus) {
        if let Some(pinned) = self.pinned_status {
            assert!(
                pinned == status,
                "Node {} already pinned as {}, cannot pin as {}",
                self.id,
                pinned,
                status
            );
        }
        self.pinned_status = Some(status);
    }

    pub fn set_queue_status(&mut self, queue_status: QueueStatus) {
        self.queue_status = queue_status;
    }

    pub fn read_queue_status(&self) -> QueueStatus {
        self.queue_status
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn status(&self) -> NodeStatus {
        if let Some(pinned) = self.pinned_status {
            return pinned;
        }
        if !self.gamestate.is_alive(0) {
            return NodeStatus::DeadIn(0);
        }

        let best = DIRECTIONS
            .into_iter()
            .map(|i| self.direction_status(i))
            .filter_map(|s| s.for_comparison())
            .max_by(|x, y| x.partial_cmp(y).unwrap());

        match best {
            None => NodeStatus::AliveFor(0), // No directions explored yet
            Some(s @ NodeStatus::AliveFor(_)) => s.increment(),
            Some(s @ NodeStatus::DeadIn(_)) => {
                if self.children.iter().all(|c| c.is_some()) {
                    s.increment()
                } else {
                    NodeStatus::AliveFor(0) // Best so far is dead but not all directions explored, so we are still alive for now
                }
            }
            _ => panic!("Invalid best status: {}", best.unwrap()),
        }
    }

    pub fn direction_status(&self, direction: Direction) -> NodeStatus {
        self.children[direction as usize]
            .as_ref()
            .map_or(NodeStatus::NotSimulated, |children| {
                if children.is_empty() {
                    return NodeStatus::DeadIn(0);
                } else if children
                    .iter()
                    .filter(|(_, status)| !matches!(status, NodeStatus::PrunedForSimilarity))
                    .all(|(_, status)| matches!(status, NodeStatus::PrunedDeadAncestor))
                {
                    return NodeStatus::PrunedDeadAncestor;
                } else if children
                    .iter()
                    .filter(|(_, status)| !matches!(status, NodeStatus::PrunedForSimilarity))
                    .all(|(_, status)| matches!(status, NodeStatus::PrunedMaxDepth))
                {
                    return NodeStatus::AliveFor(0);
                } else {
                    return children
                        .iter()
                        .filter_map(|(_, s)| s.for_comparison())
                        .min_by(|x, y| x.partial_cmp(y).unwrap())
                        .unwrap_or_else(|| panic!("{:#?}", self.children)); // Direction with children should always contain a comparable child
                }
            })
    }

    pub fn gamestate(&self) -> &GameState<BasicField> {
        &self.gamestate
    }

    pub fn children(&self) -> &[Option<Vec<(NodeId, NodeStatus)>>; 4] {
        &self.children
    }

    pub fn simulate(
        &mut self,
        similarity_distance: Option<u8>,
        fast_track_fn: Option<&dyn Fn(&Node) -> bool>,
    ) -> Option<Vec<Node>> {
        // Check fast track once

        'moveset: while let Some(move_matrix) = self.next_moveset() {
            let mut children = Vec::new();
            let direction: Direction = move_matrix.get(0).try_into().unwrap();
            let mut similarity_set: HashSet<u64> = HashSet::new();
            for moves in move_matrix {
                let mut child_gamestate = self.gamestate.clone();
                child_gamestate.next_state(moves);
                let child_id = self.id.child(moves);

                if let Some(dist) = similarity_distance {
                    let hash = child_gamestate.local_environment_hash(dist);
                    if !similarity_set.insert(hash) {
                        self.children[direction as usize]
                            .as_mut()
                            .map(|v| v.push((child_id, NodeStatus::PrunedForSimilarity)));
                        continue;
                    }
                }

                let mut child = Node::new(child_id, child_gamestate);
                if self.read_queue_status() == QueueStatus::FastTrack {
                    child.set_queue_status(QueueStatus::ChildOfFastTrack);
                }
                let child_status = child.status();
                children.push(child);
                self.children[direction as usize]
                    .as_mut()
                    .map(|child_vec| child_vec.push((child_id, child_status)));
                match child_status {
                    NodeStatus::DeadIn(0) => {
                        // Do not return children as this direction is already dead
                        continue 'moveset;
                    }
                    NodeStatus::AliveFor(0) => {}
                    _ => {
                        panic!("Invalid child status: {}", child_status);
                    }
                }
            }
            debug_assert!(!children.is_empty()); // Node must spawn children if it is alive

            if children.len() == 1 {
                children
                    .get_mut(0)
                    .map(|child| child.set_queue_status(QueueStatus::FastTrack));
            }

            if let Some(fast_track_fn) = fast_track_fn {
                for child in children.iter_mut() {
                    if fast_track_fn(child) {
                        child.set_queue_status(QueueStatus::FastTrack);
                    }
                }
            }

            return Some(children);
        }
        return None;
    }

    pub fn propagate_update_from_child(
        &mut self,
        child_id: NodeId,
        child_status: NodeStatus,
    ) -> bool {
        let old_status = self.status();
        let dir = child_id.last_direction_for(0).unwrap().unwrap() as usize;
        if let Some(entry) = self.children[dir]
            .as_mut()
            .and_then(|v| v.iter_mut().find(|(id, _)| *id == child_id))
        {
            entry.1 = child_status;
        }
        self.status() != old_status
    }

    fn next_moveset(&mut self) -> Option<MoveMatrix> {
        let mut move_matrix = self.gamestate.valid_moves();
        let directions = move_matrix.get(0).unwrap();
        for i in 0..4 {
            if self.children[i].is_none() {
                if directions[i] {
                    let direction = Direction::try_from(i).unwrap();
                    let new_move = MoveVector::from(direction);
                    move_matrix.set(0, new_move);
                    self.children[i] = Some(Vec::new());
                    return Some(move_matrix);
                } else {
                    self.children[i] = Some(Vec::new());
                }
            }
        }
        None
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{} {}", self.id, self.status())?;
        for (i, slot) in self.children.iter().enumerate() {
            let dir = Direction::try_from(i).unwrap();
            match slot {
                None => writeln!(f, "  {} unexplored", dir)?,
                Some(children) => {
                    let status = self.direction_status(i.try_into().unwrap());
                    writeln!(f, "  {} {} ({} children)", dir, status, children.len())?;
                    for (child_id, child_status) in children {
                        writeln!(f, "    {} {}", child_id, child_status)?;
                    }
                }
            }
        }
        writeln!(f, "\n{}", self.gamestate)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::read_game_state;

    use super::*;

    #[test]
    fn node_status_ordering() {
        // AliveFor: higher is better
        assert!(NodeStatus::AliveFor(5) > NodeStatus::AliveFor(3));
        // DeadIn: higher n is better because it means we survive longer
        assert!(NodeStatus::DeadIn(5) > NodeStatus::DeadIn(1));
        // Alive always beats Dead
        assert!(NodeStatus::AliveFor(0) > NodeStatus::DeadIn(100));
        // Cross-variant not equal
        assert_ne!(NodeStatus::AliveFor(3), NodeStatus::DeadIn(3));
        // max/min pick correctly
        let statuses = vec![
            NodeStatus::DeadIn(5),
            NodeStatus::AliveFor(2),
            NodeStatus::DeadIn(0),
            NodeStatus::AliveFor(0),
        ];
        assert_eq!(
            statuses
                .iter()
                .filter_map(|x| x.for_comparison())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            NodeStatus::AliveFor(2)
        );
        assert_eq!(
            statuses
                .iter()
                .filter_map(|x| x.for_comparison())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            NodeStatus::DeadIn(0)
        );
    }

    fn make_root_node(json_path: &str) -> Node {
        let gamestate = read_game_state(json_path);
        let state = GameState::<BasicField>::from(&gamestate);
        Node::new(NodeId::new(), state)
    }

    #[test]
    fn simulate_exhausts_all_directions() {
        let mut node = make_root_node("requests/example_move_request.json");
        println!("{}", node);
        while node.simulate(None, None).is_some() {
            node.simulate(None, None);
        }
        // After exhaustion, all children slots should be filled
        assert!(
            node.children.iter().all(|c| c.is_some()),
            "all children slots should be populated after exhaustion"
        );
        // All direction statuses should be AliveFor(0) or DeadIn(0)
        for i in DIRECTIONS {
            let status = node.direction_status(i);
            assert!(
                matches!(status, NodeStatus::AliveFor(0) | NodeStatus::DeadIn(0)),
                "direction {} has unexpected status: {}",
                i,
                status
            );
        }
        // Should return empty now
        println!("{}", node);
        assert!(node.simulate(None, None).is_none());
    }

    #[test]
    fn display_half_simulated_node() {
        let mut node = make_root_node("requests/test_game_start.json");
        // Simulate only the first two directions
        node.simulate(None, None);
        println!("{}", node);
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use std::hint::black_box;

    use super::*;
    use crate::read_game_state;

    fn test_nodes() -> Vec<Node> {
        [
            "requests/failure_1.json",
            "requests/failure_3.json",
            "requests/failure_4.json",
            "requests/failure_5.json",
            "requests/example_move_request_2.json",
            "requests/example_move_request_3.json",
        ]
        .iter()
        .map(|p| {
            let gamestate = read_game_state(p);
            let state = GameState::<BasicField>::from(&gamestate);
            Node::new(NodeId::new(), state)
        })
        .collect()
    }

    #[bench]
    fn bench_node_simulate(b: &mut test::Bencher) {
        let source_nodes = test_nodes();
        let mut i = 0;
        b.iter(|| {
            // Fresh clone per iteration so each call starts from a clean, unsimulated node.
            let mut node = source_nodes[i % source_nodes.len()].clone();
            i += 1;
            black_box(node.simulate(black_box(None), black_box(None)))
        });
    }

    #[bench]
    fn bench_node_status(b: &mut test::Bencher) {
        let nodes: Vec<Node> = test_nodes()
            .into_iter()
            .map(|mut n| {
                n.simulate(None, None); // explore one direction
                n
            })
            .collect();
        let mut i = 0;
        b.iter(|| {
            let node = &nodes[i % nodes.len()];
            i += 1;
            black_box(node.status())
        });
    }

    #[bench]
    fn bench_node_propagate_update_from_child(b: &mut test::Bencher) {
        // Build nodes with children so propagate_update_from_child has a real list to scan.
        let prepared: Vec<(Node, NodeId, NodeStatus)> = test_nodes()
            .into_iter()
            .filter_map(|mut parent| {
                // Simulate one direction to populate a children list.
                let children = parent.simulate(None, None)?;
                let (child_id, child_status) = children.first().map(|c| (c.id(), c.status()))?;
                Some((parent, child_id, child_status))
            })
            .collect();

        let mut i = 0;
        b.iter(|| {
            let (parent, child_id, child_status) = &prepared[i % prepared.len()];
            i += 1;
            let mut node = parent.clone();
            black_box(
                node.propagate_update_from_child(black_box(*child_id), black_box(*child_status)),
            )
        });
    }
}
