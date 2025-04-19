use crate::logic::depth_first::game::d_direction::DDirection;
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct DNodeId {
    id: Vec<DDirection>,
    sparse: bool,
}

impl DNodeId {
    #[allow(dead_code)]
    pub fn from(directions: &str) -> Self {
        let mut id = Vec::new();
        for direction in directions.chars() {
            id.push(direction.try_into().unwrap());
        }
        Self { id, sparse: false }
    }

    pub fn direction(&self) -> Option<DDirection> {
        self.id.first().copied()
    }

    pub fn set_sparse(&mut self, sparse: bool) {
        self.sparse = sparse;
    }
}

impl Display for DNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for direction in &self.id {
            write!(f, "{}", direction)?;
        }
        if self.sparse {
            write!(f, " sparse")?;
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
        self.id
            .iter()
            .cmp(other.id.iter())
            .then(self.sparse.cmp(&other.sparse))
    }
}

impl Deref for DNodeId {
    type Target = Vec<DDirection>;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl DerefMut for DNodeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let a = DNodeId::from("UD");
        let b = DNodeId::from("UD");
        let c = DNodeId::from("UDL");
        let d = DNodeId::from("UDR");

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_ne!(c, d);

        assert!(a < c);
        assert!(a < d);
        assert!(c < d);
    }

    #[test]
    fn test_sparse() {
        let a = DNodeId::from("UD");

        let mut sparse_a = a.clone();
        sparse_a.sparse = true;

        assert_ne!(sparse_a, a);

        //sparse before others
        assert!(sparse_a > a);
    }
}
