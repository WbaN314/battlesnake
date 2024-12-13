use serde::de;

use super::{d_node::DNode, d_node_id::DNodeId, d_state_id::DStateId};
use crate::logic::depth_first::game::{
    d_direction::DDirection, d_field::DSlowField, d_game_state::DGameState,
};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt::Display,
};

#[derive(Clone)]
pub struct DTree {
    nodes: BTreeMap<DNodeId, DNode>,
    queue: VecDeque<DNodeId>,
}

impl DTree {
    pub fn new(start: DGameState<DSlowField>) -> Self {
        let mut nodes = BTreeMap::new();
        let mut queue = VecDeque::new();
        let mut states = HashMap::new();
        states.insert(DStateId::default(), start.clone().into());
        nodes.insert(
            DNodeId::default(),
            DNode::simulated(DNodeId::default(), start.clone(), states),
        );
        queue.push_back(DNodeId::default());
        Self { nodes, queue }
    }

    fn scope_node(&self, id: &DNodeId, direction: DDirection) -> (DNodeId, DNode) {
        match self.nodes.get(id) {
            Some(DNode::Scoped { base, .. }) | Some(DNode::Simulated { base, .. }) => {
                let moves = [Some(direction), None, None, None];
                let mut new_id = id.clone();
                new_id.push(direction);
                let mut new_base = base.clone();
                new_base
                    .next_state(moves)
                    .move_reachable(moves, new_id.len() as u8);
                if new_base.is_alive() {
                    return (new_id.clone(), DNode::scoped(new_id, new_base));
                } else {
                    return (new_id.clone(), DNode::dead(new_id));
                }
            }
            Some(_) => panic!("Invalid node type for scoping"),
            None => panic!("Invalid node id for scoping"),
        }
    }

    fn simulate_node(&self, id: &DNodeId, direction: DDirection) -> (DNodeId, DNode) {
        match self.nodes.get(id) {
            Some(DNode::Simulated { id, states, .. }) => {
                let mut new_id = id.clone();
                new_id.push(direction);
                let scoped_node = match self.nodes.get(&new_id) {
                    Some(node @ DNode::Scoped { .. }) => node.clone(),
                    None => self.scope_node(id, direction).1,
                    Some(_) => {
                        panic!("Child node is exists but is not type Scoped");
                    }
                };
                let base = scoped_node.base().clone();
                let mut child_states = HashMap::new();
                for (state_id, state) in states {
                    for mve in state.possible_moves().generate() {
                        let mut new_state = state.clone();
                        let mut new_id = state_id.clone();
                        new_id.push(mve);
                        new_state.next_state(mve);
                        child_states.insert(new_id, new_state);
                    }
                }
                return (new_id.clone(), DNode::simulated(new_id, base, child_states));
            }
            Some(_) => panic!("Invalid node type for scoping"),
            None => panic!("Invalid node id for simulation"),
        }
    }

    fn scope_timed(&mut self) -> &mut Self {
        loop {
            let mut new_nodes = Vec::new();
            match self.queue.pop_front() {
                Some(next) => match self.nodes.get(&next) {
                    Some(DNode::Scoped { id, base }) | Some(DNode::Simulated { id, base, .. }) => {
                        let moves = base.scope_moves(id.len() as u8);
                        for i in 0..4 {
                            if moves[i as usize] {
                                let (next_id, next_node) =
                                    self.scope_node(id, (i as u8).try_into().unwrap());
                                if let DNode::Scoped { .. } = next_node {
                                    self.queue.push_back(next_id.clone())
                                }
                                new_nodes.push((next_id, next_node));
                            }
                        }
                    }
                    _ => panic!("Invalid node id for simulation"),
                },
                None => {
                    break;
                }
            }
            for (id, node) in new_nodes {
                self.nodes.insert(id, node);
            }
        }
        self
    }

    fn simulate_timed(&mut self) -> &mut Self {
        todo!()
    }
}

impl Display for DTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, _) in self.nodes.iter() {
            match self.nodes.get(id) {
                Some(DNode::Scoped { id, .. }) => {
                    writeln!(f, "{} {} (Scoped)", id.len(), id)?;
                }
                Some(DNode::Simulated { id, states, .. }) => {
                    writeln!(
                        f,
                        "{} {} (Simulated) - states: {}",
                        id.len(),
                        id,
                        states.len()
                    )?;
                }
                Some(DNode::Dead { id }) => {
                    writeln!(f, "{} {} (Dead)", id.len(), id)?;
                }
                None => panic!("Invalid node id"),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_game_state;

    #[bench]
    fn bench_scope_node_1_up(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        let tree = DTree::new(state);
        b.iter(|| {
            let tree = tree.clone();
            tree.scope_node(&DNodeId::default(), DDirection::Up);
        });
    }

    #[bench]
    fn bench_simulate_node_1_up(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        let tree = DTree::new(state);
        b.iter(|| {
            let tree = tree.clone();
            tree.simulate_node(&DNodeId::default(), DDirection::Up);
        });
    }

    #[bench]
    fn bench_scope_node_3_up(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        let tree = DTree::new(state);
        b.iter(|| {
            let mut tree = tree.clone();
            let mut id = DNodeId::default();
            for _ in 0..3 {
                let (new_id, new_node) = tree.scope_node(&id, DDirection::Up);
                tree.nodes.insert(new_id, new_node);
                id.push(DDirection::Up);
                tree.nodes.get(&id).unwrap();
            }
        });
    }

    #[bench]
    fn bench_simulate_node_3_up(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        let tree = DTree::new(state);
        b.iter(|| {
            let mut tree = tree.clone();
            let mut id = DNodeId::default();
            for _ in 0..3 {
                let (new_id, new_node) = tree.simulate_node(&id, DDirection::Up);
                tree.nodes.insert(new_id, new_node);
                id.push(DDirection::Up);
                tree.nodes.get(&id).unwrap();
            }
        });
    }

    #[test]
    fn test_scope_timed() {
        let gamestate =
            read_game_state("requests/failure_43_going_down_guarantees_getting_killed.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let mut tree = DTree::new(state.clone());
        tree.scope_timed();
        println!("{}", tree);
        println!("{}", state);
    }

    #[test]
    fn test_scope_node() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let mut tree = DTree::new(state);
        let mut id = DNodeId::default();
        let (new_id, new_node) = tree.scope_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let u = tree.nodes.get(&id).unwrap();
        println!("{}", u);
        let (new_id, new_node) = tree.scope_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uu = tree.nodes.get(&id).unwrap();
        println!("{}", uu);
        let (new_id, new_node) = tree.scope_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuu);
        let (new_id, new_node) = tree.scope_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uuuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuuu);
        match uuuu {
            DNode::Scoped { id: node_id, .. } => {
                assert_eq!(id, *node_id);
            }
            _ => panic!("Wrong node type"),
        }
        let (new_id, new_node) = tree.scope_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uuuuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuuuu);
    }

    #[test]
    fn test_simulate_node() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you, &gamestate.turn);
        println!("{}", state);
        let mut tree = DTree::new(state);
        let mut id = DNodeId::default();
        let (new_id, new_node) = tree.simulate_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let u = tree.nodes.get(&id).unwrap();
        match u {
            DNode::Simulated {
                id: ref node_id,
                ref states,
                ..
            } => {
                assert_eq!(&id, node_id);
                assert_eq!(states.len(), 36);
            }
            _ => panic!("Wrong node type"),
        }
        println!("{}", u);
        let (new_id, new_node) = tree.simulate_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uu = tree.nodes.get(&id).unwrap();
        match uu {
            DNode::Simulated {
                id: ref node_id,
                ref states,
                ..
            } => {
                assert_eq!(&id, node_id);
                assert_eq!(states.len(), 510);
            }
            _ => panic!("Wrong node type"),
        }
        println!("{}", uu);
        let (new_id, new_node) = tree.simulate_node(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uuu = tree.nodes.get(&id).unwrap();
        match uuu {
            DNode::Simulated {
                id: ref node_id,
                ref states,
                ..
            } => {
                assert_eq!(&id, node_id);
                assert_eq!(states.len(), 7215);
            }
            _ => panic!("Wrong node type"),
        }
    }
}
