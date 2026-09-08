use crate::logic::{
    general::{
        direction::{DIRECTIONS, Direction},
        field::BasicField,
        game_state::GameState,
        moves::{MoveMatrix, Moves},
        snake::Snake,
        snakes::SNAKES,
    },
    single_gamestate_nodes::{
        env_config::ENV_CONFIG, node::node_id::NodeId, situation::SituationSet,
    },
};
use core::panic;
use std::{
    collections::{HashMap, HashSet}, fmt::Display, ops::{Add, AddAssign, Deref},
};

pub mod node_id;
mod node_stats;

#[derive(Copy, Clone, Debug, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct NodeScore(pub i16);

impl Deref for NodeScore {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for NodeScore {
    type Output = NodeScore;

    fn add(self, rhs: Self) -> Self::Output {
        NodeScore(self.0 + rhs.0)
    }
}

impl AddAssign for NodeScore {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PruneReason {
    MaxDepth,
    LocalHashSimilarity,
    HeadTailDistance,
}

#[derive(Copy, Clone, Debug, Hash)]
pub enum NodeStatus {
    AliveFor(u8, NodeScore), // Number of steps where we have checked with guaranteed survival
    DeadIn(u8, NodeScore),   // Number of steps until certain death
    ProbablyDeadIn(u8, NodeScore), // Number of steps until death if opponents play optimally
    WinnerIn(u8, NodeScore), // Number of steps until inevitable victory (if opponents play optimally)
    NotSimulated,            // Status not yet determined as this direction has not been simulated
    Pruned(PruneReason),     // Node was skipped: reason given by PruneReason
}

impl NodeStatus {
    pub fn increment(self) -> NodeStatus {
        match self {
            NodeStatus::AliveFor(n, score) => NodeStatus::AliveFor(n + 1, score),
            NodeStatus::DeadIn(n, score) => NodeStatus::DeadIn(n + 1, score),
            NodeStatus::WinnerIn(n, score) => NodeStatus::WinnerIn(n + 1, score),
            NodeStatus::ProbablyDeadIn(n, score) => NodeStatus::ProbablyDeadIn(n + 1, score),
            _ => panic!("Cannot increment status: {}", self),
        }
    }

    pub fn is_comparable(self) -> bool {
        matches!(
            self,
            NodeStatus::AliveFor(_, _)
                | NodeStatus::DeadIn(_, _)
                | NodeStatus::WinnerIn(_, _)
                | NodeStatus::ProbablyDeadIn(_, _)
        )
    }

    pub fn for_comparison(self) -> Option<NodeStatus> {
        if self.is_comparable() {
            Some(self)
        } else {
            None
        }
    }

    pub fn set_score(self, score: NodeScore) -> NodeStatus {
        match self {
            NodeStatus::AliveFor(n, _) => NodeStatus::AliveFor(n, score),
            NodeStatus::DeadIn(n, _) => NodeStatus::DeadIn(n, score),
            NodeStatus::WinnerIn(n, _) => NodeStatus::WinnerIn(n, score),
            NodeStatus::ProbablyDeadIn(n, _) => NodeStatus::ProbablyDeadIn(n, score),
            _ => panic!("Cannot set score for status: {}", self),
        }
    }

    pub fn try_add_score(self, score: NodeScore) -> NodeStatus {
        match self {
            NodeStatus::AliveFor(n, s) => NodeStatus::AliveFor(n, NodeScore(s.0 + score.0)),
            NodeStatus::DeadIn(n, s) => NodeStatus::DeadIn(n, NodeScore(s.0 + score.0)),
            NodeStatus::WinnerIn(n, s) => NodeStatus::WinnerIn(n, NodeScore(s.0 + score.0)),
            NodeStatus::ProbablyDeadIn(n, s) => {
                NodeStatus::ProbablyDeadIn(n, NodeScore(s.0 + score.0))
            }
            _ => self,
        }
    }

    pub fn score(self) -> Option<NodeScore> {
        match self {
            NodeStatus::AliveFor(_, score)
            | NodeStatus::DeadIn(_, score)
            | NodeStatus::WinnerIn(_, score)
            | NodeStatus::ProbablyDeadIn(_, score) => Some(score),
            _ => None,
        }
    }

    // Max part of MinMax, pick best direction status to determine parent status.
    pub fn calculate_from_direction_states(direction_states: &[NodeStatus; 4]) -> NodeStatus {
        let best = direction_states
            .iter()
            .filter_map(|s| s.for_comparison())
            .max_by(|x, y| {
                x.partial_cmp(y)
                    .unwrap()
                    .then_with(|| x.score().unwrap().cmp(&y.score().unwrap()))
            });

        match best {
            None => NodeStatus::AliveFor(0, NodeScore(0)), // No directions explored yet
            Some(s @ NodeStatus::AliveFor(_, _)) => s.increment(),
            Some(s @ NodeStatus::WinnerIn(_, _)) => s.increment(),
            Some(s @ NodeStatus::DeadIn(_, _)) | Some(s @ NodeStatus::ProbablyDeadIn(_, _)) => {
                if direction_states
                    .iter()
                    .any(|s| matches!(s, NodeStatus::NotSimulated))
                {
                    NodeStatus::AliveFor(0, NodeScore(0)) // If there are unexplored directions, we assume we are still alive
                } else {
                    s.increment()
                }
            }
            _ => panic!("Invalid best status: {}", best.unwrap()),
        }
    }

    /// Min part of MinMax, pick worst child status to determine direction status.
    pub fn calculate_from_child_states(child_states: &Vec<(Moves, NodeStatus)>) -> NodeStatus {
        if child_states.is_empty() {
            return NodeStatus::DeadIn(0, NodeScore(0));
        }

        let worst = child_states
            .iter()
            .filter_map(|(_, s)| s.for_comparison())
            .min_by(|x, y| x.partial_cmp(y).unwrap());

        let worst_score = child_states
            .iter()
            .filter_map(|(_, s)| s.score())
            .min()
            .unwrap_or(NodeScore(0));

        match worst {
            Some(s @ NodeStatus::AliveFor(_, _)) => s.set_score(worst_score),
            Some(s @ NodeStatus::WinnerIn(_, _)) => s.set_score(worst_score),
            Some(s @ NodeStatus::ProbablyDeadIn(_, _)) => s.set_score(worst_score),
            Some(NodeStatus::DeadIn(_, _)) => {
                let disable_dead_in = child_states.iter().any(|(_, s)| {
                    matches!(s, NodeStatus::AliveFor(_, _))
                        | matches!(s, NodeStatus::ProbablyDeadIn(_, _))
                });
                let lowest_n_between_dead_and_probably_dead = child_states
                    .iter()
                    .filter_map(|(_, s)| match s {
                        NodeStatus::DeadIn(n, _) | NodeStatus::ProbablyDeadIn(n, _) => Some(*n),
                        _ => None,
                    })
                    .min()
                    .unwrap();
                if disable_dead_in {
                    NodeStatus::ProbablyDeadIn(lowest_n_between_dead_and_probably_dead, worst_score)
                } else {
                    NodeStatus::DeadIn(lowest_n_between_dead_and_probably_dead, worst_score)
                }
            }
            None => {
                if child_states
                    .iter()
                    .filter(|(_, status)| {
                        !matches!(status, NodeStatus::Pruned(PruneReason::LocalHashSimilarity))
                    })
                    .all(|(_, status)| matches!(status, NodeStatus::Pruned(PruneReason::MaxDepth)))
                {
                    NodeStatus::AliveFor(0, NodeScore(0))
                } else {
                    panic!("All children have weird states: {:?}", child_states);
                }
            }
            _ => panic!("Invalid worst status: {}", worst.unwrap()),
        }
    }
}

impl Eq for NodeStatus {}

impl PartialEq for NodeStatus {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for NodeStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (NodeStatus::AliveFor(n, _), NodeStatus::AliveFor(m, _)) => Some(n.cmp(m)),
            (NodeStatus::DeadIn(n, _), NodeStatus::DeadIn(m, _)) => Some(n.cmp(m)),
            (NodeStatus::WinnerIn(n, _), NodeStatus::WinnerIn(m, _)) => Some(m.cmp(n)),
            (NodeStatus::ProbablyDeadIn(n, _), NodeStatus::ProbablyDeadIn(m, _)) => Some(n.cmp(m)),

            // Alive, Dead
            (NodeStatus::AliveFor(_, _), NodeStatus::DeadIn(_, _)) => {
                Some(std::cmp::Ordering::Greater)
            }
            (a @ NodeStatus::DeadIn(_, _), b @ NodeStatus::AliveFor(_, _)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // Alive, Winner
            (NodeStatus::AliveFor(_, _), NodeStatus::WinnerIn(_, _)) => {
                Some(std::cmp::Ordering::Less)
            }
            (a @ NodeStatus::WinnerIn(_, _), b @ NodeStatus::AliveFor(_, _)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // Dead, Winner
            (NodeStatus::DeadIn(_, _), NodeStatus::WinnerIn(_, _)) => {
                Some(std::cmp::Ordering::Less)
            }
            (a @ NodeStatus::WinnerIn(_, _), b @ NodeStatus::DeadIn(_, _)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // ProbablyDead, Alive
            (NodeStatus::ProbablyDeadIn(_, _), NodeStatus::AliveFor(_, _)) => {
                Some(std::cmp::Ordering::Less)
            }
            (a @ NodeStatus::AliveFor(_, _), b @ NodeStatus::ProbablyDeadIn(_, _)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // ProbablyDead, Dead
            (NodeStatus::ProbablyDeadIn(_, _), NodeStatus::DeadIn(_, _)) => {
                Some(std::cmp::Ordering::Greater)
            }
            (a @ NodeStatus::DeadIn(_, _), b @ NodeStatus::ProbablyDeadIn(_, _)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            // ProbablyDead, Winner
            (NodeStatus::ProbablyDeadIn(_, _), NodeStatus::WinnerIn(_, _)) => {
                Some(std::cmp::Ordering::Less)
            }
            (a @ NodeStatus::WinnerIn(_, _), b @ NodeStatus::ProbablyDeadIn(_, _)) => {
                b.partial_cmp(a).map(|o| o.reverse())
            }

            (NodeStatus::NotSimulated, NodeStatus::NotSimulated) => Some(std::cmp::Ordering::Equal),
            (NodeStatus::Pruned(a), NodeStatus::Pruned(b)) if a == b => {
                Some(std::cmp::Ordering::Equal)
            }
            _ => None,
        }
    }
}

impl Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::AliveFor(n, s) => write!(f, "AliveFor({}, {:.0})", n, s.0),
            NodeStatus::DeadIn(n, s) => write!(f, "DeadIn({}, {:.0})", n, s.0),
            NodeStatus::ProbablyDeadIn(n, s) => write!(f, "ProbablyDeadIn({}, {:.0})", n, s.0),
            NodeStatus::WinnerIn(n, s) => write!(f, "WinnerIn({}, {:.0})", n, s.0),
            NodeStatus::NotSimulated => write!(f, "NotSimulated"),
            NodeStatus::Pruned(PruneReason::MaxDepth) => write!(f, "Pruned(MaxDepth)"),
            NodeStatus::Pruned(PruneReason::LocalHashSimilarity) => {
                write!(f, "Pruned(LocalHashSimilarity)")
            }
            NodeStatus::Pruned(PruneReason::HeadTailDistance) => {
                write!(f, "Pruned(HeadTailDistance)")
            }
        }
    }
}

#[derive(Clone)]
pub struct Node {
    id: NodeId,
    gamestate: GameState<BasicField>,
    children_states_per_direction: [Vec<(Moves, NodeStatus)>; 4],
    direction_states: [NodeStatus; 4],
    status: NodeStatus,
    local_score: NodeScore,
    pinned_status: Option<NodeStatus>,
    priority: i8,
    move_matrix: Option<MoveMatrix>,
    ordered_directions: Option<[Direction; 4]>, // Order in which directions should be simulated
    head_tail_distance: Option<u8>,
}

impl Node {
    pub fn new(id: NodeId, gamestate: GameState<BasicField>) -> Self {
        let status = if !gamestate.is_alive(0) {
            NodeStatus::DeadIn(0, NodeScore(0))
        } else if gamestate.is_winner(0) {
            NodeStatus::WinnerIn(0, NodeScore(0))
        } else {
            NodeStatus::AliveFor(0, NodeScore(0))
        };

        Self {
            id,
            gamestate,
            children_states_per_direction: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            direction_states: [NodeStatus::NotSimulated; 4],
            status,
            pinned_status: None,
            priority: 0,
            move_matrix: None,
            ordered_directions: None,
            local_score: NodeScore(0),
            head_tail_distance: None,
        }
    }

    /// Pin the node to a specific status.
    pub fn pin_status(&mut self, status: NodeStatus) {
        if let Some(pinned) = self.pinned_status {
            assert!(
                pinned == status,
                "Node {} already pinned as {}, cannot pin as {}",
                self.id,
                pinned,
                status
            );
        }
        self.pinned_status = Some(status);
    }

    pub fn set_moves(&mut self, moves: Moves) {
        self.move_matrix = Some(moves.into());
    }

    pub fn set_priority(&mut self, priority: i8) {
        self.priority = priority;
    }

    pub fn read_priority(&self) -> i8 {
        self.priority
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn with_score(
        mut self,
        situations: Option<&SituationSet>,
        parent_dead_snake_count: usize,
        parent_length: Option<u8>,
    ) -> Self {
        let our_dead_snake_count = self
            .gamestate
            .snakes()
            .clone()
            .into_iter()
            .skip(1)
            .filter(|s| matches!(s.get(), Snake::Dead { .. }))
            .count();

        let our_length = if let Snake::Alive { length, .. } = self.gamestate.snakes().cell(0).get()
        {
            Some(length)
        } else {
            None
        };

        self.local_score += NodeScore(
            (our_dead_snake_count as i16 - parent_dead_snake_count as i16)
                * ENV_CONFIG.SCORE_SIMULATION_KILL as i16,
        );

        if let (Some(our_length), Some(parent_length)) = (our_length, parent_length) {
            self.local_score += NodeScore(
                (our_length as i16 - parent_length as i16)
                    * ENV_CONFIG.SCORE_SIMULATION_FOOD as i16,
            );
        }

        if self.status == NodeStatus::WinnerIn(0, NodeScore(0)) {
            self.local_score += NodeScore(ENV_CONFIG.SCORE_SIMULATION_WINNER as i16);
        }

        if let Some(score) = situations.and_then(|s| s.score(&self.gamestate)) {
            self.local_score += NodeScore(score as i16);
        }

        self.status = self.status.try_add_score(self.local_score);
        self
    }

    pub fn status(&self) -> NodeStatus {
        if let Some(pinned) = self.pinned_status {
            return pinned;
        } else {
            return self.status;
        }
    }

    pub fn direction_status(&self, direction: Direction) -> NodeStatus {
        if let Some(pinned) = self.pinned_status {
            return pinned;
        } else {
            self.direction_states[direction as usize]
        }
    }

    fn update_direction_status_and_score(&mut self, direction: Direction) -> bool {
        let old_status = self.direction_states[direction as usize];
        let children = &self.children_states_per_direction[direction as usize];
        let new_state = NodeStatus::calculate_from_child_states(children);
        self.direction_states[direction as usize] = new_state;
        old_status != new_state
    }

    fn update_status(&mut self) -> bool {
        let old_status = self.status;
        self.status = NodeStatus::calculate_from_direction_states(&self.direction_states)
            .try_add_score(self.local_score);
        old_status != self.status
    }

    pub fn handle_update_from_child(&mut self, child_id: NodeId, child_status: NodeStatus) -> bool {
        let last_moves: Moves = child_id.last_directions().unwrap();
        let direction = last_moves[0].unwrap();

        // Find the child entry corresponding to the last moves and update its status
        if let Some(entry) = self.children_states_per_direction[direction as usize]
            .iter_mut()
            .find(|(dv, _)| *dv == last_moves)
        {
            entry.1 = child_status;
        }

        // Update the direction status based on the updated child status
        if self.update_direction_status_and_score(direction) {
            // If the direction status has changed, update the overall node status
            self.update_status()
        } else {
            false
        }
    }

    pub fn gamestate(&self) -> &GameState<BasicField> {
        &self.gamestate
    }

    /// For stats usage only, not for simulation.
    pub fn children(&self) -> [Vec<(NodeId, NodeStatus)>; 4] {

        if self.id.depth() >= NodeId::MAX_DEPTH {
            return Default::default();
        }

        let mut baseline: [HashMap<NodeId, NodeStatus>; 4] =
            self.children_states_per_direction.clone().map(|vec| {
                vec.into_iter()
                    .map(|(moves, s)| (self.id.child(moves), s))
                    .collect::<HashMap<NodeId, NodeStatus>>()
            });

        let valid_moves = self.gamestate.valid_moves();

        for direction in DIRECTIONS {
            if self.direction_status(direction) == NodeStatus::NotSimulated {
                let moves = valid_moves.pregenerate_for(direction);
                for moves in moves {
                    let child_id = self.id.child(moves);
                    baseline[direction as usize].entry(child_id).or_insert(
                        NodeStatus::NotSimulated,
                    );
                }
            } else {
                let moves = valid_moves.pregenerate_for(direction);
                for moves in moves {
                    let child_id = self.id.child(moves);
                    baseline[direction as usize].entry(child_id).or_insert(
                        NodeStatus::Pruned(PruneReason::HeadTailDistance),
                    );
                }
            }
        }

        baseline.map(|hm| hm.into_iter().collect::<Vec<(NodeId, NodeStatus)>>()).try_into().unwrap()
    }

    pub fn prepare_simulation(&mut self, direction_preference_situations: Option<&SituationSet>, head_tail_distance: Option<u8>) {
        if self.move_matrix.is_none() {
            self.head_tail_distance = head_tail_distance;
            let mut move_matrix = self.gamestate.valid_moves();
            if let Some(distance) = head_tail_distance {
                move_matrix = move_matrix.prune_head_tail(&self.gamestate, distance);
            }
            self.move_matrix = Some(move_matrix);
        }
        if self.ordered_directions.is_none() {
            self.ordered_directions = Some(self.order_directions(direction_preference_situations));
        }
    }

    /// This method can be called multiple times to simulate the node in a stepwise manner. It will return None when all directions have been simulated.
    pub fn simulate(
        &mut self,
        similarity_pruning_distance: Option<u8>,
        head_tail_distance: Option<u8>,
        direction_preference_situations: Option<&SituationSet>,
        score_situations: Option<&SituationSet>,
    ) -> Option<Vec<Node>> {
        debug_assert!(
            !matches!(self.status, NodeStatus::WinnerIn(0, _)),
            "Should never simulate a node that is a new winner"
        );
        self.prepare_simulation(direction_preference_situations, head_tail_distance);

        let dead_snake_count = self
            .gamestate
            .snakes()
            .clone()
            .into_iter()
            .skip(1)
            .filter(|s| matches!(s.get(), Snake::Dead { .. }))
            .count();

        let length = if let Snake::Alive { length, .. } = self.gamestate.snakes().cell(0).get() {
            Some(length)
        } else {
            None
        };

        'direction: while let Some(direction) = self.next_direction() {
            let mut children: Vec<Node> = Vec::new();
            let mut similarity_set: HashSet<u64> = HashSet::new();

            for moves in self
                .move_matrix
                .as_ref()
                .unwrap()
                .pregenerate_for(direction)
            {
                let mut child_gamestate = self.gamestate.clone();
                let child_id = self.id.child(moves);
                child_gamestate.next_state(moves);

                // Similarity pruning: if a similar gamestate has already been simulated, skip this child
                if let Some(dist) = similarity_pruning_distance {
                    let hash = child_gamestate.local_environment_hash(dist);
                    if !similarity_set.insert(hash) {
                        self.children_states_per_direction[direction as usize]
                            .push((moves, NodeStatus::Pruned(PruneReason::LocalHashSimilarity)));
                        continue;
                    }
                }

                let child = Node::new(child_id, child_gamestate).with_score(
                    score_situations,
                    dead_snake_count,
                    length,
                );
                let child_status = child.status();

                self.children_states_per_direction[direction as usize].push((moves, child_status));
                match child_status {
                    NodeStatus::DeadIn(0, _) => {
                        // Child is dead, do not add to children
                    }
                    NodeStatus::AliveFor(0, _) => {
                        children.push(child);
                    }
                    NodeStatus::WinnerIn(0, _) => {
                        children.push(child);
                    }
                    _ => {
                        panic!("Invalid child status: {}", child_status);
                    }
                }
            }

            self.update_direction_status_and_score(direction.into());

            if matches!(
                self.direction_status(direction),
                NodeStatus::DeadIn(_, _) | NodeStatus::ProbablyDeadIn(_, _)
            ) {
                continue 'direction;
            }

            // Node must spawn children
            debug_assert!(!children.is_empty());
            self.update_status();
            return Some(children);
        }
        self.update_status();
        return None;
    }

    fn next_direction(&mut self) -> Option<Direction> {
        for d in self.ordered_directions.unwrap() {
            if self.direction_states[d as usize] == NodeStatus::NotSimulated {
                if self.move_matrix.as_ref().unwrap().get(0).is_valid(d) {
                    return Some(d);
                } else {
                    self.update_direction_status_and_score(d.into());
                }
            }
        }
        self.update_status();
        None
    }

    fn order_directions(
        &self,
        direction_preference_situations: Option<&SituationSet>,
    ) -> [Direction; 4] {
        let mut preferred_directions = DIRECTIONS;
        let mut distance = u8::MAX;
        if let Snake::Alive { head: my_head, .. } = self.gamestate.snakes().cell(0).get() {
            for i in 1..SNAKES {
                if let Snake::Alive { head, .. } = self.gamestate.snakes().cell(i as u8).get() {
                    if my_head.distance_to(head) < distance {
                        distance = my_head.distance_to(head);
                        let directions = my_head.directions_to(head);
                        if let Some(dir1) = directions[1] {
                            let dir0 = directions[0].unwrap();
                            preferred_directions[0] = dir0;
                            preferred_directions[1] = dir1;
                            preferred_directions[2] = dir0.inverse();
                            preferred_directions[3] = dir1.inverse();
                        } else if let Some(dir0) = directions[0] {
                            if matches!(dir0, Direction::Up | Direction::Down) {
                                preferred_directions[0] = dir0;
                                preferred_directions[1] = dir0.inverse();
                                preferred_directions[2] = Direction::Left;
                                preferred_directions[3] = Direction::Right;
                            } else {
                                preferred_directions[0] = dir0;
                                preferred_directions[1] = dir0.inverse();
                                preferred_directions[2] = Direction::Up;
                                preferred_directions[3] = Direction::Down;
                            }
                            preferred_directions[0] = dir0;
                        }
                    }
                }
            }
        }

        if let Some(situations) = direction_preference_situations {
            if let Some(situation_match) = situations.check(&self.gamestate) {
                if let Some(preferred) = situation_match[0] {
                    if let Some(pos) = preferred_directions.iter().position(|&d| d == preferred) {
                        preferred_directions.copy_within(0..pos, 1);
                        preferred_directions[0] = preferred;
                    }
                }
            }
        }

        preferred_directions
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{} {}", self.id, self.status())?;
        for (i, children) in self.children_states_per_direction.iter().enumerate() {
            let dir = Direction::try_from(i).unwrap();
            let dir_status = self.direction_states[i];
            if dir_status == NodeStatus::NotSimulated {
                writeln!(f, "  {} unexplored", dir)?;
            } else {
                writeln!(f, "  {} {} ({} children)", dir, dir_status, children.len())?;
                for (dv, child_status) in children {
                    writeln!(f, "    {} {}", self.id.child(*dv), child_status)?;
                }
            }
        }
        writeln!(f, "\n{}", self.gamestate)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::read_game_state;

    use super::*;

    #[test]
    fn node_status_ordering() {
        assert!(NodeStatus::AliveFor(5, NodeScore(0)) > NodeStatus::AliveFor(3, NodeScore(0)));
        assert!(NodeStatus::DeadIn(5, NodeScore(0)) > NodeStatus::DeadIn(1, NodeScore(0)));
        assert!(NodeStatus::AliveFor(0, NodeScore(0)) > NodeStatus::DeadIn(100, NodeScore(0)));
        assert_ne!(
            NodeStatus::AliveFor(3, NodeScore(0)),
            NodeStatus::DeadIn(3, NodeScore(0))
        );
        assert!(NodeStatus::WinnerIn(1, NodeScore(0)) > NodeStatus::WinnerIn(3, NodeScore(0)));
        assert!(NodeStatus::WinnerIn(1, NodeScore(0)) > NodeStatus::AliveFor(100, NodeScore(0)));
        assert!(NodeStatus::WinnerIn(1, NodeScore(0)) > NodeStatus::DeadIn(100, NodeScore(0)));
        assert_eq!(
            NodeStatus::AliveFor(5, NodeScore(1000))
                .partial_cmp(&NodeStatus::AliveFor(5, NodeScore(0))),
            Some(std::cmp::Ordering::Equal)
        );
        let statuses = vec![
            NodeStatus::DeadIn(5, NodeScore(0)),
            NodeStatus::AliveFor(2, NodeScore(0)),
            NodeStatus::DeadIn(0, NodeScore(0)),
            NodeStatus::AliveFor(0, NodeScore(0)),
        ];
        assert_eq!(
            statuses
                .iter()
                .filter_map(|x| x.for_comparison())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            NodeStatus::AliveFor(2, NodeScore(0))
        );
        assert_eq!(
            statuses
                .iter()
                .filter_map(|x| x.for_comparison())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            NodeStatus::DeadIn(0, NodeScore(0))
        );
    }

    fn make_root_node(json_path: &str) -> Node {
        let gamestate = read_game_state(json_path);
        let state = GameState::<BasicField>::from(&gamestate);
        Node::new(NodeId::new(), state)
    }

    #[test]
    fn simulate_exhausts_all_directions() {
        let mut node = make_root_node("requests/example_move_request.json");
        println!("{}", node);
        while node.simulate(None, None, None, None).is_some() {
            node.simulate(None, None, None, None);
        }
        for i in DIRECTIONS {
            let status = node.direction_status(i);
            assert!(
                matches!(
                    status,
                    NodeStatus::AliveFor(0, _) | NodeStatus::DeadIn(0, _)
                ),
                "direction {} has unexpected status: {}",
                i,
                status
            );
        }
        println!("{}", node);
        assert!(node.simulate(None, None, None, None).is_none());
    }

    #[test]
    fn display_half_simulated_node() {
        let mut node = make_root_node("requests/test_game_start.json");
        node.simulate(None, None, None, None);
        println!("{}", node);
    }

    #[test]
    fn calculate_from_direction_states_cases() {
        assert_eq!(
            NodeStatus::calculate_from_direction_states(&[NodeStatus::NotSimulated; 4]),
            NodeStatus::AliveFor(0, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_direction_states(&[
                NodeStatus::AliveFor(3, NodeScore(0)),
                NodeStatus::AliveFor(1, NodeScore(100)),
                NodeStatus::ProbablyDeadIn(5, NodeScore(100)),
                NodeStatus::DeadIn(2, NodeScore(0))
            ]),
            NodeStatus::AliveFor(4, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_direction_states(&[
                NodeStatus::WinnerIn(2, NodeScore(0)),
                NodeStatus::AliveFor(5, NodeScore(0)),
                NodeStatus::ProbablyDeadIn(8, NodeScore(0)),
                NodeStatus::DeadIn(1, NodeScore(0))
            ]),
            NodeStatus::WinnerIn(3, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_direction_states(&[
                NodeStatus::DeadIn(5, NodeScore(0)),
                NodeStatus::DeadIn(3, NodeScore(0)),
                NodeStatus::NotSimulated,
                NodeStatus::ProbablyDeadIn(1, NodeScore(0))
            ]),
            NodeStatus::AliveFor(0, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_direction_states(&[
                NodeStatus::DeadIn(5, NodeScore(0)),
                NodeStatus::AliveFor(1, NodeScore(0)),
                NodeStatus::ProbablyDeadIn(3, NodeScore(0)),
                NodeStatus::DeadIn(1, NodeScore(0))
            ]),
            NodeStatus::AliveFor(2, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_direction_states(&[
                NodeStatus::DeadIn(5, NodeScore(0)),
                NodeStatus::DeadIn(3, NodeScore(0)),
                NodeStatus::ProbablyDeadIn(3, NodeScore(0)),
                NodeStatus::DeadIn(1, NodeScore(0))
            ]),
            NodeStatus::ProbablyDeadIn(4, NodeScore(0))
        );
    }

    #[test]
    fn calculate_from_child_states_cases() {
        let dummy: Moves = [None; 4];

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![]),
            NodeStatus::DeadIn(0, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![
                (dummy, NodeStatus::Pruned(PruneReason::MaxDepth)),
                (dummy, NodeStatus::Pruned(PruneReason::MaxDepth)),
            ]),
            NodeStatus::AliveFor(0, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![
                (dummy, NodeStatus::WinnerIn(1, NodeScore(100))),
                (dummy, NodeStatus::AliveFor(5, NodeScore(200))),
            ]),
            NodeStatus::AliveFor(5, NodeScore(100))
        );

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![
                (dummy, NodeStatus::AliveFor(3, NodeScore(0))),
                (dummy, NodeStatus::DeadIn(2, NodeScore(0))),
                (dummy, NodeStatus::WinnerIn(1, NodeScore(0))),
            ]),
            NodeStatus::ProbablyDeadIn(2, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![
                (dummy, NodeStatus::DeadIn(1, NodeScore(0))),
                (dummy, NodeStatus::DeadIn(5, NodeScore(0))),
                (dummy, NodeStatus::ProbablyDeadIn(10, NodeScore(100))),
            ]),
            NodeStatus::ProbablyDeadIn(1, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![
                (dummy, NodeStatus::DeadIn(10, NodeScore(0))),
                (dummy, NodeStatus::DeadIn(5, NodeScore(0))),
                (dummy, NodeStatus::ProbablyDeadIn(1, NodeScore(0))),
            ]),
            NodeStatus::ProbablyDeadIn(1, NodeScore(0))
        );

        assert_eq!(
            NodeStatus::calculate_from_child_states(&vec![
                (dummy, NodeStatus::WinnerIn(1, NodeScore(0))),
                (dummy, NodeStatus::DeadIn(2, NodeScore(0))),
                (dummy, NodeStatus::DeadIn(2, NodeScore(0))),
            ]),
            NodeStatus::DeadIn(2, NodeScore(0))
        );
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use std::hint::black_box;

    use super::*;
    use crate::read_game_state;

    fn test_nodes() -> Vec<Node> {
        [
            "requests/failure_01.json",
            "requests/failure_03.json",
            "requests/failure_04.json",
            "requests/failure_05.json",
            "requests/example_move_request_2.json",
            "requests/example_move_request_3.json",
        ]
        .iter()
        .map(|p| {
            let gamestate = read_game_state(p);
            let state = GameState::<BasicField>::from(&gamestate);
            Node::new(NodeId::new(), state)
        })
        .collect()
    }

    #[bench]
    fn bench_node_simulate(b: &mut test::Bencher) {
        let source_nodes = test_nodes();
        let mut i = 0;
        b.iter(|| {
            let mut node = source_nodes[i % source_nodes.len()].clone();
            i += 1;
            black_box(node.simulate(black_box(None), None, None, None))
        });
    }

    #[bench]
    fn bench_node_status(b: &mut test::Bencher) {
        let nodes: Vec<Node> = test_nodes()
            .into_iter()
            .map(|mut n| {
                n.simulate(None, None, None, None);
                n
            })
            .collect();
        let mut i = 0;
        b.iter(|| {
            let node = &nodes[i % nodes.len()];
            i += 1;
            black_box(node.status())
        });
    }

    #[bench]
    fn bench_node_propagate_update_from_child(b: &mut test::Bencher) {
        let prepared: Vec<(Node, NodeId, NodeStatus)> = test_nodes()
            .into_iter()
            .filter_map(|mut parent| {
                let children = parent.simulate(None, None, None, None)?;
                let (child_id, child_status) = children.first().map(|c| (c.id(), c.status()))?;
                Some((parent, child_id, child_status))
            })
            .collect();

        let mut i = 0;
        b.iter(|| {
            let (parent, child_id, child_status) = &prepared[i % prepared.len()];
            i += 1;
            let mut node = parent.clone();
            black_box(node.handle_update_from_child(black_box(*child_id), black_box(*child_status)))
        });
    }
}
