use std::cmp::Ordering;

use crate::logic::depth_first::game::d_game_state::DRelevanceState;

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
        self.id().len().cmp(&other.id().len())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DNodeStatus {
    #[default]
    Unknown,
    Dead,
    DeadEnd,
    TimedOut,
    Alive(DNodeAliveStatus),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DNodeAliveStatus {
    #[default]
    Unknown,
    Sometimes,
    Always,
}

#[derive(Default, Copy, Clone, Debug)]
/// Statistics of a node.
/// If statistics refer to individual gamestates, they represent the worst case scenario
pub struct DNodeStatistics {
    pub states: Option<usize>,
    pub highest_alive_snakes: Option<usize>,
    pub lowest_self_length: Option<usize>,
    pub relevant_snakes: [Option<(DRelevanceState, u8)>; 4],
}

#[cfg(test)]
mod tests {
    use super::{DNodeAliveStatus, DNodeStatus};

    #[test]
    fn test_order() {
        let should_be_ordered = [
            DNodeStatus::Unknown,
            DNodeStatus::Dead,
            DNodeStatus::DeadEnd,
            DNodeStatus::TimedOut,
            DNodeStatus::Alive(DNodeAliveStatus::Unknown),
            DNodeStatus::Alive(DNodeAliveStatus::Sometimes),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
        ];

        let mut clone = should_be_ordered.clone();
        clone.sort();

        assert_eq!(should_be_ordered, clone);
    }
}
