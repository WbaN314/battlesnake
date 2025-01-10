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
    max_depth: Option<usize>,
    simulation_status: DSimulationStatus,
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

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
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
                    if parent_id.len() >= self.max_depth.unwrap_or(usize::MAX) {
                        continue;
                    }
                    let parent = self.nodes.get(&parent_id).unwrap();
                    if let DNodeStatus::Alive(_) = parent.status() {
                        let children_status = self.calc_children(&parent_id);
                        for (id, status) in children_status {
                            match status {
                                DNodeStatus::Alive(_) => {
                                    let index = id.direction().unwrap() as usize;
                                    if id.len() < self.max_depth.unwrap_or(usize::MAX) {
                                        self.queue[index].push(id);
                                        self.queue[index].sort_unstable_by(|id1, id2| {
                                            let node1 = self.nodes.get(id1).unwrap();
                                            let node2 = self.nodes.get(id2).unwrap();
                                            node1.simulation_order(node2)
                                        });
                                    }
                                }
                                DNodeStatus::TimedOut => {
                                    simulation_status = DSimulationStatus::TimedOut;
                                    break 'simulation;
                                }
                                _ => (),
                            }
                        }
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
        self.simulation_status = simulation_status;
        self.simulation_status
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
                    let statistics = node.statistics();
                    results[index].states += statistics.states.unwrap_or(1);
                    results[index].node_ids.push(node.id().clone());
                    results[index]
                        .capture_contact_turn
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, s)| {
                            if let Some(relevance) = statistics.relevant_snakes[i] {
                                if let Some(current_relevance) = s {
                                    *s = Some(relevance.min(*current_relevance));
                                } else {
                                    *s = Some(relevance);
                                }
                            }
                        });
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
                self.nodes.get(id).unwrap().id().len()
            } else {
                0
            };
            results[i].finished = self.queue[i].is_empty();
            if !results[i].finished {
                for j in 0..4 {
                    if results[i].capture_contact_turn[j].is_none() {
                        results[i].capture_contact_turn[j] = Some(results[i].depth as u8);
                    }
                }
            }
        }

        DSimulationResult::new(results, self)
    }

    fn calc_children(&mut self, id: &DNodeId) -> Vec<(DNodeId, DNodeStatus)> {
        let mut result = Vec::new();
        match self.nodes.get(id) {
            Some(node) => {
                if let DNodeStatus::Alive(_) = node.status() {
                    let children = node.calc_children();
                    for child in children.into_iter() {
                        result.push((child.id().clone(), child.status()));
                        self.nodes.insert(child.id().clone(), child);
                    }
                }
            }
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
    direction_results: Vec<DSimulationDirectionResult>,
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
        if best_nodes.is_empty() {
            approved_directions
        } else {
            best_nodes.sort_unstable_by(|node1, node2| node1.result_order(node2));
            let last_node = best_nodes.last().unwrap();
            for node in best_nodes.iter() {
                if node.result_order(last_node) == Ordering::Equal {
                    approved_directions[*node.id().first().unwrap() as usize] = true;
                }
            }
            approved_directions
        }
    }

    pub fn capture_contact_turn(&self) -> [[Option<u8>; 4]; 4] {
        let mut capture_contact_turn: [[Option<u8>; 4]; 4] = Default::default();
        for (i, result) in self.direction_results.iter().enumerate() {
            capture_contact_turn[i] = result.capture_contact_turn;
        }
        capture_contact_turn
    }
}

impl<'a, Node: DNode> Display for DSimulationResult<'a, Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- General Info ---")?;
        writeln!(f, "Nodes: {}", self.tree.statistic_nodes())?;
        writeln!(f, "States: {}", self.tree.statistic_states())?;
        writeln!(f, "Depth: {}", self.tree.statistics_depth())?;
        writeln!(f, "Time: {:?}", self.tree.statistics_time())?;
        writeln!(f, "Status: {:?}", self.tree.simulation_status)?;

        writeln!(f, "--- Direction Results ---")?;
        for result in &self.direction_results {
            writeln!(f, "{}", result.direction)?;
            writeln!(f, "Depth: {}", result.depth)?;
            writeln!(f, "States: {}", result.states)?;
            writeln!(f, "Finished: {}", result.finished)?;
            writeln!(
                f,
                "Capture Contact: {:?}",
                result
                    .capture_contact_turn
                    .map(|rel| if let Some(rel) = rel {
                        rel.to_string()
                    } else {
                        "X".to_string()
                    })
            )?;

            if !result.node_ids.is_empty() {
                let best_node = self
                    .tree
                    .nodes
                    .get(result.node_ids.last().unwrap())
                    .unwrap();
                writeln!(f, "{}", best_node.info())?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DSimulationDirectionResult {
    pub direction: DDirection,
    pub depth: usize,
    pub node_ids: Vec<DNodeId>,
    pub states: usize,
    pub finished: bool,
    pub capture_contact_turn: [Option<u8>; 4],
}

impl DSimulationDirectionResult {
    fn new(direction: DDirection) -> Self {
        Self {
            direction,
            depth: 0,
            node_ids: Vec::new(),
            states: 0,
            finished: false,
            capture_contact_turn: [Option::default(); 4],
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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
            max_depth: None,
            simulation_status: DSimulationStatus::default(),
        }
    }
}

impl<Node: DNode> Display for DTree<Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.nodes.values() {
            writeln!(f, "{}", node.info())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use test::bench;

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
                    d_pessimistic_capture_node::DPessimisticCaptureNode, DNode, DNodeAliveStatus,
                    DNodeStatistics, DNodeStatus,
                },
            },
        },
        read_game_state,
    };

    #[bench]
    fn bench_tree_simulate(b: &mut bench::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );

        b.iter(|| {
            let root = DFullSimulationNode::new(
                DNodeId::default(),
                vec![state.clone()],
                DTreeTime::default(),
                DNodeStatus::default(),
                None,
            );
            let mut tree = DTree::default().root(root).max_depth(4);
            tree.simulate()
        });
    }

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
        tree.simulate();
        println!("{}", tree.result());
        println!("{:?}", tree.result().approved_directions());
    }

    #[test]
    fn test_simulate_with_optimistic_capture_node() {
        let gamestate = read_game_state("requests/failure_8.json");
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
            DNodeStatistics::default(),
        );
        let mut tree = DTree::default().root(root).max_depth(20);
        tree.simulate();
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
            None,
        );
        let mut tree = DTree::default().root(root);
        tree.simulate();
        println!("{}", tree.result());
        println!("{:?}", tree.result().approved_directions());
    }

    #[test]
    fn test_simulate_correct_alive_substate_propagation() {
        let gamestate = read_game_state("requests/failure_20_for_improved_area_evaluation.json");
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
            None,
        );
        let mut tree = DTree::default().root(root).max_depth(8);
        tree.simulate();
        println!("{}", tree.result());
        println!("{:?}", tree.result().approved_directions());

        let always_alive = tree.nodes.get(&DNodeId::from("DRRRRR")).unwrap();
        let sometimes_alive = tree.nodes.get(&DNodeId::from("DRRRRRR")).unwrap();
        let sometimes_alive_2 = tree.nodes.get(&DNodeId::from("DRRRRRRU")).unwrap();

        assert_eq!(
            always_alive.status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(
            sometimes_alive.status(),
            DNodeStatus::Alive(DNodeAliveStatus::Sometimes)
        );
        assert_eq!(
            sometimes_alive_2.status(),
            DNodeStatus::Alive(DNodeAliveStatus::Sometimes)
        );
    }

    #[test]
    fn test_tree_max_depth() {
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
            DTreeTime::new(Duration::from_millis(200)),
            DNodeStatus::default(),
            DNodeStatistics::default(),
        );
        let mut tree = DTree::default().root(root).max_depth(10);
        tree.simulate();
        println!("{}", tree);
        assert_eq!(tree.statistics_depth(), 10);
    }

    #[test]
    fn test_capture_contact_turn() {
        let gamestate = read_game_state("requests/failure_8.json");
        let state = DGameState::<DSlowField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DOptimisticCaptureNode::new(
            DNodeId::default(),
            state,
            DTreeTime::new(Duration::from_millis(200)),
            DNodeStatus::default(),
            DNodeStatistics::default(),
        );
        let mut tree = DTree::default().root(root).max_depth(10);
        tree.simulate();
        println!("{}", tree);
        let result = tree.result();
        let capture_contact_turn = result.capture_contact_turn();
        assert_eq!(capture_contact_turn[0], [None, Some(2), None, None]);
        assert_eq!(capture_contact_turn[1], [None, Some(5), Some(2), None]);
        assert_eq!(capture_contact_turn[2], [None, None, None, None]);
        assert_eq!(capture_contact_turn[3], [None, Some(1), None, None]);
    }

    #[test]
    fn test_capture_contact_depth() {
        let gamestate = read_game_state("requests/failure_45_panic_again.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state.clone()],
            Default::default(),
            DNodeStatus::default(),
            None,
        );

        let capture_contact_depth = Some([
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
        ]);

        let root2 = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            capture_contact_depth,
        );

        let mut tree_with_ccd = DTree::default().root(root).max_depth(4);
        tree_with_ccd.simulate();
        let result_with_ccd = tree_with_ccd.result();

        let mut tree_without_ccd = DTree::default().root(root2).max_depth(4);
        tree_without_ccd.simulate();
        let result_without_ccd = tree_without_ccd.result();

        println!("{}", result_with_ccd);
        println!("{}", result_without_ccd);

        assert_ne!(
            result_with_ccd.direction_results[2],
            result_without_ccd.direction_results[2]
        );

        let gamestate = read_game_state("requests/failure_2.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);
        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state.clone()],
            Default::default(),
            DNodeStatus::default(),
            None,
        );

        let capture_contact_depth = Some([
            [true, true, true, false],
            [true, false, true, false],
            [true, false, false, false],
            [true, false, true, false],
        ]);

        let root2 = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            capture_contact_depth,
        );

        let mut tree_with_ccd = DTree::default().root(root).max_depth(4);
        tree_with_ccd.simulate();
        let result_with_ccd = tree_with_ccd.result();

        let mut tree_without_ccd = DTree::default().root(root2).max_depth(4);
        tree_without_ccd.simulate();
        let result_without_ccd = tree_without_ccd.result();

        println!("{}", result_with_ccd);
        println!("{}", result_without_ccd);

        assert_eq!(
            result_with_ccd.direction_results[0],
            result_without_ccd.direction_results[0]
        );

        assert_ne!(
            result_with_ccd.direction_results[1],
            result_without_ccd.direction_results[1]
        );
    }
}
