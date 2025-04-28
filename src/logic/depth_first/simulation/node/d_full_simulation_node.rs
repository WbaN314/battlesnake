use log::trace;

use crate::logic::depth_first::{
    game::{
        d_direction::{DDirection, D_DIRECTION_LIST},
        d_field::DFastField,
        d_game_state::DGameState,
        d_moves_set::DMoves,
    },
    simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
};
use std::{cell::Cell, cmp::Ordering, collections::HashMap, fmt::Display, time::Instant};

use super::{DChildrenCalculationResult, DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus};

#[derive(Default, Clone)]
pub struct DFullSimulationNode {
    id: DNodeId,
    states: Vec<DGameState<DFastField>>,
    time: DTreeTime,
    status: Cell<DNodeStatus>,
    statistics: Cell<Option<DNodeStatistics>>,
    direction_relevant_snakes: Option<[[bool; 4]; 4]>,
    current_state_index: usize,
    current_child_states: [Vec<DGameState<DFastField>>; 4],
    current_child_statuses: [DNodeStatus; 4],
    state_sameness_distance: Option<u8>,
    state_sameness_set: HashMap<u64, usize>,
    sparse_simulation_distance: Option<u8>,
    fast_child: Option<Box<Self>>,
}

impl DFullSimulationNode {
    pub fn new(
        id: DNodeId,
        states: Vec<DGameState<DFastField>>,
        time: DTreeTime,
        status: DNodeStatus,
        direction_relevant_snakes: Option<[[bool; 4]; 4]>,
        state_sameness_distance: Option<u8>,
        sparse_simulation_distance: Option<u8>,
    ) -> Self {
        let mut result = Self {
            id,
            states,
            time,
            direction_relevant_snakes,
            status: Cell::new(status),
            statistics: Cell::new(None),
            current_state_index: 0,
            current_child_states: Default::default(),
            current_child_statuses: Default::default(),
            state_sameness_distance: state_sameness_distance,
            state_sameness_set: HashMap::new(),
            sparse_simulation_distance: sparse_simulation_distance,
            fast_child: None,
        };
        result.current_child_statuses = [result.status(); 4];
        result
    }

    /// Generates all possible moves for the current state.
    fn generate_all_valid_moves(&self, state: &DGameState<DFastField>) -> (Vec<DMoves>, [bool; 4]) {
        let mut possible_moves: Vec<DMoves> = Vec::new();
        let mut possible_moves_matrix = state.possible_moves([true, true, true, true]);
        if let Some(direction_relevant_snakes) = self.direction_relevant_snakes {
            if self.id.len() == 0 {
                for d in 0..4 {
                    let mut poss_moves = state
                        .possible_moves(direction_relevant_snakes[d])
                        .generate()
                        .into_iter()
                        .filter(|m| m[0] == Some(d.try_into().unwrap()))
                        .collect::<Vec<DMoves>>();
                    possible_moves.append(&mut poss_moves);
                }
            } else {
                let direction = self.id().first().unwrap();
                possible_moves_matrix =
                    state.possible_moves(direction_relevant_snakes[*direction as usize]);
                possible_moves = possible_moves_matrix.generate();
            }
        } else {
            possible_moves = possible_moves_matrix.generate();
        }
        (possible_moves, possible_moves_matrix.get(0))
    }

    fn generate_sparse_moves(&self, state: &DGameState<DFastField>) -> (Vec<DMoves>, [bool; 4]) {
        let mut possible_moves: Vec<DMoves> = Vec::new();
        let mut possible_moves_matrix = state.possible_moves([true, true, true, true]);
        if let Some(direction_relevant_snakes) = self.direction_relevant_snakes {
            if self.id.len() == 0 {
                for d in 0..4 {
                    let mut poss_moves = state
                        .possible_moves(direction_relevant_snakes[d])
                        .generate_sparse(
                            state.get_heads(),
                            self.sparse_simulation_distance.unwrap(),
                        )
                        .into_iter()
                        .filter(|m| m[0] == Some(d.try_into().unwrap()))
                        .collect::<Vec<DMoves>>();
                    possible_moves.append(&mut poss_moves);
                }
            } else {
                let direction = self.id().first().unwrap();
                possible_moves_matrix =
                    state.possible_moves(direction_relevant_snakes[*direction as usize]);
                possible_moves = possible_moves_matrix
                    .generate_sparse(state.get_heads(), self.sparse_simulation_distance.unwrap());
            }
        } else {
            possible_moves = possible_moves_matrix
                .generate_sparse(state.get_heads(), self.sparse_simulation_distance.unwrap());
        }
        (possible_moves, possible_moves_matrix.get(0))
    }
}

impl DNode for DFullSimulationNode {
    fn id(&self) -> &DNodeId {
        &self.id
    }

    fn set_status(&mut self, status: DNodeStatus) {
        self.status.set(status);
    }

    fn status(&self) -> DNodeStatus {
        match self.status.get() {
            DNodeStatus::Unknown => {
                for state in self.states.iter() {
                    if !state.get_alive()[0] {
                        self.status.set(DNodeStatus::Dead);
                        return self.status.get();
                    }
                }
                self.status
                    .set(DNodeStatus::Alive(DNodeAliveStatus::Always));
                self.status.get()
            }
            status => status,
        }
    }

    fn calc_children(&mut self) -> DChildrenCalculationResult<Self> {
        self.time.start = Instant::now();
        let mut result = Vec::new();

        while self.current_state_index < self.states.len() {
            let state = &self.states[self.current_state_index];
            self.current_state_index += 1;
            if self.time.is_timed_out() {
                return DChildrenCalculationResult::TimedOut;
            }

            if let Some(distance) = self.state_sameness_distance {
                let hash = state.quick_hash(distance);
                if self.state_sameness_set.contains_key(&hash) {
                    continue;
                } else {
                    self.state_sameness_set
                        .insert(hash, self.current_state_index);
                }
            }

            let (possible_moves, mut possible_own_directions) =
                if self.sparse_simulation_distance.is_some() {
                    self.generate_sparse_moves(state)
                } else {
                    self.generate_all_valid_moves(state)
                };

            if possible_moves.is_empty() {
                return DChildrenCalculationResult::DeadEnd;
            }

            // If move is not in possible moves matrix make best status only alive sometimes
            for (i, d) in possible_own_directions.iter().enumerate() {
                match self.current_child_statuses[i] {
                    DNodeStatus::Alive(DNodeAliveStatus::Always) if !d => {
                        self.current_child_statuses[i] =
                            DNodeStatus::Alive(DNodeAliveStatus::Sometimes)
                    }
                    _ => (),
                }
            }

            for moveset in possible_moves.into_iter() {
                let index = moveset[0].unwrap() as usize;
                if self.current_child_statuses[index] == DNodeStatus::Dead {
                    continue;
                }
                let mut new_state = state.clone();
                new_state.next_state(moveset);

                if !new_state.get_alive()[0] {
                    // Direction is invalid as there exists a state and a moveset for this direction where we die
                    trace!(
                        "Direction {} is invalid for state\n{}",
                        moveset[0].unwrap(),
                        state
                    );

                    self.current_child_statuses[index] = DNodeStatus::Dead;
                    possible_own_directions[index] = false;

                    // If this means there is only one direction left we can try to find a fasttrack with this config
                    let only_one_direction_possible =
                        possible_own_directions.iter().filter(|&&x| x).count() == 1;
                    if only_one_direction_possible {
                        let only_possible_direction: DDirection = possible_own_directions
                            .iter()
                            .position(|&x| x)
                            .unwrap_or(0)
                            .try_into()
                            .unwrap();
                        let mut new_moveset = moveset.clone();
                        new_moveset[0] = Some(only_possible_direction.try_into().unwrap());
                        let mut new_id = self.id.clone();
                        new_id.push(new_moveset[0].unwrap());
                        new_id.set_sparse(true);
                        let mut new_state_again = state.clone();
                        new_state_again.next_state(new_moveset);
                        let fast_child_node = Box::new(Self::new(
                            new_id,
                            vec![new_state_again.clone()],
                            self.time.clone(),
                            DNodeStatus::Alive(DNodeAliveStatus::Fast),
                            self.direction_relevant_snakes,
                            self.state_sameness_distance
                                .map(|distance| 0.max(distance as i8 - 2) as u8),
                            self.sparse_simulation_distance,
                        ));
                        trace!("Only {} left for state\n{}", only_possible_direction, state);
                        self.fast_child = Some(fast_child_node);
                    }
                }
                self.current_child_states[index].push(new_state);
            }
        }
        for i in 0..4 {
            if self.current_child_states[i].is_empty() {
                self.current_child_statuses[i] = DNodeStatus::Dead;
            }
        }

        // If we have a fast node, we need to add it to the result
        if self.current_state_index == self.states.len() {
            if let Some(fast_node) = self.fast_child.take() {
                trace!(
                    "Adding fast node to simulation result {}:\n{}",
                    fast_node.id(),
                    fast_node.states[0]
                );
                result.push(fast_node);
            }
        }

        let count_non_dead_children = self
            .current_child_statuses
            .iter()
            .filter(|&&x| x != DNodeStatus::Dead)
            .count();
        if count_non_dead_children == 0 {
            trace!("Node {} is dead end, no children spawned", self.id);
            return DChildrenCalculationResult::DeadEnd;
        } else {
            if self.status.get() == DNodeStatus::Alive(DNodeAliveStatus::Fast) {
                if count_non_dead_children == 1 {
                    // Fast simulation ended
                    trace!(
                        "Ended fast node {} as dead end:\n{}",
                        self.id,
                        self.states[0]
                    );
                    return DChildrenCalculationResult::Ok(result);
                } else {
                    trace!(
                        "Ended fast node {} as fast end:\n{}",
                        self.id,
                        self.states[0]
                    );
                    return DChildrenCalculationResult::FastEnd;
                }
            } else {
                for i in 0..4 {
                    if let DNodeStatus::Alive(_) = self.current_child_statuses[i] {
                        let mut id = self.id.clone();
                        id.push(D_DIRECTION_LIST[i]);
                        result.push(Box::new(Self::new(
                            id,
                            self.current_child_states[i].clone(),
                            self.time.clone(),
                            self.current_child_statuses[i],
                            self.direction_relevant_snakes,
                            self.state_sameness_distance
                                .map(|distance| 0.max(distance as i8 - 2) as u8),
                            self.sparse_simulation_distance,
                        )));
                    }
                }
                DChildrenCalculationResult::Ok(result)
            }
        }
    }

    fn info(&self) -> String {
        format!(
            "{} {:?} {} {}",
            self.id,
            self.status(),
            self.states.len(),
            self.state_sameness_set.len()
        )
    }

    fn statistics(&self) -> DNodeStatistics {
        if self.statistics.get().is_none() {
            let mut statistics = DNodeStatistics::default();
            statistics.states = Some(self.states.len());
            for state in self.states.iter() {
                let alive_snakes = state.get_alive().iter().filter(|&&x| x).count();
                statistics.highest_alive_snakes = Some(
                    statistics
                        .highest_alive_snakes
                        .unwrap_or(alive_snakes)
                        .max(alive_snakes),
                );
                let length = state.get_length();
                if let Some(length) = length {
                    statistics.lowest_self_length =
                        Some(statistics.lowest_self_length.unwrap_or(length).min(length));
                }
            }
            self.statistics.set(Some(statistics));
        }
        self.statistics.get().unwrap()
    }

    fn direction_order(&self, other: &Self) -> Ordering {
        // Best element should be last
        let self_stats = self.statistics();
        let other_stats = other.statistics();

        self.status.cmp(&other.status).then(
            self_stats
                .highest_alive_snakes
                .cmp(&other_stats.highest_alive_snakes)
                .reverse()
                .then(
                    self_stats
                        .lowest_self_length
                        .cmp(&other_stats.lowest_self_length),
                ),
        )
    }

    fn result_order(&self, other: &Self) -> Ordering {
        // Best element should be last
        let self_stats = self.statistics();
        let other_stats = other.statistics();

        self.status.cmp(&other.status).then(
            self_stats
                .highest_alive_snakes
                .cmp(&other_stats.highest_alive_snakes)
                .reverse()
                .then(
                    self_stats
                        .lowest_self_length
                        .cmp(&other_stats.lowest_self_length),
                )
                .then(self.id.len().cmp(&other.id.len())),
        )
    }

    fn simulation_order(&self, other: &Self) -> Ordering {
        // Best element should be last
        let self_stats = self.statistics();
        let other_stats = other.statistics();

        self.status
            .cmp(&other.status)
            .then(match (self.states.len(), other.states.len()) {
                (a, b) if a > 32 && b > 32 => Ordering::Equal,
                _ => self.states.len().cmp(&other.states.len()).reverse(),
            })
            .then(
                self_stats
                    .highest_alive_snakes
                    .cmp(&other_stats.highest_alive_snakes)
                    .reverse(),
            )
            .then(
                self_stats
                    .lowest_self_length
                    .cmp(&other_stats.lowest_self_length),
            )
            .then(self.id.len().cmp(&other.id.len()))
    }
}

impl Display for DFullSimulationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id)?;
        if self.states.len() <= 10 {
            for state in &self.states {
                writeln!(f, "{}", state)?;
            }
        } else {
            for state in &self.states[0..5] {
                writeln!(f, "{}", state)?;
            }
            writeln!(f, "...\n\n")?;
            for state in &self.states[self.states.len() - 5..] {
                writeln!(f, "{}", state)?;
            }
        }
        writeln!(f, "States: {}", self.states.len())?;
        writeln!(f, "Different States: {}", self.state_sameness_set.len())?;
        writeln!(f, "Alive: {:?}", self.status())?;
        writeln!(f, "{:?}", self.status())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{cmp::Ordering, time::Duration};

    use super::DFullSimulationNode;
    use crate::{
        logic::depth_first::{
            game::{d_field::DFastField, d_game_state::DGameState},
            simulation::{
                d_node_id::DNodeId,
                d_tree::DTreeTime,
                node::{DChildrenCalculationResult, DNode, DNodeAliveStatus, DNodeStatus},
            },
        },
        read_game_state,
    };

    #[test]
    fn test_calc_children() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DFastField>::from_request(&request.board, &request.you, &request.turn);
        let mut node = DFullSimulationNode::new(
            DNodeId::default(),
            vec![gamestate],
            DTreeTime::default(),
            DNodeStatus::default(),
            None,
            None,
            None,
        );
        println!("{}", node);
        let mut children = if let DChildrenCalculationResult::Ok(children) = node.calc_children() {
            children
        } else {
            panic!("No children generated");
        };
        assert_eq!(children.len(), 2);
        assert_eq!(
            children[0].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children[0].id, DNodeId::from("U"));

        assert_eq!(
            children[1].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children[1].id, DNodeId::from("R"));
        println!("{}", children[1]);
        let children_right =
            if let DChildrenCalculationResult::Ok(children) = children[1].calc_children() {
                children
            } else {
                panic!("No children generated");
            };
        assert_eq!(children_right.len(), 3);
        assert_eq!(
            children_right[0].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[0].id, DNodeId::from("RU"));
        assert_eq!(
            children_right[1].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[1].id, DNodeId::from("RD"));

        assert_eq!(
            children_right[2].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[2].id, DNodeId::from("RR"));
    }

    #[test]
    fn test_calc_children_2() {
        let request = read_game_state("requests/test_move_request_2.json");
        let gamestate =
            DGameState::<DFastField>::from_request(&request.board, &request.you, &request.turn);
        println!(
            "{:?}",
            gamestate
                .possible_moves([true, true, true, true])
                .generate()
        );
        let mut node = DFullSimulationNode::new(
            DNodeId::default(),
            vec![gamestate],
            DTreeTime::default(),
            DNodeStatus::default(),
            None,
            None,
            None,
        );
        println!("{}", node);
        let children = if let DChildrenCalculationResult::Ok(children) = node.calc_children() {
            children
        } else {
            panic!("No children generated");
        };
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_calc_children_3() {
        let gamestate = read_game_state("requests/failure_2.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let capture_contact_depth = Some([
            [true, true, true, false],
            [true, false, true, false],
            [true, false, false, false],
            [true, false, true, false],
        ]);

        let mut root = DFullSimulationNode::new(
            DNodeId::default(),
            vec![state],
            Default::default(),
            DNodeStatus::default(),
            capture_contact_depth,
            None,
            None,
        );

        let children = if let DChildrenCalculationResult::Ok(children) = root.calc_children() {
            children
        } else {
            panic!("No children generated");
        };

        assert_eq!(children.len(), 2);

        for child in children.iter() {
            println!("{}", child.info());
        }
    }

    #[test]
    fn test_calc_children_interrupt_and_continue() {
        let gamestate = read_game_state("requests/test_game_start.json");
        let state = DGameState::<DFastField>::from_request(
            &gamestate.board,
            &gamestate.you,
            &gamestate.turn,
        );
        println!("{}", state);

        let mut state_vec = Vec::new();
        for _ in 0..100 {
            state_vec.push(state.clone());
        }

        let mut root = DFullSimulationNode::new(
            DNodeId::default(),
            state_vec,
            DTreeTime::new(Duration::from_millis(20)),
            DNodeStatus::default(),
            None,
            None,
            None,
        );

        match root.calc_children() {
            DChildrenCalculationResult::Ok(_) => {
                panic!("Should not be ok yet");
            }
            DChildrenCalculationResult::DeadEnd => panic!("Dead end"),
            DChildrenCalculationResult::TimedOut => (),
            DChildrenCalculationResult::FastEnd => panic!("Fast end"),
        }

        for _ in 0..100 {
            match root.calc_children() {
                DChildrenCalculationResult::TimedOut => return,
                _ => (),
            }
        }

        panic!("Timed out node not finished in time");
    }

    #[test]
    fn test_direction_order() {
        let request = read_game_state("requests/test_move_request.json");
        let gamestate =
            DGameState::<DFastField>::from_request(&request.board, &request.you, &request.turn);
        let request = read_game_state("requests/failure_2.json");
        let gamestate_2 =
            DGameState::<DFastField>::from_request(&request.board, &request.you, &request.turn);

        let node1 = DFullSimulationNode::new(
            DNodeId::from("U"),
            vec![gamestate.clone()],
            DTreeTime::default(),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
            None,
            None,
            None,
        );

        let node2 = DFullSimulationNode::new(
            DNodeId::from("D"),
            vec![gamestate_2.clone()],
            DTreeTime::default(),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
            None,
            None,
            None,
        );

        let node3 = DFullSimulationNode::new(
            DNodeId::from("L"),
            vec![gamestate.clone()],
            DTreeTime::default(),
            DNodeStatus::Alive(DNodeAliveStatus::Sometimes),
            None,
            None,
            None,
        );

        let node4 = DFullSimulationNode::new(
            DNodeId::from("R"),
            vec![gamestate.clone()],
            DTreeTime::default(),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
            None,
            None,
            None,
        );

        // Compare nodes
        assert_eq!(node1.direction_order(&node2), Ordering::Less);
        assert_eq!(node1.direction_order(&node3), Ordering::Greater);
        assert_eq!(node3.direction_order(&node4), Ordering::Less);
        assert_eq!(node1.direction_order(&node4), Ordering::Equal);

        let mut nodes = vec![node1, node2, node3, node4];
        nodes.sort_unstable_by(|a, b| a.direction_order(b));

        assert!(nodes[0].direction_order(&nodes[3]) == Ordering::Less);
    }
}
