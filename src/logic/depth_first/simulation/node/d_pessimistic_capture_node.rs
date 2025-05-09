use std::{cell::Cell, fmt::Display};

use arrayvec::ArrayVec;

use crate::logic::depth_first::{
    game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
    simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
};

use super::{DChildrenCalculationResult, DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus};

pub struct DPessimisticCaptureNode {
    id: DNodeId,
    state: DGameState<DSlowField>,
    time: DTreeTime,
    status: Cell<DNodeStatus>,
}

impl DPessimisticCaptureNode {
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
            true => DNodeStatus::Alive(DNodeAliveStatus::Always),
            false => DNodeStatus::Dead,
        };
        Self::new(new_id, new_state, self.time.clone(), status)
    }

    fn calc_moves(&self) -> ArrayVec<DDirection, 4> {
        self.state.scope_moves_pessimistic(self.id().len() as u8)
    }
}

impl DNode for DPessimisticCaptureNode {
    fn id(&self) -> &DNodeId {
        &self.id
    }

    fn statistics(&self) -> DNodeStatistics {
        DNodeStatistics::default()
    }

    fn set_status(&mut self, status: DNodeStatus) {
        self.status.set(status);
    }

    fn status(&self) -> DNodeStatus {
        if self.status.get() == DNodeStatus::Unknown {
            if self.state.get_alive()[0] {
                self.status
                    .set(DNodeStatus::Alive(DNodeAliveStatus::default()));
            } else {
                self.status.set(DNodeStatus::Dead);
            }
        }
        self.status.get()
    }

    fn calc_children(&mut self) -> DChildrenCalculationResult<Self> {
        DChildrenCalculationResult::Ok(
            self.calc_moves()
                .into_iter()
                .map(|direction| Box::new(self.calc_child(direction)))
                .collect(),
        )
    }
}

impl Display for DPessimisticCaptureNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id)?;
        writeln!(f, "{}", self.state)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DPessimisticCaptureNode;
    use crate::{
        logic::depth_first::{
            game::{d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState},
            simulation::{
                d_node_id::DNodeId,
                d_tree::DTreeTime,
                node::{DNode, DNodeAliveStatus, DNodeStatus},
            },
        },
        read_game_state,
    };

    #[test]
    fn test_calc_child() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DPessimisticCaptureNode::new(
            DNodeId::default(),
            gamestate,
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        println!("{}", node);
        let child_up = node.calc_child(DDirection::Up);
        println!("{}", child_up);
        assert_eq!(
            child_up.status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        let child_left = node.calc_child(DDirection::Left);
        assert_eq!(child_left.status(), DNodeStatus::Dead);
    }

    #[test]
    fn test_calc_moves() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DPessimisticCaptureNode::new(
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
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&DDirection::Down));
    }
}
