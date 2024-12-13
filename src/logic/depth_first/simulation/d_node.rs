use std::{collections::HashMap, fmt::Display};

use crate::logic::depth_first::game::{
    d_field::{DFastField, DSlowField},
    d_game_state::DGameState,
};

use super::{d_node_id::DNodeId, d_state_id::DStateId};

#[derive(Clone)]
pub enum DNode {
    Scoped {
        id: DNodeId,
        base: DGameState<DSlowField>,
    },
    Simulated {
        id: DNodeId,
        base: DGameState<DSlowField>,
        states: HashMap<DStateId, DGameState<DFastField>>,
    },
    Dead {
        id: DNodeId,
    },
}

impl DNode {
    pub fn scoped(id: DNodeId, base: DGameState<DSlowField>) -> Self {
        DNode::Scoped { id, base }
    }

    pub fn simulated(
        id: DNodeId,
        base: DGameState<DSlowField>,
        states: HashMap<DStateId, DGameState<DFastField>>,
    ) -> Self {
        DNode::Simulated { id, base, states }
    }

    pub fn dead(id: DNodeId) -> Self {
        DNode::Dead { id }
    }

    pub fn id(&self) -> &DNodeId {
        match self {
            DNode::Scoped { id, .. } | DNode::Simulated { id, .. } | DNode::Dead { id } => id,
        }
    }

    pub fn base(&self) -> &DGameState<DSlowField> {
        match self {
            DNode::Scoped { base, .. } | DNode::Simulated { base, .. } => base,
            DNode::Dead { .. } => panic!("Cannot get base from dead node"),
        }
    }
}

impl Display for DNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DNode::Scoped { id, base } => {
                writeln!(f, "{} {} (Scoped)\n", id.len(), id)?;
                writeln!(f, "{}", base)?;
            }
            DNode::Simulated { id, base, states } => {
                writeln!(f, "{} {} (Simulated)\n", id.len(), id)?;
                writeln!(f, "{}", base)?;
                writeln!(f, "States: {}", states.len())?;
            }
            DNode::Dead { id } => {
                writeln!(f, "{} {} (Dead)\n", id.len(), id)?;
            }
        }
        Ok(())
    }
}
