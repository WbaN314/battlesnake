use crate::logic::depth_first::game::d_direction::{DDirection, D_DIRECTION_LIST};

use super::{
    d_node_id::DNodeId,
    node::{DNode, DNodeStatus},
};
use std::{
    cmp::Ordering,
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
        let simulation_status;
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

        // Add alive nodes to results and count the states
        for (_, node) in self.nodes.iter() {
            match node.status() {
                DNodeStatus::Alive(_) if node.id().len() > 0 => {
                    let direction = node.id().first().unwrap();
                    let index: usize = *direction as usize;
                    results[index].states += node.statistics().states.unwrap_or(1);
                    results[index].node_ids.push(node.id().clone());
                }
                _ => (),
            }
        }
        // Sort the node ids by the node result order
        for result in results.iter_mut() {
            result.node_ids.sort_unstable_by(|id1, id2| {
                let node1 = self.nodes.get(id1).unwrap();
                let node2 = self.nodes.get(id2).unwrap();
                node1.result_order(node2)
            });
        }

        for i in 0..4 {
            results[i].depth = if let Some(id) = results[i].node_ids.last() {
                self.nodes.get(&id).unwrap().id().len()
            } else {
                0
            };
            results[i].finished = self.queue[i].len() == 0
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

    pub fn approved_directions(&self) -> [bool; 4] {
        let mut best_nodes: Vec<&Node> = Vec::new();
        for direction_result in self.direction_results.iter() {
            if let Some(id) = direction_result.node_ids.last() {
                let node = self.tree.nodes.get(id).unwrap();
                best_nodes.push(node);
            }
        }

        let mut approved_directions = [false; 4];
        if best_nodes.len() == 0 {
            return approved_directions;
        } else {
            best_nodes.sort_unstable_by(|node1, node2| node1.result_order(node2));
            let last_node = best_nodes.last().unwrap();
            for node in best_nodes.iter() {
                if node.result_order(last_node) == Ordering::Equal {
                    approved_directions[*node.id().first().unwrap() as usize] = true;
                }
            }
            return approved_directions;
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
    pub node_ids: Vec<DNodeId>,
    pub states: usize,
    pub finished: bool,
}

impl DSimulationDirectionResult {
    fn new(direction: DDirection) -> Self {
        Self {
            direction,
            depth: 0,
            node_ids: Vec::new(),
            states: 0,
            finished: false,
        }
    }
}

impl Display for DSimulationDirectionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.direction)?;
        writeln!(f, "Depth: {}", self.depth)?;
        writeln!(f, "States: {}", self.states)?;
        for id in &self.node_ids {
            writeln!(f, "{}", id)?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
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
        println!("{:?}", tree.result().approved_directions());
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
        println!("{:?}", tree.result().approved_directions());
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
        println!("{:?}", tree.result().approved_directions());
    }

    #[test]
    fn test_simulate_full_2() {
        let gamestate = read_game_state("requests/failure_8.json");
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
        println!("{:?}", tree.result().approved_directions());
    }
}
