use core::fmt;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
    time::{Duration, Instant},
};

use super::{
    e_direction::{EBoolDirections, EDirection, EDirectionVec},
    e_game_state::{EGameState, EStateRating},
    e_snakes::{ESimulationError, Result},
    e_state_node::EStateNode,
};

#[derive(Clone, Copy, Debug)]
pub struct ESimulationState {
    pub depth: u8,
    pub alive: bool,
    pub area: u8,
    pub food: Option<u8>,
    pub movable: bool,
}

impl ESimulationState {
    pub fn new() -> Self {
        Self {
            depth: 0,
            alive: false,
            area: 0,
            food: None,
            movable: false,
        }
    }

    pub fn from(depth: u8, alive: bool) -> Self {
        let mut n = Self::new();
        n.depth = depth;
        n.alive = alive;
        n
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

#[derive(Clone)]
pub struct EStateTree {
    map: BTreeMap<EDirectionVec, Option<EStateNode>>,
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
        d_tree.map.insert(EDirectionVec::new(), Some(d_node));
        d_tree.current.push_back(EDirectionVec::new());
        d_tree
    }

    pub fn calc(
        &mut self,
        from: EDirectionVec,
        to: EDirection,
        distance: u8,
    ) -> Result<EStateRating> {
        let mut delete = false;
        let result;
        let calc_next_result: Option<EStateNode>;
        match self.map.get_mut(&from) {
            Some(Some(node)) => {
                match node.calc_next(to, distance, &self.start, &self.duration) {
                    Ok(r) => {
                        let rating = r.rating;
                        calc_next_result = Some(r);
                        result = Result::Ok(rating)
                    }
                    Err(ESimulationError::Death) => {
                        calc_next_result = None;
                        result = Result::Err(ESimulationError::Death)
                    }
                    Err(ESimulationError::Timer) => return Err(ESimulationError::Timer),
                }
                if node.completely_evaluated() {
                    delete = true
                }
            }
            Some(None) => {
                calc_next_result = None;
                result = Result::Err(ESimulationError::Death)
            }
            _ => {
                panic!("Invalid access")
            }
        }
        let mut fromto = from.clone();
        fromto.push(to);
        self.map.insert(fromto, calc_next_result);
        if delete {
            self.map.insert(from, None);
        }
        result
    }

    pub fn calcs(&mut self, from: EDirectionVec, distance: u8) -> EBoolDirections {
        let mut results = [false; 4];
        for d in 0..4 {
            match self.calc(from.clone(), EDirection::from_usize(d), distance) {
                Ok(_) => results[d] = true, //TODO: Handle EStateRating
                Err(ESimulationError::Death) => results[d] = false,
                Err(ESimulationError::Timer) => {
                    results[d] = false;
                    break;
                }
            }
        }
        results
    }

    pub fn simulate_timed(&mut self, distance: u8, duration: Duration) -> [ESimulationState; 4] {
        self.duration = duration;
        self.start = Instant::now();
        let mut result = [ESimulationState::new(); 4];
        let mut iteration_result: [Option<ESimulationState>; 4] = [None; 4];
        let mut current_depth = 0;
        let mut depth_increased;

        while self.start.elapsed() < self.duration {
            depth_increased = false;
            match self.current.pop_front() {
                None => {
                    // println!("Finished at {}", current_depth);
                    for i in 0..4 {
                        if let Some(iteration_result) = iteration_result[i] {
                            result[i] = iteration_result;
                        }
                        iteration_result[i] = None;
                    }
                    break;
                }
                Some(d_vec) => {
                    // determine if a new depth is starting to be evaluated
                    if d_vec.len() > current_depth {
                        depth_increased = true;
                    }
                    current_depth = d_vec.len();

                    // flush iteration results to returned results after new depth has been reached
                    if depth_increased {
                        // println!("Depth increased to {}", current_depth);
                        for i in 0..4 {
                            if let Some(iteration_result) = iteration_result[i] {
                                result[i] = iteration_result;
                            }
                            iteration_result[i] = None;
                        }
                    }

                    if current_depth > 0 && iteration_result[d_vec[0].to_usize()].is_none() {
                        iteration_result[d_vec[0].to_usize()] =
                            Some(ESimulationState::from(current_depth as u8, false));
                    }

                    let bools = self.calcs(
                        d_vec.clone(),
                        0.max(distance as i32 - d_vec.len() as i32) as u8,
                    );
                    for i in 0..4 {
                        if bools[i] {
                            let mut new = d_vec.clone();
                            new.push(EDirection::from_usize(i));
                            if let Some(iteration_result) =
                                iteration_result[new[0].to_usize()].as_mut()
                            {
                                iteration_result.alive = true;
                            }
                            self.current.push_back(new);
                        }
                    }
                }
            }
        }
        info!("Total simulation time: {:?}", self.start.elapsed());
        result
    }
}

impl Display for EStateTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (key, value) in self.map.iter() {
            s.push_str(&format!("{:?}\n", key));
            match value {
                Some(node) => s.push_str(&node.to_string()),
                None => s.push_str("Completed"),
            }
            s.push_str("\n\n");
        }
        write!(f, "{}", s)
    }
}
