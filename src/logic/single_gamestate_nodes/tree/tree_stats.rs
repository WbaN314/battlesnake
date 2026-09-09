use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::time::Duration;

use log::warn;
use tabled::{
    Table,
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

use crate::logic::general::direction::DIRECTIONS;
use crate::logic::{
    general::direction::Direction,
    single_gamestate_nodes::node::{Node, NodeStatus, PruneReason, node_id::NodeId},
};

use super::Tree;

#[derive(Debug)]
pub struct TreeStats {
    pub total_nodes: usize,
    pub max_depth_reached: u8,
    pub nodes_per_depth: Vec<(u8, usize)>,
    pub children_status_per_depth: Vec<(u8, Vec<(String, usize)>)>,
    pub nodes_by_status: Vec<(NodeStatus, usize)>,
    pub leaf_nodes: usize,
    pub alive_leaves: usize,
    pub avg_leaf_depth: f64,
    pub median_leaf_depth: f64,
    pub root_status: NodeStatus,
    pub direction_stats: Vec<DirectionStats>,
    pub queue_remaining: usize,
    pub avg_branching_factor: f32,
    pub memory_estimate_bytes: usize,
    pub duration: Duration,
}

fn status_kind(s: &NodeStatus) -> &'static str {
    match s {
        NodeStatus::AliveFor(_, _) => "AliveFor",
        NodeStatus::DeadIn(_, _) => "DeadIn",
        NodeStatus::WinnerIn(_, _) => "WinnerIn",
        NodeStatus::ProbablyDeadIn(_, _) => "ProbablyDeadIn",
        NodeStatus::NotSimulated => "NotSimulated",
        NodeStatus::Pruned(PruneReason::MaxDepth) => "Pruned(MaxDepth)",
        NodeStatus::Pruned(PruneReason::LocalHashSimilarity) => "Pruned(Hash)",
        NodeStatus::Pruned(PruneReason::HeadTailDistance) => "Pruned(HT)",
    }
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
        let mut nodes_by_status: Vec<(NodeStatus, usize)> = status_counts.into_iter().collect();
        nodes_by_status.sort_by(
            |(a, _), (b, _)| match (a.is_comparable(), b.is_comparable()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (false, false) => std::cmp::Ordering::Equal,
                _ => b.partial_cmp(a).unwrap(), // descending: best status first
            },
        );

        // Per-depth children status counts: for every node call children(), bucket (depth, kind).
        const KIND_ORDER: &[&str] = &[
            "AliveFor", "DeadIn", "ProbablyDeadIn", "WinnerIn",
            "NotSimulated", "Pruned(MaxDepth)", "Pruned(Hash)", "Pruned(HT)",
        ];
        let mut raw: BTreeMap<u8, HashMap<&'static str, usize>> = BTreeMap::new();
        for node in self.nodes.values() {
            for dir_children in node.children() {
                for (child_id, child_status) in dir_children {
                    *raw.entry(child_id.depth()).or_default()
                        .entry(status_kind(&child_status)).or_default() += 1;
                }
            }
        }
        let children_status_per_depth: Vec<(u8, Vec<(String, usize)>)> = raw
            .into_iter()
            .map(|(depth, counts)| {
                let row = KIND_ORDER.iter()
                    .filter_map(|&k| counts.get(k).map(|&c| (k.to_string(), c)))
                    .collect();
                (depth, row)
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
                !children_map.contains_key(id) && matches!(n.status(), NodeStatus::AliveFor(_, _))
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
            internal_nodes.iter().map(|(_, c)| c.len()).sum::<usize>() as f32
                / internal_nodes.len() as f32
        };

        // Per-direction stats for root
        let direction_stats = DIRECTIONS
            .into_iter()
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
            children_status_per_depth,
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
            duration: self.elapsed_simulation_time,
        }
    }

    pub fn log_depths(&self) {
        // This runs every turn in LOCAL_SIMULATION mode, so it must be cheap. It only
        // needs the per-direction subtree size and the result depth, so compute the
        // sizes in a single pass over the nodes instead of calling the full stats(),
        // which does ~10 O(N) passes (incl. 4 subtree scans) over every node and was
        // blowing the turn time budget on large trees -> timeouts -> forced "up" moves.
        let mut subtree_sizes = [0usize; 4];
        for &id in self.nodes.keys() {
            if id.depth() > 0 {
                if let Some(Some(dir)) = id.direction_at(0, 0) {
                    subtree_sizes[dir as usize] += 1;
                }
            }
        }
        let results = self.result();
        let depth_str: String = DIRECTIONS
            .iter()
            .zip(results.iter())
            .map(|(dir, status)| {
                let depth = match status {
                    NodeStatus::AliveFor(n, _)
                    | NodeStatus::DeadIn(n, _)
                    | NodeStatus::WinnerIn(n, _)
                    | NodeStatus::ProbablyDeadIn(n, _) => format!("{}", n),
                    _ => "null".to_string(),
                };
                format!(
                    "\"{}\":{{\"depth\":{},\"nodes\":{}}}",
                    dir, depth, subtree_sizes[*dir as usize]
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        warn!("DEPTHS {{{}}}", depth_str);
    }

    fn subtree_stats_for_direction(&self, direction: Direction) -> (usize, u8) {
        let mut count = 0usize;
        let mut max_depth = 0u8;
        for (&id, _) in &self.nodes {
            if id.depth() > 0 {
                if let Some(Some(dir)) = id.direction_at(0, 0) {
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
        const KIND_ORDER: &[&str] = &[
            "AliveFor", "DeadIn", "ProbablyDeadIn", "WinnerIn",
            "NotSimulated", "Pruned(MaxDepth)", "Pruned(Hash)", "Pruned(HT)",
        ];
        let used_kinds: Vec<&str> = KIND_ORDER.iter()
            .copied()
            .filter(|&k| self.children_status_per_depth.iter()
                .any(|(_, row)| row.iter().any(|(s, _)| s == k)))
            .collect();

        let mut b = Builder::default();
        let mut header = vec!["Depth".to_string()];
        header.extend(used_kinds.iter().map(|s| s.to_string()));
        b.push_record(header.iter().map(String::as_str));

        for (depth, row) in &self.children_status_per_depth {
            let row_map: HashMap<&str, usize> = row.iter().map(|(k, v)| (k.as_str(), *v)).collect();
            let mut rec = vec![depth.to_string()];
            for &kind in &used_kinds {
                rec.push(row_map.get(kind).map_or(0, |&c| c).to_string());
            }
            b.push_record(rec.iter().map(String::as_str));
        }

        let mut t = b.build();
        t.with(Style::rounded());
        t.modify(Columns::new(0..), Alignment::right());
        format!("Children status per depth:\n{t}\n\n")
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
    use crate::logic::single_gamestate_nodes::tree::tests::create_tree_from_gamestate;

    #[test]
    fn similarity_pruning_shows_in_stats() {
        for filename in &["requests/failure_01.json", "requests/failure_04.json"] {
            let mut tree = create_tree_from_gamestate(filename)
                .max_depth(4)
                .similarity_pruning(|_| 6);
            tree.simulate();
            let stats = tree.stats();
            let found = stats.children_status_per_depth.iter()
                .any(|(_, row)| row.iter().any(|(k, c)| k == "Pruned(Hash)" && *c > 0));
            assert!(found, "[similarity_pruning] {filename}: expected Pruned(LocalSim) in stats");
        }
    }
}
