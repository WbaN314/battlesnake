use super::{DNode, DNodeError};
use crate::logic::depth_first::{
    game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
    simulation::d_node_id::DNodeId,
};
use arrayvec::ArrayVec;
use std::fmt::Display;

pub struct DOptimisticCaptureNode {
    id: DNodeId,
    state: DGameState<DSlowField>,
}

impl DOptimisticCaptureNode {
    pub fn new(id: DNodeId, state: DGameState<DSlowField>) -> Self {
        Self { id, state }
    }
}

impl DNode for DOptimisticCaptureNode {
    fn id(&self) -> &DNodeId {
        &self.id
    }

    fn is_alive(&self) -> bool {
        self.state.is_alive()
    }

    fn calc_child(&self, direction: DDirection) -> Result<Box<Self>, DNodeError> {
        let moves = [Some(direction), None, None, None];
        let mut new_id = self.id.clone();
        new_id.push(direction);
        let mut new_state = self.state.clone();
        new_state
            .next_state(moves)
            .move_reachable(moves, new_id.len() as u8);
        if new_state.is_alive() {
            return Ok(Box::new(Self::new(new_id, new_state)));
        } else {
            return Err(DNodeError::Dead);
        }
    }

    fn calc_moves(&self) -> ArrayVec<DDirection, 4> {
        let turn = self.id.len() as u8 + 1;
        self.state.scope_moves_optimistic(turn)
    }
}

impl Display for DOptimisticCaptureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;
        write!(f, "{}", self.state)?;
        Ok(())
    }
}

impl Ord for DOptimisticCaptureNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DOptimisticCaptureNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DOptimisticCaptureNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DOptimisticCaptureNode {}

#[cfg(test)]
mod tests {
    use super::DOptimisticCaptureNode;
    use crate::{
        logic::depth_first::{
            game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
            simulation::{d_node_id::DNodeId, node::DNode},
        },
        read_game_state,
    };

    #[test]
    fn test_calc_child() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DOptimisticCaptureNode::new(DNodeId::default(), gamestate);
        println!("{}", node);
        let child_up = node.calc_child(DDirection::Up).unwrap();
        println!("{}", child_up);
        assert!(child_up.is_alive());
        let child_left = node.calc_child(DDirection::Left);
        assert!(child_left.is_err());
    }

    #[test]
    fn test_calc_moves() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DOptimisticCaptureNode::new(DNodeId::default(), gamestate);
        let moves = node.calc_moves();
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&DDirection::Up));
        assert!(moves.contains(&DDirection::Right));
        let new_node = node
            .calc_child(DDirection::Up)
            .unwrap()
            .calc_child(DDirection::Right)
            .unwrap()
            .calc_child(DDirection::Right)
            .unwrap()
            .calc_child(DDirection::Right)
            .unwrap();
        println!("{}", new_node);
        let moves = new_node.calc_moves();
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&DDirection::Down));
        assert!(moves.contains(&DDirection::Right));
        assert!(moves.contains(&DDirection::Up));
        let new_node = new_node.calc_child(DDirection::Right).unwrap();
        println!("{}", new_node);
        let moves = new_node.calc_moves();
        assert_eq!(moves.len(), 0);
    }
}
