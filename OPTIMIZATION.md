# Optimization Log

Fixed-work benchmarks (`cargo bench --lib bench_simulation`):

| Step | max_depth (ns) | max_nodes (ns) | Flamegraph |
|------|---------------|----------------|------------|
| 0 — baseline | 37,669,337 | 151,734,891 | `flamegraph_0_before_any_optimisation.svg` |
| 1 — FxHash | 30,676,550 (−18.6%) | 126,207,012 (−16.8%) | `flamegraph_1_after_fxhash.svg` |

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

## Next

Profile now points here (shares of budget-bounded run):

1. **Allocation churn — ~50%.** `pregenerate`/`pregenerate_for` alloc a fresh `Vec<Moves>` per call; `children: Vec<Node>` + `similarity_set` per direction; `distances` in `prune_head_tail`. → `arrayvec::ArrayVec` (already a dep) + reuse scratch buffers.
2. **PriorityQueue BTreeMap — ~17%.** `pop` does `iter().next()` + separate `remove` (double traversal) → `pop_first()`. Bounded `(i8, u8)` key range → could be a fixed array of `VecDeque`s.
3. **Cargo profile.** No `[profile.release]`. Add `lto = "fat"`, `codegen-units = 1`. Cheap, zero code risk.
4. **Minor.** `NodeStatus::partial_cmp` recursive reverse arms; `calculate_from_child_states` walks children up to 4×.

Re-profile after each step — shares reshuffle every time.
