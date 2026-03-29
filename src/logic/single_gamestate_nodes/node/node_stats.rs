use super::Node;

impl Node {
    pub fn count_potential_children_all(&self) -> [usize; 4] {
        let move_matrix = self.gamestate.valid_moves();
        let valid_directions = move_matrix.get(0).unwrap();
        // Product of other snakes' valid move counts (snake 0 is fixed to 1 direction)
        let others_product: usize = (1..4).map(|i| move_matrix.get(i).count_valid(1)).product();
        let mut result = [0usize; 4];
        for i in 0..4 {
            if valid_directions[i] {
                result[i] = others_product;
            }
        }
        result
    }

    pub fn count_potential_children_from_evaluated_directions(&self) -> [usize; 4] {
        let move_matrix = self.gamestate.valid_moves();
        let valid_directions = move_matrix.get(0).unwrap();
        // Product of other snakes' valid move counts (snake 0 is fixed to 1 direction)
        let others_product: usize = (1..4).map(|i| move_matrix.get(i).count_valid(1)).product();
        let mut result = [0usize; 4];
        for i in 0..4 {
            if valid_directions[i] && self.children[i].is_some() {
                result[i] = others_product;
            }
        }
        result
    }
}
