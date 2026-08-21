use core::panic;
use std::{collections::HashSet, fmt::Display};
use crate::logic::{
    general::{
        direction::{DIRECTIONS, Direction},
        field::BasicField,
        game_state::GameState,
        moves::{MoveMatrix, MoveVector, Moves},
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
    AliveFor(u8), // Number of steps where we have checked with guaranteed survival
    DeadIn(u8),   // Number of steps until certain death
    WinnerIn(u8), // Number of steps until inevitable victory (if opponents play optimally)
    // TODO: Conditional(u8, u8), // Number of steps we could stay alive if x suboptimal opponent moves
    NotSimulated, // Status not yet determined as this direction has not been simulated
    PrunedFromAncestor, // Node was skipped: an ancestor direction is dead
    PrunedMaxDepth, // Node was skipped: max depth reached
    PrunedForSimilarity, // Node was skipped: a similar gamestate is already in the node
}

impl NodeStatus {
    pub fn increment(self) -> NodeStatus {
        match self {
            NodeStatus::AliveFor(n) => NodeStatus::AliveFor(n + 1),
            NodeStatus::DeadIn(n) => NodeStatus::DeadIn(n + 1),
            NodeStatus::WinnerIn(n) => NodeStatus::WinnerIn(n + 1),
            _ => panic!("Cannot increment status: {}", self),
        }
    }

    pub fn is_comparable(self) -> bool {
        matches!(
            self,
            NodeStatus::AliveFor(_) | NodeStatus::DeadIn(_) | NodeStatus::WinnerIn(_)
        )
    }

    pub fn for_comparison(self) -> Option<NodeStatus> {
        if self.is_comparable() {
            Some(self)
        } else {
            None
        }
    }

    // Max part of MinMax, pick best direction status to determine parent status.
    pub fn calculate_from_direction_states(direction_states: &[NodeStatus; 4]) -> NodeStatus {
        let best = direction_states
            .iter()
            .filter_map(|s| s.for_comparison())
            .max_by(|x, y| x.partial_cmp(y).unwrap());

        match best {
            None => NodeStatus::AliveFor(0), // No directions explored yet
            Some(s @ NodeStatus::AliveFor(_)) => s.increment(),
            Some(s @ NodeStatus::WinnerIn(_)) => s.increment(),
            Some(s @ NodeStatus::DeadIn(_)) => {
                if direction_states
                    .iter()
                    .any(|s| matches!(s, NodeStatus::NotSimulated))
                {
                    NodeStatus::AliveFor(0)
                } else {
                    s.increment()
                }
            }
            _ => panic!("Invalid best status: {}", best.unwrap()),
        }
    }

    /// Min part of MinMax, pick worst child status to determine direction status.
    pub fn calculate_from_child_states(child_states: &Vec<(Moves, NodeStatus)>) -> NodeStatus {
        if child_states.is_empty() {
            return NodeStatus::DeadIn(0);
        }

        let worst = child_states
            .iter()
            .filter_map(|(_, s)| s.for_comparison())
            .min_by(|x, y| x.partial_cmp(y).unwrap());

        match worst {
            Some(s @ NodeStatus::AliveFor(_)) => s,
            Some(s @ NodeStatus::DeadIn(_)) => s,
            Some(s @ NodeStatus::WinnerIn(_)) => s,
            None => {
                if child_states
                    .iter()
                    .filter(|(_, status)| !matches!(status, NodeStatus::PrunedForSimilarity))
                    .all(|(_, status)| matches!(status, NodeStatus::PrunedFromAncestor))
                {
                    return NodeStatus::PrunedFromAncestor;
                } else if child_states
                    .iter()
                    .filter(|(_, status)| !matches!(status, NodeStatus::PrunedForSimilarity))
                    .all(|(_, status)| matches!(status, NodeStatus::PrunedMaxDepth))
                {
                    return NodeStatus::AliveFor(0);
                } else {
                    panic!("All children have weird states: {:?}", child_states);
                }
            }
            _ => panic!("Invalid worst status: {}", worst.unwrap()),
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
            (NodeStatus::AliveFor(n), NodeStatus::AliveFor(m)) => Some(n.cmp(m)),
            (NodeStatus::DeadIn(n), NodeStatus::DeadIn(m)) => Some(n.cmp(m)),
            (NodeStatus::WinnerIn(n), NodeStatus::WinnerIn(m)) => Some(m.cmp(n)),

            // Alive, Dead
            (NodeStatus::AliveFor(_), NodeStatus::DeadIn(_)) => Some(std::cmp::Ordering::Greater),
            (a @ NodeStatus::DeadIn(_), b @ NodeStatus::AliveFor(_)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // Alive, Winner
            (NodeStatus::AliveFor(_), NodeStatus::WinnerIn(_)) => Some(std::cmp::Ordering::Less),
            (a @ NodeStatus::WinnerIn(_), b @ NodeStatus::AliveFor(_)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // Dead, Winner
            (NodeStatus::DeadIn(_), NodeStatus::WinnerIn(_)) => Some(std::cmp::Ordering::Less),
            (a @ NodeStatus::WinnerIn(_), b @ NodeStatus::DeadIn(_)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            (NodeStatus::NotSimulated, NodeStatus::NotSimulated) => Some(std::cmp::Ordering::Equal),
            (NodeStatus::PrunedFromAncestor, NodeStatus::PrunedFromAncestor) => {
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
            NodeStatus::PrunedFromAncestor => write!(f, "PrunedDeadAncestor"),
            NodeStatus::PrunedMaxDepth => write!(f, "PrunedMaxDepth"),
            NodeStatus::PrunedForSimilarity => write!(f, "PrunedForSimilarity"),
            NodeStatus::WinnerIn(n) => write!(f, "WinnerIn({})", n),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    id: NodeId,
    gamestate: GameState<BasicField>,
    children_states_per_direction: [Vec<(Moves, NodeStatus)>; 4],
    direction_states: [NodeStatus; 4],
    status: NodeStatus,
    pinned_status: Option<NodeStatus>,
    priority: i8,
    move_matrix: MoveMatrix,
    simulated_snakes: [bool; SNAKES],
}

impl Node {
    pub fn new(id: NodeId, gamestate: GameState<BasicField>) -> Self {
        let status = if !gamestate.is_alive(0) {
            NodeStatus::DeadIn(0)
        } else if gamestate.is_winner(0) {
            NodeStatus::WinnerIn(0)
        } else {
            NodeStatus::AliveFor(0)
        };

        let move_matrix = gamestate.valid_moves();

        Self {
            id,
            gamestate,
            children_states_per_direction: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            direction_states: [NodeStatus::NotSimulated; 4],
            status,
            pinned_status: None,
            priority: 0,
            move_matrix,
            simulated_snakes: [true; SNAKES],
        }
    }

    /// Pin the node to a specific status.
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

    pub fn set_simulated_snakes(&mut self, simulated_snakes: [bool; SNAKES]) {
        self.simulated_snakes = simulated_snakes;
    }

    pub fn set_priority(&mut self, priority: i8) {
        self.priority = priority;
    }

    pub fn read_priority(&self) -> i8 {
        self.priority
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn status(&self) -> NodeStatus {
        if let Some(pinned) = self.pinned_status {
            return pinned;
        } else {
            return self.status;
        }
    }

    pub fn direction_status(&self, direction: Direction) -> NodeStatus {
        if let Some(pinned) = self.pinned_status {
            return pinned;
        } else {
            self.direction_states[direction as usize]
        }
    }

    fn update_direction_status(&mut self, direction_index: usize) -> bool {
        let old_status = self.direction_states[direction_index];
        let children = self.children_states_per_direction[direction_index].as_ref();
        let new_status = NodeStatus::calculate_from_child_states(children);
        self.direction_states[direction_index] = new_status;
        old_status != new_status
    }

    fn update_status(&mut self) -> bool {
        let old_status = self.status;
        self.status = NodeStatus::calculate_from_direction_states(&self.direction_states);
        old_status != self.status
    }

    pub fn handle_update_from_child(&mut self, child_id: NodeId, child_status: NodeStatus) -> bool {
        let last_moves: Moves = child_id.last_directions().unwrap();
        let direction_index = last_moves[0].unwrap() as usize;

        // Find the child entry corresponding to the last moves and update its status
        if let Some(entry) = self.children_states_per_direction[direction_index]
            .iter_mut()
            .find(|(dv, _)| *dv == last_moves)
        {
            entry.1 = child_status;
        }

        // Update the direction status based on the updated child status
        if self.update_direction_status(direction_index) {
            // If the direction status has changed, update the overall node status
            self.update_status()
        } else {
            false
        }
    }

    pub fn gamestate(&self) -> &GameState<BasicField> {
        &self.gamestate
    }

    /// For stats usage only, not for simulation.
    pub fn children(&self) -> [Vec<(NodeId, NodeStatus)>; 4] {
        self.children_states_per_direction.clone().map(|vec| {
            vec.into_iter()
                .map(|(dv, s)| (self.id.child(dv), s))
                .collect()
        })
    }

    /// This method can be called multiple times to simulate the node in a stepwise manner. It will return None when all directions have been simulated.
    pub fn simulate(
        &mut self,
        similarity_pruning_distance: Option<u8>,
    ) -> Option<Vec<Node>> {
        debug_assert!(
            self.status != NodeStatus::WinnerIn(0),
            "Should never simulate a node that is a new winner"
        );

        'direction: while let Some((direction, move_matrix)) = self.next_direction() {
            let mut children: Vec<Node> = Vec::new();
            let mut similarity_set: HashSet<u64> = HashSet::new();

            for moves in move_matrix {
                let mut child_gamestate = self.gamestate.clone();
                let child_id = self.id.child(moves);
                child_gamestate.next_state(moves);

                // Similarity pruning: if a similar gamestate has already been simulated, skip this child
                if let Some(dist) = similarity_pruning_distance {
                    let hash = child_gamestate.local_environment_hash(dist);
                    if !similarity_set.insert(hash) {
                        self.children_states_per_direction[direction as usize]
                            .push((moves, NodeStatus::PrunedForSimilarity));
                        continue;
                    }
                }

                let child = Node::new(child_id, child_gamestate);
                let child_status = child.status();

                self.children_states_per_direction[direction as usize].push((moves, child_status));
                if !matches!(child_status, NodeStatus::DeadIn(_)) {
                    children.push(child);
                }

                match child_status {
                    NodeStatus::DeadIn(0) => {
                        self.update_direction_status(direction.into());
                        continue 'direction;
                    }
                    NodeStatus::AliveFor(0) => {}
                    NodeStatus::WinnerIn(0) => {}
                    _ => {
                        panic!("Invalid child status: {}", child_status);
                    }
                }
            }

            // All children are dead, mark direction as dead
            if children.len() == 0 {
                self.update_direction_status(direction.into());
                continue 'direction;
            }

            // Node must spawn children
            debug_assert!(!children.is_empty());
            self.update_direction_status(direction.into());
            self.update_status();
            return Some(children);
        }
        self.update_status();
        return None;
    }

    fn next_direction(&mut self) -> Option<(Direction, MoveMatrix)> {
        for d in DIRECTIONS {
            if self.direction_states[d as usize] == NodeStatus::NotSimulated {
                if self.move_matrix.get(0).is_valid(d) {
                    let new_move_vector = MoveVector::from(d);
                    let mut stripped_move_matrix = self.move_matrix.clone();
                    stripped_move_matrix.set(0, new_move_vector);
                    return Some((d, stripped_move_matrix));
                } else {
                    self.update_direction_status(d.into());
                }
            }
        }
        self.update_status();
        None
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{} {}", self.id, self.status())?;
        for (i, children) in self.children_states_per_direction.iter().enumerate() {
            let dir = Direction::try_from(i).unwrap();
            let dir_status = self.direction_states[i];
            if dir_status == NodeStatus::NotSimulated {
                writeln!(f, "  {} unexplored", dir)?;
            } else {
                writeln!(f, "  {} {} ({} children)", dir, dir_status, children.len())?;
                for (dv, child_status) in children {
                    writeln!(f, "    {} {}", self.id.child(*dv), child_status)?;
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
        // WinnerIn: Lower n is better because it means we win sooner
        assert!(NodeStatus::WinnerIn(1) > NodeStatus::WinnerIn(3));
        assert!(NodeStatus::WinnerIn(1) > NodeStatus::AliveFor(100));
        assert!(NodeStatus::WinnerIn(1) > NodeStatus::DeadIn(100));
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
        while node.simulate(None).is_some() {
            node.simulate(None);
        }
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
        assert!(node.simulate(None).is_none());
    }

    #[test]
    fn display_half_simulated_node() {
        let mut node = make_root_node("requests/test_game_start.json");
        // Simulate only the first two directions
        node.simulate(None);
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
            black_box(node.simulate(black_box(None)))
        });
    }

    #[bench]
    fn bench_node_status(b: &mut test::Bencher) {
        let nodes: Vec<Node> = test_nodes()
            .into_iter()
            .map(|mut n| {
                n.simulate(None); // explore one direction
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
                let children = parent.simulate(None)?;
                let (child_id, child_status) = children.first().map(|c| (c.id(), c.status()))?;
                Some((parent, child_id, child_status))
            })
            .collect();

        let mut i = 0;
        b.iter(|| {
            let (parent, child_id, child_status) = &prepared[i % prepared.len()];
            i += 1;
            let mut node = parent.clone();
            black_box(node.handle_update_from_child(black_box(*child_id), black_box(*child_status)))
        });
    }
}
