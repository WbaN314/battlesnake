use std::fmt::{Display, Formatter};

use crate::logic::shared::e_direction::{EDirection, EDirectionVec};

use super::{
    node_rating::{Finished, NodeRating},
    simulation_node::SimulationNode,
    simulation_tree::SimulationTree,
};

pub struct SimulationResult {
    extracted_ratings: Vec<(EDirectionVec, NodeRating<Finished>)>,
}

impl SimulationResult {
    pub fn from(simulation_tree: SimulationTree) -> Self {
        let mut extracted_ratings = simulation_tree
            .map
            .iter()
            .filter_map(|(d_vec, s_node)| match *s_node.borrow() {
                SimulationNode::Completed(ref rating) => {
                    Some((d_vec.clone(), rating.clone().into()))
                }
                _ => None,
            })
            .collect::<Vec<(EDirectionVec, NodeRating<Finished>)>>();
        extracted_ratings.sort_by(|a, b| a.0.cmp(&b.0));
        Self { extracted_ratings }
    }

    pub fn get_best_direction(&self) -> EDirection {
        // TODO: Improve this
        self.extracted_ratings
            .last()
            .unwrap()
            .0
            .first()
            .unwrap_or(&EDirection::Up)
            .clone()
    }
}

impl Display for SimulationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (direction, rating) in self.extracted_ratings.iter() {
            writeln!(f, "{}-> {}", direction, rating)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{
        logic::{
            depth_first::simulation_parameters::SimulationParameters,
            shared::e_game_state::EGameState,
        },
        read_game_state,
    };

    #[test]
    fn test_add_all_child_parent_converts_to_result() {
        let game_state = read_game_state("requests/failure_30_grab_food_leads_to_death.json");
        let e_game_state = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", e_game_state);
        let parameters = SimulationParameters::new()
            .simulation_duration(Duration::from_millis(200))
            .prune_hash_radius(10)
            .move_snake_heads_radius(10);
        let result = SimulationTree::from(e_game_state)
            .parameters(parameters)
            .print()
            .simulate_timed();
        println!("\n{}", result);
    }
}
