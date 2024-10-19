use crate::logic::legacy::shared::{
    e_direction::{EDirection, EDirectionVec},
    e_game_state::EGameState,
    e_snakes::{ESimulationError, Result},
};
use core::fmt;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
    time::{Duration, Instant},
};

use super::e_state_node::{ENodeRating, EStateNode};

#[derive(Clone, Debug)]
pub struct ESimulationState {
    pub depth: u8,
    pub alive: bool,
    pub snake_count: Vec<u8>,
    pub my_length: Vec<u8>,
}

impl ESimulationState {
    pub fn new() -> Self {
        Self {
            depth: 0,
            alive: false,
            snake_count: Vec::new(),
            my_length: Vec::new(),
        }
    }

    pub fn update(&mut self, iteration_result: &Option<EIterationState>) {
        if let Some(iteration_result) = iteration_result {
            self.snake_count.push(iteration_result.lowest_snakes_count);
            self.my_length.push(iteration_result.highest_length);
            self.alive = true;
            self.depth += 1;
        } else {
            self.alive = false;
        }
    }
}

impl PartialOrd for ESimulationState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.depth.partial_cmp(&other.depth)
    }
}

impl PartialEq for ESimulationState {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for ESimulationState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for ESimulationState {}

pub struct EIterationState {
    lowest_snakes_count: u8,
    highest_length: u8,
}

impl EIterationState {
    pub fn new() -> Self {
        Self {
            lowest_snakes_count: u8::MAX,
            highest_length: 0,
        }
    }

    /// Creates a iteration state option from a node rating option
    /// Used at the first depth level
    /// If none then this is an invalid direction
    pub fn from_ratings(node_ratings: &[Option<ENodeRating>; 4]) -> Option<Self> {
        for i in 0..4 {
            if let Some(node_rating) = node_ratings[i].as_ref() {
                let mut n = Self::new();
                n.lowest_snakes_count = node_rating.highest_snake_count;
                n.highest_length = node_rating.lowest_length;
                n.update(node_ratings);
                return Some(n);
            }
        }
        None
    }

    pub fn from_rating(node_rating: &Option<ENodeRating>) -> Option<Self> {
        if let Some(node_rating) = node_rating {
            let mut n = Self::new();
            n.lowest_snakes_count = node_rating.highest_snake_count;
            n.highest_length = node_rating.lowest_length;
            return Some(n);
        }
        None
    }

    pub fn update(&mut self, node_ratings: &[Option<ENodeRating>; 4]) {
        for i in 0..4 {
            if let Some(node_rating) = &node_ratings[i] {
                self.lowest_snakes_count = node_rating
                    .highest_snake_count
                    .min(self.lowest_snakes_count);
                self.highest_length = node_rating.lowest_length.max(self.highest_length);
            }
        }
    }
}

#[derive(Clone)]
enum EStateTreeNode {
    EStateNode(EStateNode),
    ENodeRating(ENodeRating),
    EDeath,
}

#[derive(Clone)]
pub struct EStateTree {
    map: BTreeMap<EDirectionVec, EStateTreeNode>,
    current: VecDeque<EDirectionVec>,
    duration: Duration,
    start: Instant,
}

impl EStateTree {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            current: VecDeque::from(Vec::new()),
            duration: Duration::new(0, 0),
            start: Instant::now(),
        }
    }

    pub fn from(state: EGameState) -> Self {
        let mut d_tree = Self::new();
        let d_node = EStateNode::from(vec![state]);
        d_tree
            .map
            .insert(EDirectionVec::new(), EStateTreeNode::EStateNode(d_node));
        d_tree.current.push_back(EDirectionVec::new());
        d_tree
    }

    pub fn calc(
        &mut self,
        from: EDirectionVec,
        to: EDirection,
        distance: u8,
    ) -> Result<ENodeRating> {
        let mut delete = false;
        let result;
        let calc_next_result: EStateTreeNode;
        match self.map.get_mut(&from) {
            Some(EStateTreeNode::EStateNode(node)) => {
                match node.calc_next(to, distance, &self.start, &self.duration) {
                    Ok(r) => {
                        let rating = r.rating.clone();
                        calc_next_result = EStateTreeNode::EStateNode(r);
                        result = Result::Ok(rating)
                    }
                    Err(ESimulationError::Death) => {
                        calc_next_result = EStateTreeNode::EDeath;
                        result = Result::Err(ESimulationError::Death)
                    }
                    Err(ESimulationError::Timer) => return Err(ESimulationError::Timer),
                    _ => panic!("Invalid result"),
                }
                if node.completely_evaluated() {
                    delete = true
                }
            }
            Some(EStateTreeNode::ENodeRating(_)) => panic!("Accessing rating in calc"),
            Some(EStateTreeNode::EDeath) => panic!("Accessing death in calc"),
            None => panic!("Accessing non existing node"),
        }
        let mut fromto = from.clone();
        fromto.push(to);
        self.map.insert(fromto, calc_next_result);
        if delete {
            let rating = match self.map.remove(&from) {
                Some(EStateTreeNode::EStateNode(node)) => EStateTreeNode::ENodeRating(node.rating),
                _ => panic!("Removed non node from tree"),
            };
            self.map.insert(from, rating);
        }
        result
    }

    pub fn calcs(&mut self, from: EDirectionVec, distance: u8) -> [Option<ENodeRating>; 4] {
        let mut results = [None, None, None, None];
        for d in 0..4 {
            results[d] = match self.calc(from.clone(), EDirection::from_usize(d), distance) {
                Ok(node_rating) => Some(node_rating),
                Err(ESimulationError::Death) => None,
                Err(ESimulationError::Timer) => {
                    break;
                }
                _ => panic!("Invalid result"),
            }
        }
        results
    }

    pub fn simulate_timed(&mut self, distance: u8, duration: Duration) -> [ESimulationState; 4] {
        self.duration = duration;
        self.start = Instant::now();
        let mut result: [ESimulationState; 4] = [
            ESimulationState::new(),
            ESimulationState::new(),
            ESimulationState::new(),
            ESimulationState::new(),
        ];
        let mut iteration_result: [Option<EIterationState>; 4] = [None, None, None, None];
        let mut current_depth = 0;
        let mut depth_increased;

        while self.start.elapsed() < self.duration {
            depth_increased = false;
            match self.current.pop_front() {
                None => {
                    // flush iteration results to returned results after processing que is emptied
                    for i in 0..4 {
                        result[i].update(&iteration_result[i]);
                    }
                    break;
                }
                Some(d_vec) => {
                    // determine if a new depth is starting to be evaluated and set current depth
                    if d_vec.len() > current_depth {
                        depth_increased = true;
                        current_depth = d_vec.len();
                    }

                    // flush iteration results to returned results after new depth has been reached
                    if depth_increased {
                        for i in 0..4 {
                            result[i].update(&iteration_result[i]);
                            iteration_result[i] = None;
                        }
                    }

                    // Calculate the 4 child nodes for the current node and return ratings
                    let node_ratings = self.calcs(
                        d_vec.clone(),
                        0.max(4.max(distance as i32 - 2 * current_depth as i32)) as u8,
                    );

                    // Push keys to new generated nodes to end of processing queue
                    for i in 0..4 {
                        if node_ratings[i].is_some() {
                            let mut new = d_vec.clone();
                            new.push(EDirection::from_usize(i));
                            self.current.push_back(new);
                        }
                    }

                    // If current depth is not zero all 4 new nodes belong to the same root direction
                    if current_depth != 0 {
                        // If the current depth contains already an iteration result for this root direction, update it
                        if let Some(iteration_result) =
                            iteration_result[d_vec[0].to_usize()].as_mut()
                        {
                            iteration_result.update(&node_ratings);
                        // If the current depth has no iteration result yet, create it from all 4 child nodes
                        } else {
                            iteration_result[d_vec[0].to_usize()] =
                                EIterationState::from_ratings(&node_ratings);
                        }
                    // If current depth is zero each of the 4 nodes is the root direction
                    } else {
                        for i in 0..4 {
                            iteration_result[i] = EIterationState::from_rating(&node_ratings[i])
                        }
                    }
                }
            }
        }
        // info!("Total simulation time: {:?}", self.start.elapsed());
        result
    }
}

impl Display for EStateTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (key, value) in self.map.iter() {
            s.push_str(&format!("{:?}\n", key));
            match value {
                EStateTreeNode::EStateNode(node) => s.push_str(&node.to_string()),
                EStateTreeNode::ENodeRating(rating) => s.push_str(&rating.to_string()),
                EStateTreeNode::EDeath => s.push_str("Death"),
            }
            s.push_str("\n\n");
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod json_requests {

    use crate::read_game_state;

    use super::*;

    #[ignore]
    #[test]
    fn evaluate_best_move() {
        let filename = "failure_33_do_not_move_left_as_you_can_get_killed.json";
        let s = String::from("requests/") + filename;
        let game_state = read_game_state(&s);
        let e_game_state = EGameState::from(&game_state.board, &game_state.you);
        let mut tree = EStateTree::from(e_game_state);
        let result = tree.simulate_timed(100, Duration::from_millis(100000));
        for i in 0..4 {
            println!(
                "{} {} {} {:?}",
                EDirection::from_usize(i),
                result[i].depth,
                result[i].alive,
                result[i].snake_count
            );
        }
    }
}
