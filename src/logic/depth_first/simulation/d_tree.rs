use log::{debug, trace};

use crate::logic::depth_first::game::d_direction::{D_DIRECTION_LIST, DDirection};

use super::{
    d_node_id::DNodeId,
    node::{DChildrenCalculationResult, DNode, DNodeAliveStatus, DNodeStatus},
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
    statistics: DTreeStatistics,
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
        self.time.start = Instant::now();
        let mut count = 0;
        'simulation: loop {
            if self.time.is_timed_out() {
                simulation_status = DSimulationStatus::TimedOut;
                break 'simulation;
            }
            let direction = count % 4;
            let direction_timer = Instant::now();
            match self.queue[direction].pop() {
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
            self.statistics.time_per_direction[direction] += direction_timer.elapsed();
            count += 1;
        }
        self.calc_statistics();
        self.simulation_status = simulation_status;

        trace!(
            "{:#?}",
            self.nodes
                .iter()
                .map(|(_, node)| (*node).info())
                .collect::<Vec<_>>()
        );
        trace!(
            "U Queue:\n{:?}",
            self.queue[0]
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        );
        trace!(
            "D Queue:\n{:?}",
            self.queue[1]
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        );
        trace!(
            "L Queue:\n{:?}",
            self.queue[2]
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        );
        trace!(
            "R Queue:\n{:?}",
            self.queue[3]
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        );

        self.simulation_status
    }

    pub fn result(&self) -> DSimulationResult<'_, Node> {
        let mut results = Vec::new();
        for d in D_DIRECTION_LIST {
            results.push(DSimulationDirectionResult::new(d));
        }

        let best_status = self
            .nodes
            .iter()
            .filter_map(|(id, node)| {
                if id == &DNodeId::default() {
                    None
                } else {
                    Some(node.status())
                }
            })
            .max()
            .unwrap_or(DNodeStatus::Dead);

        // Add alive nodes to results and count the states
        for (_, node) in self.nodes.iter() {
            match (node.status(), best_status) {
                (DNodeStatus::Alive(DNodeAliveStatus::Fast), _) => (), // Ignore fast nodes
                (DNodeStatus::Alive(_), DNodeStatus::Alive(_))
                | (DNodeStatus::DeadEndIn(_), DNodeStatus::DeadEndIn(_))
                    if node.id().len() > 0 =>
                {
                    let direction = node.id().first().unwrap();
                    let index: usize = *direction as usize;
                    let statistics = node.statistics();
                    results[index].states += statistics.states.unwrap_or(1);
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
                self.nodes.get(id).unwrap().id().len()
            } else {
                0
            };
            if self.queue[i].is_empty() {
                results[i].finished = DDirectionFinished::Always;
            } else {
                results[i].finished = DDirectionFinished::Sometimes;
                for id in self.queue[i].iter() {
                    if let DNodeStatus::Alive(alive_status) = self.nodes.get(id).unwrap().status() {
                        if alive_status == DNodeAliveStatus::Always {
                            results[i].finished = DDirectionFinished::Never;
                        }
                    }
                }
            }
        }

        DSimulationResult::new(results, self)
    }

    fn calc_children(&mut self, id: &DNodeId) -> Vec<(DNodeId, DNodeStatus)> {
        let mut result = Vec::new();
        match self.nodes.get_mut(id) {
            Some(node) => {
                if let DNodeStatus::Alive(_) = node.status() {
                    debug!("Simulating: {}", node.info());
                    let children = node.calc_children();

                    match children {
                        DChildrenCalculationResult::FastEnd => {
                            // Only possible for fast nodes that open up to have multiple choices again
                            debug!("Simulation FastEnd: {}", node.info());
                        }
                        DChildrenCalculationResult::DeadEnd => {
                            debug!("Simulation DeadEnd: {}", node.info());
                            let mut id = id.clone();
                            if id.is_sparse() {
                                id.set_sparse(false);
                                let mut reverse_depth: u8 = 0;
                                while !self.nodes.contains_key(&id) {
                                    id.set_sparse(true);
                                    self.nodes.get_mut(&id).map(|existing_node| {
                                        let new_status = DNodeStatus::DeadEndIn(reverse_depth);
                                        debug!(
                                            "Changing status to {:?}: {}",
                                            new_status,
                                            existing_node.info()
                                        );
                                        existing_node.set_status(new_status)
                                    });
                                    id.set_sparse(false);
                                    id = id.parent().unwrap();
                                    reverse_depth += 1;
                                }
                                id.set_sparse(true);
                                self.nodes.get_mut(&id).map(|existing_node| {
                                    let new_status = DNodeStatus::DeadEndIn(reverse_depth);
                                    debug!(
                                        "Changing status to {:?}: {}",
                                        new_status,
                                        existing_node.info()
                                    );
                                    existing_node.set_status(new_status)
                                });
                                id.set_sparse(false);
                                self.nodes.get_mut(&id).map(|existing_node| {
                                    let new_status = DNodeStatus::DeadEndIn(reverse_depth);
                                    debug!(
                                        "Changing status to {:?}: {}",
                                        new_status,
                                        existing_node.info()
                                    );
                                    existing_node.set_status(new_status)
                                });
                            } else {
                                let new_status = DNodeStatus::DeadEndIn(0);
                                debug!("Changing status to {:?}: {}", new_status, node.info());
                                node.set_status(new_status);
                            }
                        }
                        DChildrenCalculationResult::TimedOut => {
                            debug!("Simulation TimedOut: {}", node.info());
                            self.queue[*id.first().unwrap_or(&DDirection::Up) as usize]
                                .push(id.clone());
                        }
                        DChildrenCalculationResult::Ok(children) => {
                            // Check for special case timeout in node calc_child
                            // Just put it back into the queue
                            // The node itself tracks its child generation progress internally
                            debug!("Simulation Ok: {}", node.info());
                            for child in children.into_iter() {
                                debug!("Child: {}", child.info());
                                if child.id().is_sparse() && self.nodes.contains_key(child.id()) {
                                    self.statistics.ignored_fast_nodes += 1;
                                    debug!("Child ignored: {}", child.info());
                                    continue;
                                }
                                result.push((child.id().clone(), child.status()));
                                self.nodes.insert(child.id().clone(), child);
                            }
                        }
                    }
                } else {
                    panic!("Calculating children of a non alive node");
                }
            }
            _ => panic!("Node not found"),
        }
        result
    }

    fn get_statistics(&self) -> DTreeStatistics {
        self.statistics
    }

    fn calc_statistics(&mut self) {
        self.statistics.states = self.statistic_states();
        self.statistics.nodes = self.statistic_nodes();
        self.statistics.depth = self.statistic_depth();
        self.statistics.time = self.statistic_time();
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

    fn statistic_time(&self) -> Duration {
        self.time.start.elapsed()
    }

    fn statistic_depth(&self) -> usize {
        let mut depth = 0;
        for (_, node) in self.nodes.iter() {
            depth = depth.max(node.id().len());
        }
        depth
    }
}

#[derive(Copy, Clone, Default)]
pub struct DTreeStatistics {
    pub states: usize,
    pub nodes: usize,
    pub depth: usize,
    pub time: Duration,
    pub time_per_direction: [Duration; 4],
    pub ignored_fast_nodes: usize,
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
        let mut approved_directions = [true; 4];

        // Check if any direction is never finished
        // if so, all others types are not approved
        // If not, only Never is not approved
        let mut best_evaulation_status = DDirectionFinished::Always;
        for direction_result in self.direction_results.iter() {
            if direction_result.finished > best_evaulation_status {
                best_evaulation_status = direction_result.finished.clone();
            }
        }

        let max_depth_of_best_statuses = self
            .direction_results
            .iter()
            .filter(|result| result.finished == best_evaulation_status)
            .map(|result| result.depth)
            .max()
            .unwrap();

        for direction_result in self.direction_results.iter() {
            if self.tree.simulation_status == DSimulationStatus::Finished
                && direction_result.depth < max_depth_of_best_statuses
            {
                approved_directions[direction_result.direction as usize] = false;
            } else if direction_result.finished < best_evaulation_status {
                approved_directions[direction_result.direction as usize] = false;
            }
        }

        // Check the best nodes for each direction
        for (i, direction_result) in self.direction_results.iter().enumerate() {
            if approved_directions[i] {
                if let Some(id) = direction_result.node_ids.last() {
                    let node = self.tree.nodes.get(id).unwrap();
                    best_nodes.push(node);
                } else {
                    approved_directions[i] = false;
                }
            }
        }
        approved_directions = if best_nodes.is_empty() {
            approved_directions
        } else {
            best_nodes.sort_unstable_by(|node1, node2| node1.direction_order(node2));
            let best_node = best_nodes.last().unwrap();
            for node in best_nodes.iter() {
                if node.direction_order(best_node) == Ordering::Less {
                    approved_directions[*node.id().first().unwrap() as usize] = false;
                }
            }
            approved_directions
        };

        approved_directions
    }
}

impl<'a, Node: DNode> Display for DSimulationResult<'a, Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statistics = self.tree.get_statistics();
        writeln!(f, "--- General Info ---")?;
        writeln!(f, "Nodes: {}", statistics.nodes)?;
        writeln!(f, "States: {}", statistics.states)?;
        writeln!(f, "Depth: {}", statistics.depth)?;
        writeln!(f, "Time: {:?}", statistics.time)?;
        writeln!(f, "Ignored Fast Nodes: {}", statistics.ignored_fast_nodes)?;
        writeln!(f, "Status: {:?}", self.tree.simulation_status)?;

        writeln!(f, "--- Direction Results ---")?;
        for (i, result) in self.direction_results.iter().enumerate() {
            writeln!(f, "{}", result.direction)?;
            writeln!(f, "Depth: {}", result.depth)?;
            writeln!(f, "States: {}", result.states)?;
            writeln!(f, "Direction Time: {:?}", statistics.time_per_direction[i])?;
            writeln!(f, "Finished: {:?}", result.finished)?;

            if !result.node_ids.is_empty() {
                let best_node = self
                    .tree
                    .nodes
                    .get(result.node_ids.last().unwrap())
                    .unwrap();
                writeln!(f, "{}", best_node.id())?;
                writeln!(f, "{}", best_node.statistics())?;
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
    pub finished: DDirectionFinished,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DDirectionFinished {
    Always,
    Sometimes,
    Never,
}

impl DSimulationDirectionResult {
    fn new(direction: DDirection) -> Self {
        Self {
            direction,
            depth: 0,
            node_ids: Vec::new(),
            states: 0,
            finished: DDirectionFinished::Never,
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
            statistics: DTreeStatistics::default(),
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

    use crate::{
        logic::depth_first::{
            game::{
                d_field::{DFastField, DSlowField},
                d_game_state::DGameState,
            },
            simulation::{
                d_node_id::DNodeId,
                d_tree::{DDirectionFinished, DTree, DTreeTime},
                node::{
                    DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus,
                    d_full_simulation_node::DFullSimulationNode,
                    d_optimistic_capture_node::DOptimisticCaptureNode,
                    d_pessimistic_capture_node::DPessimisticCaptureNode,
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
            None,
            None,
        );
        let mut tree = DTree::default().root(root).time(Duration::from_millis(200));
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
            None,
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
        assert_eq!(tree.statistic_depth(), 10);
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
            None,
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
            None,
            None,
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
            None,
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
            None,
            None,
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

    #[test]
    fn test_split_processing_time_equal_between_directions() {
        let gamestate = read_game_state("requests/failure_8.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let relevant_snakes = [
            [true, true, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [true, true, false, false],
        ];

        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            DTreeTime::new(Duration::from_millis(20)),
            DNodeStatus::default(),
            Some(relevant_snakes),
            None,
            None,
        );
        let mut tree = DTree::default().root(root).time(Duration::from_millis(200));
        tree.simulate();
        println!("{}", tree);
        let result = tree.result();
        println!("{}", result);
        assert_eq!(
            result.direction_results[0].finished,
            DDirectionFinished::Sometimes
        );
        assert_eq!(
            result.direction_results[1].finished,
            DDirectionFinished::Never
        );
        assert_eq!(
            result.direction_results[2].finished,
            DDirectionFinished::Always
        );
        assert_eq!(
            result.direction_results[3].finished,
            DDirectionFinished::Sometimes
        );
    }

    #[test]
    fn test_ddirectionfinished_ordering() {
        assert!(DDirectionFinished::Always < DDirectionFinished::Sometimes);
        assert!(DDirectionFinished::Sometimes < DDirectionFinished::Never);
    }

    #[test]
    fn test_that_fast_nodes_are_not_inserted_multiple_time() {
        let gamestate =
            read_game_state("requests/failure_33_do_not_move_left_as_you_can_get_killed.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        state = state.play(["L", "", "", "D"]);
        println!("{}", state);

        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            None,
            None,
            None,
        );
        let mut tree = DTree::default().root(root).max_depth(3);
        tree.simulate();
        println!("{}", tree);
        assert_eq!(tree.statistics.ignored_fast_nodes, 1);

        let gamestate =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        state = state.play(["D", "", "D", ""]);
        println!("{}", state);

        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            None,
            None,
            None,
        );
        let mut tree = DTree::default().root(root).max_depth(4);
        tree.simulate();
        println!("{}", tree);
        assert_eq!(tree.statistics.ignored_fast_nodes, 3);
    }

    #[test]
    fn test_dead_end_propagation_to_spawn_node() {
        let gamestate =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        state = state.play(["D", "", "D", ""]);
        println!("{}", state);

        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            None,
            None,
            None,
        );
        let mut tree_6 = DTree::default().root(root.clone()).max_depth(6);
        tree_6.simulate();
        println!("{}", tree_6);
        assert_eq!(
            tree_6.nodes.get(&"D".into()).unwrap().status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(tree_6.nodes.len(), 13);
        let mut tree_7 = DTree::default().root(root).max_depth(7);
        tree_7.simulate();
        println!("{}", tree_7);
        assert_eq!(
            tree_7.nodes.get(&"D".into()).unwrap().status(),
            DNodeStatus::DeadEndIn(5)
        );
        assert_eq!(
            tree_7.nodes.get(&"D-s".into()).unwrap().status(),
            DNodeStatus::DeadEndIn(5)
        );
        assert_eq!(
            tree_7.nodes.get(&"DD-s".into()).unwrap().status(),
            DNodeStatus::DeadEndIn(4)
        );
        assert_eq!(
            tree_7.nodes.get(&"DDDDDD-s".into()).unwrap().status(),
            DNodeStatus::DeadEndIn(0)
        );
        assert_eq!(tree_7.nodes.len(), 8);
    }

    #[test]
    fn test_dead_end() {
        let gamestate =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let mut state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        state = state.play(["UULU", "", "D", ""]);
        println!("{}", state);
        let root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            None,
            None,
            None,
        );
        let mut tree = DTree::default().root(root).max_depth(4);
        tree.simulate();
        println!("{}", tree);
        assert_eq!(
            tree.nodes.get(&"R".into()).unwrap().status(),
            DNodeStatus::DeadEndIn(0)
        );
    }
}
