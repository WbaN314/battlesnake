use crate::logic::shared::e_direction::EDirectionVec;

use super::simulation_tree::SimulationTree;

#[derive(Debug)]
pub struct DirectionRating {
    best_snake_progression: Vec<u8>,
}

impl DirectionRating {
    pub fn new() -> Self {
        Self {
            best_snake_progression: vec![0; 4],
        }
    }

    pub fn from(simulation_tree: &SimulationTree, id: &EDirectionVec) -> Option<Self> {
        // TODO
        let direction_parent = simulation_tree.map.get(id).unwrap();
        println!("{:?}", direction_parent);
        Some(Self::new())
    }

    pub fn update(&mut self, direction: usize, progression: u8) {
        self.best_snake_progression[direction] =
            self.best_snake_progression[direction].max(progression);
    }

    pub fn get(&self, direction: usize) -> u8 {
        self.best_snake_progression[direction]
    }
}
