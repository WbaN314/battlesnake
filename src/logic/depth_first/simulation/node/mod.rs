use crate::logic::depth_first::game::d_direction::DDirection;
use arrayvec::ArrayVec;
use std::fmt::Display;

use super::d_node_id::DNodeId;

pub mod d_optimistic_capture_node;
pub mod d_pessimistic_capture_node;

pub trait DNode: Ord + Display {
    fn id(&self) -> &DNodeId;
    fn calc_child(&self, direction: DDirection) -> Result<Box<Self>, DNodeError>;
    fn calc_moves(&self) -> ArrayVec<DDirection, 4>;
    fn is_alive(&self) -> bool;
}

#[derive(Debug)]
pub enum DNodeError {
    TimedOut,
    Dead,
}
