use core::panic;
use std::fmt::Display;

use crate::logic::{
    game::{
        direction::{self, Direction},
        field::BasicField,
        game_state::GameState,
        moves::{MoveMatrix, MoveMatrixIter, MoveVector, Moves},
    },
    new_year_new_snake::node_id::NodeId,
};

#[derive(Copy, Clone, Debug)]
pub enum NodeStatus {
    AliveFor(u8), // Number of steps where we have checked with guaranteed survival
    DeadIn(u8),   // Number of steps until inevitable death (if opponents play optimally)
}

impl NodeStatus {
    pub fn improve(self) -> NodeStatus {
        match self {
            NodeStatus::AliveFor(n) => NodeStatus::AliveFor(n + 1),
            NodeStatus::DeadIn(n) => NodeStatus::DeadIn(n + 1),
        }
    }
}

impl Eq for NodeStatus {}

impl PartialEq for NodeStatus {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for NodeStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for NodeStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (NodeStatus::AliveFor(n), NodeStatus::AliveFor(m)) => n.partial_cmp(m),
            (NodeStatus::DeadIn(n), NodeStatus::DeadIn(m)) => m.partial_cmp(n),
            (NodeStatus::AliveFor(_), NodeStatus::DeadIn(_)) => Some(std::cmp::Ordering::Greater),
            (NodeStatus::DeadIn(_), NodeStatus::AliveFor(_)) => Some(std::cmp::Ordering::Less),
        }
    }
}

impl Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::AliveFor(n) => write!(f, "AliveFor({})", n),
            NodeStatus::DeadIn(n) => write!(f, "DeadIn({})", n),
        }
    }
}

pub struct Node {
    id: NodeId,
    gamestate: GameState<BasicField>,
    status: NodeStatus,
    children: [Option<(NodeStatus, Vec<(NodeId, NodeStatus)>)>; 4],
}

impl Node {
    pub fn new(id: NodeId, gamestate: GameState<BasicField>) -> Self {
        let status = if gamestate.is_alive(0) {
            NodeStatus::AliveFor(0)
        } else {
            NodeStatus::DeadIn(0)
        };
        Self {
            id,
            gamestate,
            status,
            children: [None, None, None, None],
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn status(&self) -> NodeStatus {
        self.status
    }

    pub fn gamestate(&self) -> &GameState<BasicField> {
        &self.gamestate
    }

    /// Returns None if all directions have been simulated
    pub fn simulate(&mut self) -> Option<Vec<Node>> {
        if let Some(move_matrix) = self.next_moveset() {
            let mut children = Vec::new();
            let direction: Direction = move_matrix.get(0).try_into().unwrap();
            for moves in move_matrix {
                let mut child_gamestate = self.gamestate.clone();
                child_gamestate.next_state(moves);
                let child = Node::new(self.id.child(moves), child_gamestate);
                let child_id = child.id();
                let child_status = child.status();
                children.push(child);
                self.children[direction as usize]
                    .as_mut()
                    .map(|(_, child_vec)| child_vec.push((child_id, child_status)));
                match child_status {
                    NodeStatus::AliveFor(0) => {}
                    NodeStatus::DeadIn(0) => {
                        self.children[direction as usize]
                            .as_mut()
                            .map(|(direction_status, _)| {
                                *direction_status = NodeStatus::DeadIn(0);
                            });
                        self.status = self.status.max(NodeStatus::DeadIn(1));
                        // Do not return children as this direction is already dead
                        return Some(Vec::new());
                    }
                    _ => {
                        panic!("Invalid child status: {}", child_status);
                    }
                }
            }
            debug_assert!(!children.is_empty()); // Node must spawn children if it is alive
            self.status = self.status.max(NodeStatus::AliveFor(1));
            return Some(children);
        }
        None
    }

    pub fn update_from_child(&mut self, child_id: NodeId, child_status: NodeStatus) -> bool {
        let last_move = child_id.last_move_for(0).unwrap();
        let best_direction_status = self
            .children
            .iter()
            .filter_map(|x| {
                if let Some((status, _)) = x {
                    Some(*status)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(NodeStatus::DeadIn(0));
        self.children[last_move as usize]
            .as_mut()
            .map(|(direction_status, child_vec)| {
                let mut worst_child_status = child_status;
                for (id, status) in child_vec.iter_mut() {
                    if *id == child_id {
                        *status = child_status;
                    }
                    if *status < worst_child_status {
                        worst_child_status = *status;
                    }
                }
                if worst_child_status != *direction_status {
                    *direction_status = worst_child_status;
                }
            });
        let best_direction_status_again = self
            .children
            .iter()
            .filter_map(|x| {
                if let Some((status, _)) = x {
                    Some(*status)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(NodeStatus::DeadIn(0));
        if best_direction_status_again != best_direction_status {
            self.status = best_direction_status_again.improve();
            return true;
        } else {
            return false;
        }
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
                    self.children[i] = Some((NodeStatus::AliveFor(0), Vec::new()));
                    return Some(move_matrix);
                } else {
                    self.children[i] = Some((NodeStatus::DeadIn(0), Vec::new()));
                }
            }
        }
        None
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.id, self.status)?;
        writeln!(f, "{}", self.gamestate)?;
        for (i, slot) in self.children.iter().enumerate() {
            let dir = Direction::try_from(i).unwrap();
            match slot {
                None => writeln!(f, "  {}: unexplored", dir)?,
                Some((status, children)) => {
                    writeln!(f, "  {} {} ({} children)", dir, status, children.len())?;
                    for (child_id, child_status) in children {
                        writeln!(f, "    {} {}", child_id, child_status)?;
                    }
                }
            }
        }
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
        // DeadIn: smaller n is better (reversed)
        assert!(NodeStatus::DeadIn(1) > NodeStatus::DeadIn(5));
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
        assert_eq!(statuses.iter().max().unwrap(), &NodeStatus::AliveFor(2));
        assert_eq!(statuses.iter().min().unwrap(), &NodeStatus::DeadIn(5));
    }

    fn make_root_node(json_path: &str) -> Node {
        let gamestate = read_game_state(json_path);
        let state = GameState::<BasicField>::from(&gamestate);
        Node::new(NodeId::new(), state)
    }

    #[test]
    fn simulate_exhausts_all_directions() {
        let mut node = make_root_node("requests/example_move_request.json");
        while node.simulate().is_some() {
            node.simulate();
        }
        // After exhaustion, all children slots should be filled
        assert!(
            node.children.iter().all(|c| c.is_some()),
            "all children slots should be populated after exhaustion"
        );
        // All direction statuses should be AliveFor(0) or DeadIn(0)
        for (i, slot) in node.children.iter().enumerate() {
            let (status, _) = slot.as_ref().unwrap();
            assert!(
                matches!(status, NodeStatus::AliveFor(0) | NodeStatus::DeadIn(0)),
                "direction {} has unexpected status: {}",
                i,
                status
            );
        }
        // Should return empty now
        assert!(node.simulate().is_none());
    }

    #[test]
    fn display_half_simulated_node() {
        let mut node = make_root_node("requests/example_move_request.json");
        // Simulate only the first two directions
        node.simulate();
        node.simulate();
        println!("{}", node);
    }
}
