use arrayvec::ArrayVec;
use serde::de;

use crate::logic::{
    depth_first::game::d_direction::{DDirection, D_DIRECTION_LIST},
    legacy::shared::e_snakes::SNAKES,
};

use super::{
    d_node_id::DNodeId,
    node::{self, DNode, DNodeStatus},
};
use std::{
    collections::BTreeMap,
    fmt::Display,
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
    queue: [Vec<DNodeId>; 4],
    time: DTreeTime,
}

impl<Node> DTree<Node>
where
    Node: DNode,
{
    pub fn root(mut self, root: Node) -> Self {
        let id = root.id().clone();
        self.nodes.insert(id.clone(), Box::new(root));
        self.queue[0].push(id);
        self
    }

    pub fn time(mut self, duration: Duration) -> Self {
        self.time = DTreeTime::new(duration);
        self
    }

    pub fn simulate(&mut self) -> DSimulationStatus {
        let mut simulation_status = DSimulationStatus::default();
        let mut count = 0;
        'simulation: loop {
            if self.time.is_timed_out() {
                simulation_status = DSimulationStatus::TimedOut;
                break 'simulation;
            }
            match self.queue[count % 4].pop() {
                Some(parent_id) => {
                    let parent = self.nodes.get(&parent_id).unwrap();
                    match parent.status() {
                        DNodeStatus::Alive(_) => {
                            let children_status = self.calc_children(&parent_id);
                            for (id, status) in children_status {
                                match status {
                                    DNodeStatus::Alive(_) => {
                                        let index = id.direction().unwrap() as usize;
                                        self.queue[index].push(id);
                                        self.queue[index].sort_unstable_by(|id1, id2| {
                                            let node1 = self.nodes.get(id1).unwrap();
                                            let node2 = self.nodes.get(id2).unwrap();
                                            node1.simulation_order(node2)
                                        });
                                    }
                                    DNodeStatus::TimedOut => {
                                        simulation_status = DSimulationStatus::TimedOut;
                                        break 'simulation;
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                None => {
                    if self.queue.iter().all(|q| q.is_empty()) {
                        simulation_status = DSimulationStatus::Finished;
                        break 'simulation;
                    }
                }
            }
            count += 1;
        }
        simulation_status
    }

    pub fn result(&self) -> DSimulationResult<Node> {
        let mut results = Vec::new();
        for d in D_DIRECTION_LIST {
            results.push(DSimulationDirectionResult::new(d));
        }
        for (_, node) in self.nodes.iter() {
            match node.status() {
                DNodeStatus::Alive(_) if node.id().len() > 0 => {
                    let direction = node.id().first().unwrap();
                    let index: usize = *direction as usize;
                    results[index].states += node.statistics().states.unwrap_or(1);
                    let depth = node.id().len();
                    if depth > results[index].depth {
                        results[index].depth = depth;
                        results[index].best.clear();
                        results[index].best.push(node.id().clone());
                    } else if depth == results[index].depth {
                        results[index].best.push(node.id().clone());
                    }
                }
                _ => (),
            }
        }
        DSimulationResult::new(results, self)
    }

    fn calc_children(&mut self, id: &DNodeId) -> Vec<(DNodeId, DNodeStatus)> {
        let mut result = Vec::new();
        match self.nodes.get(id) {
            Some(node) => match node.status() {
                DNodeStatus::Alive(_) => {
                    let children = node.calc_children();
                    for child in children.into_iter() {
                        result.push((child.id().clone(), child.status()));
                        self.nodes.insert(child.id().clone(), child);
                    }
                }
                _ => (),
            },
            _ => panic!("Node not found"),
        }
        result
    }

    fn statistic_states(&self) -> usize {
        let mut states = 0;
        for (_, node) in self.nodes.iter() {
            states += node.statistics().states.unwrap_or(1);
        }
        states
    }

    fn statistic_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn statistics_time(&self) -> Duration {
        self.time.start.elapsed()
    }

    fn statistics_depth(&self) -> usize {
        let mut depth = 0;
        for (_, node) in self.nodes.iter() {
            if node.status() != DNodeStatus::TimedOut {
                depth = depth.max(node.id().len());
            }
        }
        depth
    }
}

pub struct DSimulationResult<'a, Node: DNode> {
    pub direction_results: Vec<DSimulationDirectionResult>,
    tree: &'a DTree<Node>,
}

impl<'a, Node: DNode> DSimulationResult<'a, Node> {
    pub fn new(direction_results: Vec<DSimulationDirectionResult>, tree: &'a DTree<Node>) -> Self {
        Self {
            direction_results,
            tree,
        }
    }

    pub fn direction(&mut self) -> DDirection {
        let mut best_nodes: [Option<&Node>; 4] = [None; 4];
        for i in 0..4 {
            for direction_result in self.direction_results.iter_mut() {
                direction_result.best.sort_unstable_by(|id1, id2| {
                    let node1 = self.tree.nodes.get(id1).unwrap();
                    let node2 = self.tree.nodes.get(id2).unwrap();
                    node1.result_order(node2)
                });
                best_nodes[i] = if let Some(id) = direction_result.best.last() {
                    Some(self.tree.nodes.get(id).unwrap())
                } else {
                    None
                }
            }
        }
        best_nodes.sort_unstable_by(|opt1, opt2| match (opt1, opt2) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(node1), Some(node2)) => node1.result_order(node2),
        });

        match best_nodes.last() {
            Some(Some(node)) => node.id().first().unwrap().clone(),
            _ => DDirection::Up,
        }
    }
}

impl<'a, Node: DNode> Display for DSimulationResult<'a, Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for result in &self.direction_results {
            writeln!(f, "{}", result)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct DSimulationDirectionResult {
    pub direction: DDirection,
    pub depth: usize,
    pub best: Vec<DNodeId>,
    pub states: usize,
}

impl DSimulationDirectionResult {
    fn new(direction: DDirection) -> Self {
        Self {
            direction,
            depth: 0,
            best: Vec::new(),
            states: 0,
        }
    }
}

impl Display for DSimulationDirectionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.direction)?;
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "States: {}", self.states)?;
        for id in &self.best {
            writeln!(f, "{}", id)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub enum DSimulationStatus {
    #[default]
    Unknown,
    TimedOut,
    Finished,
}

impl<Node: DNode> Default for DTree<Node> {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            queue: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            time: DTreeTime::default(),
        }
    }
}

impl<Node: DNode> Display for DTree<Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, node) in &self.nodes {
            writeln!(f, "{}", node.info())?;
        }
        writeln!(f, "")?;
        writeln!(f, "Nodes: {}", self.statistic_nodes())?;
        writeln!(f, "States: {}", self.statistic_states())?;
        writeln!(f, "Depth: {}", self.statistics_depth())?;
        writeln!(f, "Time: {:?}", self.statistics_time())?;
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
        let mut tree = DTree::default().root(root).time(Duration::from_millis(50));
        let status = tree.simulate();
        println!("{}", tree);
        println!("{:?}\n", status);
        println!("{}", tree.result());
        println!("{}", tree.result().direction());
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
        let mut tree = DTree::default().root(root).time(Duration::from_millis(50));
        let status = tree.simulate();
        println!("{}", tree);
        println!("{:?}\n", status);
        println!("{}", tree.result());
        println!("{}", tree.result().direction());
    }

    #[test]
    fn test_simulate_full() {
        let gamestate = read_game_state("requests/test_move_request_2b.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            DTreeTime::new(Duration::from_millis(200)),
            DNodeStatus::default(),
        );
        let mut tree = DTree::default().root(root);
        let status = tree.simulate();
        println!("{}", tree);
        println!("{:?}\n", status);
        println!("{}", tree.result());
        println!("{}", tree.result().direction());
    }
}
