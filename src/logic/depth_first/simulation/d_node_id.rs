use crate::logic::depth_first::game::d_direction::DDirection;
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use super::d_state_id::DStateId;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct DNodeId(Vec<DDirection>);

impl DNodeId {
    pub fn new(directions: Vec<DDirection>) -> Self {
        Self(directions)
    }
}

impl Display for DNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for direction in &self.0 {
            write!(f, "{}", direction)?;
        }
        Ok(())
    }
}

impl PartialOrd for DNodeId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNodeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl Deref for DNodeId {
    type Target = Vec<DDirection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DNodeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for DNodeId {
    fn default() -> Self {
        Self(vec![])
    }
}

impl From<DStateId> for DNodeId {
    fn from(state_id: DStateId) -> Self {
        DNodeId(state_id[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let a = DNodeId(vec![DDirection::Up, DDirection::Down]);
        let b = DNodeId(vec![DDirection::Up, DDirection::Down]);
        let c = DNodeId(vec![DDirection::Up, DDirection::Down, DDirection::Left]);
        let d = DNodeId(vec![DDirection::Up, DDirection::Down, DDirection::Right]);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_ne!(c, d);

        assert!(a < c);
        assert!(a < d);
        assert!(c < d);
    }

    #[test]
    fn test_from_state_id() {
        let mut state_id = DStateId::default();
        state_id.push([Some(DDirection::Up), Some(DDirection::Down), None, None]);
        state_id.push([Some(DDirection::Down), None, None, None]);

        let node_id: DNodeId = state_id.into();
        assert_eq!(node_id, DNodeId(vec![DDirection::Up, DDirection::Down]));
    }
}
