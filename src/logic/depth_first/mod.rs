use super::Brain;

mod direction_rating;
mod node;
mod node_rating;
mod simulation_node;
mod simulation_parameters;
mod simulation_result;
mod simulation_state;
mod simulation_tree;
mod state_rating;

pub struct DepthFirstSnake {}

impl DepthFirstSnake {
    pub fn new() -> Self {
        todo!()
    }
}

impl Brain for DepthFirstSnake {
    fn logic(
        &self,
        _game: &crate::Game,
        _turn: &i32,
        _board: &crate::Board,
        _you: &crate::Battlesnake,
    ) -> super::Direction {
        todo!()
    }
}
