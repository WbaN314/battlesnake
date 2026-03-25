use std::fmt::Display;
use std::str::FromStr;

use crate::logic::game::{direction::Direction, moves::Moves, snakes::SNAKES};

/// Bits reserved for storing the depth (max 15, fits in 4 bits).
const DEPTH_BITS: u32 = 4;
/// Bits per snake direction (2 bits: U=0, D=1, L=2, R=3). `None` maps to `Up`.
const BITS_PER_SNAKE: u32 = 2;
/// Bits consumed per tree level (4 snakes × 2 bits).
const BITS_PER_LEVEL: u32 = BITS_PER_SNAKE * SNAKES as u32; // 8
/// Maximum supported tree depth: (128 − 4) / 8 = 15.
const MAX_DEPTH: u8 = ((128 - DEPTH_BITS) / BITS_PER_LEVEL) as u8;
/// Number of flag bits stored at the top of the u128.
const FLAGS_BITS: u32 = 128 - DEPTH_BITS - MAX_DEPTH as u32 * BITS_PER_LEVEL; // 4

const DEPTH_MASK: u128 = (1u128 << DEPTH_BITS) - 1;
const LEVEL_MASK: u128 = (1u128 << BITS_PER_LEVEL) - 1;
const FLAGS_SHIFT: u32 = 128 - FLAGS_BITS;
const FLAGS_MASK: u128 = ((1u128 << FLAGS_BITS) - 1) << FLAGS_SHIFT;

pub type DirectionVector = [Direction; SNAKES as usize];

/// Compact node identifier for the game tree, packed into a single `u128`.
///
/// Layout (LSB first):
/// ```text
/// [depth: 4 bits][level 0: 8 bits]...[level 14: 8 bits][flags: 4 bits (MSB)]
/// ```
/// Each level encodes 4 snake directions (2 bits each):
/// ```text
/// [snake 0: 2 bits][snake 1: 2 bits][snake 2: 2 bits][snake 3: 2 bits]
/// ```
/// Direction encoding: Up=0, Down=1, Left=2, Right=3. `None` maps to `Up`.
/// Flags are stored in the top 4 bits, addressable by index 0..3.
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct NodeId {
    data: u128,
}

/// Decodes 2 bits from a shifted u128 value into a Direction.
/// Only the lowest 2 bits of `val` are considered.
#[inline(always)]
fn decode(val: u128) -> Direction {
    match val & 0b11 {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => unreachable!(),
    }
}

impl NodeId {
    pub fn new() -> Self {
        NodeId { data: 0 }
    }

    pub fn depth(&self) -> u8 {
        (self.data & DEPTH_MASK) as u8
    }

    pub fn child(&self, moves: Moves) -> Self {
        let mut child = *self;
        child.push(moves);
        child
    }

    pub fn push(&mut self, moves: Moves) {
        let depth = self.depth();
        debug_assert!(
            depth < MAX_DEPTH,
            "Maximum tree depth ({MAX_DEPTH}) exceeded"
        );

        let mut encoded: u128 = 0;
        for (i, &dir) in moves.iter().enumerate() {
            let bits = dir.unwrap_or(Direction::Up) as u128;
            encoded |= bits << (i as u32 * BITS_PER_SNAKE);
        }

        let shift = DEPTH_BITS + depth as u32 * BITS_PER_LEVEL;
        self.data = (self.data & !DEPTH_MASK) | (depth as u128 + 1) | (encoded << shift);
    }

    pub fn parent(&self) -> Option<Self> {
        let depth = self.depth();
        if depth == 0 {
            return None;
        }

        let shift = DEPTH_BITS + (depth as u32 - 1) * BITS_PER_LEVEL;
        let new_data = (self.data & !(LEVEL_MASK << shift)) - 1;

        Some(NodeId { data: new_data })
    }

    pub fn last_direction_for(&self, snake: u8) -> Option<Direction> {
        let depth = self.depth();
        if depth == 0 {
            return None;
        }

        let shift =
            DEPTH_BITS + (depth as u32 - 1) * BITS_PER_LEVEL + snake as u32 * BITS_PER_SNAKE;
        Some(decode(self.data >> shift))
    }

    pub fn last_directions(&self) -> Option<DirectionVector> {
        let depth = self.depth();
        if depth == 0 {
            return None;
        }

        let mut directions = [Direction::Up; SNAKES];
        let shift = DEPTH_BITS + (depth as u32 - 1) * BITS_PER_LEVEL;
        let level_data = (self.data >> shift) & LEVEL_MASK;
        for snake in 0..SNAKES as u8 {
            directions[snake as usize] = decode(level_data >> (snake as u32 * BITS_PER_SNAKE));
        }
        Some(directions)
    }

    /// Returns the direction of a specific snake at a specific level.
    pub fn direction_at(&self, level: u8, snake: u8) -> Option<Direction> {
        if level >= self.depth() {
            return None;
        }
        let shift = DEPTH_BITS + level as u32 * BITS_PER_LEVEL + snake as u32 * BITS_PER_SNAKE;
        Some(decode(self.data >> shift))
    }

    /// Returns the value of a flag bit (0-indexed from MSB, max index: FLAGS_BITS-1).
    #[inline(always)]
    pub fn read_flag(&self, index: u8) -> bool {
        debug_assert!((index as u32) < FLAGS_BITS);
        self.data & (1u128 << (FLAGS_SHIFT + index as u32)) != 0
    }

    /// Sets or clears a flag bit (0-indexed from MSB, max index: FLAGS_BITS-1).
    #[inline(always)]
    pub fn set_flag(&mut self, index: u8, value: bool) {
        debug_assert!((index as u32) < FLAGS_BITS);
        let bit = 1u128 << (FLAGS_SHIFT + index as u32);
        if value {
            self.data |= bit;
        } else {
            self.data &= !bit;
        }
    }

    /// Returns all flag bits as a u8 (lower FLAGS_BITS bits used).
    #[inline(always)]
    pub fn read_flags(&self) -> u8 {
        ((self.data & FLAGS_MASK) >> FLAGS_SHIFT) as u8
    }

    /// Sets all flag bits from a u8 (lower FLAGS_BITS bits used).
    #[inline(always)]
    pub fn set_flags(&mut self, flags: u8) {
        self.data = (self.data & !FLAGS_MASK) | ((flags as u128) << FLAGS_SHIFT);
    }
}

/// Displays the node path grouped by depth level:
/// `level0moves-level1moves-level2moves-...`
///
/// e.g. `DRDL-DUDU-UUDD-RDDR` (depth 4, 4 snakes per level)
///
/// Each group contains one direction per snake in order.
impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = self.depth();
        if depth == 0 {
            return write!(f, "ROOT");
        }

        for level in 0..depth {
            if level > 0 {
                write!(f, "-")?;
            }
            for snake in 0..SNAKES as u8 {
                write!(f, "{}", self.direction_at(level, snake).unwrap())?;
            }
        }
        Ok(())
    }
}

impl FromStr for NodeId {
    type Err = String;

    /// Parses a string like `"DRDL-DUDU-UUDD-RDDR"` or `"ROOT"` into a `NodeId`.
    /// Each group is one depth level containing one direction per snake.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "ROOT" {
            return Ok(NodeId::new());
        }

        let groups: Vec<&str> = s.split('-').collect();
        let depth = groups.len();
        if depth == 0 || depth > MAX_DEPTH as usize {
            return Err(format!("Invalid depth: {depth}"));
        }
        if !groups.iter().all(|g| g.len() == SNAKES) {
            return Err(format!(
                "Each level group must have {} characters (one per snake)",
                SNAKES
            ));
        }

        let mut node = NodeId::new();
        for group in &groups {
            let mut moves: Moves = [None; SNAKES];
            for (snake, ch) in group.bytes().enumerate() {
                let dir = Direction::try_from(ch as char)
                    .map_err(|_| format!("Invalid direction char: '{}'", ch as char))?;
                moves[snake] = Some(dir);
            }
            node.push(moves);
        }

        Ok(node)
    }
}

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        s.parse().expect("Invalid NodeId string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::game::direction::Direction::*;

    #[test]
    fn new_node_is_root() {
        let root = NodeId::new();
        assert_eq!(root.depth(), 0);
        assert_eq!(root.parent(), None);
        assert_eq!(root.last_direction_for(0), None);
        assert_eq!(root.to_string(), "ROOT");
    }

    #[test]
    fn single_child() {
        let root = NodeId::new();
        let child = root.child([Some(Down), Some(Right), Some(Up), Some(Left)]);

        assert_eq!(child.depth(), 1);
        assert_eq!(child.last_direction_for(0), Some(Down)); // snake 0
        assert_eq!(child.last_direction_for(1), Some(Right)); // snake 1
        assert_eq!(child.last_direction_for(2), Some(Up)); // snake 2
        assert_eq!(child.last_direction_for(0), Some(Down));
        assert_eq!(child.last_direction_for(1), Some(Right));
        assert_eq!(child.last_direction_for(2), Some(Up));
        assert_eq!(child.last_direction_for(3), Some(Left));
        assert_eq!(child.to_string(), "DRUL");
    }

    #[test]
    fn two_levels_display() {
        let root = NodeId::new();
        let lvl1 = root.child([Some(Down), Some(Right), Some(Down), Some(Left)]);
        let lvl2 = lvl1.child([Some(Up), Some(Up), Some(Down), Some(Right)]);

        assert_eq!(lvl2.depth(), 2);
        assert_eq!(lvl2.to_string(), "DRDL-UUDR");
    }

    #[test]
    fn parent_round_trip() {
        let root = NodeId::new();
        let c1 = root.child([Some(Down), Some(Right), Some(Up), Some(Left)]);
        let c2 = c1.child([Some(Up), Some(Up), Some(Down), Some(Down)]);

        assert_eq!(c2.parent(), Some(c1));
        assert_eq!(c1.parent(), Some(root));
    }

    #[test]
    fn none_directions_default_to_up() {
        let root = NodeId::new();
        let child = root.child([Some(Down), None, Some(Up), None]);

        // None maps to Up
        assert_eq!(child.to_string(), "DUUU");
    }

    #[test]
    fn max_depth_reachable() {
        let mut node = NodeId::new();
        for _ in 0..MAX_DEPTH {
            node = node.child([Some(Up), Some(Down), Some(Left), Some(Right)]);
        }
        assert_eq!(node.depth(), MAX_DEPTH);
    }

    #[test]
    #[should_panic(expected = "Maximum tree depth")]
    fn exceeding_max_depth_panics() {
        let mut node = NodeId::new();
        for _ in 0..=MAX_DEPTH {
            node = node.child([Some(Up), Some(Down), Some(Left), Some(Right)]);
        }
    }

    #[test]
    fn four_level_display() {
        let node = NodeId::new()
            .child([Some(Down), Some(Right), Some(Down), Some(Left)])
            .child([Some(Down), Some(Up), Some(Down), Some(Up)])
            .child([Some(Up), Some(Up), Some(Down), Some(Down)])
            .child([Some(Right), Some(Down), Some(Down), Some(Right)]);

        assert_eq!(node.to_string(), "DRDL-DUDU-UUDD-RDDR");
    }

    #[test]
    fn flags_default_to_zero() {
        let node = NodeId::new();
        assert_eq!(node.read_flags(), 0);
        for i in 0..4 {
            assert!(!node.read_flag(i));
        }
    }

    #[test]
    fn set_and_read_individual_flags() {
        let mut node = NodeId::new();
        node.set_flag(0, true);
        assert!(node.read_flag(0));
        assert!(!node.read_flag(1));

        node.set_flag(2, true);
        assert!(node.read_flag(0));
        assert!(node.read_flag(2));
        assert_eq!(node.read_flags(), 0b0101);

        node.set_flag(0, false);
        assert!(!node.read_flag(0));
        assert!(node.read_flag(2));
    }

    #[test]
    fn set_and_read_all_flags() {
        let mut node = NodeId::new();
        node.set_flags(0b1010);
        assert!(!node.read_flag(0));
        assert!(node.read_flag(1));
        assert!(!node.read_flag(2));
        assert!(node.read_flag(3));
        assert_eq!(node.read_flags(), 0b1010);
    }

    #[test]
    fn flags_independent_of_depth_and_moves() {
        let mut node = NodeId::new().child([Some(Down), Some(Right), Some(Down), Some(Left)]);
        node.set_flags(0b1111);

        assert_eq!(node.depth(), 1);
        assert_eq!(node.to_string(), "DRDL");
        assert_eq!(node.read_flags(), 0b1111);
    }

    #[test]
    fn flags_not_inherited_by_child() {
        let mut parent = NodeId::new();
        parent.set_flag(0, true);
        let child = parent.child([Some(Up), Some(Up), Some(Up), Some(Up)]);
        // child copies parent data, so flags are inherited
        assert!(child.read_flag(0));
    }

    #[test]
    fn parse_root() {
        assert_eq!("ROOT".parse::<NodeId>().unwrap(), NodeId::new());
        assert_eq!(NodeId::from("ROOT"), NodeId::new());
    }

    #[test]
    fn parse_round_trip() {
        let original = "DRDL-DUDU-UUDD-RDDR";
        let node: NodeId = original.parse().unwrap();
        assert_eq!(node.to_string(), original);
        assert_eq!(node.depth(), 4);
    }

    #[test]
    fn parse_single_level() {
        let node: NodeId = "DRUL".parse().unwrap();
        assert_eq!(node.depth(), 1);
        assert_eq!(node.to_string(), "DRUL");
    }

    #[test]
    fn parse_invalid_char() {
        assert!("DXDL-DUDU-UUDD-RDDR".parse::<NodeId>().is_err());
    }

    #[test]
    fn parse_wrong_group_length() {
        assert!("DRD-DUDU".parse::<NodeId>().is_err());
    }

    #[test]
    fn parse_empty_group() {
        assert!("".parse::<NodeId>().is_err());
    }

    #[test]
    fn last_directions_root_is_none() {
        assert_eq!(NodeId::new().last_directions(), None);
    }

    #[test]
    fn last_directions_returns_final_level() {
        let node = NodeId::new()
            .child([Some(Down), Some(Right), Some(Down), Some(Left)])
            .child([Some(Up), Some(Up), Some(Left), Some(Right)]);
        assert_eq!(node.last_directions(), Some([Up, Up, Left, Right]));
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use super::*;
    use crate::logic::game::direction::Direction::*;
    use std::hint::black_box;

    #[bench]
    fn bench_node_id_tree_walk(b: &mut test::Bencher) {
        let moves_sequence = [
            [Some(Down), Some(Right), Some(Down), Some(Left)],
            [Some(Up), Some(Up), Some(Left), Some(Right)],
            [Some(Down), Some(Down), Some(Down), Some(Up)],
            [Some(Right), Some(Left), Some(Up), Some(Down)],
            [Some(Up), Some(Down), Some(Right), Some(Left)],
        ];
        b.iter(|| {
            let mut node = NodeId::new();
            for moves in &moves_sequence {
                node = black_box(node).child(black_box(*moves));
            }
            let _ = black_box(node.depth());
            let _ = black_box(node.last_direction_for(0));
            let _ = black_box(node.last_directions());
            let _ = black_box(node.direction_at(2, 1));
            let _ = black_box(node.read_flags());
            // Walk back up
            while let Some(parent) = black_box(node).parent() {
                node = parent;
            }
            black_box(node)
        });
    }
}
