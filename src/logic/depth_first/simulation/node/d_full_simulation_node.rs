use std::{cell::Cell, fmt::Display};

use arrayvec::ArrayVec;
use itertools::Itertools;

use crate::logic::{
    depth_first::{
        game::{d_direction::DDirection, d_field::DFastField, d_game_state::DGameState},
        simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
    },
    legacy::shared::e_snakes::SNAKES,
};

use super::{DNode, DNodeStatus, DNodeStatusDead};

pub struct DFullSimulationNode {
    id: DNodeId,
    states: Vec<DGameState<DFastField>>,
    time: DTreeTime,
    status: Cell<DNodeStatus>,
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
                    if !state.is_alive() {
                        self.status.set(DNodeStatus::Dead(DNodeStatusDead::Unknown));
                        return self.status.get();
                    }
                }
                self.status.set(DNodeStatus::Alive);
                return self.status.get();
            }
            status => status,
        }
    }

    fn calc_children(&self) -> Vec<Box<Self>> {}
}

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
mod tests {}
