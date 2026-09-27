# mindist-min_dist_param-unbounded-recursion-zero-accuracy

`kurbo/src/mindist.rs`'s `min_dist_param` (kurbo/src/mindist.rs:29-164) recursively subdivides the
`u`/`v` parameter domain to find the minimum distance between two Bezier curves, with the caller's
`epsilon` (`accuracy`) as the *only* termination check (`(umax - umin).abs() < epsilon || ... `) and
no recursion-depth counter anywhere. `epsilon` is the fully public `PathSeg::min_dist(&self, other:
PathSeg, accuracy: f64) -> MinDistance` (kurbo/src/bezpath.rs:1307)'s `accuracy` parameter, passed
straight through unclamped. This came from a named candidate (batch-assigned suspect, distinct from
already-investigated candidate 006's u8-overflow finding) flagging that `accuracy = 0.0` should make
the width check unsatisfiable forever.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct execution against the real public API, release build, in an isolated worktree | `PathSeg::Line(...).min_dist(PathSeg::Line(...), 0.0)` on two plain, non-degenerate parallel `Line`s | Reproduced immediately - process aborts with "has overflowed its stack" |
| Repeat runs, same build/input | 4 runs total (1 initial + 3 explicit repeats) | 4/4 stack overflow, deterministic |
| Same input, debug build (`cargo build` default `dev` profile) | `accuracy = 0.0` | Also overflows the stack identically - no earlier panic/assert intervenes in either profile |
| Manual boundary-value probing of a first fix attempt (`epsilon.max(1e-9)`, no depth cap) across a range of small positive `epsilon` on a slightly-skewed (non-parallel) `Line` pair | `eps` from 0.1 down to 1e-6, timing each call | Confirmed the underlying subdivision search does very little effective pruning for this near-degenerate geometry - runtime scales close to O(1/epsilon), not O(log(1/epsilon)) (0.1: 184us, 1e-6: 14.3s); re-running the exact original `accuracy = 0.0` repro with only that floor (no depth cap) still did not return within 180s (timed out, did not crash) - see `output.txt` |

Direct execution on the very first attempt already gave a deterministic, repeatable crash, so no
further escalation (miri/ASAN/fuzzing) was needed for the crash finding itself; the second table row
above is boundary-value probing done specifically to validate that the fix actually fixes the crash
(not just relocates it into an effectively-infinite hang), per the fix-verification discipline in
`SKILL.md`'s "Critical pitfalls" #3.

## Status

**Confirmed**. Reproduces deterministically 4/4 runs, release and debug builds, default
environment, same input each time (see `output.txt`).

**Fully reproducible**: **Yes**. The repro calls kurbo's real, unmodified public
`PathSeg::min_dist` with two ordinary `Line` segments and `accuracy = 0.0` - the complete external
input is a single `f64` supplied directly by the immediate caller of a public API, not something
produced by an upstream peer/protocol, so there is no second participant to fake here
(`CONFIRMATION.md` section 11's "single external artifact, no other side" case applies directly).

## Class

crash (stack overflow / process abort, both debug and release builds)

## Root cause

`min_dist_param` recursively subdivides `(umin, umax) x (vmin, vmax)` into four quadrants
(mindist.rs:150-187) until `(umax - umin).abs() < epsilon || (vmax - vmin).abs() < epsilon` becomes
true (mindist.rs:57), or one of two pruning shortcuts fires first (`is_outside`, or a boundary-alignment
check). For `epsilon == 0.0`, `x.abs() < 0.0` is false for every finite `x` (including `x == 0.0`
itself, since `abs()` is never negative) - so this exit condition can *never* become true on its
own, no matter how many times the interval is halved. `PathSeg::min_dist` (kurbo/src/bezpath.rs:1307)
passes its `accuracy` parameter straight through as this `epsilon` with no validation, clamping, or
special-casing of non-positive values anywhere in the call chain. With no independent
recursion-depth counter to fall back on, and the two pruning shortcuts not firing for these two
input `Line`s (both consistently `is_outside == false` and none of the four boundary-alignment
conditions holding at any level visited), recursion proceeds until the call stack is exhausted and
the process aborts.

A related but distinct discovery made while fixing this: merely flooring `epsilon` to a small
positive constant (no depth cap) converts the crash into an effectively-unbounded *hang* instead of
a fix, for the same reason the crash existed in the first place - the pruning shortcuts
(`is_outside`/boundary checks) can fail to rule out any of the 4 children at many/most levels for
near-parallel or near-symmetric input curves, so total work scales close to O(1/epsilon) rather than
the O(log(1/epsilon)) a well-pruned binary search would give (see `output.txt`'s probe data). This
is why the fix below adds a hard, epsilon-independent depth backstop rather than only bounding
`epsilon`.

## Introduced by

81fbd0691382837b49c4cf26b48bd6dea78b45e4 "Compute the minimum distance between two pathsegs."
(2021-08-25) - `git show 81fbd0691382837b49c4cf26b48bd6dea78b45e4:src/mindist.rs` shows the
identical `(umax - umin).abs() < epsilon || (vmax - vmin).abs() < epsilon` termination check with no
depth counter, and `git show 81fbd0691382837b49c4cf26b48bd6dea78b45e4:src/bezpath.rs` shows
`PathSeg::min_dist`'s `accuracy` already being passed straight through unclamped, both present since
this is the commit that first introduced `mindist.rs` and its `PathSeg::min_dist` entry point
(the same commit already implicated, for a different, independent bug in the same file, in
candidate 019's "Introduced by").

## The fix

Applied directly to `kurbo/src/mindist.rs` in a scratch worktree at
ca273499e3e48bd2de6f02aa8e99a148984e45f3 (see `patch.diff`):

- Split `min_dist_param` into a thin public wrapper (unchanged signature, so `bezpath.rs`'s call
  site needs no changes) that calls a new private `min_dist_param_rec`, adding one extra
  parameter: `depth: u32`, starting at `0` and incremented by 1 on every recursive call.
- The termination check gains a third disjunct: `... || depth >= MAX_DEPTH`, with
  `const MAX_DEPTH: u32 = 12`. Once reached, subdivision stops immediately, exactly like the
  `epsilon` check succeeding - independent of what `epsilon` actually is (including `0.0`,
  negative, or NaN).
- `MAX_DEPTH = 12` was chosen empirically from the boundary-value probing above: reasonable/test-
  suite accuracies (1e-3 to 0.5 in kurbo's own existing `mindist` tests) all terminate via the
  normal `epsilon`-width check at a depth well under 10, so `MAX_DEPTH` is never the limiting
  factor for any sane caller; a worst-case fully-unpruned tree at depth 12 is `4^12 = 16,777,216`
  nodes, which the probing above shows completes in well under 100ms even for the specific
  near-degenerate geometry that made the original bug and the epsilon-only fix attempt both run
  unacceptably long.

Verified against `repro/`, before and after, release build:

- Before: `PathSeg::Line(...).min_dist(PathSeg::Line(...), 0.0)` aborts with "has overflowed its
  stack" (4/4 runs, both build profiles).
- After: same call returns normally (`distance = 5.0`, sane `t1`/`t2`) in well under 100ms, 100% of
  runs.
- Varied beyond the exact triggering input (`CONFIRMATION.md` section 7): re-ran with a
  non-parallel/skewed `Line` pair at `accuracy = 0.0` (also crashed before, returns
  `distance ~= 4.789131500095557` after) and with a full sweep of small positive `epsilon`
  (0.1 down to 1e-6) on that same skewed pair - all now return in well under 100ms, versus up to
  14.3s before the fix (see `output.txt`).
- Project test suite (`cargo test --release -p kurbo`): **before** patch: 185 passed, 2 failed
  (`bezpath::tests::test_elements_to_segments_starts_on_closepath`,
  `bezpath::tests::test_must_not_start_on_quad` - both pre-existing, unrelated to `mindist.rs`,
  identical before and after). **After** patch: 185 passed, 2 failed - same two pre-existing
  failures, no new failures, no fewer passes. The `mindist` module's own 6 tests
  (`test_choose`, `test_d_rk`, `test_overflow`, `test_out_of_order`, `test_line_curve`,
  `test_mindist`) all pass after the patch.
- `cargo clippy -p kurbo --lib`: no new warnings introduced by the patch (one pre-existing,
  unrelated warning in `common.rs` present both before and after).

## What's not done / open questions

- `MAX_DEPTH = 12` is a pragmatic empirical choice, not a value derived from a formal worst-case
  analysis of the algorithm's pruning behavior - a maintainer with access to the original Chen et
  al. paper's own convergence guarantees might derive a tighter or better-justified bound, or
  decide a configurable/documented cap is preferable to a hardcoded one.
- This patch does not attempt to fix the underlying O(1/epsilon)-ish pruning weakness itself (why
  `is_outside`/the boundary checks fail to rule out children for near-parallel/near-symmetric
  curves) - only bounds its worst case. A caller passing a very small (but valid, positive)
  `epsilon` on an adversarial curve pair will now get a fast-but-coarser answer (silently capped at
  `MAX_DEPTH`'s effective resolution) rather than the slow-but-more-precise answer they might have
  gotten before (had they been willing to wait); this is a silent precision/behavior change for
  that specific caller population, not just an unreachable-crash fix, and is worth a maintainer's
  own judgment call on whether `MAX_DEPTH` should be raised, made configurable, or documented in
  `min_dist_param`'s/`min_dist`'s own doc comments.
- Additionally checked (not just theorized): `accuracy = -1.0` reproduces the identical stack
  overflow pre-fix (confirmed by direct execution) and returns sanely post-fix; `accuracy` of
  `NaN`, `-0.0`, and `f64::NEG_INFINITY` all return sane, finite `distance` values post-fix as
  well (all four via the real public `PathSeg::min_dist`, release build) - consistent with the fix
  covering the whole class (it no longer depends on `epsilon`'s value at all once `MAX_DEPTH` is
  hit), not just the exact `accuracy = 0.0` input the candidate named.
