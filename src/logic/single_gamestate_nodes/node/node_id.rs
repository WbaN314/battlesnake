use std::fmt::{Debug, Display};
use std::str::FromStr;

use crate::logic::game::{direction::Direction, moves::Moves, snakes::SNAKES};

/// Bits reserved for storing the depth (5 bits supports up to 31 depth levels).
const DEPTH_BITS: u32 = 5;
/// Bits per snake direction at a level: 2 bits for real moves (Up/Down/Left/Right).
/// None moves are tracked separately via `none_start`.
const BITS_PER_SNAKE: u32 = 2;
/// Bits consumed per tree level (4 snakes × 2 bits).
const BITS_PER_LEVEL: u32 = BITS_PER_SNAKE * SNAKES as u32; // 8
/// Bits used to store the first-None level per snake (5 bits supports depths 0-30 + sentinel).
const NONE_START_BITS: u32 = 5;
/// Sentinel value meaning a snake has never gone None.
const NONE_SENTINEL: u8 = 0x1F; // 31
/// Total header bits before level data: depth + 4 × none_start.
const HEADER_BITS: u32 = DEPTH_BITS + NONE_START_BITS * SNAKES as u32; // 25
/// Total bits in the backing store ([u128; 2]).
const TOTAL_BITS: u32 = 256;
/// Maximum supported tree depth: (256 − 25) / 8 = 28.
const MAX_DEPTH: u8 = ((TOTAL_BITS - HEADER_BITS) / BITS_PER_LEVEL) as u8;
/// Spare bits after all level data, used as user flags.
const FLAGS_BITS: u32 = TOTAL_BITS - HEADER_BITS - MAX_DEPTH as u32 * BITS_PER_LEVEL; // 7
/// Bit position of the flags field (top of data[1]).
const FLAGS_START: u32 = TOTAL_BITS - FLAGS_BITS;
/// Initial data[0] value: depth=0, all none_start = NONE_SENTINEL (bits 5-24 all 1s).
const NONE_SENTINEL_INITIAL: u128 =
    ((1u128 << (NONE_START_BITS * SNAKES as u32)) - 1) << DEPTH_BITS;

pub type DirectionVector = [Option<Direction>; SNAKES as usize];

/// Compact node identifier for the game tree, packed into two `u128` values (256 bits total).
///
/// Layout (LSB first):
/// ```text
/// [depth: 5][none_start×4: 20][level 0: 8]...[level 27: 8][flags: 7]
/// ```
/// Each level encodes 4 snake directions (2 bits each, real moves only):
/// ```text
/// [snake 0: 2 bits][snake 1: 2 bits][snake 2: 2 bits][snake 3: 2 bits]
/// ```
/// Real direction encoding: Up=0, Down=1, Left=2, Right=3.
/// `None` moves are not stored per-level. Instead, `none_start[snake]` records the first
/// depth at which that snake went `None` (sentinel `31` = never). All levels ≥ `none_start`
/// for that snake implicitly return `None`.
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct NodeId {
    data: [u128; 2],
}

#[inline(always)]
fn encode_real_dir(dir: Direction) -> u128 {
    match dir {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    }
}

#[inline(always)]
fn decode_real_dir(val: u128) -> Direction {
    match val & 0b11 {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => unreachable!(),
    }
}

#[inline(always)]
fn dir_char(dir: Option<Direction>) -> char {
    match dir {
        None => '_',
        Some(Direction::Up) => 'U',
        Some(Direction::Down) => 'D',
        Some(Direction::Left) => 'L',
        Some(Direction::Right) => 'R',
    }
}

impl NodeId {
    pub const MAX_DEPTH: u8 = MAX_DEPTH;
    /// Number of flag bits available (7). Valid flag values are `0 ..= (1 << MAX_FLAGS) - 1`.
    pub const MAX_FLAGS: u32 = FLAGS_BITS;

    pub fn new() -> Self {
        // depth=0, all none_start = NONE_SENTINEL (bits 5-24 all 1s), rest zero.
        NodeId {
            data: [NONE_SENTINEL_INITIAL, 0],
        }
    }

    /// Reads `len` bits starting at `start` (LSB-first across the two u128s).
    #[inline(always)]
    fn read_bits(&self, start: u32, len: u32) -> u128 {
        debug_assert!(len > 0 && len <= 64);
        let mask = (1u128 << len) - 1;
        if start >= 128 {
            (self.data[1] >> (start - 128)) & mask
        } else if start + len <= 128 {
            (self.data[0] >> start) & mask
        } else {
            let bits_in_low = 128 - start;
            let low = self.data[0] >> start;
            let high = self.data[1] & ((1u128 << (len - bits_in_low)) - 1);
            low | (high << bits_in_low)
        }
    }

    /// Writes `len` bits of `val` starting at `start` (LSB-first across the two u128s).
    #[inline(always)]
    fn write_bits(&mut self, start: u32, len: u32, val: u128) {
        debug_assert!(len > 0 && len <= 64);
        let mask = (1u128 << len) - 1;
        let val = val & mask;
        if start >= 128 {
            let s = start - 128;
            self.data[1] = (self.data[1] & !(mask << s)) | (val << s);
        } else if start + len <= 128 {
            self.data[0] = (self.data[0] & !(mask << start)) | (val << start);
        } else {
            let bits_in_low = 128 - start;
            let low_mask = (1u128 << bits_in_low) - 1;
            self.data[0] = (self.data[0] & !(low_mask << start)) | ((val & low_mask) << start);
            let high_bits = len - bits_in_low;
            let high_mask = (1u128 << high_bits) - 1;
            self.data[1] = (self.data[1] & !high_mask) | (val >> bits_in_low);
        }
    }

    /// Returns the first level at which `snake` went None, or `NONE_SENTINEL` if never.
    #[inline(always)]
    fn none_start(&self, snake: u8) -> u8 {
        self.read_bits(DEPTH_BITS + snake as u32 * NONE_START_BITS, NONE_START_BITS) as u8
    }

    #[inline(always)]
    fn set_none_start(&mut self, snake: u8, val: u8) {
        self.write_bits(
            DEPTH_BITS + snake as u32 * NONE_START_BITS,
            NONE_START_BITS,
            val as u128,
        );
    }

    pub fn depth(&self) -> u8 {
        self.read_bits(0, DEPTH_BITS) as u8
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
            match dir {
                None => {
                    let ns = self.none_start(i as u8);
                    if ns == NONE_SENTINEL {
                        self.set_none_start(i as u8, depth);
                    } else {
                        debug_assert!(
                            ns < depth,
                            "Snake {} none_start {} should be < depth {}",
                            i,
                            ns,
                            depth
                        );
                    }
                    // 2-bit slot stays 0 — never read once none_start is set
                }
                Some(d) => {
                    debug_assert_eq!(
                        self.none_start(i as u8),
                        NONE_SENTINEL,
                        "Snake {} already went None at level {}",
                        i,
                        self.none_start(i as u8)
                    );
                    encoded |= encode_real_dir(d) << (i as u32 * BITS_PER_SNAKE);
                }
            }
        }
        let shift = HEADER_BITS + depth as u32 * BITS_PER_LEVEL;
        self.write_bits(shift, BITS_PER_LEVEL, encoded);
        self.write_bits(0, DEPTH_BITS, depth as u128 + 1);
    }

    pub fn parent(&self) -> Option<Self> {
        let depth = self.depth();
        if depth == 0 {
            return None;
        }
        let last = depth - 1;
        let mut parent = *self;
        // Reset none_start for any snake that first went None at the level being removed.
        for snake in 0..SNAKES as u8 {
            if self.none_start(snake) == last {
                parent.set_none_start(snake, NONE_SENTINEL);
            }
        }
        let shift = HEADER_BITS + last as u32 * BITS_PER_LEVEL;
        parent.write_bits(shift, BITS_PER_LEVEL, 0);
        parent.write_bits(0, DEPTH_BITS, last as u128);
        Some(parent)
    }

    /// Returns the direction stored for `snake` at the last level.
    /// Outer `None` = root (depth 0). Inner `None` = snake went None at or before this level.
    pub fn last_direction_for(&self, snake: u8) -> Option<Option<Direction>> {
        let depth = self.depth();
        if depth == 0 {
            return None;
        }
        self.direction_at(depth - 1, snake)
    }

    pub fn last_directions(&self) -> Option<DirectionVector> {
        let depth = self.depth();
        if depth == 0 {
            return None;
        }
        let mut directions = [None; SNAKES];
        for snake in 0..SNAKES as u8 {
            directions[snake as usize] = self.direction_at(depth - 1, snake).unwrap();
        }
        Some(directions)
    }

    /// Returns the 7 spare flag bits stored at the top of the node id.
    #[inline(always)]
    pub fn read_flags(&self) -> u8 {
        self.read_bits(FLAGS_START, FLAGS_BITS) as u8
    }

    /// Overwrites the flag bits. Only the low `MAX_FLAGS` (7) bits of `val` are used.
    #[inline(always)]
    pub fn set_flags(&mut self, val: u8) {
        self.write_bits(FLAGS_START, FLAGS_BITS, val as u128);
    }

    pub fn is_fast_tracked(&self) -> bool {
        self.read_bits(FLAGS_START, 1) != 0
    }

    pub fn set_fast_tracked(&mut self, val: bool) {
        self.write_bits(FLAGS_START, 1, val as u128);
    }

    /// Returns the direction of `snake` at `level`.
    /// Outer `None` = level >= depth. Inner `None` = snake went None at or before this level.
    pub fn direction_at(&self, level: u8, snake: u8) -> Option<Option<Direction>> {
        if level >= self.depth() {
            return None;
        }
        if level >= self.none_start(snake) {
            return Some(None);
        }
        let shift = HEADER_BITS + level as u32 * BITS_PER_LEVEL + snake as u32 * BITS_PER_SNAKE;
        Some(Some(decode_real_dir(self.read_bits(shift, BITS_PER_SNAKE))))
    }
}

/// Displays the node path grouped by depth level: `level0moves-level1moves-...`
///
/// Each character is one of U/D/L/R (direction) or X (None move).
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
                write!(f, "{}", dir_char(self.direction_at(level, snake).unwrap()))?;
            }
        }
        let flags = self.flags();
        if flags != 0 {
            write!(f, "+[{:07b}]", flags)?;
        }
        Ok(())
    }
}

impl Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for NodeId {
    type Err = String;

    /// Parses a string like `"DRDL-DUDU-UUDD"` or `"ROOT"` into a `NodeId`.
    /// Each group is one depth level with one char per snake: U/D/L/R or X (None).
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
                moves[snake] = match ch {
                    b'_' => None,
                    b'U' => Some(Direction::Up),
                    b'D' => Some(Direction::Down),
                    b'L' => Some(Direction::Left),
                    b'R' => Some(Direction::Right),
                    _ => return Err(format!("Invalid direction char: '{}'", ch as char)),
                };
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
        assert_eq!(child.last_direction_for(0), Some(Some(Down)));
        assert_eq!(child.last_direction_for(1), Some(Some(Right)));
        assert_eq!(child.last_direction_for(2), Some(Some(Up)));
        assert_eq!(child.last_direction_for(3), Some(Some(Left)));
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
    fn none_directions_stored_as_x() {
        let root = NodeId::new();
        let child = root.child([Some(Down), None, Some(Up), None]);

        assert_eq!(child.to_string(), "D_U_");
        assert_eq!(child.direction_at(0, 0), Some(Some(Down)));
        assert_eq!(child.direction_at(0, 1), Some(None));
        assert_eq!(child.direction_at(0, 2), Some(Some(Up)));
        assert_eq!(child.direction_at(0, 3), Some(None));
    }

    #[test]
    fn none_propagates_to_children() {
        // Once a snake goes None, it stays None — none_start is inherited.
        let c1 = NodeId::new().child([Some(Down), None, Some(Up), None]);
        let c2 = c1.child([Some(Right), None, Some(Left), None]);

        assert_eq!(c2.to_string(), "D_U_-R_L_");
        assert_eq!(c2.direction_at(0, 1), Some(None));
        assert_eq!(c2.direction_at(1, 1), Some(None));
        assert_eq!(c2.none_start(1), 0);
        assert_eq!(c2.none_start(3), 0);
    }

    #[test]
    fn none_parent_resets_none_start() {
        // If a snake went None at the last level, parent() should restore none_start to sentinel.
        let root = NodeId::new();
        let c1 = root.child([Some(Down), None, Some(Up), None]);
        assert_eq!(c1.parent(), Some(root));
        let parent = c1.parent().unwrap();
        assert_eq!(parent.none_start(1), NONE_SENTINEL);
        assert_eq!(parent.none_start(3), NONE_SENTINEL);
    }

    #[test]
    fn none_parent_preserves_earlier_none_start() {
        // none_start at level 0 should survive popping level 1.
        let c1 = NodeId::new().child([Some(Down), None, Some(Up), None]);
        let c2 = c1.child([Some(Right), None, Some(Left), None]);
        let back_to_c1 = c2.parent().unwrap();
        assert_eq!(back_to_c1, c1);
        assert_eq!(back_to_c1.none_start(1), 0);
        assert_eq!(back_to_c1.none_start(3), 0);
    }

    #[test]
    fn none_directions_round_trip() {
        let node: NodeId = "D_U_".parse().unwrap();
        assert_eq!(node.depth(), 1);
        assert_eq!(node.to_string(), "D_U_");
        assert_eq!(node.last_direction_for(0), Some(Some(Down)));
        assert_eq!(node.last_direction_for(1), Some(None));
        assert_eq!(node.last_direction_for(2), Some(Some(Up)));
        assert_eq!(node.last_direction_for(3), Some(None));
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
        assert!("DQDL-DUDU-UUDD-RDDR".parse::<NodeId>().is_err());
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
        assert_eq!(
            node.last_directions(),
            Some([Some(Up), Some(Up), Some(Left), Some(Right)])
        );
    }

    #[test]
    fn boundary_crossing_level_12() {
        // Level 12 starts at bit 25+12*8=121 and ends at bit 129, crossing the u128 boundary.
        let mut node = NodeId::new();
        for _ in 0..12 {
            node = node.child([Some(Up), Some(Up), Some(Up), Some(Up)]);
        }
        node = node.child([Some(Down), Some(Right), Some(Left), None]);
        assert_eq!(node.depth(), 13);
        assert_eq!(node.last_direction_for(0), Some(Some(Down)));
        assert_eq!(node.last_direction_for(1), Some(Some(Right)));
        assert_eq!(node.last_direction_for(2), Some(Some(Left)));
        assert_eq!(node.last_direction_for(3), Some(None));
        let parent = node.parent().unwrap();
        assert_eq!(parent.depth(), 12);
        assert_eq!(
            parent.child([Some(Down), Some(Right), Some(Left), None]),
            node
        );
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
            // Walk back up
            while let Some(parent) = black_box(node).parent() {
                node = parent;
            }
            black_box(node)
        });
    }
}
