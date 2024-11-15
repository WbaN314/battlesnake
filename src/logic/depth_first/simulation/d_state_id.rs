use crate::logic::{depth_first::game::d_direction::DDirection, legacy::shared::e_snakes::SNAKES};

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct DStateId(Vec<[Option<DDirection>; SNAKES as usize]>);

impl DStateId {
    pub fn push(&mut self, directions: [Option<DDirection>; SNAKES as usize]) {
        self.0.push(directions);
    }
}

impl Default for DStateId {
    fn default() -> Self {
        Self(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<DStateId>(), 0);
    }
}
