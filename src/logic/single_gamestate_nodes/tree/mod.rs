use crate::logic::{
    general::{direction::Direction, field::BasicField, game_state::GameState, moves::Moves}, single_gamestate_nodes::node::{Node, NodeStatus, node_id::NodeId},
};
use log::{debug, trace};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt,
    rc::Rc,
    time::{Duration, Instant},
};

mod tree_stats;

#[derive(Clone)]
pub struct Tree {
    pub(super) nodes: HashMap<NodeId, Node>,
    pub(super) queue: PriorityQueue,
    pub(super) elapsed_simulation_time: Duration,
    max_depth: u8,
    max_time: Option<Duration>,
    max_nodes: usize,
    all_root_directions: bool,
    similarity_distance_fn: Option<fn(u8) -> u8>,
    fast_track_fn: Option<Rc<dyn Fn(&Node) -> Option<Moves>>>,
}

impl Tree {
    pub fn new(root: GameState<BasicField>) -> Self {
        let node = Node::new(NodeId::new(), root);
        let queue = PriorityQueue::from(node.id());
        let nodes = HashMap::from([(node.id(), node)]);
        Self {
            nodes,
            queue,
            max_depth: NodeId::MAX_DEPTH,
            max_time: None,
            max_nodes: usize::MAX,
            elapsed_simulation_time: Duration::ZERO,
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
        fast_track_fn: impl Fn(&Node) -> Option<Moves> + 'static,
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

        // Simulate all root directions first once
        if self.all_root_directions {
            let root_id = self.queue.pop().unwrap();
            while self.simulate_node(root_id) {}
        }

        // Simulate nodes according to queue
        while let Some(node_id) = self.queue.pop() {
            let node_status = self.nodes.get(&node_id).unwrap().status();

            // Simulation stopping conditions
            if deadline.is_some_and(|d| Instant::now() >= d) {
                debug!("Reached time limit, stopping simulation");
                break;
            } else if self.nodes.len() >= self.max_nodes {
                debug!("Reached node limit, stopping simulation");
                break;
            }

            // Node skipping conditions
            if matches!(node_status, NodeStatus::WinnerIn(0)) {
                debug!("Skipping {} because it is a {}", node_id, node_status);
                continue;
            } else if node_id.depth() >= self.max_depth {
                debug!("Pruning {} because of max depth", node_id);
                self.nodes
                    .get_mut(&node_id)
                    .unwrap()
                    .pin_status(NodeStatus::PrunedMaxDepth);
                self.propagate_status(node_id, NodeStatus::PrunedMaxDepth);
                continue;
            }

            // Simulate the node
            self.simulate_node(node_id);
        }
        self.elapsed_simulation_time = start.elapsed();
    }

    fn simulate_node(&mut self, node_id: NodeId) -> bool {
        let node = self.nodes.get_mut(&node_id).unwrap();
        let node_priority = node.read_priority();
        debug!(
            "{} -> Simulating with priority {} at depth {}",
            node_id,
            node_priority,
            node_id.depth()
        );

        // Determine used simularity distance for this node based on its depth
        let similarity_pruning_distance = self
            .similarity_distance_fn
            .as_ref()
            .map(|f| f(node_id.depth()));

        let child_nodes = node.simulate(similarity_pruning_distance);
        let node_status = node.status();
        self.propagate_status(node_id, node_status);

        match child_nodes {
            Some(mut children) => {
                debug!("{} -> Spawned {} children", node_id, children.len());

                self.set_children_priorities(node_status, node_priority, &mut children);

                // Add children to the queue and nodes map
                for child in children.into_iter() {
                    let child_id = child.id();
                    let child_priority = child.read_priority();
                    self.queue.push(child_id, child_priority);
                    self.nodes.insert(child_id, child);
                }
                true
            }
            None => {
                debug!("{} has exhausted all directions", node_id);
                if let Some(parent_node_id) = node_id.parent() {
                    let parent_priority = self.nodes.get(&parent_node_id).unwrap().read_priority();
                    self.queue.push(parent_node_id, parent_priority);
                }
                false
            }
        }
    }

    fn set_children_priorities(
        &mut self,
        _parent_status: NodeStatus,
        parent_priority: i8,
        children: &mut Vec<Node>,
    ) {
        // If the parent has a positive priority, decay it and assign to children
        if parent_priority > 0 {
            for child in children.iter_mut() {
                child.set_priority(parent_priority - 1);
            }
        }

        if children.len() == 1 {
            children[0].set_priority(2);
        }
        // If a fast track function is defined, use it to set priorities and simulated snakes for children
        else if let Some(fast_track_fn) = self.fast_track_fn.as_ref() {
            for child in children.iter_mut() {
                if let Some(moves) = fast_track_fn(&child) {
                    child.set_priority(2);
                    child.set_moves(moves);
                }
            }
        }
    }

    fn propagate_status(&mut self, node_id: NodeId, node_status: NodeStatus) {
        let mut current_node_id = node_id;
        let mut current_node_status = node_status;

        while let Some(parent_node_id) = current_node_id.parent() {
            trace!(
                "Propagating child status {} to parent {}",
                current_node_status, parent_node_id
            );
            let parent_node = self.nodes.get_mut(&parent_node_id).unwrap();

            // Update and propagate only if the parent node's status has changed
            if parent_node.handle_update_from_child(current_node_id, current_node_status) {
                current_node_id = parent_node_id;
                current_node_status = parent_node.status();
                trace!(
                    "Status for {} updated to {}",
                    parent_node_id, current_node_status
                );
                continue;
            } else {
                trace!(
                    "Status for {} unchanged {}",
                    parent_node_id,
                    parent_node.status()
                );
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
pub(super) struct PriorityQueue {
    buckets: BTreeMap<(i8, u8), VecDeque<NodeId>>,
}

impl PriorityQueue {
    fn new() -> Self {
        Self {
            buckets: BTreeMap::new(),
        }
    }

    fn from(id: NodeId) -> Self {
        let mut q = Self::new();
        q.push(id, 0);
        q
    }

    fn push(&mut self, id: NodeId, priority: i8) {
        self.buckets
            .entry((-priority, id.depth()))
            .or_default()
            .push_back(id);
    }

    fn pop(&mut self) -> Option<NodeId> {
        let (&(priority, depth), queue) = self.buckets.iter_mut().next()?;
        let id = queue.pop_front();
        if queue.is_empty() {
            self.buckets.remove(&(priority, depth));
        }
        id
    }

    fn len(&self) -> usize {
        self.buckets.values().map(|q| q.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        logic::{
            general::{direction::DIRECTIONS, snake::Snake},
            single_gamestate_nodes::situation::Situation,
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
            "requests/failure_01.json",
            "requests/failure_02.json",
            "requests/failure_03.json",
            "requests/failure_04.json",
            "requests/failure_05.json",
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
    fn test_priority_queue_ordering() {
        let mut q = PriorityQueue::new();
        q.push(NodeId::from("ROOT"), 0);
        q.push(NodeId::from("UUUU"), 0);
        q.push(NodeId::from("UDDD"), -1);
        q.push(NodeId::from("ULLL"), 1);
        q.push(NodeId::from("UDDD-UDDD"), -1);
        q.push(NodeId::from("UUUU-UUUU"), 0);
        q.push(NodeId::from("ULLL-ULLL"), 1);

        let mut popped = Vec::new();
        while let Some(id) = q.pop() {
            popped.push(id);
        }

        assert_eq!(
            popped,
            vec![
                NodeId::from("ULLL"),
                NodeId::from("ULLL-ULLL"),
                NodeId::from("ROOT"),
                NodeId::from("UUUU"),
                NodeId::from("UUUU-UUUU"),
                NodeId::from("UDDD"),
                NodeId::from("UDDD-UDDD")
            ]
        );
    }

    #[test]
    fn correct_tree_state_propagation_1() {
        let mut tree = create_tree_from_gamestate("requests/failure_01.json").max_depth(4);
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
            NodeStatus::AliveFor(3)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::AliveFor(3)
        );

        let mut tree = create_tree_from_gamestate("requests/failure_02.json").max_depth(4);
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

        let mut tree = create_tree_from_gamestate("requests/failure_03.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(
            root.direction_status(Direction::Up),
            NodeStatus::ProbablyDeadIn(3)
        );
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

        let mut tree = create_tree_from_gamestate("requests/failure_04.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::AliveFor(4));
        assert_eq!(
            root.direction_status(Direction::Up),
            NodeStatus::ProbablyDeadIn(3)
        );
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
            NodeStatus::ProbablyDeadIn(0)
        );

        let mut tree = create_tree_from_gamestate("requests/failure_05.json").max_depth(4);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::ProbablyDeadIn(2));
        assert_eq!(
            root.direction_status(Direction::Up),
            NodeStatus::ProbablyDeadIn(1)
        );
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::ProbablyDeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::DeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::ProbablyDeadIn(0)
        );
    }

    #[test]
    fn correct_tree_state_propagation_2() {
        let mut tree = create_tree_from_gamestate("requests/failure_64.json").max_depth(8);
        tree.simulate();

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        println!("{}", tree.nodes.get(&"RD__".parse().unwrap()).unwrap());
        println!("{}", tree.nodes.get(&"RD__-RD__".parse().unwrap()).unwrap());
        println!("{}", tree.nodes.get(&"RD__-RD__-RL__".parse().unwrap()).unwrap());
        assert_eq!(root.status(), NodeStatus::ProbablyDeadIn(1));
        assert_eq!(root.direction_status(Direction::Up), NodeStatus::DeadIn(0));
        assert_eq!(
            root.direction_status(Direction::Down),
            NodeStatus::DeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::ProbablyDeadIn(0)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::DeadIn(5)
        );
    }

    #[test]
    fn correct_tree_state_propagation_3() {
        let mut tree = create_tree_from_gamestate("requests/editor_1.json").max_depth(4);
        tree.simulate();

        println!("{}", tree);

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        println!("{}", tree.nodes.get(&"RL__".parse().unwrap()).unwrap());
        let winner = tree.nodes.get(&"RL__-RU__".parse().unwrap()).unwrap();
        println!("{}", winner);
        assert_eq!(winner.status(), NodeStatus::WinnerIn(0));
        assert_eq!(root.status(), NodeStatus::WinnerIn(2));
    }

    #[test]
    fn correct_tree_state_propagation_4() {
        let mut tree = create_tree_from_gamestate("requests/failure_69.json").max_depth(3);
        tree.simulate();

        println!("{}", tree);

        let root = tree.nodes.get(&"ROOT".parse().unwrap()).unwrap();
        println!("{}", root);
        assert_eq!(root.status(), NodeStatus::ProbablyDeadIn(1));

        assert_eq!(
            root.direction_status(Direction::Left),
            NodeStatus::DeadIn(2)
        );
        assert_eq!(
            root.direction_status(Direction::Right),
            NodeStatus::ProbablyDeadIn(0)
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
                0.0,
                "Fast Track",
            )
            .condition(
                |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                    (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a <= b,
                    _ => false,
                },
            ),
        );
        test_against_base_simulation(
            |tree| {
                let situation = situation.clone();
                tree.fast_track(move |node| {
                    if let Some(situation_match) = situation.check(node.gamestate()) {
                        Some(*situation_match)
                    } else {
                        None
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
            "requests/failure_43.json",
        )
        .all_root_directions()
        .similarity_pruning(|_| 6)
        .fast_track(move |node| {
            if let Some(situation_match) = situation.check(node.gamestate()) {
                Some(*situation_match)
            } else {
                None
            }
        })
        .max_time(Duration::from_millis(200));
        tree.simulate();
        assert_eq!(tree.result()[1], NodeStatus::ProbablyDeadIn(7));
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
                0.0,
                "Fast Track",
            )
            .condition(
                |snakes| match (snakes.cell(0).get(), snakes.cell(1).get()) {
                    (Snake::Alive { length: a, .. }, Snake::Alive { length: b, .. }) => a <= b,
                    _ => false,
                },
            ),
        );

        let mut tree = create_tree_from_gamestate(
            "requests/failure_43.json",
        )
        .all_root_directions()
        .similarity_pruning(|_| 6)
        .fast_track(move |node| {
            if let Some(situation_match) = situation.check(node.gamestate()) {
                Some(*situation_match)
            } else {
                None
            }
        })
        .max_time(Duration::from_millis(200));
        tree.simulate();
        // println!("{}", tree);
        println!("{}", tree.stats());
        println!("{}", tree.nodes.get(&"ROOT".try_into().unwrap()).unwrap());
        println!(
            "{}",
            tree.nodes.get(&"DUDD-DURR".try_into().unwrap()).unwrap()
        );
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use std::hint::black_box;

    use super::*;
    use crate::read_game_state;

    fn test_gamestates() -> Vec<GameState<BasicField>> {
        [
            "requests/failure_01.json",
            "requests/failure_03.json",
            "requests/failure_04.json",
            "requests/failure_05.json",
            "requests/example_move_request_2.json",
            "requests/example_move_request_3.json",
        ]
        .iter()
        .map(|p| {
            let gamestate = read_game_state(p);
            GameState::<BasicField>::from(&gamestate)
        })
        .collect()
    }

    #[bench]
    fn bench_tree_simulate_max_nodes(b: &mut test::Bencher) {
        let states = test_gamestates();
        let mut i = 0;
        b.iter(|| {
            let mut tree = Tree::new(states[i % states.len()].clone()).max_nodes(10000);
            i += 1;
            black_box(tree.simulate())
        });
    }

    #[bench]
    fn bench_depth_queue_push_pop(b: &mut test::Bencher) {
        let root = NodeId::new();
        // Pre-build a set of ids at varied depths to push/pop each iteration.
        use crate::logic::general::direction::Direction::*;
        let ids: Vec<NodeId> = vec![
            root,
            root.child([Some(Up), Some(Down), Some(Left), Some(Right)]),
            root.child([Some(Down), Some(Up), Some(Right), Some(Left)]),
            root.child([Some(Left), Some(Right), Some(Up), Some(Down)])
                .child([Some(Up), Some(Down), Some(Left), Some(Right)]),
            root.child([Some(Right), Some(Left), Some(Down), Some(Up)])
                .child([Some(Down), Some(Up), Some(Right), Some(Left)])
                .child([Some(Left), Some(Right), Some(Up), Some(Down)]),
        ];
        let mut i = 0;

        b.iter(|| {
            let mut q = PriorityQueue::new();
            q.push(black_box(ids[i % ids.len()]), 0);
            let _ = black_box(q.pop());
            i += 1;
        });
    }
}
