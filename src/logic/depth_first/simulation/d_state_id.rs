use std::ops::{Deref, DerefMut};

use crate::logic::{depth_first::game::d_direction::DDirection, legacy::shared::e_snakes::SNAKES};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct DStateId([Vec<DDirection>; SNAKES as usize]);

impl DStateId {
    pub fn push(&mut self, moves: [Option<DDirection>; SNAKES as usize]) {
        for (snake, direction) in moves.iter().enumerate() {
            if let Some(direction) = direction {
                self.0[snake].push(*direction);
            }
        }
    }
}

impl Default for DStateId {
    fn default() -> Self {
        Self([Vec::new(), Vec::new(), Vec::new(), Vec::new()])
    }
}

impl Deref for DStateId {
    type Target = [Vec<DDirection>; SNAKES as usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DStateId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<DStateId>(), 96);
    }

    #[test]
    fn test_push() {
        let mut state_id = DStateId::default();
        state_id.push([Some(DDirection::Up), None, None, None]);
        state_id.push([None, Some(DDirection::Down), None, None]);
        state_id.push([None, None, Some(DDirection::Left), None]);
        state_id.push([None, None, None, Some(DDirection::Right)]);
        state_id.push([Some(DDirection::Left), None, None, Some(DDirection::Right)]);
        assert_eq!(
            state_id,
            DStateId([
                vec![DDirection::Up, DDirection::Left],
                vec![DDirection::Down],
                vec![DDirection::Left],
                vec![DDirection::Right, DDirection::Right]
            ])
        );
    }
}
