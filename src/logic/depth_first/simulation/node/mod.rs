use std::{cmp::Ordering, fmt::Display};

use super::d_node_id::DNodeId;

pub mod d_full_simulation_node;
pub mod d_optimistic_capture_node;
pub mod d_pessimistic_capture_node;

pub trait DNode {
    fn id(&self) -> &DNodeId;
    fn calc_children(&mut self) -> DChildrenCalculationResult<Self>;
    fn status(&self) -> DNodeStatus;
    fn set_status(&mut self, status: DNodeStatus);
    fn info(&self) -> String {
        format!("{} {:?}", self.id(), self.status())
    }
    fn statistics(&self) -> DNodeStatistics;
    /// Width first per default
    fn simulation_order(&self, other: &Self) -> Ordering {
        self.status()
            .cmp(&other.status())
            .then(self.id().len().cmp(&other.id().len()))
    }
    /// Width first per default
    fn result_order(&self, other: &Self) -> Ordering {
        self.status()
            .cmp(&other.status())
            .then(self.id().len().cmp(&other.id().len()))
    }
    // Result order per default. Used for final single node comparison in tree result.
    fn direction_order(&self, other: &Self) -> Ordering {
        self.result_order(other)
    }
}

pub enum DChildrenCalculationResult<T: DNode + ?Sized> {
    FastEnd, // Fast track ended because of too many options
    DeadEnd,
    TimedOut,
    Ok(Vec<Box<T>>),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DNodeStatus {
    #[default]
    Unknown,
    Dead,
    DeadEndIn(u8),
    Alive(DNodeAliveStatus),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DNodeAliveStatus {
    #[default]
    Unknown,
    Sometimes,
    Always,
    Fast,
}

#[derive(Default, Copy, Clone, Debug)]
/// Statistics of a node.
/// If statistics refer to individual gamestates, they represent the worst case scenario
pub struct DNodeStatistics {
    pub states: Option<usize>,
    pub highest_alive_snakes: Option<usize>,
    pub lowest_self_length: Option<usize>,
    pub relevant_snakes: [Option<u8>; 4],
}

impl Display for DNodeStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "States: {:?}", self.states)?;
        writeln!(f, "Alive Snakes: {:?}", self.highest_alive_snakes)?;
        writeln!(f, "Self Length: {:?}", self.lowest_self_length)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DNodeAliveStatus, DNodeStatus};

    #[test]
    fn test_order() {
        let should_be_ordered = [
            DNodeStatus::Unknown,
            DNodeStatus::Dead,
            DNodeStatus::Alive(DNodeAliveStatus::Unknown),
            DNodeStatus::Alive(DNodeAliveStatus::Sometimes),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
        ];

        let mut clone = should_be_ordered;
        clone.sort();

        assert_eq!(should_be_ordered, clone);
    }
}
