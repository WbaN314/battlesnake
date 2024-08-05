#![allow(dead_code)]
use super::simulation_node::SimulationNode;
use crate::logic::efficient_game_objects::{
    e_direction::{EDirection, EDirectionVec},
    e_game_state::EGameState,
};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

pub struct SimulationTree {
    map: BTreeMap<EDirectionVec, RefCell<SimulationNode>>,
}

impl SimulationTree {
    pub fn from(initial_state: EGameState) -> Self {
        let mut map = BTreeMap::new();
        let root = SimulationNode::new(initial_state);
        map.insert(EDirectionVec::new(), RefCell::new(root));
        SimulationTree { map }
    }

    // adds a child and transforms the parent to result if no longer needed
    pub fn simulate_and_add_children(&mut self, parent_id: &EDirectionVec, distance: u8) {
        let children = match self.map.get(parent_id) {
            Some(simulation_node) => {
                let mut simulation_node = simulation_node.borrow_mut();
                simulation_node.calculate_children(distance)
            }
            None => {
                panic!("Parent node not found");
            }
        };
        for d in 0..4 {
            let mut child_id = parent_id.clone();
            child_id.push(EDirection::from_usize(d));
            self.map
                .insert(child_id.clone(), RefCell::new(children[d].clone()));
            self.propagate_rating_upwards(&child_id);
        }
        self.map
            .get_mut(parent_id)
            .unwrap()
            .borrow_mut()
            .transform_to_completed();
    }

    fn propagate_rating_upwards(&mut self, id: &EDirectionVec) {
        let mut id = id.clone();
        while id.len() > 0 {
            match self.map.get(&id).unwrap().borrow().get_rating() {
                Some(child_rating) => {
                    id.pop();
                    let mut parent = self.map.get(&id).unwrap().borrow_mut();
                    parent.update_rating(&child_rating);
                }
                None => break,
            }
        }
    }
}

impl Display for SimulationTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, node) in self.map.iter() {
            writeln!(f, "{}: {}", id, node.borrow())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::u8;

    use super::*;
    use crate::logic::{
        efficient_game_objects::e_game_state::EGameState, json_requests::read_game_state,
    };

    #[test]
    fn test_add_all_child_parent_converts_to_result() {
        let game_state = read_game_state("requests/example_move_request_3.json");
        let e_game_state = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", e_game_state);
        let mut simulation_tree = SimulationTree::from(e_game_state);
        let root_id = EDirectionVec::new();
        simulation_tree.simulate_and_add_children(&root_id, u8::MAX);
        simulation_tree
            .simulate_and_add_children(&EDirectionVec::from(vec![EDirection::Down]), u8::MAX);
        println!("{}", simulation_tree);
    }
}
