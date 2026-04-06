use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

use log::{debug, trace};

mod tree_stats;

use crate::logic::{
    game::{direction::Direction, field::BasicField, game_state::GameState, snakes::SNAKES},
    single_gamestate_nodes::node::{Node, NodeStatus, SimulationResult, node_id::NodeId},
};

#[derive(Clone)]
pub struct Tree {
    pub(super) nodes: HashMap<NodeId, Node>,
    pub(super) queue: DepthQueue,
    pub(super) elapsed: Duration,
    max_depth: u8,
    max_time: Option<Duration>,
    max_nodes: usize,
    dead_ancestor_pruning: bool,
    all_root_directions: bool,
    similarity_distance_fn: Option<fn(u8) -> u8>,
    fast_track_fn: Option<Rc<dyn Fn(&Node) -> Option<[Option<Direction>; SNAKES]>>>,
}

impl Tree {
    pub const MAX_DEPTH: u8 = NodeId::MAX_DEPTH;

    pub fn new(root: GameState<BasicField>) -> Self {
        let node = Node::new(NodeId::new(), root);
        let queue = DepthQueue::from(node.id());
        let nodes = HashMap::from([(node.id(), node)]);
        Self {
            nodes,
            queue,
            max_depth: NodeId::MAX_DEPTH,
            max_time: None,
            max_nodes: usize::MAX,
            elapsed: Duration::ZERO,
            dead_ancestor_pruning: false,
            all_root_directions: false,
            similarity_distance_fn: None,
            fast_track_fn: None,
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

    pub fn max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = max_nodes;
        self
    }

    pub fn dead_ancestor_pruning(mut self) -> Self {
        self.dead_ancestor_pruning = true;
        self
    }

    pub fn similarity_pruning(mut self, distance_fn: fn(u8) -> u8) -> Self {
        self.similarity_distance_fn = Some(distance_fn);
        self
    }

    pub fn all_root_directions(mut self) -> Self {
        self.all_root_directions = true;
        self
    }

    pub fn fast_track(
        mut self,
        fast_track_fn: impl Fn(&Node) -> Option<[Option<Direction>; SNAKES]> + 'static,
    ) -> Self {
        self.fast_track_fn = Some(Rc::new(fast_track_fn));
        self
    }

    pub fn result(&self) -> [NodeStatus; 4] {
        let root = self.nodes.get(&NodeId::new()).unwrap();
        [
            root.direction_status(Direction::Up),
            root.direction_status(Direction::Down),
            root.direction_status(Direction::Left),
            root.direction_status(Direction::Right),
        ]
    }

    pub fn simulate(&mut self) {
        let start = Instant::now();
        let deadline = self.max_time.map(|d| Instant::now() + d);
        // Get next node to simulate and check early termination conditions

        if self.all_root_directions {
            let root_id = self.queue.pop().unwrap();
            while self.simulate_node(root_id) {
                // Keep simulating the root until all directions are exhausted. This ensures we have status information for all root directions, which is important for testing and debugging, even if we won't explore all of them in a real simulation due to time/depth constraints.
            }
        }

        while let Some(node_id) = self.queue.pop() {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                debug!("Reached time limit, stopping simulation");
                break;
            }
            if self.nodes.len() >= self.max_nodes {
                debug!("Reached node limit, stopping simulation");
                break;
            }
            if node_id.depth() >= self.max_depth {
                debug!("Pruning {} because of max depth", node_id);
                self.nodes
                    .get_mut(&node_id)
                    .unwrap()
                    .pin_status(NodeStatus::PrunedMaxDepth);
                self.propagate_status(node_id, NodeStatus::PrunedMaxDepth);
                continue;
            }
            let node_status = self.nodes.get(&node_id).unwrap().status();
            if self.dead_ancestor_pruning
                && !matches!(node_status, NodeStatus::DeadIn(_))
                && let Some((ancestor_id, ancestor_direction_status, direction)) =
                    self.dead_ancestor_direction(node_id)
            {
                debug!(
                    "Pruning {} as ancestor {} has direction status {} for direction {}",
                    node_id, ancestor_id, ancestor_direction_status, direction
                );
                self.nodes
                    .get_mut(&node_id)
                    .unwrap()
                    .pin_status(NodeStatus::PrunedDeadAncestor);
                self.propagate_status(node_id, NodeStatus::PrunedDeadAncestor);
                if let Some(parent_id) = node_id.parent() {
                    trace!(
                        "Adding parent {} to the queue for dead ancestor pruning",
                        parent_id
                    );
                    self.queue.push(parent_id);
                }
                continue;
            }
            self.simulate_node(node_id);
        }
        self.elapsed = start.elapsed();
    }

    fn simulate_node(&mut self, node_id: NodeId) -> bool {
        debug!("Simulating {}", node_id);
        let similarity_distance = self
            .similarity_distance_fn
            .as_ref()
            .map(|f| f(node_id.depth()));
        let node = self.nodes.get_mut(&node_id).unwrap();
        let simulation_result = node.simulate(similarity_distance, self.fast_track_fn.as_deref());
        let node_status = node.status();
        let node_is_fast_tracked = node.is_fast_tracked();
        self.propagate_status(node_id, node_status);
        match simulation_result {
            (_, SimulationResult::NoChildren) => {
                debug!("{} has spawned no children", node_id);
                // No children for this direction, reque the node itself to simulate the next direction
                if node_is_fast_tracked {
                    trace!("Fast Tracked: Adding {} to the front of queue", node_id);
                    self.queue.push_front(node_id);
                } else {
                    trace!("Adding {} to the queue", node_id);
                    self.queue.push(node_id);
                }
                true
            }
            (children, SimulationResult::Normal) => {
                debug!("{} has spawned {} children", node_id, children.len());
                for child in children {
                    let child_id = child.id();
                    if child.is_fast_tracked() {
                        trace!(
                            "Fast Tracked: Adding child {} to the front of queue",
                            child_id
                        );
                        self.queue.push_front(child_id);
                    } else {
                        trace!("Adding child {} to the queue", child_id);
                        self.queue.push(child_id);
                    }
                    self.nodes.insert(child_id, child);
                }
                true
            }
            (_, SimulationResult::Exhausted) => {
                // All directions exhausted. Go one level up to simulate the next direction of the parent
                debug!("{} has exhausted all directions", node_id);
                if let Some(parent_id) = node_id.parent()
                    && matches!(node_status, NodeStatus::DeadIn(_))
                {
                    if node_is_fast_tracked {
                        trace!(
                            "Fast Tracked: Adding parent {} to the front of queue",
                            parent_id
                        );
                        self.queue.push_front(parent_id);
                    } else {
                        trace!("Adding parent {} to the queue", parent_id);
                        self.queue.push(parent_id);
                    }
                }
                false
            }
        }
    }

    fn propagate_status(&mut self, node_id: NodeId, node_status: NodeStatus) {
        let mut changing_node_id = node_id;
        let mut node_status = node_status;
        while let Some(parent_id) = changing_node_id.parent() {
            trace!(
                "Propagating child status {} to parent {}",
                node_status, parent_id
            );
            let parent = self.nodes.get_mut(&parent_id).unwrap();
            if parent.propagate_update_from_child(changing_node_id, node_status) {
                changing_node_id = parent_id;
                node_status = parent.status();
                trace!("Status for {} updated to {}", parent_id, node_status);
            } else {
                trace!("Status for {} unchanged {}", parent_id, parent.status());
                break;
            }
        }
    }

    fn dead_ancestor_direction(&self, node_id: NodeId) -> Option<(NodeId, NodeStatus, Direction)> {
        let mut id = node_id;
        while let Some(parent_id) = id.parent() {
            if let Some(parent) = self.nodes.get(&parent_id) {
                let direction = id.last_direction_for(0).unwrap().unwrap();
                let parent_direction_status = parent.direction_status(direction);
                if matches!(
                    parent_direction_status,
                    NodeStatus::DeadIn(_) | NodeStatus::PrunedDeadAncestor
                ) {
                    return Some((parent_id, parent_direction_status, direction));
                }
            }
            id = parent_id;
        }
        None
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
                match (status_a.is_comparable(), status_b.is_comparable()) {
                    (true, false) => return std::cmp::Ordering::Less,
                    (false, true) => return std::cmp::Ordering::Greater,
                    (false, false) => return std::cmp::Ordering::Equal,
                    _ => status_a.partial_cmp(&status_b).unwrap(),
                }
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

#[derive(Clone)]
pub(super) struct DepthQueue {
    buckets: BTreeMap<u8, VecDeque<NodeId>>,
}

impl DepthQueue {
    fn new() -> Self {
        Self {
            buckets: BTreeMap::new(),
        }
    }

    fn from(id: NodeId) -> Self {
        let mut q = Self::new();
        q.push(id);
        q
    }

    fn push(&mut self, id: NodeId) {
        self.buckets.entry(id.depth()).or_default().push_back(id);
    }

    fn push_front(&mut self, id: NodeId) {
        self.buckets.entry(0).or_default().push_front(id);
    }

    fn pop(&mut self) -> Option<NodeId> {
        let (&depth, queue) = self.buckets.iter_mut().next()?;
        let id = queue.pop_front();
        if queue.is_empty() {
            self.buckets.remove(&depth);
        }
        id
    }

    fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }

    pub(super) fn len(&self) -> usize {
        self.buckets.values().map(|q| q.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;
    use crate::{
        logic::{
            game::{direction::DIRECTIONS, snake::Snake},
            single_gamestate_nodes::situation::{Situation, SituationMatch},
        },
        read_game_state,
    };

    pub(super) fn create_tree_from_gamestate(filename: &str) -> Tree {
        let gamestate = read_game_state(filename);
        let root = GameState::<BasicField>::from(&gamestate);
        Tree::new(root)
    }

    fn test_against_base_simulation(
        tree_configurator: impl Fn(Tree) -> Tree,
        tree_comparator: impl Fn(&Tree, &Tree, &str) -> (),
    ) {
        let test_gamestates = vec![
            "requests/failure_1.json",
            "requests/failure_2.json",
            "requests/failure_3.json",
            "requests/failure_4.json",
            "requests/failure_5.json",
        ];
        for filename in test_gamestates {
            let mut base_tree = create_tree_from_gamestate(filename).max_depth(4);
            let mut test_tree = tree_configurator(base_tree.clone());
            base_tree.simulate();
            test_tree.simulate();
            tree_comparator(&base_tree, &test_tree, filename);
        }
    }

    #[test]
    fn correct_tree_state_propagation() {
        let mut tree = create_tree_from_gamestate("requests/failure_1.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(root.direction_status(Direction::Up), NodeStatus::DeadIn(0));
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::AliveFor(3)
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::NotSimulated
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::NotSimulated
        );

        let mut tree = create_tree_from_gamestate("requests/failure_2.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(
            root.direction_status(Direction::Up),
            NodeStatus::AliveFor(3)
        );
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::NotSimulated
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::NotSimulated
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::NotSimulated
        );

        let mut tree = create_tree_from_gamestate("requests/failure_3.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(root.direction_status(Direction::Up), NodeStatus::DeadIn(3));
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::AliveFor(3)
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::DeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::DeadIn(0)
        );

        let mut tree = create_tree_from_gamestate("requests/failure_4.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(root.direction_status(Direction::Up), NodeStatus::DeadIn(3));
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::DeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::AliveFor(3)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::DeadIn(0)
        );

        let mut tree = create_tree_from_gamestate("requests/failure_5.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::DeadIn(2));
        assert_eq!(root.direction_status(Direction::Up), NodeStatus::DeadIn(1));
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::DeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::DeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::DeadIn(0)
        );
    }

    #[test]
    fn option_all_root_directions() {
        test_against_base_simulation(
            |tree| tree.all_root_directions(),
            |baseline_tree, tree, filename| {
                let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                assert!(
                    (!matches!(
                        root.direction_status(Direction::Up),
                        NodeStatus::NotSimulated
                    ) && !matches!(
                        root.direction_status(Direction::Down),
                        NodeStatus::NotSimulated
                    ) && !matches!(
                        root.direction_status(Direction::Left),
                        NodeStatus::NotSimulated
                    ) && !matches!(
                        root.direction_status(Direction::Right),
                        NodeStatus::NotSimulated
                    )),
                    "All root directions should be simulated for {}",
                    filename
                );
                // If direction is simulated in baseline, status should match
                let baseline_root = baseline_tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                for i in DIRECTIONS.into_iter() {
                    let baseline_status = baseline_root.direction_status(i);
                    if baseline_status != NodeStatus::NotSimulated {
                        let status = root.direction_status(i);
                        assert_eq!(
                            status, baseline_status,
                            "Direction {} should have same status as baseline for {}",
                            i, filename
                        );
                    };
                }
            },
        );
    }

    #[test]
    fn option_dead_ancestor_pruning() {
        test_against_base_simulation(
            |tree| tree.dead_ancestor_pruning(),
            |baseline_tree, tree, filename| {
                let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                let baseline_root = baseline_tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                assert_eq!(
                    root.status(),
                    baseline_root.status(),
                    "Root status should be same as baseline for {}",
                    filename
                );
                for i in DIRECTIONS.into_iter() {
                    assert_eq!(
                        root.direction_status(i),
                        baseline_root.direction_status(i),
                        "Root direction {} should have same status as baseline for {}",
                        i,
                        filename
                    );
                }
            },
        );
    }

    #[test]
    fn option_similarity_pruning() {
        test_against_base_simulation(
            |tree| tree.similarity_pruning(|_depth| 6),
            |baseline_tree, tree, filename| {
                let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                let baseline_root = baseline_tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                assert_eq!(
                    root.status(),
                    baseline_root.status(),
                    "Root status should be same as baseline for {}",
                    filename
                );
                for i in DIRECTIONS.into_iter() {
                    assert_eq!(
                        root.direction_status(i),
                        baseline_root.direction_status(i),
                        "Root direction {} should have same status as baseline for {}",
                        i,
                        filename
                    );
                }
            },
        );
    }

    #[test]
    fn option_fast_track() {
        let situation = Rc::new(
            Situation::multi_recommending(
                "
                W . *
                W A .
                W N B
                ",
                [Some(Direction::Up), Some(Direction::Up), None, None],
            )
            .full_symmetry()
            .condition(|snakes| {
                if let [
                    Snake::Alive { length: a, .. },
                    Snake::Alive { length: b, .. },
                    _,
                    _,
                ] = snakes
                {
                    a <= b
                } else {
                    false
                }
            }),
        );
        test_against_base_simulation(
            |tree| {
                let situation = situation.clone();
                tree.fast_track(move |node| {
                    if let Some(SituationMatch::Recommend(dirs)) = situation.check(node.gamestate())
                    {
                        return Some(dirs);
                    } else {
                        return None;
                    }
                })
            },
            |baseline_tree, tree, filename| {
                let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                let baseline_root = baseline_tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
                assert_eq!(
                    root.status(),
                    baseline_root.status(),
                    "Root status should be same as baseline for {}",
                    filename
                );
                for i in DIRECTIONS.into_iter() {
                    assert_eq!(
                        root.direction_status(i),
                        baseline_root.direction_status(i),
                        "Root direction {} should have same status as baseline for {}",
                        i,
                        filename
                    );
                }
            },
        );
        let mut tree = create_tree_from_gamestate(
            "requests/failure_43_going_down_guarantees_getting_killed.json",
        )
        .all_root_directions()
        .dead_ancestor_pruning()
        .similarity_pruning(|_| 6)
        .fast_track(move |node| {
            if let Some(SituationMatch::Recommend(dirs)) = situation.check(node.gamestate()) {
                info!("Fast tracking {}", node.id());
                return Some(dirs);
            } else {
                return None;
            }
        })
        .max_time(Duration::from_millis(200));
        tree.simulate();
        assert_eq!(tree.result()[1], NodeStatus::DeadIn(7));
    }

    #[test]
    fn display_tree() {
        let situation = Rc::new(
            Situation::multi_recommending(
                "
                W . .
                W A .
                W N B
                ",
                [Some(Direction::Up), Some(Direction::Up), None, None],
            )
            .full_symmetry()
            .condition(|snakes| {
                if let [
                    Snake::Alive { length: a, .. },
                    Snake::Alive { length: b, .. },
                    _,
                    _,
                ] = snakes
                {
                    a <= b
                } else {
                    false
                }
            }),
        );

        let mut tree = create_tree_from_gamestate("requests/failure_1.json")
            .all_root_directions()
            .dead_ancestor_pruning()
            .similarity_pruning(|_| 6)
            .fast_track(move |node| {
                if let Some(SituationMatch::Recommend(dirs)) = situation.check(node.gamestate()) {
                    return Some(dirs);
                } else {
                    return None;
                }
            })
            .max_time(Duration::from_millis(200));
        tree.simulate();
        // println!("{}", tree);
        println!("{}", tree.stats());
        println!("{}", tree.nodes.get(&"ROOT".try_into().unwrap()).unwrap());
    }
}
