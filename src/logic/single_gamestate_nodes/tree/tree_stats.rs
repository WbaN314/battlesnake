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

const ALL_PRUNED_STATUSES: &[NodeStatus] = &[
    NodeStatus::Pruned(PruneReason::MaxDepth),
    NodeStatus::Pruned(PruneReason::LocalHashSimilarity),
];

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
    pub avg_branching_factor: f32,
    pub memory_estimate_bytes: usize,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct PruningDepthStats {
    pub depth: u8,
    pub potential: usize,
    pub dir_skip: usize,
    pub dead_break: usize,
    pub ht_pruned: usize,
    /// Counts per pruned NodeStatus variant, sorted by Display string for stable ordering.
    /// New Pruned* variants appear here automatically once added to ALL_PRUNED_STATUSES.
    pub pruned: Vec<(NodeStatus, usize)>,
    pub simulated: usize,
}

impl PruningDepthStats {
    pub fn total_pruned_nodes(&self) -> usize {
        self.pruned.iter().map(|(_, c)| c).sum()
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

        // Per-depth pruning breakdown based on children() calls.
        // For each non-pruned node, call children() and classify each entry by status:
        //   NotSimulated              → dir_skip   (direction never started)
        //   DeadIn(0, _)              → dead_break (explored but immediately dead)
        //   Pruned(HeadTailDistance)  → ht_pruned  (HT-pruned, never explored)
        //   Pruned(*)                 → pruned vec (MaxDepth = real node; LocalHashSimilarity = virtual)
        //   everything else           → simulated  (actual node in tree)
        // Pruned nodes are skipped: they are leaf entries in their parent's children() output
        // and calling children() on them would generate phantom HT entries.
        //
        // Edge case: when simulate() explores a direction and the resulting direction_status is
        // DeadIn or ProbablyDeadIn, it `continue`s without returning the alive/winner children,
        // so they are recorded in children_states_per_direction as AliveFor(0)/WinnerIn(0) but
        // never inserted into self.nodes. We detect these by checking the direction status and
        // count them as dead_break.
        let mut potential_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        let mut dir_skip_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        let mut dead_break_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        let mut ht_pruned_by_depth: BTreeMap<u8, usize> = BTreeMap::new();
        let mut pruned_by_depth: BTreeMap<u8, HashMap<String, (NodeStatus, usize)>> =
            BTreeMap::new();
        let mut simulated_by_depth: BTreeMap<u8, usize> = BTreeMap::new();

        for node in self.nodes.values() {
            if matches!(node.status(), NodeStatus::Pruned(_)) {
                continue;
            }
            for (dir_idx, dir_children) in node.children().into_iter().enumerate() {
                let direction = Direction::try_from(dir_idx).unwrap();
                let dir_is_dead = matches!(
                    node.direction_status(direction),
                    NodeStatus::DeadIn(_, _) | NodeStatus::ProbablyDeadIn(_, _)
                );
                for (child_id, child_status) in dir_children {
                    let depth = child_id.depth();
                    *potential_by_depth.entry(depth).or_default() += 1;
                    match child_status {
                        NodeStatus::NotSimulated => {
                            *dir_skip_by_depth.entry(depth).or_default() += 1;
                        }
                        NodeStatus::DeadIn(0, _) => {
                            *dead_break_by_depth.entry(depth).or_default() += 1;
                        }
                        NodeStatus::Pruned(PruneReason::HeadTailDistance) => {
                            *ht_pruned_by_depth.entry(depth).or_default() += 1;
                        }
                        s @ NodeStatus::Pruned(_) => {
                            pruned_by_depth
                                .entry(depth)
                                .or_default()
                                .entry(format!("{}", s))
                                .or_insert((s, 0))
                                .1 += 1;
                        }
                        NodeStatus::AliveFor(0, _) | NodeStatus::WinnerIn(0, _)
                            if dir_is_dead =>
                        {
                            // Alive/winner child of a dead/probablydead direction: was explored
                            // but dropped (continue 'direction in simulate), not in the tree.
                            *dead_break_by_depth.entry(depth).or_default() += 1;
                        }
                        _ => {
                            *simulated_by_depth.entry(depth).or_default() += 1;
                        }
                    }
                }
            }
        }

        let pruning_per_depth: Vec<PruningDepthStats> = potential_by_depth
            .keys()
            .filter(|&&depth| depth <= max_depth_reached)
            .map(|&depth| {
                let potential = potential_by_depth[&depth];
                let dir_skip = dir_skip_by_depth.get(&depth).copied().unwrap_or(0);
                let dead_break = dead_break_by_depth.get(&depth).copied().unwrap_or(0);
                let ht_pruned = ht_pruned_by_depth.get(&depth).copied().unwrap_or(0);
                let simulated = simulated_by_depth.get(&depth).copied().unwrap_or(0);
                let mut pruned: Vec<(NodeStatus, usize)> = pruned_by_depth
                    .get(&depth)
                    .map_or(vec![], |m| m.values().map(|&(s, c)| (s, c)).collect());
                pruned.sort_by_key(|(s, _)| format!("{}", s));
                PruningDepthStats {
                    depth,
                    potential,
                    dir_skip,
                    dead_break,
                    ht_pruned,
                    pruned,
                    simulated,
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
            duration: self.elapsed_simulation_time,
        }
    }

    pub fn log_depths(&self) {
        let stats = self.stats();
        let results = self.result();
        let depth_str: String = DIRECTIONS
            .iter()
            .zip(stats.direction_stats.iter())
            .zip(results.iter())
            .map(|((dir, ds), status)| {
                let depth = match status {
                    NodeStatus::AliveFor(n, _)
                    | NodeStatus::DeadIn(n, _)
                    | NodeStatus::WinnerIn(n, _)
                    | NodeStatus::ProbablyDeadIn(n, _) => format!("{}", n),
                    _ => "null".to_string(),
                };
                format!("\"{}\":{{\"depth\":{},\"nodes\":{}}}", dir, depth, ds.subtree_size)
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
        let all_pruned = ALL_PRUNED_STATUSES;

        let mut b = Builder::default();
        let mut header: Vec<String> = vec![
            "Depth".into(), "Potential".into(), "Dir Prune".into(), "Dead Prune".into(), "HT Prune".into(),
        ];
        header.extend(all_pruned.iter().map(|s| format!("{}", s)));
        header.extend(["Simulated".into(), "%".into()]);
        b.push_record(header.iter().map(String::as_str).collect::<Vec<_>>());

        let mut tot_potential = 0usize;
        let mut tot_dir_skip = 0usize;
        let mut tot_dead_break = 0usize;
        let mut tot_ht_pruned = 0usize;
        let mut tot_by_type = vec![0usize; all_pruned.len()];
        let mut tot_simulated = 0usize;

        for p in &self.pruning_per_depth {
            let type_counts: Vec<usize> = all_pruned
                .iter()
                .map(|s| {
                    p.pruned.iter().find(|(ps, _)| ps == s).map_or(0, |(_, c)| *c)
                })
                .collect();
            let total_pruned = p.dir_skip + p.dead_break + p.ht_pruned + type_counts.iter().sum::<usize>();
            let rate = if p.potential > 0 {
                format!("{:.1}%", total_pruned as f64 / p.potential as f64 * 100.0)
            } else {
                "-".to_string()
            };
            let mut row: Vec<String> = vec![
                p.depth.to_string(), p.potential.to_string(),
                p.dir_skip.to_string(), p.dead_break.to_string(), p.ht_pruned.to_string(),
            ];
            row.extend(type_counts.iter().map(|c| c.to_string()));
            row.push(p.simulated.to_string());
            row.push(rate);
            b.push_record(row);

            tot_potential += p.potential;
            tot_dir_skip += p.dir_skip;
            tot_dead_break += p.dead_break;
            tot_ht_pruned += p.ht_pruned;
            for (i, c) in type_counts.iter().enumerate() { tot_by_type[i] += c; }
            tot_simulated += p.simulated;
        }

        let tot_pruned = tot_dir_skip + tot_dead_break + tot_ht_pruned + tot_by_type.iter().sum::<usize>();
        let tot_rate = if tot_potential > 0 {
            format!("{:.1}%", tot_pruned as f64 / tot_potential as f64 * 100.0)
        } else {
            "-".to_string()
        };
        let mut tot_row: Vec<String> = vec![
            "Total".into(), tot_potential.to_string(),
            tot_dir_skip.to_string(), tot_dead_break.to_string(), tot_ht_pruned.to_string(),
        ];
        tot_row.extend(tot_by_type.iter().map(|c| c.to_string()));
        tot_row.push(tot_simulated.to_string());
        tot_row.push(tot_rate);
        b.push_record(tot_row);

        let mut t = b.build();
        t.with(Style::rounded());
        t.modify(Columns::new(0..=5 + all_pruned.len()), Alignment::right());
        let legend = concat!(
            "  Potential  = valid move combos for all valid directions\n",
            "  Dir Prune  = directions never started (NotSimulated in children)\n",
            "  Dead Prune = children explored but immediately dead (DeadIn(0))\n",
            "  HT Prune   = combinations eliminated by head-tail distance pruning\n",
            "  Simulated  = nodes actually in the tree (alive/dead/winner/probablydead)\n",
            "  %          = (all pruned) / Potential\n",
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
            single_gamestate_nodes::{
                node::{NodeStatus, PruneReason},
                tree::tests::create_tree_from_gamestate,
            },
        },
    };

    fn check_invariants(stats: &super::TreeStats, label: &str, filename: &str) {
        let nodes_per_depth: std::collections::HashMap<u8, usize> =
            stats.nodes_per_depth.iter().copied().collect();

        for p in &stats.pruning_per_depth {
            let total_pruned_nodes = p.total_pruned_nodes();
            assert_eq!(
                p.dir_skip + p.dead_break + p.ht_pruned + total_pruned_nodes + p.simulated,
                p.potential,
                "[{label}] {filename} depth {}: dir_skip({}) + dead_break({}) + ht_pruned({}) + pruned({}) + simulated({}) != potential({})",
                p.depth, p.dir_skip, p.dead_break, p.ht_pruned, total_pruned_nodes, p.simulated, p.potential,
            );

            // simulated (non-pruned tree nodes) + MaxDepth-pruned (real nodes in tree) = nodes at depth
            let real_pruned = p.pruned.iter()
                .filter(|(s, _)| *s == NodeStatus::Pruned(PruneReason::MaxDepth))
                .map(|(_, c)| c)
                .sum::<usize>();
            let nodes_at_depth = nodes_per_depth.get(&p.depth).copied().unwrap_or(0);
            assert_eq!(
                p.simulated + real_pruned,
                nodes_at_depth,
                "[{label}] {filename} depth {}: simulated({}) + real_pruned({}) != nodes_per_depth({})",
                p.depth, p.simulated, real_pruned, nodes_at_depth,
            );
        }

        let total_tree: usize = stats
            .pruning_per_depth
            .iter()
            .map(|p| {
                let real_pruned = p.pruned.iter()
                    .filter(|(s, _)| *s == NodeStatus::Pruned(PruneReason::MaxDepth))
                    .map(|(_, c)| c)
                    .sum::<usize>();
                p.simulated + real_pruned
            })
            .sum();
        assert_eq!(
            total_tree,
            stats.total_nodes - 1,
            "[{label}] {filename}: sum of tree({}) != total_nodes - 1({})",
            total_tree,
            stats.total_nodes - 1,
        );
    }

    #[test]
    fn similarity_pruning_shows_in_stats() {
        let filenames = ["requests/failure_01.json", "requests/failure_04.json"];
        for filename in &filenames {
            let mut tree = create_tree_from_gamestate(filename)
                .max_depth(4)
                .similarity_pruning(|_| 6); // large HT distance keeps opponents' real moves → more combos → hash collisions
            tree.simulate();
            let stats = tree.stats();
            check_invariants(&stats, "similarity_pruning", filename);

            let total_sim: usize = stats
                .pruning_per_depth
                .iter()
                .map(|p| {
                    p.pruned
                        .iter()
                        .find(|(s, _)| *s == NodeStatus::Pruned(PruneReason::LocalHashSimilarity))
                        .map_or(0, |(_, c)| *c)
                })
                .sum();
            assert!(
                total_sim > 0,
                "[similarity_pruning] {filename}: expected Pruned(LocalHashSimilarity) entries in stats but got 0"
            );
        }
    }

    #[test]
    fn head_tail_pruning_shows_in_stats() {
        let filenames = ["requests/failure_01.json", "requests/failure_04.json"];
        for filename in &filenames {
            let mut tree = create_tree_from_gamestate(filename)
                .max_depth(4)
                .head_tail_pruning(|_| 1);
            tree.simulate();
            let stats = tree.stats();
            check_invariants(&stats, "head_tail_pruning", filename);

            let total_ht: usize = stats.pruning_per_depth.iter().map(|p| p.ht_pruned).sum();
            assert!(
                total_ht > 0,
                "[head_tail_pruning] {filename}: expected ht_pruned > 0 in stats but got 0"
            );
        }
    }
}
