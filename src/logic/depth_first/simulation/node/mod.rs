use crate::logic::depth_first::game::d_direction::DDirection;
use arrayvec::ArrayVec;
use std::{default, fmt::Display};

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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DNodeStatus {
    #[default]
    Unknown,
    Alive,
    Dead,
    TimedOut,
    DeadEnd,
}

#[derive(Default)]
pub struct DNodeStatistics {
    pub states: Option<usize>,
}
