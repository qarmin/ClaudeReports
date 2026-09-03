# merge_ordered_into_unchecked_positions_partition_invariant

`merge_ord_into_vec`/`merge_ord_into_split_vec` (`src/collectables/alg/merge_collected/merge_ordered_into.rs:25-103`)
k-way merge per-thread `ValsAndIdx<T>` results using raw-pointer `copy_from_nonoverlapping`
(`extend_from_nonoverlapping` for the split-vec variant) and a final `dst.set_len(total_len)`,
trusting - with no runtime assertion anywhere in the function - that each `ValsAndIdx.positions`
list exactly and non-overlappingly partitions its own `values` buffer (`sum(positions[].len) ==
values.len()`, and the running `beg` offsets the merge loop derives from consecutive `len`s
actually match real element boundaries in `values`). This is the *live* copy of the merge algorithm
(every real collector in `src/collectables/{inf,inf_use,res,res_use,opt,opt_use}/{vec,vec2,
split_vec_*}.rs` calls through it), distinct from the dead `ValIdx::collect_into` already ruled out
in candidate 016. If the invariant were ever violated, the unchecked pointer arithmetic would
silently read/write out of bounds or leave part of `dst` reading spuriously-"initialized" memory
after `set_len`, instead of panicking.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual invariant proof by reading every mutator of `ValsAndIdx.values`/`.positions` | Whole-crate `grep -rn "\.values\b"` over `src/` (see below) plus reading `src/results/vals_and_idx.rs` and all 3 real per-thread `collect` implementations (`src/{infallible,result,option}/thread_execution/collect.rs`) | `values`/`positions` are only ever mutated together, in lockstep, inside `ValsAndIdx::extend`/`extend_res`/`extend_opt` (`src/results/vals_and_idx.rs`): each call appends some values then immediately pushes exactly one `IdxLen{idx, len}` recording the count just appended. No other `src/` file mutates either field directly (the only other touches are `merge_ordered_into.rs`'s own reads plus its post-merge `vec.values.set_len(0)`, which runs *after* the merge). `mod results;` in `src/lib.rs` is not `pub`, so `ValsAndIdx` is not reachable from outside the crate either - there is no code path, internal or external, that can push to `values` without also pushing the matching `positions` entry. The invariant candidate 020 asks about therefore holds by construction on every value ever passed into `merge_ord_into_vec`/`merge_ord_into_split_vec` in this codebase |
| Debug-assertions stress test (`tests/bughunt_020_merge_stress.rs`, run against the real public `orx_parallel::Par`/`ParOption` API, not a hand-built `ValsAndIdx`) - variable-length `flat_map` (0/1/2/3 outputs per input index) across 15 input sizes x 5 thread counts x 5 chunk sizes for the infallible collector, and 7 sizes x 3 x 3 for the `Option` collector, comparing against a sequential reference | `cargo test --test bughunt_020_merge_stress` (debug build, `debug-assertions = true` is cargo's test-profile default; workspace `Cargo.toml` does not override it), 375 infallible combos + 63 option combos | All combos passed, output byte-for-byte identical to the sequential reference (see `output.txt`) - no panic, no OOB, no reordering |
| Same stress test under `miri` (catches OOB/uninitialized-read/aliasing UB directly, independent of whether it happens to crash externally) | `rustup run nightly cargo miri test --test bughunt_020_merge_stress`, nightly miri 0.1.0 (c656540d64, 2026-08-21), same combo matrix as above | Both tests reported `ok` under miri - no UB diagnostic on any combo (see `output.txt`). A separate, unrelated miri complaint ("the main thread terminated without waiting for all remaining threads") fires *after* both tests already passed - this is the test harness's own thread-pool/global-pool shutdown behavior, not anything inside `merge_ord_into_vec`; noted as a possible new suspect below, out of scope for this candidate |

## Status

**Not reproducible.** The invariant this candidate worried about is maintained by construction
(the only two mutators of `ValsAndIdx`'s fields keep `values` and `positions` in lockstep), and a
375+63-combo stress test across variable-length per-index outputs, multiple thread counts and
chunk sizes, run under both a debug-assertions build and `miri`, found no violation - no panic, no
wrong output, no UB diagnostic. This covers the ordered-`Vec<T>` collector (`merge_ord_into_vec`,
used by every `Vec`-returning collector for `Par`/`ParOption` across the sizes/thread-count/
chunk-size space exercised) and, via the shared `ValsAndIdx` type, the same construction guarantee
that also covers `merge_ord_into_split_vec` (not separately stress-tested with `SplitVec` as the
destination, since it shares the identical per-thread `results: Vec<ValsAndIdx<T>>` input and the
identical position-partition assumption - see "What's not done" below). The `Result`-collector
variant (`extend_res`, early-return-on-`Err`) was reasoned about by code reading only, not
separately stress-tested (see below).

**Fully reproducible**: N/A (Not reproducible - nothing reproduced, so there is no repro to run).

## Class

N/A (Not reproducible).

## Root cause

Not a bug in the live code as it exists today: `ValsAndIdx::extend`/`extend_res`/`extend_opt` are
the *only* writers of `.values`/`.positions` anywhere in the crate (private `mod results;`, so not
reachable from outside either), and each one appends to `values` and pushes exactly one matching
`IdxLen{idx, len}` in the same call, in order. That makes "positions exactly and non-overlappingly
partitions values" a true structural invariant of every `ValsAndIdx` value that can ever exist in
this codebase, not merely an assumption `merge_ord_into_vec` happens to get lucky on - there is
no mutation path left that could desynchronize the two fields. The absence of a runtime assertion
in `merge_ord_into_vec` is a real latent risk *if* a future change ever adds a third way to
construct/mutate a `ValsAndIdx` (e.g. a hypothetical bulk constructor, or making the module `pub`
so external code could poke at the public fields) without preserving this invariant - but there is
no such path today, so there is nothing to reproduce against the current tree.

## Introduced by

N/A - not applicable to a Not-reproducible finding (no bug exists at HEAD to attribute to an
introducing commit).

## The fix

No fix applies to a Not-reproducible finding. If a defense-in-depth `debug_assert!` were still
considered worthwhile purely to guard against a *future* regression (not because today's code is
unsafe), the natural place would be a `debug_assert_eq!(vec.positions.iter().map(|p| p.len).sum::<usize>(), vec.values.len())`
per `vec` at the top of `merge_ord_into_vec`/`merge_ord_into_split_vec` - left as a suggestion, not
applied, since there is no bug to verify a fix against this session.

- Project test suite: N/A, no patch was applied (unmodified tree). Baseline recorded anyway:
  `cargo test --lib` -> 99 passed; 0 failed; 0 ignored (matches the baseline candidate 016 recorded
  on the same commit).

## What's not done / open questions

- `merge_ord_into_split_vec` (the `SplitVec<T, G>` destination variant, same file) was not
  separately stress-tested with a `SplitVec`-collecting entry point - it shares the exact same
  `results: Vec<ValsAndIdx<T>>` input and the same position-partition assumption as
  `merge_ord_into_vec`, and the construction-invariant argument above applies identically (it does
  not depend on which merge function reads the `ValsAndIdx`s), but this was reasoned rather than
  independently exercised end-to-end through a `SplitVec` collector.
- The `Result`-collector path (`extend_res`, `src/result/thread_execution/collect.rs`) was checked
  by code reading only (confirmed it pushes exactly one matching `IdzLen` per call, same as
  `extend`/`extend_opt`, including on the early-`Err`-return branch) but not exercised by the stress
  test itself, since an early `Err` return never reaches `merge_ord_into_vec` at all (the whole
  per-thread `Vec<ValsAndIdx<T>>` collection is discarded on any thread's first `Err`) - there was
  nothing additional for a stress test to add here beyond what candidate 006
  (`006.sort_slice_mergesubtask_panic_safety`) already covers for this same merge's panic-safety
  under `Err`/panic conditions.
- This candidate does not re-litigate whether `merge_ord_into_vec`'s merge loop is *panic-safe*
  mid-copy (a different concern, already the subject of candidate 006) - it only covers whether the
  positions-partition-values invariant this candidate specifically names can be violated through any
  real path in this codebase.
- A stray, unrelated miri-only complaint surfaced while running the stress test ("the main thread
  terminated without waiting for all remaining threads", after both tests already reported `ok`) -
  looks like this project's global/persistent thread pool not being joined before the miri process
  exits; not investigated further here since it is orthogonal to `merge_ord_into_vec` and did not
  affect either test's `ok` result - noted as a possible new suspect for a future candidate.
