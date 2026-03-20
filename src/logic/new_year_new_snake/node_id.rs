use crate::logic::game::moves::Moves;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct NodeId {}

impl NodeId {
    pub fn new() -> Self {
        todo!()
    }

    pub fn child(&self, moves: Moves) -> Self {
        todo!()
    }

    pub fn parent(&self) -> Option<Self> {
        todo!()
    }
}
