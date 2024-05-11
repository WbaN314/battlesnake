use core::fmt;
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
    time::{Duration, Instant},
};

use super::{
    e_direction::{EBoolDirections, EDirection, EDirectionVec},
    e_game_state::EGameState,
    e_snakes::{Death, Result},
    e_state_node::EStateNode,
};

#[derive(Clone, Copy, Debug)]
pub struct ESimulationState {
    depth: usize,
    alive: bool,
}

impl ESimulationState {
    pub fn new() -> Self {
        Self {
            depth: 0,
            alive: false,
        }
    }

    pub fn from(depth: usize, alive: bool) -> Self {
        Self { depth, alive }
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
}

impl EStateTree {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            current: VecDeque::from(Vec::new()),
        }
    }

    pub fn from(state: EGameState) -> Self {
        let mut d_tree = Self::new();
        let d_node = EStateNode::from(vec![state]);
        d_tree.map.insert(EDirectionVec::new(), Some(d_node));
        d_tree.current.push_back(EDirectionVec::new());
        d_tree
    }

    pub fn calc(&mut self, from: EDirectionVec, to: EDirection, distance: u8) -> Result<()> {
        let mut delete = false;
        let result;
        let calc_next_result: Option<EStateNode>;
        match self.map.get_mut(&from) {
            Some(Some(node)) => {
                match node.calc_next(to, distance) {
                    Ok(r) => {
                        calc_next_result = Some(r);
                        result = Result::Ok(())
                    }
                    Err(_) => {
                        calc_next_result = None;
                        result = Result::Err(Death)
                    }
                }
                if node.completely_evaluated() {
                    delete = true
                }
            }
            Some(None) => {
                calc_next_result = None;
                result = Result::Err(Death)
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
                Ok(_) => results[d] = true,
                Err(_) => results[d] = false,
            }
        }
        results
    }

    pub fn simulate_timed(&mut self, distance: u8, milliseconds: u64) -> [ESimulationState; 4] {
        let mut result = [ESimulationState::new(); 4];
        let mut iteration_result: [Option<ESimulationState>; 4] = [None; 4];
        let timer = Instant::now();
        let mut current_depth = 0;
        let mut depth_increased;

        while timer.elapsed() < Duration::from_millis(milliseconds) {
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
                            Some(ESimulationState::from(current_depth, false));
                    }

                    // println!("Pop front: {}", &d_vec);

                    let bools = self.calcs(
                        d_vec.clone(),
                        0.max(distance as i32 - d_vec.len() as i32) as u8,
                    );
                    // println!("{} {:?}", &d_vec, &bools);
                    for i in 0..4 {
                        if bools[i] {
                            let mut new = d_vec.clone();
                            new.push(EDirection::from_usize(i));
                            // println!("Push back: {}", &new);
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
