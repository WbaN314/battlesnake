use std::cmp::Ordering;

use super::d_node_id::DNodeId;

pub mod d_full_simulation_node;
pub mod d_optimistic_capture_node;
pub mod d_pessimistic_capture_node;

pub trait DNode {
    fn id(&self) -> &DNodeId;
    fn calc_children(&self) -> Vec<Box<Self>>;
    fn status(&self) -> DNodeStatus;
    fn info(&self) -> String {
        format!("{} {:?}", self.id(), self.status())
    }
    fn statistics(&self) -> DNodeStatistics {
        DNodeStatistics::default()
    }
    fn simulation_order(&self, other: &Self) -> Ordering {
        self.id().cmp(other.id())
    }
    fn result_order(&self, other: &Self) -> Ordering {
        self.id().cmp(other.id())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DNodeStatus {
    #[default]
    Unknown,
    Alive(DNodeAliveStatus),
    Dead,
    TimedOut,
    DeadEnd,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DNodeAliveStatus {
    #[default]
    Unknown,
    Always,
    Sometimes,
}

#[derive(Default, Copy, Clone)]
/// Statistics of a node.
/// If statistics refer to individual gamestates, they represent the worst case scenario
pub struct DNodeStatistics {
    pub states: Option<usize>,
    pub highest_alive_snakes: Option<usize>,
    pub lowest_self_length: Option<usize>,
}
