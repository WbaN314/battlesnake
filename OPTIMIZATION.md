# Optimization Log

Fixed-work benchmarks (`cargo bench --lib bench_simulation`):

| Step | max_depth (ns) | max_nodes (ns) | Flamegraph |
|------|---------------|----------------|------------|
| 0 — baseline | 37,669,337 | 151,734,891 | `flamegraph_0_before_any_optimisation.svg` |
| 1 — FxHash | 30,676,550 (−18.6%) | 126,207,012 (−16.8%) | `flamegraph_1_after_fxhash.svg` |
| 2 — ArrayVec pregenerate | ~29,600,000 (−3.5%) | ~123,900,000 (−1.9%) | `flamegraph_2_after_arrayvec_movelist.svg` |

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

## Next

Shares from the latest run (`flamegraph_2`). Allocation is largely tapped for *cheap* wins —
what looked like "~50% alloc" was mostly stack `memmove` + teardown, not removable mallocs.

1. **PriorityQueue — 17.6%** (`BTreeMap<(i8,u8), VecDeque<NodeId>>`). Now the top target.
   `pop` does `iter().next()` + separate `remove` (double traversal) → `pop_first()`. Bounded
   `(i8, u8)` key range → could be a fixed array of `VecDeque`s, killing the btree allocs too.
2. **Cargo profile — free.** No `[profile.release]`. Add `lto = "fat"`, `codegen-units = 1`.
   Zero code risk, typically 5–15%. Best ROI.
3. **memmove 8.5% — `GameState` clone per child.** Not a malloc; a stack copy. Only removable
   by mutate-in-place + undo — large, risky refactor of the search core. Defer.
4. **Minor.** `NodeStatus::partial_cmp` recursive reverse arms; `calculate_from_child_states`
   walks children up to 4×. Node-local Vecs (`children` 2.4%, `children_states` 1.5%) are small.

Re-profile after each step — shares reshuffle every time.
