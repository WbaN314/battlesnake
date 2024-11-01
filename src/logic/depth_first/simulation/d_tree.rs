use super::{d_node::DNode, d_node_id::DNodeId};
use crate::logic::depth_first::game::{d_direction::DDirection, d_game_state::DGameState};
use std::collections::BTreeMap;

pub struct DTree {
    nodes: BTreeMap<DNodeId, DNode>,
}

impl DTree {
    pub fn new(start: DGameState) -> Self {
        let mut nodes = BTreeMap::new();
        nodes.insert(DNodeId::default(), DNode::Scoped(DNodeId::default(), start));
        Self { nodes }
    }

    fn scope(&mut self, id: &DNodeId, direction: DDirection) -> &mut Self {
        match self.nodes.get(id) {
            Some(DNode::Scoped { base, .. }) => {
                let moves = [Some(direction), None, None, None];
                let mut new_id = id.clone();
                new_id.push(direction);
                let mut new_base = base.clone();
                new_base
                    .next_state(moves)
                    .move_reachable(moves, new_id.len() as u8);
                self.nodes
                    .insert(new_id.clone(), DNode::Scoped(new_id, new_base));
            }
            _ => panic!("Invalid node id for scoping"),
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::read_game_state;

    use super::*;

    #[test]
    fn test_scope() {
        let gamestate = read_game_state("requests/test_move_request.json");
        let state = DGameState::from_request(&gamestate.board, &gamestate.you);
        println!("{}", state);
        let mut tree = DTree::new(state);
        let mut id = DNodeId::default();
        tree.scope(&id, DDirection::Up);
        id.push(DDirection::Up);
        let u = tree.nodes.get(&id).unwrap();
        println!("{}", u);
        tree.scope(&id, DDirection::Up);
        id.push(DDirection::Up);
        let uu = tree.nodes.get(&id).unwrap();
        println!("{}", uu);
        tree.scope(&id, DDirection::Up);
        id.push(DDirection::Up);
        let uuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuu);
        tree.scope(&id, DDirection::Up);
        id.push(DDirection::Up);
        let uuuu = tree.nodes.get(&id).unwrap();
        println!("{}", uuuu);
        match uuuu {
            DNode::Scoped { id: node_id, .. } => {
                assert_eq!(id, *node_id);
            }
            _ => panic!("Wrong node type"),
        }
    }
}
