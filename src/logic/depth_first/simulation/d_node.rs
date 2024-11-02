use std::fmt::Display;

use crate::logic::depth_first::game::d_game_state::DGameState;

use super::d_node_id::DNodeId;

#[derive(Clone)]
pub enum DNode {
    Scoped { id: DNodeId, base: DGameState },
}

impl DNode {
    #[allow(non_snake_case)]
    pub fn Scoped(id: DNodeId, base: DGameState) -> Self {
        DNode::Scoped { id, base }
    }
}

impl Display for DNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DNode::Scoped { id, base } => {
                writeln!(f, "{} {} (Scoped)\n", id.len(), id)?;
                writeln!(f, "{}", base)?;
            }
        }
        Ok(())
    }
}
