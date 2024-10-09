use crate::logic::shared::e_game_state::EGameState;

pub struct StateRating {
    pub snakes_alive: u8,
    pub current_length: u8,
}
impl StateRating {
    pub fn from(state: &EGameState) -> Self {
        let snakes_alive = state.snakes.count_alive();
        let current_length = state.snakes.get(0).as_ref().unwrap().length;
        Self {
            snakes_alive,
            current_length,
        }
    }
}
