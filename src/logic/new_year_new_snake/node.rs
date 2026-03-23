use core::panic;

use crate::logic::{
    game::{
        direction::Direction,
        field::BasicField,
        game_state::GameState,
        moves::{MoveMatrix, MoveMatrixIter, MoveVector, Moves},
    },
    new_year_new_snake::node_id::NodeId,
};

#[derive(Copy, Clone)]
pub enum NodeStatus {
    AliveFor(u8), // Number of steps where we have checked with guaranteed survival
    DeadIn(u8),   // Number of steps until inevitable death (if opponents play optimally)
}

pub struct Node {
    id: NodeId,
    gamestate: GameState<BasicField>,
    status: NodeStatus,
    directions: [bool; 4],
    children: Vec<NodeId>,
    children_updated: usize,
    children_direction: Option<Direction>,
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
            directions: [false; 4],
            children: Vec::new(),
            children_updated: 0,
            children_direction: None,
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

    pub fn simulate(&mut self) -> Vec<Node> {
        while let Some(move_matrix) = self.next_moveset() {
            let mut children = Vec::new();
            for moves in move_matrix {
                let mut child_gamestate = self.gamestate.clone();
                child_gamestate.next_state(moves);
                if child_gamestate.is_alive(0) {
                    let child_id = self.id.child(moves);
                    self.children.push(child_id);
                    let node = Node::new(child_id, child_gamestate);
                    children.push(node);
                } else {
                    self.children_direction = None;
                    self.children = Vec::new();
                    self.status = NodeStatus::DeadIn(1);
                    break;
                }
            }
            self.status = NodeStatus::AliveFor(1);
            return children;
        }
        Vec::new()
    }

    pub fn update_from_child(&mut self, child_id: NodeId, child_status: NodeStatus) -> bool {
        let last_decision = child_id.last_decision();
        if last_decision != self.children_direction {
            panic!(
                "Invalid child ID: expected last decision {:?}, got {:?}",
                self.children_direction, last_decision
            );
        }
        match (self.status, child_status) {
            (NodeStatus::AliveFor(n), NodeStatus::AliveFor(m)) => {
                if n + 1 == m {
                    self.children_updated += 1;
                    if self.children_updated == self.children.len() {
                        self.status = NodeStatus::AliveFor(n + 1);
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    panic!(
                        "Invalid status update: parent is AliveFor({}), child is AliveFor({})",
                        n, m
                    );
                }
            }
            (NodeStatus::AliveFor(_), NodeStatus::DeadIn(m)) => {
                self.status = NodeStatus::DeadIn(m + 1);
                return true;
            }
            _ => return false,
        }
    }

    fn next_moveset(&mut self) -> Option<MoveMatrix> {
        let mut move_matrix = self.gamestate.valid_moves();
        let directions = move_matrix.get(0).unwrap();
        for i in 0..4 {
            if !self.directions[i] {
                self.directions[i] = true;
                if directions[i] {
                    let direction = Direction::try_from(i).unwrap();
                    self.children_direction = Some(direction);
                    let new_move = MoveVector::from(direction);
                    move_matrix.set(0, new_move);
                    return Some(move_matrix);
                }
            }
        }
        None
    }
}
