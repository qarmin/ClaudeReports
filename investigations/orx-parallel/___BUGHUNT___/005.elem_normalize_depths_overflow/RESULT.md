# Elem::normalize_depths overflows/wraps the breadth-first sort key for wide/deep par_recursive trees

`Elem::normalize_depths` (`src/infallible/recursive/execution/elem.rs:18-25`) computes
`elem.depth = elem.depth * depth_coef + elem.width` where `depth_coef = max_width + 1` is
recomputed independently at every recursion level, to build a single `usize` sort key encoding
breadth-first order for `par_recursive(...).collect()`/`.first()`/`.find()` under the default
`IterationOrder::Ordered`. It is called once per level from `src/infallible/recursive/execution/
collect.rs:65,85` and `src/infallible/recursive/execution/next.rs:52,74`, with no overflow
checking on either the multiply or the add. Source: `___FINDINGS___/findings.toml`
(`overflow_underflow`, `hard`). Candidate slug in `IDEAS.md`/batch: `elem_normalize_depths_overflow`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Debug-assertions build, real end-to-end `par_recursive` call through the crate's public API, single-spine construction (a chain of nodes each producing 1000 children per level, only the last child continuing, so total visited nodes stay ~8000 while the encoded key still compounds a branching-factor-1000 radix every level) | `cargo build`/`cargo run` (dev profile, default `overflow-checks = true`), `BRANCH=1000`, `LEVELS=8` | Panicked deterministically: `attempt to multiply with overflow` at `elem.rs:22:30`. 3/3 identical runs. |
| 2. Manual boundary-value construction, release build (default `overflow-checks = false`), a two-spine tree (a single root splitting into spine A and spine B, `BRANCH=3`, `LEVELS=42`, `3^42 ~ 1.09e20 >> usize::MAX ~ 1.8e19` on 64-bit) checked against a documented correctness invariant: since spine A's path key starts strictly smaller than spine B's and the identical recurrence applies to both, every A-lineage element visited at a given tree depth must sort strictly before every B-lineage element visited at the same depth, under the default `IterationOrder::Ordered` - unless the key has overflowed | `cargo run --release`, `BRANCH=3`, `LEVELS=42` (final repro in `repro/`) | Silent, wrong final order: an A-lineage element sorts *after* a B-lineage element at the same tree depth, at tree depths 41 and 42 specifically (exactly where `3^d` first exceeds `usize::MAX`) - and nowhere else. 3/3 identical runs (violation onset depth and violation count identical every run; the exact reported output index has +/-1 jitter from ordinary parallel-execution/thread-scheduling noise, unrelated to the bug). |
| 3. Negative controls to isolate cause (not counted against the soft cap - these are boundary-value variations of approach 2, not a distinct tool) | Same construction, `LEVELS=40` (`3^40 ~ 1.2e19 < usize::MAX`, no overflow expected); and a different branching factor `BRANCH=5, LEVELS=30` (`5^30 ~ 9.3e20`, also overflows) | `LEVELS=40`: `ORDER OK`, no violation - confirms the violation in approach 2 is specifically caused by the overflow, not a pre-existing, unrelated ordering bug in the two-spine construction itself. `BRANCH=5, LEVELS=30`: also triggers the bug (and, after the fix below, also triggers the fix's panic) - confirms the bug (and the fix) generalize beyond the one exact branching factor first tried. |

## Status

**Confirmed**. Reproduces 3/3 runs under a debug-assertions build (deterministic panic, approach
1), and 3/3 runs under a release build via the documented order-invariant violation (approach 2)
- both via the crate's real, public `par_recursive` entry point, no fixed thread count/seed
pinned. The release-mode manifestation (silent, wrong final order - not a crash) is the more
concerning one in practice, since it produces no diagnostic at all in a normal release build;
the debug-mode panic is a lower-severity but earlier warning sign of the same root cause.

**Fully reproducible**: **Yes**. `repro/src/main.rs` drives the crate's real, public
`orx_parallel::par_recursive` API directly (not a hand-built stand-in for it) with a real
`extend` closure, and lets the crate's own real parallel execution/sort machinery run
end-to-end to produce `.collect()`'s final `Vec`. This is a single-participant finding (how the
crate's own sort-key encoding behaves for a caller-supplied recursion shape), not an interaction
between two independent programs/peers - CONFIRMATION.md section 11's "real second participant"
requirement does not apply (see that section's own carve-out: input to a public entry point
*is* the genuine external input; there is no "other side" being impersonated here). The
`extend` closure in `repro/` is exactly the kind of caller-supplied argument the public
`par_recursive` API is documented to accept.

## Class

overflow_underflow (also logic: the practical, user-visible effect in a release build is a
silently wrong output ordering, not a crash)

## Root cause

`Elem::normalize_depths` folds a node's accumulated path through the tree into a single `usize`
via `elem.depth = elem.depth * depth_coef + elem.width`, using a mixed-radix positional-number
encoding where the radix (`depth_coef = max_width + 1`) is recomputed fresh at every recursion
level from the branching factor observed at that level. This encoding is a valid, monotonic
sort key for lexicographic path ordering *only as long as the multiply-then-add never
overflows* - for a tree with branching factor `b` sustained over `k` levels, the key's magnitude
grows like `O(b^k)`, so it silently exceeds `usize::MAX` (~1.8e19 on 64-bit) after roughly
`log_b(1.8e19)` levels (e.g. ~7 levels at `b=1000`, ~41 levels at `b=3`, ~64 levels for a plain
binary tree). In a debug build (default `overflow-checks = true`), the first overflowing
multiply/add panics with the generic `attempt to multiply with overflow` message, with no
indication in the message of what actually went wrong (recursion too deep/wide) or where the
real, user-actionable fix lives. In a release build (default `overflow-checks = false`), the
computation wraps silently, and the wrapped value is used directly as `Elem`'s primary/secondary
`Ord` sort key (`depth` first, then `width` - see `elem.rs:42-49`), which both `next.rs`'s
`sort_unstable_by_key(|x| x.depth)` and `collect.rs`'s final `result.sort()` rely on to
reconstruct the deterministic breadth-first order documented for the default
`IterationOrder::Ordered` (see `par_recursive`'s own doc comment in
`src/into_parallel/into_par_rec_iter.rs:11-13`). Once the key wraps, elements that should sort
in strict left-to-right order within a level can end up in the wrong relative order, exactly as
observed in approach 2 above (an A-lineage element, whose whole path history should keep it
strictly before any B-lineage element at the same depth, sorts after one instead, at precisely
the two deepest levels where `3^d` exceeds `usize::MAX`).

## Introduced by

cb55cb27 "implement stable next for recursive" (2026-08-27). This commit added
`src/infallible/recursive/execution/elem.rs` from scratch (new file), including
`normalize_depths` with the exact unchecked `elem.depth * depth_coef + elem.width` expression
already present verbatim in the very first version of the file (confirmed via `git log -S
"depth_coef"` and `git show cb55cb27 -- .../elem.rs`, which shows the whole file, including this
line, as newly added, `+` lines, no prior history to inherit the bug from).

## The fix

Replace the unchecked `elem.depth * depth_coef + elem.width` with `checked_mul`/`checked_add`,
panicking with an explicit, actionable message identifying the real cause (recursion too
deep/wide for the encoded key) and the two ways to avoid it (reduce depth/branching, or switch
to `IterationOrder::Arbitrary`, which does not use this encoding). This does not eliminate the
underlying limitation (the single-`usize` mixed-radix encoding fundamentally cannot represent
arbitrarily deep/wide trees) - findings.toml's own suggested fix notes a full solution would
require switching to a per-level path vector with lexicographic comparison instead of one
folded integer, which is a larger, invasive change (touching the `Ord` impl, both call sites'
sort keys, and `Elem`'s memory layout) out of scope for a bounded fix in one session. The
applied fix instead converts the failure mode from "silently wrong output in release, an
unexplained generic panic in debug" into "an explicit, identically-worded, actionable panic in
every build profile" - i.e. fail-fast instead of silent corruption, which is the "at minimum"
half of findings.toml's suggested fix.

Verified against the exact repro from approach 2 (`BRANCH=3, LEVELS=42`): before the patch,
`repro/`'s release build printed `ORDER VIOLATED ... TOTAL VIOLATIONS: 2` (exit 1); after the
patch, the same binary panics with the new message instead (exit 101) - the silent corruption
is gone, replaced by a deterministic, explicit failure. Also checked per CONFIRMATION.md section
7: the non-overflowing negative control (`LEVELS=40`) still gives `ORDER OK` with the patch
applied (no change in behavior for inputs that don't overflow), and a different branching factor
(`BRANCH=5, LEVELS=30`, also overflows) also triggers the new panic correctly, confirming the
fix isn't narrowly tailored to the one exact construction first used to find the bug.

- Project test suite (`cargo test` in the worktree, `/tmp/bughunt_worktrees/005`): **99 passed;
  0 failed; 0 ignored** (`--lib`) and **260 passed; 0 failed; 16 ignored** (full, including
  doctests) both before and after the patch - identical counts, no regressions. (No existing
  test constructs a recursion deep/wide enough to hit this overflow.)

## What's not done / open questions

- The applied fix is fail-fast, not a full correctness fix: a real workload that legitimately
  needs a recursion deep/wide enough to hit this limit will now panic instead of silently
  getting a wrong order. A maintainer may prefer the more invasive per-level-path-vector
  encoding findings.toml describes, which would preserve correct ordering at arbitrary
  depth/width instead of failing - that redesign was judged out of scope for a bounded fix this
  session (it changes `Elem`'s layout, `Ord` impl, and both `collect.rs`/`next.rs` call sites).
- Only the two call chains that already existed at the time of this candidate
  (`collect.rs:65,85` and `next.rs:52,74`) were checked; both go through the same
  `Elem::normalize_depths`, so the single-function fix covers both without needing a second,
  separate patch.
- Not checked: whether `IterationOrder::Arbitrary` (which the fix's panic message suggests as a
  workaround) actually avoids this code path entirely end-to-end for every recursive operation
  (`collect`/`first`/`find`) - this was inferred from the doc comment describing `Arbitrary` as
  not requiring deterministic ordering, but not independently re-verified by tracing every
  `Arbitrary`-mode call site this session.
- No fuzzing/loom campaign was needed or run - this is a deterministic arithmetic bug, fully
  characterized by manual boundary-value construction (approaches 1-2 above).
