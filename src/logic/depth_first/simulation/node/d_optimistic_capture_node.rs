use super::{DNode, DNodeStatus};
use crate::logic::depth_first::{
    game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
    simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
};
use arrayvec::ArrayVec;
use std::{cell::Cell, cmp::Ordering, fmt::Display};

pub struct DOptimisticCaptureNode {
    id: DNodeId,
    state: DGameState<DSlowField>,
    time: DTreeTime,
    status: Cell<DNodeStatus>,
}

impl DOptimisticCaptureNode {
    pub fn new(
        id: DNodeId,
        state: DGameState<DSlowField>,
        time: DTreeTime,
        status: DNodeStatus,
    ) -> Self {
        Self {
            id,
            state,
            time,
            status: Cell::new(status),
        }
    }

    fn calc_child(&self, direction: DDirection) -> Self {
        let moves = [Some(direction), None, None, None];
        let mut new_id = self.id.clone();
        new_id.push(direction);
        let mut new_state = self.state.clone();
        new_state
            .next_state(moves)
            .move_reachable(moves, new_id.len() as u8);
        let status = match new_state.get_alive()[0] {
            true => DNodeStatus::Alive,
            false => DNodeStatus::Dead,
        };
        Self::new(new_id, new_state, self.time.clone(), status)
    }

    fn calc_moves(&self) -> ArrayVec<DDirection, 4> {
        let turn = self.id.len() as u8 + 1;
        self.state.scope_moves_optimistic(turn)
    }
}

impl DNode for DOptimisticCaptureNode {
    fn id(&self) -> &DNodeId {
        &self.id
    }

    fn status(&self) -> DNodeStatus {
        match self.status.get() {
            DNodeStatus::Unknown => {
                if self.state.get_alive()[0] {
                    self.status.set(DNodeStatus::Alive);
                } else {
                    self.status.set(DNodeStatus::Dead);
                }
            }
            _ => (),
        }
        self.status.get()
    }

    fn calc_children(&self) -> Vec<Box<Self>> {
        self.calc_moves()
            .into_iter()
            .map(|direction| Box::new(self.calc_child(direction)))
            .collect()
    }
}

impl Ord for DOptimisticCaptureNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DOptimisticCaptureNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DOptimisticCaptureNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DOptimisticCaptureNode {}

impl Display for DOptimisticCaptureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id)?;
        writeln!(f, "{}", self.state)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DOptimisticCaptureNode;
    use crate::{
        logic::depth_first::{
            game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
            simulation::{
                d_node_id::DNodeId,
                d_tree::DTreeTime,
                node::{DNode, DNodeStatus},
            },
        },
        read_game_state,
    };

    #[test]
    fn test_calc_child() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DOptimisticCaptureNode::new(
            DNodeId::default(),
            gamestate,
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        println!("{}", node);
        let child_up = node.calc_child(DDirection::Up);
        println!("{}", child_up);
        assert_eq!(child_up.status(), DNodeStatus::Alive);
        let child_left = node.calc_child(DDirection::Left);
        assert_eq!(child_left.status(), DNodeStatus::Dead);
    }

    #[test]
    fn test_calc_moves() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DOptimisticCaptureNode::new(
            DNodeId::default(),
            gamestate,
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        let moves = node.calc_moves();
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&DDirection::Up));
        assert!(moves.contains(&DDirection::Right));
        let new_node = node
            .calc_child(DDirection::Up)
            .calc_child(DDirection::Right)
            .calc_child(DDirection::Right)
            .calc_child(DDirection::Right);
        println!("{}", new_node);
        let moves = new_node.calc_moves();
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&DDirection::Down));
        assert!(moves.contains(&DDirection::Right));
        assert!(moves.contains(&DDirection::Up));
        let new_node = new_node.calc_child(DDirection::Right);
        println!("{}", new_node);
        let moves = new_node.calc_moves();
        assert_eq!(moves.len(), 0);
    }
}
