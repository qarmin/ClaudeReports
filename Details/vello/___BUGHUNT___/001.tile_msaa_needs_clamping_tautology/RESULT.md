# tile_msaa_needs_clamping_tautology

`sparse_strips/vello_common/src/tile.rs`, function `Tiles::make_tiles_msaa` (line ~964). The
candidate description flagged that `needs_clamping` (line 1096, `min_x < line_left_x || max_x >=
line_right_x`) is computed from values (`min_x`/`max_x`) that are, by construction, always exactly
equal to `line_left_x`/`line_right_x` (both pairs derive from the same `p0_x.min(p1_x)`/
`p0_x.max(p1_x)` comparison a few lines earlier) - making the check unconditionally `true` for any
non-NaN `f32` input, so the "unclamped fast path" it's supposed to gate (`run_loops!(false)`,
documented at line 1167 as "Safe because we checked bounds earlier") never actually executes for
ordinary input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static/mathematical proof | Read the exact formula; `min_x`/`max_x` and `line_left_x`/`line_right_x` are the same values under the same comparison, so `needs_clamping` reduces to `(p0_x < p0_x) \|\| (p1_x >= p1_x)` = `false \|\| true` = always `true` for finite `p0_x`,`p1_x`. | Tautology proven algebraically for any non-NaN pair. |
| Property check (repro/, 2,000,000 random finite `(p0_x, p1_x)` pairs in `[-20000, 20000]`, deterministic splitmix64 seed, debug build with overflow-checks) | Reimplements the exact source formula standalone | 0 counterexamples across 2 separate runs - confirms the tautology holds empirically, not just for hand-picked values. |
| Real public-API call with a `NaN` line coordinate (the one input class that *does* make comparisons in the formula evaluate to `false`, since any comparison against NaN is `false`) - `Tiles::make_tiles_msaa` on pristine code, `Line{ p0: (NaN, 5.0), p1: (50.0, 90.0) }`, 100x100 viewport | `cargo run --release`, debug-assertions/overflow-checks on, 2 repeat runs | Reproducible 2/2: `needs_clamping` does become `false` (as predicted), the dead "unclamped" branch runs, and every emitted tile collapses to a spurious `x=0` column instead of spanning tile columns 0..12 as the line's finite endpoint implies. No crash on pristine code (Rust's `NaN as u16` saturates to `0`, which happens to stay in-bounds here). |
| Fault injection (source modified only inside the isolated worktree, reverted before finishing): forced `needs_clamping = false` unconditionally, ran `cargo test -p vello_common --lib tile::` | debug build | 10/60 tests failed, including a debug-assertion **`attempt to add with overflow`** panic in the exact pre-existing regression test for issue #1321 (`tile::tests::overflow`), plus 9 tests where the MSAA output diverges from the `make_tiles_analytic_aa` reference oracle used by `assert_tiles_match`. Confirms the "unclamped" branch is genuinely broken, not merely a redundant optimization, if it were ever reached for realistic geometry. |
| Fault injection, second pass: fixed only `needs_clamping` to compare against the real tile-grid bounds (`line_left_x < 0.0 \|\| line_right_x >= tile_columns as f32`, matching the check's own doc comment) plus added the missing `tile_columns` clamp to the unclamped branch's `x_end` | debug build, `cargo test -p vello_common --lib tile::` | 5/60 tests still failed - all horizontal (non-tile-aligned) lines. Root-caused to a **second, independent** defect: horizontal lines make `x_slope = dx/dy = dx/0.0 = ±infinity`, and `row_top_x = p0_x + (row_top_y - p0_y) * x_slope` computes `0.0 * infinity = NaN` whenever `row_top_y == p0_y` exactly. The always-executed "clamped" branch's `f32::min(...).max(line_left_x)`/`f32::max(...).min(line_right_x)` happens to discard that NaN in favor of `line_left_x`/`line_right_x`; the "unclamped" branch has no such rescue and collapses to a spurious `x=0` tile - the same failure mode the NaN-input check above showed. |

## Status

**Confirmed** - as a dead/tautological conditional whose guarded branch is independently broken in
two distinct ways. Reproduces deterministically:

- The tautology itself: 2/2 identical runs of a 2,000,000-sample property check, 0 counterexamples
  (a pure arithmetic fact about `f32` comparisons, not flaky in any way).
- The NaN-reachable manifestation (spurious `x=0` phantom tile column) via the real public API:
  2/2 identical runs, deterministic (no fixed seed needed - `f32::NAN` is exact).
- The two latent defects in the dead branch (missing viewport-bounds clamp; missing NaN-rescue for
  horizontal lines), demonstrated deterministically via fault injection (not part of the shipped
  `repro/`, since it required modifying private/internal source rather than the public API - see
  `output.txt`'s "Supplementary experiment" section).

**Important scope note**: on the real, unmodified codebase, this does **not** currently produce
wrong rendering output for any finite (non-NaN) input - `needs_clamping` being unconditionally true
means production code always takes the safe "clamped" path. The only way to reach the dead branch
on pristine code is a `NaN` line coordinate, which (a) is unusual but not impossible input (a
degenerate 0/0 upstream transform could plausibly produce one - see the sibling `extract_scales`
backlog entry in `IDEAS.md`), and (b) does not crash even then, because `NaN as u16` saturates to
`0` in Rust rather than producing an arbitrary/undefined value. So this is a real, verified defect
in the source (dead code that doesn't do what its own comment says, masking two further bugs in the
branch it dead-gates) rather than a bug with observable impact on any currently-shipped rendering
output.

**Fully reproducible**: **Yes** - both the tautology check and the NaN-driven public-API
manifestation are 100% deterministic (no seed/layout/timing dependency), run through
`vello_common`'s real public API (`Tiles::new`/`Tiles::make_tiles_msaa`/`Line`/`Point`), single
participant, no faked second party (this is a single caller handing a genuine external
value - a NaN float - to a parser-like entry point, matching CONFIRMATION.md section 11's
"malformed input to a public `open_file`-style API" exception, not the case it warns about).

## Class

logic (dead/tautological branch condition; the branch it dead-gates independently has a missing
bounds clamp and a missing NaN-rescue clamp, neither of which is currently reachable through any
finite/non-NaN input in the shipped code).

## Root cause

`needs_clamping = min_x < line_left_x || max_x >= line_right_x` where, a few lines earlier,
`(line_left_x, line_right_x) = if p0_x < p1_x { (p0_x, p1_x) } else { (p1_x, p0_x) }` and
`min_x = p0_x.min(p1_x)`, `max_x = p0_x.max(p1_x)`. For any non-NaN `f32` pair these two
computations produce identical values by construction (both are just "the smaller of `p0_x`,
`p1_x`" and "the larger of `p0_x`, `p1_x`"), so the check reduces to `(x < x) || (y >= y)` =
`false || true` = always `true`. The doc comment above it ("Check if the line is fully within the
horizontal viewport bounds") describes an intent - comparing against the tile grid/viewport bounds
(`0`/`tile_columns`) - that the code never actually implements; it compares the line's own bounds
against themselves instead. `git log -S` shows this exact shape was introduced whole when the MSAA
tiling path was split out into its own function (see "Introduced by"), i.e. it was never a
correctly-working optimization that later regressed - the fast path this was meant to enable
appears to have been dead on arrival.

## Introduced by

4ea47b47 "Separate make_tiles into AA and MSAA versions. Fully cases out MSAA version (#1319)" (2025-12-11)

## The fix

Removed the tautological check and the dead "unclamped" branch entirely, so `make_tiles_msaa`
always takes what used to be the (currently-always-executed, behaviorally unchanged) "clamped"
path - i.e. deleted `needs_clamping`/`min_x`/`max_x`, collapsed `process_row!`'s `if $clamped {..}
else {..}` to just the clamped body, and dropped the now-unused `$clamped` parameter from
`process_row!`/`run_loops!` (inlining `run_loops!`'s single call site, since it no longer needs to
be invoked twice with different arguments).

This is deliberately **not** an attempt to repair the intended fast-path optimization (compare
`needs_clamping` against the real viewport bounds, as its own comment describes) - the second fault
injection experiment (see `output.txt`) shows that doing only that is not sufficient: it still
leaves a second, independent defect (the horizontal-line NaN-rescue issue) live and reachable for
entirely ordinary, non-adversarial in-viewport horizontal lines, as shown by 5 pre-existing tests
failing. Properly restoring the optimization would need the horizontal-line case fixed too (e.g.
special-casing `dy == 0` before computing `x_slope`), which is a larger, riskier change than
warranted for removing a currently-dead, currently-harmless-in-production code path. Deleting the
dead branch is the minimal change that removes the risk of either latent defect ever becoming live
via a future "obvious cleanup" of the tautology, while being behaviorally identical to today's
shipped output for every input (verified below).

Verification against `repro/` (see `output.txt` for full captured output):

- Before (pristine): "Check 2" (NaN via real API) prints `REPRODUCED` and the process exits 1.
  Reproduced 2/2 runs.
- After (`patch.diff` applied): "Check 2" prints "did not reproduce" (empty tile list - the line
  is now silently dropped for every row rather than emitting a phantom `x=0` column; still not
  necessarily "correct" for an already-undefined NaN input, but the specific wrong-geometry
  artifact is gone) and the process exits 0. Reproduced 2/2 runs (i.e. the fix holds 2/2).
- "Check 1" (the tautology property check) reports the same result before and after by design - it
  reimplements the formula standalone rather than calling into the tree, since the formula itself
  is an arithmetic fact unaffected by deleting the dead code that used it; it's included as
  supporting evidence for the root cause, not as a before/after differential.

Project test suite (mandatory per SKILL.md step 7):

- `cargo test -p vello_common --lib`: **159 passed, 0 failed, 1 ignored**, both before and after
  the patch (identical).
- Full workspace (`cargo test --workspace --exclude vello_sparse_tests`, with
  `VELLO_SKIP_LFS_SNAPSHOTS=all` to skip unrelated snapshot-PNG tests that fail in this sandbox
  purely because the `git-lfs` binary isn't installed - confirmed unrelated to this candidate by
  checking the same 6 tests fail identically on an untouched checkout): **504 passed, 0 failed**,
  both before and after the patch (identical counts, no new failures, no tests newly passing).

The fix was also checked against more than the exact triggering NaN value (CONFIRMATION.md section
7): the fault-injection experiments in `output.txt` additionally covered the missing-bounds-clamp
class (issue #1321's `overflow` regression test, and lines extending past the viewport on the
right edge in `sloped_line_crossing_right`/`clip_bottom_right_corner`) and the horizontal-line NaN
class (`horizontal_line_left_to_right_three_tile`, `horizontal_line_multi_tile`,
`horizontal_line_right_to_left_three_tile`, `resize_works_correctly`, `same_tile_right`) - the
shipped fix (deleting the dead branch outright, rather than patching either sub-case in isolation)
is verified safe against both classes simultaneously, since it never reaches either code path that
had the defect.

## What's not done / open questions

- The "unclamped fast path" optimization the original author intended is not restored - only its
  (unreachable, doubly-broken) attempt is removed. If the performance win was real and worth
  pursuing, a future change would need to special-case horizontal lines (`dy == 0`, avoiding the
  `dx/0.0` -> `+-infinity` -> `0*infinity` -> NaN chain) before it could safely skip the
  `line_left_x`/`line_right_x` clamp - not attempted here, out of scope for a bounded bug-hunt fix.
- Not checked: whether `make_tiles_analytic_aa` (the sibling AA-mode tiling function in the same
  file) has an analogous "looks like an optimization, is actually tautological/dead" pattern
  anywhere in its own code - it uses a different algorithm (per-tile coarse binning, not per-row
  min/max clamping) so the exact same shape doesn't obviously apply, but it wasn't specifically
  audited for this class of issue.
- Not checked: whether any other line coordinate could reach the dead branch besides `NaN`
  (`+-infinity` was checked analytically and does not - IEEE-754 total ordering with infinities
  behaves like normal finite comparisons for this formula's purposes - but this wasn't fuzzed
  exhaustively across every f32 bit pattern, only 2,000,000 random finite samples plus the NaN
  case by construction).
- Whether real upstream code (flattening, transforms) can actually produce a `NaN` line coordinate
  in practice wasn't investigated in this session - the sibling `IDEAS.md` backlog entry for
  `sparse_strips/vello_common/src/util.rs:141` (`extract_scales`) suggests at least one plausible
  source of NaN propagation elsewhere in the same crate, worth checking together with this finding
  in a future session.
