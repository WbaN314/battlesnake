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
    pub fn direction(&self) -> Option<DDirection> {
        self.id.first().copied()
    }

    pub fn set_sparse(&mut self, sparse: bool) {
        self.sparse = sparse;
    }

    pub fn is_sparse(&self) -> bool {
        self.sparse
    }

    pub fn parent(&self) -> Option<Self> {
        if self.id.is_empty() {
            return None;
        }
        let mut parent = self.clone();
        parent.id.pop();
        Some(parent)
    }
}

impl From<&str> for DNodeId {
    fn from(directions: &str) -> Self {
        let mut id = Vec::new();
        let mut sparse = false;

        let mut directions = directions.to_string();
        if directions.ends_with("-s") {
            directions.truncate(directions.len() - 2);
            sparse = true;
        }

        for direction in directions.chars() {
            id.push(direction.try_into().unwrap());
        }
        Self { id, sparse }
    }
}

impl Display for DNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for direction in &self.id {
            write!(f, "{}", direction)?;
        }
        if self.sparse {
            write!(f, "-s")?;
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

    #[test]
    fn test_parent() {
        let a = DNodeId::from("UD");
        let b = a.parent().unwrap();

        assert_eq!(b, DNodeId::from("U"));
        assert_eq!(b.parent().unwrap(), DNodeId::from(""));
        assert_eq!(b.parent().unwrap().parent(), None);

        let c = DNodeId::from("UDL");
        let d = c.parent().unwrap();
        assert_eq!(d, a);

        let mut c_sparse = c.clone();
        c_sparse.sparse = true;
        let d_sparse = c_sparse.parent().unwrap();
        assert_ne!(d_sparse, a);
    }
}
