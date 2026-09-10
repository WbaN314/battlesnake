# Optimization Log

Fixed-work benchmarks (`cargo bench --lib bench_simulation`):

| Step | max_depth (ns) | max_nodes (ns) | Flamegraph |
|------|---------------|----------------|------------|
| 0 — baseline | 37,669,337 | 151,734,891 | `flamegraph_0_before_any_optimisation.svg` |
| 1 — FxHash | 30,676,550 (−18.6%) | 126,207,012 (−16.8%) | `flamegraph_1_after_fxhash.svg` |
| 2 — ArrayVec pregenerate | ~29,600,000 (−3.5%) | ~123,900,000 (−1.9%) | `flamegraph_2_after_arrayvec_movelist.svg` |
| 3 — flat PriorityQueue | ~29,600,000 (flat) | ~118,000,000 (−4.8%) | `flamegraph_3_after_priority_queue.svg` |
| 4 — pre-size node map | ~29,100,000 (flat) | ~109,400,000 (−7.5%) | — |

> `bench_snake_logic` is budget-bounded (fixed wall-clock) — use it for flamegraph *shares*, not timing. Use the `bench_simulation_*` benches for timing.

## Step 1 — FxHash instead of SipHash

Swapped the default (SipHash) hasher for `rustc_hash::Fx*` on the hot maps:
- `nodes: HashMap<NodeId, Node>` → `FxHashMap` — `tree/mod.rs`
- `similarity_set: HashSet<u64>` → `FxHashSet` — `node/mod.rs`

Why: `NodeId` is `[u128; 2]` (32 bytes), hashed on every `get`/`insert` and on every `propagate_status` walk. SipHash's DoS resistance is useless for internal keys. `similarity_set` was SipHashing a value that is *already* a hash.

Flamegraph share (budget-bounded run):

| | before | after |
|---|---|---|
| SipHash | 24.4% | 0.9% |
| FxHash | 0% | 7.8% |
| **total hashing** | **24.4%** | **8.7%** |

Cold maps (`Display`, `tree_stats.rs`) left on std `HashMap` — not in the search loop.

## Step 2 — ArrayVec for `pregenerate`

`MoveMatrix::pregenerate`/`generate` returned a heap `Vec<Moves>` allocated+freed per
direction per node. `Moves` is 4 bytes, capped at 4⁴ = 256 combos → `ArrayVec<Moves, 256>`
(stack, no heap). `moves.rs`.

Small win (~2–3.5%) despite `pregenerate` showing 13% inclusive on the flamegraph. Lessons:
- 13% was on `bench_snake_logic` (deep/wide); fixed benches call `pregenerate` far less.
- 712 was *inclusive* — the generation loops stayed; only alloc/free was removed.
- macOS xzone allocator recycles same-size small blocks nearly for free.

Flamegraph confirms the mechanism (budget-bounded run, shares):

| | after FxHash | after ArrayVec |
|---|---|---|
| `pregenerate` | 19.4% | 14.3% (−5 pts: alloc slice gone, gen logic kept) |
| malloc/free/realloc | 41.1% | 31.4% (−10 pts) |
| memmove (GameState copy) | 6.2% | 8.5% (bigger slice of smaller pie) |

Cap is `ArrayVec<Moves, 256>` (4⁴). Real ceiling is 3⁴=81 (snakes can't reverse) except
the length-1 root — but ArrayVec leaves unused capacity uninitialized, so 256 costs no
runtime over 81 and avoids a panic on the root. `SmallVec` would be needed for true
inline+spill, not worth the dep/branch.

Note: much of the remaining "alloc" is not cheap-to-remove mallocs — `GameState<BasicField>`
is fully stack-allocated (fixed arrays, `Copy` fields), so per-child `.clone()` is a pure
`memmove`, and `drop_glue::<Tree>` is bulk teardown. Neither is a quick swap.

Likely worth more on the production budget-bounded path.

## Step 3 — flat-array PriorityQueue

Replaced `BTreeMap<(i8,u8), VecDeque<NodeId>>` with a flat `Vec<VecDeque<NodeId>>` of
`PRIORITY_SLOTS × DEPTH_SLOTS` (~145) buckets + a forward `cursor` — `tree/mod.rs`.

Why: priority is only `-1..=2` and depth `0..=28`, so the key space is tiny and bounded.
The btree paid ~17.6% for node navigation (401) + rebalancing (50) + `Entry`/`OccupiedEntry`
(322) + `VecDeque` (111) — much of it *churn*: `pop` removed a bucket the instant its deque
emptied, and the next `push` recreated it (btree insert/remove/rebalance + deque alloc/free),
every step of the search. The flat array gives O(1) push, near-O(1) pop via the cursor, and
buckets are **cleared, never freed** → zero churn. Empty `VecDeque`s don't allocate until first
push, so the 145-bucket array is cheap to build. Priority window has a slack slot (`MIN=-2`)
+ `debug_assert` so future priorities don't silently mis-order.

Result: `max_nodes` −4.8% (queue-heavy, 10k nodes); `max_depth` flat (depth-4 tree barely
touches the queue). Ordering is identical to the old `(-priority, depth)` btree: ascending
flat index = highest priority then shallowest, FIFO within a bucket.

Flamegraph confirms (budget-bounded run, shares): **PriorityQueue 21.3% → 0.6%** — btree
navigation + rebalancing + VecDeque churn erased. The 21-point share drop ≫ the 4.8% timing
win because `bench_snake_logic` pounds the queue far harder than the fixed benches; the
*production* path (budget-bounded) therefore reclaims ~21% of each move's budget for real
search. Extracted to `tree/priority_queue.rs` afterwards (pure move, no logic change).

## Step 4 — pre-size the node map

`Tree::simulate` reserves the `nodes` `FxHashMap` up front instead of letting it grow —
`tree/mod.rs`. Measured node counts (`report_search_stats`, release, 200 ms): ~210k–330k,
avg ~249k. Growing there means ~19 doubling-resizes, each re-inserting every live `Node`
(~390 B) → ~190 MB of `memmove`; peak size is unchanged, so pre-sizing just does the one
allocation instead.

Gated because the count is only predictable when bounded by time (production, ~250k →
reserve `EXPECTED_TIME_BOUNDED_NODES = 300_000`) or nodes (`reserve(max_nodes.min(EXPECTED))`).
Depth-only searches are left to grow — a flat reserve there would memset a 512 KB control
array for a shallow tree and regress `max_depth`. Can't live in `Tree::new`: the builder sets
the bounds *after* construction, so `new` sees only defaults.

Result: `max_nodes` −7.5% (reserves 10k, skips ~13 resizes); `max_depth` flat (gate declines
— no regression). Production win (skips ~19 resizes / ~190 MB) shows only on the budget path.

## Next

Shares from `flamegraph_3` (post-PQ, pre-map-presize). PriorityQueue is gone (21.3% → 0.6%);
the freed time spread across everything else. Remaining allocation is now diffuse, and step 4
(map pre-size) has since removed the map's resize/grow slice.

1. **memmove ~6.8% — `GameState` clone per child.** Not a malloc; a stack copy. Only removable
   by mutate-in-place + undo — large, risky refactor of the search core. Defer.
2. **Tree teardown ~6.2%.** Freeing the whole map + all nodes at end; needs pooling/arena to
   avoid. Structural, defer.
3. **Diffuse small Vecs.** `children: Vec<Node>` (140), `RawVec<(usize,u8)>` (94),
   `children_states` (81). Individually small; `ArrayVec` on the bounded ones could help but
   low payoff.
4. **Minor.** `NodeStatus::partial_cmp` recursive reverse arms; `calculate_from_child_states`
   walks children up to 4×.

Re-profile after each step — shares reshuffle every time. (Map pre-size done; re-flamegraph
to see the map's grow slice shrink and where time went.)
