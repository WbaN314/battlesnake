use std::fmt;

use crate::logic::{game::direction::Direction, new_year_new_snake::node::NodeStatus};

#[derive(Debug)]
pub struct TreeStats {
    pub total_nodes: usize,
    pub total_tracked_children: usize,
    pub max_depth_reached: u8,
    pub nodes_per_depth: Vec<(u8, usize)>,
    pub nodes_by_status: Vec<(NodeStatus, usize)>,
    pub leaf_nodes: usize,
    pub alive_leaves: usize,
    pub root_status: NodeStatus,
    pub direction_stats: Vec<DirectionStats>,
    pub queue_remaining: usize,
    pub avg_branching_factor: f64,
}

#[derive(Debug)]
pub struct DirectionStats {
    pub direction: Direction,
    pub status: Option<NodeStatus>,
    pub subtree_size: usize,
    pub max_depth: u8,
}

impl fmt::Display for TreeStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Root status:          {}", self.root_status)?;
        writeln!(f, "Total nodes in tree:  {}", self.total_nodes)?;
        writeln!(f, "Tracked children:     {}", self.total_tracked_children)?;
        writeln!(
            f,
            "Pruned (dead) nodes:  {}",
            self.total_tracked_children - (self.total_nodes - 1)
        )?;
        writeln!(f, "Max depth reached:    {}", self.max_depth_reached)?;
        writeln!(f, "Leaf nodes:           {}", self.leaf_nodes)?;
        writeln!(f, "  Alive (unexpanded): {}", self.alive_leaves)?;
        writeln!(f, "Avg branching factor: {:.2}", self.avg_branching_factor)?;
        writeln!(f, "Queue remaining:      {}", self.queue_remaining)?;
        writeln!(f, "")?;
        writeln!(f, "Nodes per depth:")?;
        let max_count_width = self
            .nodes_per_depth
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
            writeln!(
                f,
                "  depth {:>dw$}: {:>cw$} nodes",
                depth,
                count,
                dw = max_depth_width,
                cw = max_count_width,
            )?;
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
