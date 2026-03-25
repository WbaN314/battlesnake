use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::time::Duration;

use crate::logic::{
    game::direction::Direction,
    new_year_new_snake::node::{Node, NodeStatus, node_id::NodeId},
};

use super::Tree;

#[derive(Debug)]
pub struct TreeStats {
    pub total_nodes: usize,
    pub total_potential_children: usize,
    pub total_tracked_children: usize,
    pub max_depth_reached: u8,
    pub nodes_per_depth: Vec<(u8, usize)>,
    pub pruned_per_depth: Vec<(u8, usize)>,
    pub nodes_by_status: Vec<(NodeStatus, usize)>,
    pub leaf_nodes: usize,
    pub alive_leaves: usize,
    pub avg_leaf_depth: f64,
    pub median_leaf_depth: f64,
    pub root_status: NodeStatus,
    pub direction_stats: Vec<DirectionStats>,
    pub queue_remaining: usize,
    pub avg_branching_factor: f64,
    pub memory_estimate_bytes: usize,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct DirectionStats {
    pub direction: Direction,
    pub status: Option<NodeStatus>,
    pub subtree_size: usize,
    pub max_depth: u8,
}

impl Tree {
    pub fn stats(&self) -> TreeStats {
        let root_id = NodeId::new();
        let root = &self.nodes[&root_id];

        // Group nodes by depth and build parent->children map
        let mut by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        let mut children_map: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for &id in self.nodes.keys() {
            *by_depth.entry(id.depth()).or_default() += 1;
            if let Some(parent_id) = id.parent() {
                children_map.entry(parent_id).or_default().push(id);
            }
        }

        let max_depth_reached = by_depth.keys().last().copied().unwrap_or(0);
        let nodes_per_depth: Vec<(u8, usize)> = by_depth.into_iter().collect();

        // Leaf nodes = nodes with no children in the tree
        let leaf_ids: Vec<&NodeId> = self
            .nodes
            .keys()
            .filter(|id| !children_map.contains_key(id))
            .collect();
        let leaf_nodes = leaf_ids.len();

        // Leaf depth distribution
        let mut leaf_depths: Vec<u8> = leaf_ids.iter().map(|id| id.depth()).collect();
        leaf_depths.sort();
        let avg_leaf_depth = if leaf_depths.is_empty() {
            0.0
        } else {
            leaf_depths.iter().map(|&d| d as f64).sum::<f64>() / leaf_depths.len() as f64
        };
        let median_leaf_depth = if leaf_depths.is_empty() {
            0.0
        } else {
            let mid = leaf_depths.len() / 2;
            if leaf_depths.len() % 2 == 0 {
                (leaf_depths[mid - 1] as f64 + leaf_depths[mid] as f64) / 2.0
            } else {
                leaf_depths[mid] as f64
            }
        };

        // Count nodes by exact status
        let mut status_counts: BTreeMap<NodeStatus, usize> = BTreeMap::new();
        for node in self.nodes.values() {
            *status_counts.entry(node.status()).or_default() += 1;
        }
        let nodes_by_status: Vec<(NodeStatus, usize)> = status_counts.into_iter().collect();

        // Total children tracked internally by nodes (includes pruned dead directions)
        let total_tracked_children: usize = self
            .nodes
            .values()
            .flat_map(|n| n.children().iter())
            .filter_map(|slot| slot.as_ref())
            .map(|v| v.len())
            .sum();

        // Total potential children (all valid move combinations, before any pruning)
        let total_potential_children: usize = self
            .nodes
            .values()
            .map(|n| n.count_potential_children().iter().sum::<usize>())
            .sum();

        // Pruned nodes per depth (potential - actual spawned in tree)
        let mut pruned_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        for (&parent_id, node) in &self.nodes {
            let child_depth = parent_id.depth() + 1;
            let potential: usize = node.count_potential_children().iter().sum();
            let actual = children_map.get(&parent_id).map_or(0, |v| v.len());
            let pruned = potential.saturating_sub(actual);
            if pruned > 0 {
                *pruned_by_depth.entry(child_depth).or_default() += pruned;
            }
        }
        let pruned_per_depth: Vec<(u8, usize)> = pruned_by_depth.into_iter().collect();

        // Memory estimate (node data + HashMap overhead ~48 bytes/entry)
        let memory_estimate_bytes =
            self.nodes.len() * (std::mem::size_of::<NodeId>() + std::mem::size_of::<Node>() + 48);

        // Alive leaves = leaf nodes that are alive
        let alive_leaves = self
            .nodes
            .iter()
            .filter(|(id, n)| {
                !children_map.contains_key(id) && matches!(n.status(), NodeStatus::AliveFor(_))
            })
            .count();

        // Average branching factor (among internal nodes only)
        let internal_nodes: Vec<_> = children_map
            .iter()
            .filter(|(_, children)| !children.is_empty())
            .collect();
        let avg_branching_factor = if internal_nodes.is_empty() {
            0.0
        } else {
            internal_nodes.iter().map(|(_, c)| c.len()).sum::<usize>() as f64
                / internal_nodes.len() as f64
        };

        // Per-direction stats for root
        let direction_stats = (0..4)
            .map(|i| {
                let direction = Direction::try_from(i).unwrap();
                let status = root.direction_status(i);

                // Count subtree size by counting nodes whose root direction matches
                let (subtree_size, max_depth) = self.subtree_stats_for_direction(direction);

                DirectionStats {
                    direction,
                    status,
                    subtree_size,
                    max_depth,
                }
            })
            .collect();

        let queue_remaining = self.queue.len();

        TreeStats {
            total_nodes: self.nodes.len(),
            total_potential_children,
            total_tracked_children,
            max_depth_reached,
            nodes_per_depth,
            pruned_per_depth,
            nodes_by_status,
            leaf_nodes,
            alive_leaves,
            avg_leaf_depth,
            median_leaf_depth,
            root_status: root.status(),
            direction_stats,
            queue_remaining,
            avg_branching_factor,
            memory_estimate_bytes,
            duration: self.elapsed,
        }
    }

    fn subtree_stats_for_direction(&self, direction: Direction) -> (usize, u8) {
        let mut count = 0usize;
        let mut max_depth = 0u8;
        for (&id, _) in &self.nodes {
            if id.depth() > 0 {
                if let Some(dir) = id.direction_at(0, 0) {
                    if dir == direction {
                        count += 1;
                        max_depth = max_depth.max(id.depth());
                    }
                }
            }
        }
        (count, max_depth)
    }
}

impl fmt::Display for TreeStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // --- Overview ---
        writeln!(f, "Overview:")?;
        writeln!(f, "  Root status:          {}", self.root_status)?;
        writeln!(f, "  Total nodes in tree:  {}", self.total_nodes)?;
        writeln!(f, "  Max depth reached:    {}", self.max_depth_reached)?;
        writeln!(
            f,
            "  Avg branching factor: {:.2}",
            self.avg_branching_factor
        )?;
        let ebf = if self.max_depth_reached > 0 && self.leaf_nodes > 0 {
            (self.leaf_nodes as f64).powf(1.0 / self.max_depth_reached as f64)
        } else {
            0.0
        };
        writeln!(f, "  Eff branching factor: {:.2}", ebf)?;
        writeln!(f, "  Queue remaining:      {}", self.queue_remaining)?;
        let nps = if self.duration.as_secs_f64() > 0.0 {
            self.total_nodes as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        };
        writeln!(f, "  Duration:             {:.2?}", self.duration)?;
        writeln!(f, "  Nodes/sec:            {:.0}", nps)?;
        let (mem_value, mem_unit) = if self.memory_estimate_bytes >= 1024 * 1024 {
            (self.memory_estimate_bytes as f64 / (1024.0 * 1024.0), "MB")
        } else {
            (self.memory_estimate_bytes as f64 / 1024.0, "KB")
        };
        writeln!(f, "  Memory estimate:      {:.1} {}", mem_value, mem_unit)?;
        writeln!(f, "")?;

        // --- Pruning ---
        writeln!(f, "Pruning:")?;
        writeln!(
            f,
            "  Potential children:         {}",
            self.total_potential_children
        )?;
        writeln!(
            f,
            "  Spawned children:           {}",
            self.total_tracked_children
        )?;
        let not_spawned = self
            .total_potential_children
            .saturating_sub(self.total_tracked_children);
        let in_tree = self.total_nodes - 1; // exclude root
        let spawned_not_in_tree = self.total_tracked_children.saturating_sub(in_tree);
        writeln!(f, "  Pruned (not spawned):       {}", not_spawned)?;
        writeln!(f, "  Pruned (dead, not in tree): {}", spawned_not_in_tree)?;
        let total_pruned = not_spawned + spawned_not_in_tree;
        let pruning_rate = if self.total_potential_children > 0 {
            total_pruned as f64 / self.total_potential_children as f64 * 100.0
        } else {
            0.0
        };
        writeln!(f, "  Pruning rate:               {:.1}%", pruning_rate)?;
        writeln!(f, "")?;

        // --- Leaf nodes ---
        writeln!(f, "Leaf nodes:")?;
        writeln!(f, "  Total:              {}", self.leaf_nodes)?;
        writeln!(f, "  Alive (unexpanded): {}", self.alive_leaves)?;
        writeln!(f, "  Avg leaf depth:     {:.2}", self.avg_leaf_depth)?;
        writeln!(f, "  Median leaf depth:  {:.1}", self.median_leaf_depth)?;
        writeln!(f, "")?;

        // --- Nodes per depth ---
        writeln!(f, "Nodes per depth:")?;
        let pruned_map: std::collections::BTreeMap<u8, usize> =
            self.pruned_per_depth.iter().copied().collect();
        let max_count_width = self
            .nodes_per_depth
            .iter()
            .map(|(_, c)| c.to_string().len())
            .max()
            .unwrap_or(1);
        let max_pruned_width = self
            .pruned_per_depth
            .iter()
            .map(|(_, c)| c.to_string().len())
            .max()
            .unwrap_or(1);
        let max_depth_width = self
            .nodes_per_depth
            .iter()
            .map(|(d, _)| d.to_string().len())
            .max()
            .unwrap_or(1)
            .max(2);
        for (depth, count) in &self.nodes_per_depth {
            let pruned = pruned_map.get(depth).copied().unwrap_or(0);
            if pruned > 0 {
                writeln!(
                    f,
                    "  depth {:>dw$}: {:>cw$} nodes  ({:>pw$} pruned)",
                    depth,
                    count,
                    pruned,
                    dw = max_depth_width,
                    cw = max_count_width,
                    pw = max_pruned_width,
                )?;
            } else {
                writeln!(
                    f,
                    "  depth {:>dw$}: {:>cw$} nodes",
                    depth,
                    count,
                    dw = max_depth_width,
                    cw = max_count_width,
                )?;
            }
        }
        writeln!(f, "")?;
        writeln!(f, "Nodes by status:")?;
        let max_status_width = self
            .nodes_by_status
            .iter()
            .map(|(s, _)| format!("{}:", s).len())
            .max()
            .unwrap_or(1);
        let max_status_count_width = self
            .nodes_by_status
            .iter()
            .map(|(_, c)| c.to_string().len())
            .max()
            .unwrap_or(1);
        for (status, count) in &self.nodes_by_status {
            writeln!(
                f,
                "  {:<sw$} {:>cw$}",
                format!("{}:", status),
                count,
                sw = max_status_width,
                cw = max_status_count_width,
            )?;
        }
        writeln!(f, "")?;
        writeln!(f, "Direction analysis:")?;
        let max_dir_width = self
            .direction_stats
            .iter()
            .map(|ds| format!("{}", ds.direction).len())
            .max()
            .unwrap_or(1);
        let max_status_str_width = self
            .direction_stats
            .iter()
            .map(|ds| match ds.status {
                Some(s) => format!("{}", s).len(),
                None => "unexplored".len(),
            })
            .max()
            .unwrap_or(1);
        let max_subtree_width = self
            .direction_stats
            .iter()
            .map(|ds| ds.subtree_size.to_string().len())
            .max()
            .unwrap_or(1);
        for ds in &self.direction_stats {
            let status_str = match ds.status {
                Some(s) => format!("{}", s),
                None => "unexplored".to_string(),
            };
            writeln!(
                f,
                "  {:>dw$}: {:>sw$}  ({:>tw$} nodes, max depth {})",
                ds.direction,
                status_str,
                ds.subtree_size,
                ds.max_depth,
                dw = max_dir_width,
                sw = max_status_str_width,
                tw = max_subtree_width,
            )?;
        }
        Ok(())
    }
}
