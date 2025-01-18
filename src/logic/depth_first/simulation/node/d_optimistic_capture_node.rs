use super::{DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus};
use crate::logic::{
    depth_first::{
        game::{
            d_direction::DDirection,
            d_field::DSlowField,
            d_game_state::{DGameState, DRelevanceState},
        },
        simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
    },
    legacy::shared::e_snakes::SNAKES,
};
use arrayvec::ArrayVec;
use std::{cell::Cell, fmt::Display};

pub struct DOptimisticCaptureNode {
    id: DNodeId,
    state: DGameState<DSlowField>,
    time: DTreeTime,
    status: Cell<DNodeStatus>,
    child_alive_helper: Cell<[DNodeAliveStatus; 4]>,
    statistics: Cell<DNodeStatistics>,
}

impl DOptimisticCaptureNode {
    pub fn new(
        id: DNodeId,
        state: DGameState<DSlowField>,
        time: DTreeTime,
        status: DNodeStatus,
        statistics: DNodeStatistics,
    ) -> Self {
        Self {
            id,
            state,
            time,
            status: Cell::new(status),
            child_alive_helper: Cell::new([DNodeAliveStatus::default(); 4]),
            statistics: Cell::new(statistics),
        }
    }

    fn calc_child(&self, direction: DDirection) -> Self {
        let moves = [Some(direction), None, None, None];
        let mut new_id = self.id.clone();
        new_id.push(direction);
        let mut new_state = self.state.clone();
        new_state
            .move_tails()
            .move_reachable(moves, new_id.len() as u8);

        // Get relevant snakes, i.e. snakes that have reachable set for next move
        let relevant_snakes = new_state.relevant_snakes(direction, new_id.len() as u8);
        let mut statistics = self.statistics();
        for i in 0..SNAKES as usize {
            if statistics.relevant_snakes[i].is_none() {
                match relevant_snakes[i] {
                    DRelevanceState::Head | DRelevanceState::Body => {
                        statistics.relevant_snakes[i] = Some(new_id.len() as u8)
                    }
                    _ => (),
                }
            }
        }

        new_state.move_heads(moves);
        let status = match new_state.get_alive()[0] {
            true => DNodeStatus::Alive(self.child_alive_helper.get()[direction as usize]),
            false => DNodeStatus::Dead,
        };
        Self::new(new_id, new_state, self.time.clone(), status, statistics)
    }

    fn calc_moves(&self) -> ArrayVec<DDirection, 4> {
        let turn = self.id.len() as u8 + 1;
        let mut child_direction_states = [DNodeAliveStatus::default(); 4];
        let optimistic = self.state.scope_moves_optimistic(turn);
        let pessimistic = self.state.scope_moves_pessimistic(turn);
        for m in pessimistic.iter() {
            let direction = m;
            let index = *direction as usize;
            match self.status() {
                DNodeStatus::Alive(
                    param @ (DNodeAliveStatus::Always | DNodeAliveStatus::Sometimes),
                ) => {
                    child_direction_states[index] = param;
                }
                _ => {
                    panic!("Calc moves node with status {:?}", self.status());
                }
            }
        }
        for m in optimistic.iter() {
            let direction = m;
            let index = *direction as usize;
            if child_direction_states[index] == DNodeAliveStatus::default() {
                child_direction_states[index] = DNodeAliveStatus::Sometimes;
            }
        }
        self.child_alive_helper.set(child_direction_states);
        optimistic
    }
}

impl DNode for DOptimisticCaptureNode {
    fn id(&self) -> &DNodeId {
        &self.id
    }

    fn statistics(&self) -> DNodeStatistics {
        self.statistics.get()
    }

    fn status(&self) -> DNodeStatus {
        if self.status.get() == DNodeStatus::Unknown {
            if self.state.get_alive()[0] {
                self.status
                    .set(DNodeStatus::Alive(DNodeAliveStatus::Always));
            } else {
                self.status.set(DNodeStatus::Dead);
            }
        }
        self.status.get()
    }

    fn info(&self) -> String {
        format!("{} {:?}", self.id, self.status())
    }

    fn calc_children(&mut self) -> Vec<Box<Self>> {
        self.calc_moves()
            .into_iter()
            .map(|direction| Box::new(self.calc_child(direction)))
            .collect()
    }
}

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
                node::{DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus},
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
            DNodeStatistics::default(),
        );
        println!("{}", node);
        node.calc_moves();
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
        let node = DOptimisticCaptureNode::new(
            DNodeId::default(),
            gamestate,
            DTreeTime::default(),
            DNodeStatus::default(),
            DNodeStatistics::default(),
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

        let request = read_game_state("requests/failure_7.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let mut node = DOptimisticCaptureNode::new(
            DNodeId::default(),
            gamestate,
            DTreeTime::default(),
            DNodeStatus::default(),
            DNodeStatistics::default(),
        );
        println!("{}", node);
        assert_eq!(
            node.child_alive_helper.get(),
            [
                DNodeAliveStatus::Unknown,
                DNodeAliveStatus::Unknown,
                DNodeAliveStatus::Unknown,
                DNodeAliveStatus::Unknown
            ]
        );
        let mut children = node.calc_children();
        assert_eq!(
            node.child_alive_helper.get(),
            [
                DNodeAliveStatus::Unknown,
                DNodeAliveStatus::Unknown,
                DNodeAliveStatus::Always,
                DNodeAliveStatus::Always
            ]
        );
        assert_eq!(children.len(), 2);
        let r = children.pop().unwrap();
        assert_eq!(r.id(), &DNodeId::from("R"));
        let mut l = children.pop().unwrap();
        assert_eq!(l.id(), &DNodeId::from("L"));

        l.calc_children();
        assert_eq!(
            l.child_alive_helper.get(),
            [
                DNodeAliveStatus::Unknown,
                DNodeAliveStatus::Sometimes,
                DNodeAliveStatus::Sometimes,
                DNodeAliveStatus::Unknown
            ]
        );
    }

    #[test]
    fn test_relevant_snakes() {
        let request = read_game_state("requests/failure_2.json");
        let gamestate =
            DGameState::<DSlowField>::from_request(&request.board, &request.you, &request.turn);
        let node = DOptimisticCaptureNode::new(
            DNodeId::default(),
            gamestate,
            DTreeTime::default(),
            DNodeStatus::default(),
            DNodeStatistics::default(),
        );
        println!("{}", node);

        let r = node.calc_child(DDirection::Right);

        println!("{}", r);

        let relevant_snakes = r.statistics().relevant_snakes;

        assert_eq!(relevant_snakes[0], None);
        assert_eq!(relevant_snakes[1], None);
        assert_eq!(relevant_snakes[2], Some(1));

        let rd = r.calc_child(DDirection::Down);
        let relevant_snakes = rd.statistics().relevant_snakes;

        println!("{}", rd);

        assert_eq!(relevant_snakes[0], None);
        assert_eq!(relevant_snakes[1], None);
        assert_eq!(relevant_snakes[2], Some(1));
    }
}
