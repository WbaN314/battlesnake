use std::collections::VecDeque;

use crate::logic::single_gamestate_nodes::node::node_id::NodeId;

/// Priority range headroom: real values today are `-1..=2` (with `-1` lowest priority,
/// popped last). The extra slot below `-1` is slack so a future priority tweak doesn't
/// force a structural change. `push` debug-asserts values stay inside this window.
const MIN_PRIORITY: i8 = -2;
const MAX_PRIORITY: i8 = 2;
const PRIORITY_SLOTS: usize = (MAX_PRIORITY - MIN_PRIORITY + 1) as usize;
const DEPTH_SLOTS: usize = NodeId::MAX_DEPTH as usize + 1;
/// Total buckets. Ascending index == pop order (highest priority first, then shallowest).
const BUCKET_COUNT: usize = PRIORITY_SLOTS * DEPTH_SLOTS;

/// Flattened bucket index for `(priority, depth)`. Higher priority maps to a lower index
/// (popped first); within a priority, shallower depth maps to a lower index.
#[inline]
fn bucket_index(priority: i8, depth: u8) -> usize {
    debug_assert!(
        (MIN_PRIORITY..=MAX_PRIORITY).contains(&priority),
        "priority {priority} outside [{MIN_PRIORITY}, {MAX_PRIORITY}] — widen the window"
    );
    debug_assert!((depth as usize) < DEPTH_SLOTS, "depth {depth} exceeds MAX_DEPTH");
    (MAX_PRIORITY - priority) as usize * DEPTH_SLOTS + depth as usize
}

/// A bucketed priority queue over the tiny, bounded `(priority, depth)` key space.
///
/// The previous `BTreeMap<(i8, u8), VecDeque<NodeId>>` paid for tree navigation and
/// rebalancing on every push/pop, plus alloc/free churn as buckets emptied and refilled.
/// Since the key space is only `PRIORITY_SLOTS × DEPTH_SLOTS` (~145) buckets, a flat array
/// gives O(1) push, near-O(1) pop via a forward cursor, and — critically — buckets are
/// cleared, never freed, so there is no allocation churn during the search. Empty
/// `VecDeque`s don't allocate until first pushed, so construction is cheap.
#[derive(Clone)]
pub(in crate::logic::single_gamestate_nodes) struct PriorityQueue {
    buckets: Vec<VecDeque<NodeId>>,
    /// Lowest bucket index that may be non-empty; only ever moves forward on pop, and
    /// backward when a push lands below it. Avoids rescanning drained buckets.
    cursor: usize,
    len: usize,
}

impl PriorityQueue {
    pub(super) fn new() -> Self {
        Self {
            buckets: (0..BUCKET_COUNT).map(|_| VecDeque::new()).collect(),
            cursor: BUCKET_COUNT,
            len: 0,
        }
    }

    pub(super) fn from(id: NodeId) -> Self {
        let mut q = Self::new();
        q.push(id, 0);
        q
    }

    pub(super) fn push(&mut self, id: NodeId, priority: i8) {
        let i = bucket_index(priority, id.depth());
        self.buckets[i].push_back(id);
        if i < self.cursor {
            self.cursor = i;
        }
        self.len += 1;
    }

    pub(super) fn pop(&mut self) -> Option<NodeId> {
        while self.cursor < BUCKET_COUNT && self.buckets[self.cursor].is_empty() {
            self.cursor += 1;
        }
        let id = self.buckets.get_mut(self.cursor)?.pop_front();
        if id.is_some() {
            self.len -= 1;
        }
        id
    }

    pub(super) fn len(&self) -> usize {
        self.len
    }
}
