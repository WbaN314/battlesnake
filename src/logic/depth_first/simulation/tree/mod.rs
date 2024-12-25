use super::{
    d_node_id::DNodeId,
    node::{DNode, DNodeError},
};
use crate::logic::depth_first::game::d_direction::DDirection;
use std::{
    collections::BTreeMap,
    fmt::Display,
    time::{Duration, Instant},
};

pub struct DTree<Node: DNode> {
    nodes: BTreeMap<DNodeId, Box<Node>>,
    queue: Vec<DNodeId>,
}

impl<Node> DTree<Node>
where
    Node: DNode,
{
    pub fn new(start_node: Node) -> Self {
        let initial_id = start_node.id();
        let mut nodes = BTreeMap::new();
        let mut queue = Vec::new();
        queue.push(initial_id.clone());
        nodes.insert(initial_id.clone(), Box::new(start_node));
        Self { nodes, queue }
    }

    fn simulate(&mut self, duration: Duration) -> DSimulationStatus {
        loop {
            match self.queue.pop() {
                Some(id) => {
                    let parent = self.nodes.get(&id).unwrap();
                    for direction in parent.calc_moves() {
                        match self.calc_child(&id, direction) {
                            Ok(_) => (),
                            Err(DNodeError::Dead) => (),
                            Err(DNodeError::TimedOut) => return DSimulationStatus::TimedOut,
                        }
                    }
                }
                None => return DSimulationStatus::Finished,
            }
        }
    }

    fn calc_child(&mut self, id: &DNodeId, direction: DDirection) -> Result<DNodeId, DNodeError> {
        match self.nodes.get(id) {
            Some(node) if node.is_alive() => {
                let child = node.calc_child(direction);
                match child {
                    Ok(child) => {
                        let id = child.id().clone();
                        self.queue.push(id.clone());
                        self.nodes.insert(id.clone(), child);
                        Ok(id)
                    }
                    Err(error) => Err(error),
                }
            }
            _ => panic!("Node not found"),
        }
    }
}

enum DSimulationStatus {
    TimedOut,
    Finished,
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
            game::{d_field::DSlowField, d_game_state::DGameState},
            simulation::{
                d_node_id::DNodeId,
                node::{
                    d_optimistic_capture_node::DOptimisticCaptureNode,
                    d_pessimistic_capture_node::DPessimisticCaptureNode,
                },
                tree::DTree,
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
        let start_node = DPessimisticCaptureNode::new(DNodeId::default(), state);
        let mut tree = DTree::new(start_node);
        tree.simulate(Duration::from_millis(100));
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
        let start_node = DOptimisticCaptureNode::new(DNodeId::default(), state);
        let mut tree = DTree::new(start_node);
        tree.simulate(Duration::from_millis(100));
        println!("{}", tree);
    }
}
