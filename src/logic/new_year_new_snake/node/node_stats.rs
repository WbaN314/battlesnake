use super::Node;

impl Node {
    /// Returns the number of children that would be spawned per explored direction,
    /// computed from the gamestate's valid moves. Invalid directions (e.g. reversing
    /// into own body) are excluded — those are filtered by valid_moves() for all
    /// snakes and don't count as pruned. Unexplored directions return 0.
    pub fn count_potential_children(&self) -> [usize; 4] {
        let move_matrix = self.gamestate.valid_moves();
        let valid_directions = move_matrix.get(0).unwrap();
        // Product of other snakes' valid move counts (snake 0 is fixed to 1 direction)
        let others_product: usize = (1..4).map(|i| move_matrix.get(i).count_valid(1)).product();
        let mut result = [0usize; 4];
        for i in 0..4 {
            if self.children[i].is_some() && valid_directions[i] {
                result[i] = others_product;
            }
        }
        result
    }
}
