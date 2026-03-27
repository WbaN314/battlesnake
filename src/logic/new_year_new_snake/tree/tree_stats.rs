use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::time::Duration;

use tabled::{
    Table,
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

use crate::logic::{
    game::direction::Direction,
    new_year_new_snake::node::{Node, NodeStatus, node_id::NodeId},
};

use super::Tree;

#[derive(Debug)]
pub struct TreeStats {
    pub total_nodes: usize,
    pub max_depth_reached: u8,
    pub nodes_per_depth: Vec<(u8, usize)>,
    pub pruning_per_depth: Vec<PruningDepthStats>,
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
pub struct PruningDepthStats {
    pub depth: u8,
    pub potential: usize,  // A: valid combos for all valid directions
    pub dir_skip: usize,   // A - B: combos for directions never started
    pub dead_break: usize, // B - Tree: explored-direction combos not in tree
    pub tree: usize,       // nodes present in the tree at this depth
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
        let nodes_per_depth: Vec<(u8, usize)> = by_depth.iter().map(|(&d, &n)| (d, n)).collect();

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
        let mut status_counts: HashMap<NodeStatus, usize> = HashMap::new();
        for node in self.nodes.values() {
            *status_counts.entry(node.status()).or_default() += 1;
        }
        let nodes_by_status: Vec<(NodeStatus, usize)> = status_counts.into_iter().collect();

        // Per-depth pruning breakdown:
        //   A = count_potential_children_all  (all valid dirs, explored or not)
        //   B = count_potential_children_from_evaluated_directions (explored dirs only)
        //   dir_skip   = A - B  (valid directions never started)
        //   dead_break = B - Tree  (explored-direction combos not reaching the tree)
        let mut potential_all_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        let mut potential_eval_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        for (&parent_id, node) in &self.nodes {
            let child_depth = parent_id.depth() + 1;
            let a: usize = node.count_potential_children_all().iter().sum();
            let b: usize = node
                .count_potential_children_from_evaluated_directions()
                .iter()
                .sum();
            *potential_all_by_depth.entry(child_depth).or_default() += a;
            *potential_eval_by_depth.entry(child_depth).or_default() += b;
        }
        let pruning_per_depth: Vec<PruningDepthStats> = potential_all_by_depth
            .keys()
            .map(|&depth| {
                let a = potential_all_by_depth[&depth];
                let b = potential_eval_by_depth.get(&depth).copied().unwrap_or(0);
                let tree = by_depth.get(&depth).copied().unwrap_or(0);
                PruningDepthStats {
                    depth,
                    potential: a,
                    dir_skip: a.saturating_sub(b),
                    dead_break: b.saturating_sub(tree),
                    tree,
                }
            })
            .collect();

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
                let status = root.direction_status(i).for_comparison();
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
            max_depth_reached,
            nodes_per_depth,
            pruning_per_depth,
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
        write!(f, "{}", self.overview_table())?;
        write!(f, "{}", self.pruning_table())?;
        write!(f, "{}", self.leaf_table())?;
        write!(f, "{}", self.status_table())?;
        write!(f, "{}", self.direction_table())
    }
}

impl TreeStats {
    fn section(title: &str, table: Table) -> String {
        format!("{title}:\n{table}\n\n")
    }

    fn kv_table(title: &str, rows: &[(&str, String)]) -> String {
        let mut b = Builder::default();
        for (k, v) in rows {
            b.push_record([k.to_string(), v.clone()]);
        }
        let mut t = b.build();
        t.with(Style::rounded());
        t.modify(Columns::new(1..=1), Alignment::right());
        Self::section(title, t)
    }

    fn overview_table(&self) -> String {
        let ebf = if self.max_depth_reached > 0 && self.leaf_nodes > 0 {
            (self.leaf_nodes as f64).powf(1.0 / self.max_depth_reached as f64)
        } else {
            0.0
        };
        let nps = if self.duration.as_secs_f64() > 0.0 {
            self.total_nodes as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        };
        let (mem_value, mem_unit) = if self.memory_estimate_bytes >= 1024 * 1024 {
            (self.memory_estimate_bytes as f64 / (1024.0 * 1024.0), "MB")
        } else {
            (self.memory_estimate_bytes as f64 / 1024.0, "KB")
        };
        Self::kv_table(
            "Overview",
            &[
                ("Root status", format!("{}", self.root_status)),
                ("Total nodes", self.total_nodes.to_string()),
                ("Max depth", self.max_depth_reached.to_string()),
                (
                    "Avg branching factor",
                    format!("{:.2}", self.avg_branching_factor),
                ),
                ("Eff branching factor", format!("{:.2}", ebf)),
                ("Queue remaining", self.queue_remaining.to_string()),
                ("Duration", format!("{:.2?}", self.duration)),
                ("Nodes/sec", format!("{:.0}", nps)),
                ("Memory estimate", format!("{:.1} {}", mem_value, mem_unit)),
            ],
        )
    }

    fn pruning_table(&self) -> String {
        let mut b = Builder::default();
        b.push_record(["Depth", "Potential", "Dir Prune", "Dead Prune", "Tree", "%"]);
        let (mut tot_potential, mut tot_dir_skip, mut tot_dead_break, mut tot_tree) =
            (0usize, 0usize, 0usize, 0usize);
        for p in &self.pruning_per_depth {
            let pruned = p.dir_skip + p.dead_break;
            let rate = if p.potential > 0 {
                format!("{:.1}%", pruned as f64 / p.potential as f64 * 100.0)
            } else {
                "-".to_string()
            };
            b.push_record([
                p.depth.to_string(),
                p.potential.to_string(),
                p.dir_skip.to_string(),
                p.dead_break.to_string(),
                p.tree.to_string(),
                rate,
            ]);
            tot_potential += p.potential;
            tot_dir_skip += p.dir_skip;
            tot_dead_break += p.dead_break;
            tot_tree += p.tree;
        }
        let tot_pruned = tot_dir_skip + tot_dead_break;
        let tot_rate = if tot_potential > 0 {
            format!("{:.1}%", tot_pruned as f64 / tot_potential as f64 * 100.0)
        } else {
            "-".to_string()
        };
        b.push_record([
            "Total".to_string(),
            tot_potential.to_string(),
            tot_dir_skip.to_string(),
            tot_dead_break.to_string(),
            tot_tree.to_string(),
            tot_rate,
        ]);
        let mut t = b.build();
        t.with(Style::rounded());
        t.modify(Columns::new(0..=4), Alignment::right());
        let legend = concat!(
            "  Potential  = valid move combos for all valid directions, regardless of whether they were explored\n",
            "  Dir Prune  = valid directions never started; the node was pruned before this direction was tried\n",
            "  Dead Prune = evaluated children that didn't reach the tree; a DeadIn(0) child cut the direction short\n",
            "  Tree       = nodes that actually made it into the search tree at this depth\n",
            "  %          = (Dir Prune + Dead Prune) / Potential\n",
            "\n",
            "  Invariants:\n",
            "    Dir Prune + Dead Prune + Tree == Potential  (all potential combos are fully accounted for)\n",
            "    Tree == nodes_per_depth[depth]             (Tree column matches actual node count in tree)\n",
            "    sum(Tree across all depths) == total_nodes - 1  (all non-root nodes accounted for)\n",
        );
        format!("Pruning:\n{t}\n\n{legend}\n")
    }

    fn leaf_table(&self) -> String {
        Self::kv_table(
            "Leaf nodes",
            &[
                ("Total", self.leaf_nodes.to_string()),
                ("Alive (unexpanded)", self.alive_leaves.to_string()),
                ("Avg leaf depth", format!("{:.2}", self.avg_leaf_depth)),
                (
                    "Median leaf depth",
                    format!("{:.1}", self.median_leaf_depth),
                ),
            ],
        )
    }

    fn status_table(&self) -> String {
        let mut b = Builder::default();
        b.push_record(["Status", "Count"]);
        for (status, count) in &self.nodes_by_status {
            b.push_record([format!("{}", status), count.to_string()]);
        }
        let mut t = b.build();
        t.with(Style::rounded());
        t.modify(Columns::new(1..=1), Alignment::right());
        Self::section("Nodes by status", t)
    }

    fn direction_table(&self) -> String {
        let mut b = Builder::default();
        b.push_record(["Direction", "Status", "Subtree", "Max Depth"]);
        for ds in &self.direction_stats {
            let status_str = match ds.status {
                Some(s) => format!("{}", s),
                None => "unexplored".to_string(),
            };
            b.push_record([
                format!("{}", ds.direction),
                status_str,
                ds.subtree_size.to_string(),
                ds.max_depth.to_string(),
            ]);
        }
        let mut t = b.build();
        t.with(Style::rounded());
        t.modify(Columns::new(2..=2), Alignment::right());
        t.modify(Columns::new(3..=3), Alignment::right());
        format!("Direction analysis:\n{t}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::{
            game::{field::BasicField, game_state::GameState},
            new_year_new_snake::tree::tests::create_tree_from_gamestate,
        },
        read_game_state,
    };

    #[test]
    fn pruning_stats_are_consistent() {
        for filename in &["requests/failure_1.json", "requests/failure_4.json"] {
            let mut tree = create_tree_from_gamestate(filename).max_depth(4);
            tree.simulate();
            let stats = tree.stats();

            let nodes_per_depth: std::collections::HashMap<u8, usize> =
                stats.nodes_per_depth.iter().copied().collect();

            for p in &stats.pruning_per_depth {
                // Core invariant: the three parts must sum to potential
                assert_eq!(
                    p.dir_skip + p.dead_break + p.tree,
                    p.potential,
                    "{filename} depth {}: dir_skip({}) + dead_break({}) + tree({}) != potential({})",
                    p.depth,
                    p.dir_skip,
                    p.dead_break,
                    p.tree,
                    p.potential,
                );

                // Tree count must match actual nodes in the tree at this depth
                let nodes_at_depth = nodes_per_depth.get(&p.depth).copied().unwrap_or(0);
                assert_eq!(
                    p.tree, nodes_at_depth,
                    "{filename} depth {}: pruning tree({}) != nodes_per_depth({})",
                    p.depth, p.tree, nodes_at_depth,
                );
            }

            // Sum of tree counts across depths (excluding root at depth 0) must equal total_nodes - 1
            let total_tree: usize = stats.pruning_per_depth.iter().map(|p| p.tree).sum();
            assert_eq!(
                total_tree,
                stats.total_nodes - 1,
                "{filename}: sum of tree across depths({}) != total_nodes - 1({})",
                total_tree,
                stats.total_nodes - 1,
            );
        }
    }
}
