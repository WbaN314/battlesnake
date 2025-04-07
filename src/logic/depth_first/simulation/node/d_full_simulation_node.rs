use crate::logic::depth_first::{
    game::{
        d_direction::D_DIRECTION_LIST,
        d_field::DFastField,
        d_game_state::DGameState,
        d_moves_set::{DMoves, DMovesSet},
    },
    simulation::{d_node_id::DNodeId, d_tree::DTreeTime},
};
use std::{cell::Cell, cmp::Ordering, collections::HashMap, fmt::Display, time::Instant};

use super::{DNode, DNodeAliveStatus, DNodeStatistics, DNodeStatus};

#[derive(Default)]
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
}

impl DFullSimulationNode {
    pub fn new(
        id: DNodeId,
        states: Vec<DGameState<DFastField>>,
        time: DTreeTime,
        status: DNodeStatus,
        direction_relevant_snakes: Option<[[bool; 4]; 4]>,
        state_sameness_distance: Option<u8>,
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
            sparse_simulation_distance: None,
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

    fn calc_children(&mut self) -> Vec<Box<Self>> {
        self.time.start = Instant::now();
        let mut timed_out = false;
        while self.current_state_index < self.states.len() {
            let state = &self.states[self.current_state_index];
            self.current_state_index += 1;
            if self.time.is_timed_out() {
                timed_out = true;
                break;
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

            let (possible_moves, possible_moves_matrix) =
                if self.sparse_simulation_distance.is_some() {
                    self.generate_sparse_moves(state)
                } else {
                    self.generate_all_valid_moves(state)
                };

            if possible_moves.is_empty() {
                self.status.set(DNodeStatus::DeadEnd);
                return Vec::new();
            }

            // If move is not in possible moves matrix make best status only alive sometimes
            for (i, d) in possible_moves_matrix.iter().enumerate() {
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
                    self.current_child_statuses[index] = DNodeStatus::Dead;
                }
                self.current_child_states[index].push(new_state);
            }
        }
        for i in 0..4 {
            if self.current_child_states[i].is_empty()
                && self.current_child_statuses[i] != DNodeStatus::TimedOut
            {
                self.current_child_statuses[i] = DNodeStatus::Dead;
            }
        }
        let mut result = Vec::new();
        for i in 0..4 {
            let mut id = self.id.clone();
            id.push(D_DIRECTION_LIST[i]);
            result.push(Box::new(Self::new(
                id,
                self.current_child_states[i].clone(),
                self.time.clone(),
                if timed_out {
                    DNodeStatus::TimedOut
                } else {
                    self.current_child_statuses[i]
                },
                self.direction_relevant_snakes,
                self.state_sameness_distance
                    .map(|distance| 0.max(distance as i8 - 2) as u8),
            )));
        }
        result
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
                node::{DNode, DNodeAliveStatus, DNodeStatus},
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
        );
        println!("{}", node);
        let mut children = node.calc_children();
        assert_eq!(children.len(), 4);
        assert_eq!(
            children[0].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children[0].id, DNodeId::from("U"));
        assert_eq!(children[1].status(), DNodeStatus::Dead);
        assert_eq!(children[1].id, DNodeId::from("D"));
        assert_eq!(children[2].status(), DNodeStatus::Dead);
        assert_eq!(children[2].id, DNodeId::from("L"));
        assert_eq!(
            children[3].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children[3].id, DNodeId::from("R"));
        println!("{}", children[3]);
        let children_right = children[3].calc_children();
        assert_eq!(children_right.len(), 4);
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
        assert_eq!(children_right[2].status(), DNodeStatus::Dead);
        assert_eq!(children_right[2].id, DNodeId::from("RL"));
        assert_eq!(
            children_right[3].status(),
            DNodeStatus::Alive(DNodeAliveStatus::Always)
        );
        assert_eq!(children_right[3].id, DNodeId::from("RR"));
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
        );
        println!("{}", node);
        let children = node.calc_children();
        println!("{}", children[0]);
        println!("{}", children[1]);
        println!("{}", children[2]);
        println!("{}", children[3]);
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
        );

        let children = root.calc_children();

        assert_ne!(root.status(), DNodeStatus::DeadEnd);
        assert_eq!(children.len(), 4);

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
        );

        let children = root.calc_children();

        for child in children {
            assert_eq!(child.status(), DNodeStatus::TimedOut);
        }

        for _ in 0..100 {
            let children = root.calc_children();
            println!("{}", root.current_state_index);
            if children[0].status() != DNodeStatus::TimedOut {
                return;
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
        );

        let node2 = DFullSimulationNode::new(
            DNodeId::from("D"),
            vec![gamestate_2.clone()],
            DTreeTime::default(),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
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
        );

        let node4 = DFullSimulationNode::new(
            DNodeId::from("R"),
            vec![gamestate.clone()],
            DTreeTime::default(),
            DNodeStatus::Alive(DNodeAliveStatus::Always),
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
