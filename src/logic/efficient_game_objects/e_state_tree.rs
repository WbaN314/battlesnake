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

    pub fn simulate_timed(&mut self, distance: u8, milliseconds: u64) -> [usize; 4] {
        let mut result = [0; 4];

        let timer = Instant::now();
        while timer.elapsed() < Duration::from_millis(milliseconds) {
            match self.current.pop_front() {
                None => break,
                Some(d_vec) => {
                    let bools = self.calcs(
                        d_vec.clone(),
                        0.max(distance as i32 - d_vec.len() as i32) as u8,
                    );
                    // println!("{:?} {:?}", &d_vec, &bools);
                    for i in 0..4 {
                        if bools[i] {
                            let mut new = d_vec.clone();
                            new.push(EDirection::from_usize(i));
                            self.current.push_back(new);
                        }
                    }
                }
            }
        }

        for key in self.map.keys().rev() {
            // if self.map.get(key).unwrap().is_some() {
            //    println!("{:?} {}", key, self.map.get(key).unwrap().clone().unwrap())
            // }
            if key.len() == 0 {
                break;
            } else if result[key[0].to_usize()] < key.len() {
                result[key[0].to_usize()] = key.len();
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
