use std::fmt::Display;

use crate::logic::depth_first::game::{
    d_field::{DFastField, DSlowField},
    d_game_state::DGameState,
};

use super::d_node_id::DNodeId;

#[derive(Clone)]
pub enum DNode {
    Scoped {
        id: DNodeId,
        base: DGameState<DSlowField>,
    },
    Simulated {
        id: DNodeId,
        base: DGameState<DSlowField>,
        states: Vec<DGameState<DFastField>>,
    },
}

impl DNode {
    pub fn scoped(id: DNodeId, base: DGameState<DSlowField>) -> Self {
        DNode::Scoped { id, base }
    }

    pub fn simulated(
        id: DNodeId,
        base: DGameState<DSlowField>,
        states: Vec<DGameState<DFastField>>,
    ) -> Self {
        DNode::Simulated { id, base, states }
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
        }
        Ok(())
    }
}
