# Re-check of GitHub issue #165 (min/max clamp swaps + `_use` variant None-handling divergence)

GitHub issue #165 is a still-open Claude Code Review Report against the pre-rewrite codebase with
three concrete findings: **LOGIC_1** (a thread-count computation used `.min(1)` instead of
`.max(1)`, forcing 1 thread), **LOGIC_3** (`_use`-suffixed thread-compute loops did unconditional
`None => break` instead of checking `iter.is_completed_when_none_returned()`, risking dropped
results), and **LOGIC_5**/**UAF_1** (a `min_split_len` parameter in a parallel merge routine was
clamped with `.min(.., 3)` instead of `.max(.., 3)`, letting a tiny caller-supplied value flow
through to an unsafe `first_unchecked()`-style call on a possibly-empty slice). The maintainer's
own reply asked for re-analysis since none of the originally-named files exist after a full
architecture rewrite. This candidate re-checks whether the same *bug patterns* recur anywhere in
the current, post-rewrite codebase (`base_commit` b9214fbe).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Static grep/read sweep: every `.min(`/`.max(` in `src/` near thread/pool sizing, split lengths, chunk parameters; every `clamp(` call (crate-wide); every `unsafe`/`_unchecked` call site in split/merge/partition code | Static reading only, no build | No `.min(1)`-where-`.max(1)`-was-meant pattern found (`src/pool/par_thread_pool.rs:130-132`, `pool_impl/{once,basic}.rs`, `sort/sort_slice.rs` all use `min`/`max` correctly for their stated purpose). Zero `clamp(` calls exist crate-wide. No `min_split_len` identifier or equivalent exists anywhere in current `src/`. The only merge routine with `unsafe` reachable from the public API (`sort/sort_slice.rs`'s `MergeSubTask`/`find_split`) has explicit `len_a == 0`/`len_b == 0` branches, not a length clamp feeding an unchecked accessor |
| 2. Diff every `_use` module tree's `thread_execution/*.rs` against its non-`_use` sibling (`infallible` vs `infallible_use`, `option` vs `option_use`, `result` vs `result_use`) for `next.rs`/`next_any.rs`/`collect.rs`/`collect_arb.rs`/`reduce.rs` | `diff -u`, all 3 sibling pairs, 5 files each = 15 diffs | Every `None => break` site in every file, on both sides of every pair, is gated identically: `None if iter.is_completed_when_none_returned() => break` - zero unconditional `None => break` anywhere, zero divergence between a `_use` file and its non-`_use` sibling on this point |
| 3. Dependency-level check of whether the LOGIC_3 pattern is even *observable* through the current public API: read every `ConcurrentIter` impl in `orx-concurrent-iter` 4.3.0 (the crate's sole `ConcurrentIter`-providing dependency) | Read `~/.cargo/registry/.../orx-concurrent-iter-4.3.0/src/**/con_iter*.rs`, all 17 `is_completed_when_none_returned` impls | Every single implementation (`range`, `slice`, `slice_mut`, `vec`, `vec_deque`, `vec_drain`, `iter`, `chain` (both known/unknown-len), `enumerate`, `cloned`, `copied`, `jagged_arrays` (owned/reference), `empty`) hard-codes `true`. There is currently no `ConcurrentIter` source pluggable into this crate for which `None` can ever be transient - so even a hypothetical unconditional `None => break` would currently be behaviorally identical to the guarded version for every real source type |
| 4. Git archaeology (`git log -S`/`git show`) for whether the LOGIC_5/UAF_1 pattern ever actually existed in this repo's own history (not just the pre-rewrite report) | `git log --oneline -S"min_split_len" --all`, `git show <commit>` | **It did.** `d22fd263` ("introduce min_split_len to parallel merging", 2026-02-22) added `src/experiment/algorithms/merge_sorted_slices/par.rs` with a real `unsafe fn handle_extend` merge/split routine gated by `params.min_split_len`. The very next commit, `485929ac` ("fix mermory safety error", 2026-02-22), "fixed" a safety bug by adding `let min_split_len = core::cmp::min(params.min_split_len, 3);` - the **exact LOGIC_5/UAF_1 shape**: a caller-supplied `min_split_len` below 3 (e.g. 0) still produces `min(0, 3) == 0`, which does not enforce the invariant (`x >= 3`) the adjacent `unsafe { left_right.first_unchecked() }` a few lines below relies on (comment: "since left.len() >= 2, then left_right.len() > 0" - not guaranteed once the threshold can be 0). This file/module was never touched again after `485929ac` and was deleted wholesale in `24e64bd9` ("arch", 2026-03-05, confirmed via `git merge-base --is-ancestor` to be on the current branch's own history, not an abandoned branch) as part of the architecture rewrite - it does not exist in current `HEAD` and the current crate does not even depend on `orx-concurrent-recursive-iter` (the dependency that module used) |
| 5. Real end-to-end dynamic check via the crate's public API, debug-assertions + overflow-checks build, 2 repeat runs each: (a) `.num_threads(n)`/`.num_threads(Auto)` over a 2,000,000-element range, recording every distinct `std::thread::current().id()` actually observed inside `for_each`; (b) `use_new(...)` pipelines (`collect`/`reduce`/`any`/`all`) over 300,000 elements across 6 thread counts x 5 chunk sizes (30 combinations), checked against a plain sequential computation; (c) `par_experimental_sort` (the only public unsafe merge routine) on reverse-sorted inputs at every length 0..=64, 1000..=1100 (straddling the current `n < 1024` sequential-fallback threshold), and {2048,4096,10000,50000}, at 6 thread counts | `repro/` (see below), `cargo run` per binary, 2/2 identical passes each | `num_threads(8)` on the large input used exactly 8 distinct threads (never 1); `num_threads(Auto)` used all 24 available. All 30 `use_new` combinations matched the sequential reference exactly (no dropped/duplicated elements). All `par_experimental_sort` boundary lengths/thread-counts matched `sort_unstable()` exactly - no panic, no wrong output |
| 6. Miri escalation on the same `par_experimental_sort` boundary sweep (smaller range for interpreter overhead: len 0..=20, 3 thread counts), 2 repeat runs | `cargo +nightly miri run`, `MIRIFLAGS=-Zmiri-ignore-leaks` | Clean both runs, no Stacked/Tree Borrows violation reported in the unsafe raw-pointer merge/copy code. (Without `-Zmiri-ignore-leaks`, Miri reports "the main thread terminated without waiting for all remaining threads" - this is a known, already-documented artifact of the thread pool's teardown under Miri's leak checker, first noted by candidate 004's `RESULT.md`, unrelated to this candidate's three patterns) |

Existing project tests already partially covered ground (2): `src/sort/sort_slice.rs`'s own
`#[cfg(test)]` module (`test_sort_empty_and_single`, `test_sort_small_slices`) already exercises
0/1-length inputs against the live merge sort, consistent with approach 5's finding.

## Status

**Not reproducible.** None of the three patterns from issue #165 exist in the current codebase:

- **LOGIC_1** (thread-count `.min(1)`/`.max(1)` swap): no such swap exists; every `.min`/`.max` call
  near thread/pool sizing was read and is correct for its stated purpose (approach 1), and a real
  dynamic run confirms `num_threads(8)` on a large input actually uses 8 threads, never silently
  clamped to 1 (approach 5a, 2/2 runs).
- **LOGIC_3** (`_use` variant unconditional `None => break`): every `_use` thread-compute loop
  already gates identically to its non-`_use` sibling on `is_completed_when_none_returned()`
  (approach 2, exhaustive diff, zero divergence found) - and, further, no `ConcurrentIter` type
  currently pluggable into this crate can even return a transient `None` in the first place
  (approach 3 - every implementation in the pinned `orx-concurrent-iter` 4.3.0 hard-codes `true`),
  so this bug class is currently unreachable through the public API regardless of the guard's
  presence. A real dynamic run over 30 `(num_threads, chunk_size)` combinations found zero dropped
  elements (approach 5b, 2/2 runs).
- **LOGIC_5/UAF_1** (`min_split_len` `.min(.., 3)`/`.max(.., 3)` swap feeding an unsafe accessor):
  this exact bug pattern **did exist historically in this repo's own git history**
  (`485929ac`, 2026-02-22 - see approach 4), but lived in a module (`src/experiment/algorithms/
  merge_sorted_slices/`) that was deleted wholesale in the architecture rewrite (`24e64bd9`,
  2026-03-05) before ever being independently fixed, and does not exist in current `HEAD`. The only
  currently-reachable unsafe merge routine (`src/sort/sort_slice.rs`, the `experimental`-feature
  `par_experimental_sort`) uses a structurally different design (explicit `len_a == 0`/`len_b == 0`
  branches, and a `n < 1024` guard that keeps the whole parallel-merge code path unreached below
  that size) with no length-clamp-feeding-unchecked-accessor shape at all - confirmed correct at
  every boundary length 0..=64 and across the `n < 1024` threshold, under both a debug-assertions
  build and Miri (approaches 1, 5c, 6).

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not applicable in the "confirmed bug in current code" sense. Per-pattern reasoning:

- LOGIC_1: the current thread-count computation (`ParThreadPool::max_num_threads_for_computation`,
  `src/pool/par_thread_pool.rs:120-141`) never applies `.min(1)` where `.max(1)` belongs; every
  `.max(1)` there exists specifically to avoid a zero-thread `NonZeroUsize` construction from a
  zero-length input, and every `.min(...)` there exists specifically to cap by an actual upper
  bound (input size, requested max, or pool capacity) - the intended semantics, not a copy-paste
  inversion.
- LOGIC_3: this candidate's own hypothesis (a copy-paste divergence between `_use` and non-`_use`
  siblings) does not hold - both sides of every sibling pair were generated/maintained identically
  on this point. Additionally, the underlying precondition for the bug to matter (a `ConcurrentIter`
  that can return a transient `None`) is not satisfied by any type currently shipped in this
  crate's own dependency graph.
- LOGIC_5/UAF_1: this pattern is a real, git-verifiable historical bug in this exact repository
  (introduced by `485929ac`, "fix mermory safety error", 2026-02-22, itself already a botched fix
  of a related issue introduced one commit earlier by `d22fd263`), but it lived and died entirely
  within a module (`src/experiment/algorithms/merge_sorted_slices/`) that no longer exists - removed
  wholesale by the rewrite commit `24e64bd9` ("arch", 2026-03-05) before ever being independently
  patched. Nothing in the current `src/sort/` merge-sort implementation reuses that module's
  design or its `min_split_len` concept.

## Introduced by

N/A - no bug confirmed in current `HEAD`. For the record (not a "this candidate is Confirmed"
claim, see "Status" above): the historical LOGIC_5/UAF_1 instance found during archaeology was
introduced by 485929ac "fix mermory safety error" (2026-02-22) and removed by 24e64bd9 "arch"
(2026-03-05), both confirmed ancestors of current `HEAD` via `git merge-base --is-ancestor`.

## The fix

Not applicable - `SKILL.md` step 6 reserves a bounded fix/`patch.diff` for a `Confirmed` candidate,
and this one is `Not reproducible`. No `patch.diff` is included.

- Project test suite: not run before/after a fix, since no fix was attempted (nothing reproduced
  to fix). For reference, `cargo test --lib` on pristine `base_commit` (before any local edits, no
  `--features experimental`) passes cleanly: **99 passed, 0 failed, 0 ignored** - the same baseline
  candidate 007 recorded.

## What's not done / open questions

- This candidate's `repro/` needed candidate 001's already-confirmed one-line fix
  (`collect_arb_over_bag_bool_type_mismatch`) applied to the clone purely so the crate compiles at
  all with `--features experimental` (required to reach `par_experimental_sort`) - that fix is
  unrelated to this candidate's own three patterns and is not claimed as this candidate's finding;
  see the `justfile`'s `apply-prereq-fix` recipe and its header note.
- `sort_boundary`/`miri_sort_boundary` only exercise `par_experimental_sort`, the one unsafe merge
  routine currently reachable from the public API. `src/collectables/alg/merge_collected/
  merge_ordered_into.rs` (the other real merge routine in the crate, used by ordinary
  `collect`/non-experimental paths) was read (approach 1) and has no length-clamp/unsafe-accessor
  shape to check in the first place (its `unsafe` blocks are plain pointer copies driven by
  positions already recorded during collection, not by a caller-tunable length parameter) - not
  independently stress-tested here since there is no comparable clamp to target.
- The Miri leak-checker artifact noted in approach 6 is not this candidate's finding - it was
  already recorded by candidate 004's `RESULT.md`; not re-investigated further here.
- If a future session ever reintroduces a caller-tunable minimum-split-length parameter into a
  parallel merge routine (e.g. if `par_experimental_sort` grows a public `min_split_len`-style
  knob), the exact LOGIC_5/UAF_1 shape confirmed real in this repo's own history (section above) is
  worth re-checking against the new code, since it has now demonstrably happened once already in
  this codebase.
