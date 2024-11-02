use super::{d_node::DNode, d_node_id::DNodeId};
use crate::logic::depth_first::game::{d_direction::DDirection, d_game_state::DGameState};
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
};

#[derive(Clone)]
pub struct DTree {
    nodes: BTreeMap<DNodeId, DNode>,
    queue: VecDeque<DNodeId>,
}

impl DTree {
    pub fn new(start: DGameState) -> Self {
        let mut nodes = BTreeMap::new();
        let mut queue = VecDeque::new();
        nodes.insert(DNodeId::default(), DNode::Scoped(DNodeId::default(), start));
        queue.push_back(DNodeId::default());
        Self { nodes, queue }
    }

    fn scope(&self, id: &DNodeId, direction: DDirection) -> (DNodeId, DNode) {
        match self.nodes.get(id) {
            Some(DNode::Scoped { base, .. }) => {
                let moves = [Some(direction), None, None, None];
                let mut new_id = id.clone();
                new_id.push(direction);
                let mut new_base = base.clone();
                new_base
                    .next_state(moves)
                    .move_reachable(moves, new_id.len() as u8);
                return (new_id.clone(), DNode::Scoped(new_id, new_base));
            }
            _ => panic!("Invalid node id for scoping"),
        }
    }

    fn simulate(&mut self) -> &mut Self {
        loop {
            let mut new_nodes = Vec::new();
            match self.queue.pop_front() {
                Some(next) => match self.nodes.get(&next) {
                    Some(DNode::Scoped { base, id }) => {
                        let moves = base.scope_moves(id.len() as u8);
                        for i in 0..4 {
                            if moves[i as usize] {
                                let (next_id, next_node) =
                                    self.scope(id, (i as u8).try_into().unwrap());
                                let mut id = id.clone();
                                id.push(DDirection::from((i as u8).try_into().unwrap()));
                                self.queue.push_back(id);
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
}

impl Display for DTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, _) in self.nodes.iter() {
            match self.nodes.get(id) {
                Some(DNode::Scoped { id, .. }) => {
                    writeln!(f, "{} {} (Scoped)", id.len(), id)?;
                }
                _ => panic!("Invalid node type"),
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
    fn bench_simulate(b: &mut test::Bencher) {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        let mut tree = DTree::new(state);
        b.iter(|| {
            let mut tree = tree.clone();
            tree.simulate();
        });
    }

    #[test]
    fn test_simulate() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
        let mut tree = DTree::new(state);
        tree.simulate();
        println!("{}", tree);
    }

    #[test]
    fn test_scope() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
        let mut tree = DTree::new(state);
        let mut id = DNodeId::default();
        let (new_id, new_node) = tree.scope(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let u = tree.nodes.get(&id).unwrap();
        println!("{}", u);
        let (new_id, new_node) = tree.scope(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uu = tree.nodes.get(&id).unwrap();
        println!("{}", uu);
        let (new_id, new_node) = tree.scope(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuu);
        let (new_id, new_node) = tree.scope(&id, DDirection::Up);
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
        let (new_id, new_node) = tree.scope(&id, DDirection::Up);
        tree.nodes.insert(new_id, new_node);
        id.push(DDirection::Up);
        let uuuuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuuuu);
    }
}
