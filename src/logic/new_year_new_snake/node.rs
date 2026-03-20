use crate::logic::{
    game::{field::BasicField, game_state::GameState},
    new_year_new_snake::node_id::NodeId,
};

#[derive(Copy, Clone)]
pub enum NodeStatus {
    AliveFor(u8), // Number of steps where we have checked with guaranteed survival
    DeadIn(u8),   // Number of steps until inevitable death (if opponents play optimally)
}

pub struct Node {
    id: NodeId,
    gamestate: GameState<BasicField>,
    status: NodeStatus,
}

impl Node {
    pub fn new(id: NodeId, gamestate: GameState<BasicField>) -> Self {
        let status = if gamestate.is_alive(0) {
            NodeStatus::AliveFor(0)
        } else {
            NodeStatus::DeadIn(0)
        };
        Self {
            id,
            gamestate,
            status,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn status(&self) -> NodeStatus {
        self.status
    }

    pub fn gamestate(&self) -> &GameState<BasicField> {
        &self.gamestate
    }

    pub fn simulate(&mut self) -> Vec<Node> {
        // Simulate the node by calculating children, for one self direction return all valid other snake move based children
        // Only return the children for one direction where all children are alive
        // Keep track of which directions have been simulated
        // If called again, simulate the next valid direction, until all directions have been simulated
        // If no directions are valid, the node is a leaf and should be marked as DeadIn(1)
        // Update the status of the node based on child statuses
        todo!()
    }

    pub fn update_from_child(&mut self, child: &Node) -> bool {
        // Update the status of the node based on the status of a child
        // The node needs to keep track of all of its children and their statuses to do this correctly
        // If all children for one self direction are AliveFor(n), then this node is AliveFor(n+1)
        // If any child is DeadIn(0), then this node is DeadIn(1)
        // If any child is DeadIn(n) and no child is DeadIn(0), then this node is DeadIn(n+1)
        // Return true if the status of the node has changed, false otherwise
        todo!()
}
