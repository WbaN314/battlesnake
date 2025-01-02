use crate::logic::{
    depth_first::{
        game::{
            d_direction::{DDirection, D_DIRECTION_LIST},
            d_field::DFastField,
            d_game_state::DGameState,
        },
        simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
    },
    legacy::shared::e_snakes::SNAKES,
};
use std::{cell::Cell, cmp::Ordering, fmt::Display, ops::Deref};

use super::{DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus};

#[derive(Default)]
pub struct DFullSimulationNode {
    id: DNodeId,
    states: Vec<DGameState<DFastField>>,
    time: DTreeTime,
    status: Cell<DNodeStatus>,
    statistics: Cell<Option<DNodeStatistics>>,
    leaf: bool,
}

impl DFullSimulationNode {
    pub fn new(
        id: DNodeId,
        states: Vec<DGameState<DFastField>>,
        time: DTreeTime,
        status: DNodeStatus,
    ) -> Self {
        Self {
            id,
            states: states,
            time,
            status: Cell::new(status),
            statistics: Cell::new(None),
            leaf: true,
        }
    }
}

impl DNode for DFullSimulationNode {
    fn id(&self) -> &DNodeId {
        &self.id
    }

    fn status(&self) -> DNodeStatus {
        match self.status.get() {
            DNodeStatus::Unknown => {
                for state in self.states.iter() {
                    if !state.get_alive()[0] {
                        self.status.set(DNodeStatus::Dead);
                        return self.status.get();
                    }
                }
                self.status
                    .set(DNodeStatus::Alive(DNodeAliveStatus::default()));
                return self.status.get();
            }
            status => status,
        }
    }

    fn calc_children(&self) -> Vec<Box<Self>> {
        let mut states = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut statuses = [DNodeStatus::Alive(DNodeAliveStatus::Always); 4];
        for state in self.states.iter() {
            if self.time.is_timed_out() {
                for i in 0..4 {
                    match statuses[i] {
                        DNodeStatus::Alive(_) => {
                            statuses[i] = DNodeStatus::TimedOut;
                        }
                        _ => (),
                    }
                }
                break;
            }
            let possible_moves_matrix = state.possible_moves();
            let possible_moves = possible_moves_matrix.generate();
            if possible_moves.is_empty() {
                self.status.set(DNodeStatus::DeadEnd);
                return Vec::new();
            }
            for (i, d) in possible_moves_matrix.get(0).iter().enumerate() {
                match statuses[i] {
                    DNodeStatus::Alive(DNodeAliveStatus::Always) if !d => {
                        statuses[i] = DNodeStatus::Alive(DNodeAliveStatus::Sometimes)
                    }
                    _ => (),
                }
            }
            for moveset in possible_moves.into_iter() {
                let index = moveset[0].unwrap() as usize;
                if statuses[index] == DNodeStatus::Dead {
                    continue;
                }
                let mut new_state = state.clone();
                new_state.next_state(moveset);
                if !new_state.get_alive()[0] {
                    statuses[index] = DNodeStatus::Dead;
                }
                states[index].push(new_state);
            }
        }
        for i in 0..4 {
            if states[i].is_empty() && statuses[i] != DNodeStatus::TimedOut {
                statuses[i] = DNodeStatus::Dead;
            }
        }
        let mut result = Vec::new();
        for i in 0..4 {
            let mut id = self.id.clone();
            id.push(D_DIRECTION_LIST[i]);
            result.push(Box::new(Self::new(
                id,
                states[i].clone(),
                self.time.clone(),
                statuses[i],
            )));
        }
        result
    }

    fn info(&self) -> String {
        format!("{} {:?} {}", self.id, self.status(), self.states.len())
    }

    fn statistics(&self) -> DNodeStatistics {
        if self.statistics.get().is_none() {
            let mut statistics = DNodeStatistics::default();
            statistics.states = Some(self.states.len());
            for state in self.states.iter() {
                let alive_snakes = state.get_alive().iter().filter(|&&x| x).count();
                statistics.highest_alive_snakes = Some(
                    statistics
                        .highest_alive_snakes
                        .unwrap_or(alive_snakes)
                        .max(alive_snakes),
                );
                let length = state.get_length();
                if let Some(length) = length {
                    statistics.lowest_self_length =
                        Some(statistics.lowest_self_length.unwrap_or(length).min(length));
                }
            }
            self.statistics.set(Some(statistics));
        }
        self.statistics.get().unwrap()
    }
}

impl Ord for DFullSimulationNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.states.len().cmp(&other.states.len()).reverse()
    }
}

impl PartialOrd for DFullSimulationNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DFullSimulationNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DFullSimulationNode {}

impl Display for DFullSimulationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id)?;
        if self.states.len() <= 10 {
            for state in &self.states {
                writeln!(f, "{}", state)?;
            }
        } else {
            for state in &self.states[0..5] {
                writeln!(f, "{}", state)?;
            }
            writeln!(f, "...\n\n")?;
            for state in &self.states[self.states.len() - 5..] {
                writeln!(f, "{}", state)?;
            }
        }
        writeln!(f, "States: {}", self.states.len())?;
        writeln!(f, "{:?}", self.status())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DFullSimulationNode;
    use crate::{
        logic::depth_first::{
            game::{d_field::DFastField, d_game_state::DGameState},
            simulation::{
                d_node_id::DNodeId,
                d_tree::DTreeTime,
                node::{DNode, DNodeAliveStatus, DNodeStatus},
            },
        },
        read_game_state,
    };

    #[test]
    fn test_calc_children() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DFastField>::from_request(&request.board, &request.you, &request.turn);
        let node = DFullSimulationNode::new(
            DNodeId::default(),
            vec![gamestate],
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        println!("{}", node);
        let children = node.calc_children();
        assert_eq!(children.len(), 4);
        assert_eq!(
            children[0].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children[0].id, DNodeId::from("U"));
        assert_eq!(children[1].status(), DNodeStatus::Dead);
        assert_eq!(children[1].id, DNodeId::from("D"));
        assert_eq!(children[2].status(), DNodeStatus::Dead);
        assert_eq!(children[2].id, DNodeId::from("L"));
        assert_eq!(
            children[3].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children[3].id, DNodeId::from("R"));
        println!("{}", children[3]);
        let children_right = children[3].calc_children();
        assert_eq!(children_right.len(), 4);
        assert_eq!(
            children_right[0].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[0].id, DNodeId::from("RU"));
        assert_eq!(
            children_right[1].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[1].id, DNodeId::from("RD"));
        assert_eq!(children_right[2].status(), DNodeStatus::Dead);
        assert_eq!(children_right[2].id, DNodeId::from("RL"));
        assert_eq!(
            children_right[3].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[3].id, DNodeId::from("RR"));
    }

    #[test]
    fn test_calc_children_2() {
        let request = read_game_state("requests/test_move_request_2.json");
        let gamestate =
            DGameState::<DFastField>::from_request(&request.board, &request.you, &request.turn);
        println!("{:?}", gamestate.possible_moves().generate());
        let node = DFullSimulationNode::new(
            DNodeId::default(),
            vec![gamestate],
            DTreeTime::default(),
            DNodeStatus::default(),
        );
        println!("{}", node);
        let children = node.calc_children();
        println!("{}", children[0]);
        println!("{}", children[1]);
        println!("{}", children[2]);
        println!("{}", children[3]);
    }
}
