#![allow(dead_code)]
use super::{
    node::Node, simulation_node::SimulationNode, simulation_parameters::SimulationParameters,
    simulation_result::SimulationResult,
};
use crate::logic::shared::{
    e_direction::{EDirection, EDirectionVec},
    e_game_state::EGameState,
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    fmt::{Display, Formatter},
};

pub struct SimulationTree {
    pub map: BTreeMap<EDirectionVec, RefCell<SimulationNode>>,
    priority_queue: VecDeque<EDirectionVec>,
    parameters: SimulationParameters,
    print: bool,
}

impl SimulationTree {
    pub fn from(initial_state: EGameState) -> Self {
        let mut map = BTreeMap::new();
        let root = SimulationNode::new(vec![initial_state]);
        map.insert(EDirectionVec::new(), RefCell::new(root));
        SimulationTree {
            map,
            priority_queue: VecDeque::new(),
            parameters: SimulationParameters::new(),
            print: false,
        }
    }

    pub fn parameters(self, parameters: SimulationParameters) -> Self {
        Self { parameters, ..self }
    }

    pub fn print(self) -> Self {
        Self {
            print: true,
            ..self
        }
    }

    // adds a child and transforms the parent to result if no longer needed
    pub fn simulate_and_add_children(&mut self, parent_id: &EDirectionVec) {
        let children = match self.map.get(parent_id) {
            Some(simulation_node) => {
                let mut simulation_node = simulation_node.borrow_mut();
                simulation_node.calculate_children(&self.parameters)
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
            match children[d] {
                SimulationNode::Relevant(_) => {
                    self.priority_queue.push_front(child_id);
                }
                _ => (),
            }
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

    /// Sorts the priority queqe ascending on nodes
    fn prioritize_priority_queue(&mut self) {
        self.priority_queue.make_contiguous().sort_by(|a_id, b_id| {
            match (
                &*self.map.get(a_id).unwrap().borrow(),
                &*self.map.get(b_id).unwrap().borrow(),
            ) {
                (SimulationNode::Relevant(a_node), SimulationNode::Relevant(b_node)) => {
                    Node::compare_including_id(a_id, b_id, a_node, b_node)
                }
                _ => {
                    panic!("Only relevant nodes should be in the priority queqe when prioritizing")
                }
            } // Best value is last in queqe
        });
    }

    pub fn simulate_timed(mut self) -> SimulationResult {
        self.priority_queue.push_front(EDirectionVec::new());
        while self.priority_queue.len() > 0 && !self.parameters.is_time_up() {
            let id = self.priority_queue.pop_back().unwrap();
            if self.print {
                println!("{}-> {}", id, self.map.get(&id).unwrap().borrow());
            }
            self.simulate_and_add_children(&id);
            self.prioritize_priority_queue();
        }
        SimulationResult::from(self)
    }

    pub fn print_states(&self, id: &EDirectionVec) {
        match self.map.get(id) {
            Some(simulation_node) => {
                simulation_node.borrow().print_states();
            }
            None => {
                panic!("Node not found");
            }
        }
    }

    pub fn print_node_ratings(&self) {
        for node in self.map.iter() {
            println!("{} -> {:?}", node.0, node.1.borrow().get_rating());
        }
    }
}

impl Display for SimulationTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (id, node) in self.map.iter() {
            writeln!(f, "{}-> {}", id, node.borrow())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{logic::shared::e_game_state::EGameState, read_game_state};
    use test::Bencher;

    #[bench]
    fn bench_simulate_and_add_children(b: &mut Bencher) {
        b.iter(|| {
            let game_state = read_game_state("requests/failure_1.json");
            let e_game_state = EGameState::from(&game_state.board, &game_state.you);
            let mut simulation_tree = SimulationTree::from(e_game_state);
            simulation_tree.simulate_and_add_children(&EDirectionVec::new());
            simulation_tree.simulate_and_add_children(&EDirectionVec::from(vec![EDirection::Left]));
            simulation_tree.simulate_and_add_children(&EDirectionVec::from(vec![
                EDirection::Left,
                EDirection::Left,
            ]));
            simulation_tree.simulate_and_add_children(&EDirectionVec::from(vec![
                EDirection::Left,
                EDirection::Left,
                EDirection::Down,
            ]));
        });
    }

    #[test]
    fn test_add_all_child_parent_converts_to_result() {
        let game_state = read_game_state("requests/failure_1.json");
        let e_game_state = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", e_game_state);
        let mut simulation_tree = SimulationTree::from(e_game_state);
        let root_id = EDirectionVec::new();
        simulation_tree.simulate_and_add_children(&root_id);
        // simulation_tree
        //     .simulate_and_add_children(&EDirectionVec::from(vec![EDirection::Left]), u8::MAX);
        // simulation_tree.simulate_and_add_children(
        //     &EDirectionVec::from(vec![EDirection::Left, EDirection::Left]),
        //     u8::MAX,
        // );
        println!("{}", simulation_tree);
        simulation_tree.print_states(&EDirectionVec::from(vec![EDirection::Left]));
    }

    #[test]
    fn test_simulate_timed() {
        let game_state =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let e_game_state = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", e_game_state);
        let parameters = SimulationParameters::new()
            .simulation_duration(Duration::from_millis(100))
            .prune_hash_radius(5);
        let result = SimulationTree::from(e_game_state)
            .parameters(parameters)
            .simulate_timed();
        println!("{}", result);
    }
}
