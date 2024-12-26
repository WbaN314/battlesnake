use arrayvec::ArrayVec;

use super::{
    d_node_id::DNodeId,
    node::{DNode, DNodeStatus},
};
use crate::logic::depth_first::game::d_direction::DDirection;
use std::{
    collections::BTreeMap,
    fmt::Display,
    os::macos::raw::stat,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct DTreeTime {
    pub start: Instant,
    pub duration: Option<Duration>,
}

impl DTreeTime {
    pub fn new(duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            duration: Some(duration),
        }
    }

    pub fn is_timed_out(&self) -> bool {
        match self.duration {
            Some(duration) => self.start.elapsed() > duration,
            None => false,
        }
    }
}

impl Default for DTreeTime {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            duration: None,
        }
    }
}

pub struct DTree<Node: DNode> {
    nodes: BTreeMap<DNodeId, Box<Node>>,
    queue: Vec<DNodeId>,
    time: DTreeTime,
}

impl<Node> DTree<Node>
where
    Node: DNode,
{
    pub fn root(mut self, root: Node) -> Self {
        let id = root.id().clone();
        self.nodes.insert(id.clone(), Box::new(root));
        self.queue.push(id);
        self
    }

    pub fn time(mut self, duration: Duration) -> Self {
        self.time = DTreeTime::new(duration);
        self
    }

    fn simulate(&mut self) -> DSimulationStatus {
        loop {
            if self.time.is_timed_out() {
                return DSimulationStatus::TimedOut;
            }
            match self.queue.pop() {
                Some(parent_id) => {
                    let parent = self.nodes.get(&parent_id).unwrap();
                    match parent.status() {
                        DNodeStatus::Alive => {
                            let children_status = self.calc_children(&parent_id);
                            for (id, status) in children_status {
                                match status {
                                    DNodeStatus::Alive => {
                                        self.queue.push(id);
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                None => return DSimulationStatus::Finished,
            }
        }
    }

    fn calc_children(&mut self, id: &DNodeId) -> Vec<(DNodeId, DNodeStatus)> {
        let mut result = Vec::new();
        match self.nodes.get(id) {
            Some(node) if node.status() == DNodeStatus::Alive => {
                let children = node.calc_children();
                for child in children.into_iter() {
                    result.push((child.id().clone(), child.status()));
                    self.nodes.insert(child.id().clone(), child);
                }
            }
            _ => panic!("Node not found"),
        }
        result
    }
}

enum DSimulationStatus {
    TimedOut,
    Finished,
}

impl<Node: DNode> Default for DTree<Node> {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            queue: Vec::new(),
            time: DTreeTime::default(),
        }
    }
}

impl<Node: DNode> Display for DTree<Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, _) in &self.nodes {
            writeln!(f, "{}", id)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        logic::depth_first::{
            game::{
                d_field::{DFastField, DSlowField},
                d_game_state::DGameState,
            },
            simulation::{
                d_node_id::DNodeId,
                d_tree::{DTree, DTreeTime},
                node::{
                    d_full_simulation_node::DFullSimulationNode,
                    d_optimistic_capture_node::DOptimisticCaptureNode,
                    d_pessimistic_capture_node::DPessimisticCaptureNode, DNodeStatus,
                },
            },
        },
        read_game_state,
    };

    #[test]
    fn test_simulate_with_pessimistic_capture_node() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DPessimisticCaptureNode::new(
            DNodeId::default(),
            state,
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        let mut tree = DTree::default().root(root);
        tree.simulate();
        println!("{}", tree);
    }

    #[test]
    fn test_simulate_with_optimistic_capture_node() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DOptimisticCaptureNode::new(
            DNodeId::default(),
            state,
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        let mut tree = DTree::default().root(root);
        tree.simulate();
        println!("{}", tree);
    }

    #[test]
    fn test_simulate_full() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        let mut tree = DTree::default().root(root);
        tree.simulate();
        println!("{}", tree);
    }
}
