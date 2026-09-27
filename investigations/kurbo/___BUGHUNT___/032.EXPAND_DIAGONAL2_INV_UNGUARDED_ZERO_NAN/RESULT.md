# EXPAND_DIAGONAL2_INV_UNGUARDED_ZERO_NAN

`kurbo/src/expand.rs`'s new (PR #575/#580) `expand_path`/`expand_path_signed` public API takes a
`Diagonal2` expansion factor whose doc comment says both components "should be positive" but does
not reject/clamp any other value. `ExpandCtx::do_quad` (line 214) and `ExpandCtx::do_cubic` (line
244) both call `self.expand.inv()` unconditionally; `Diagonal2::inv`'s own doc already admits
"Will of course produce infinities if a component is zero" - this looked like the same
unguarded-NaN-into-public-output bug class as already-confirmed candidates 011/023/024/025, in a
module none of those covered.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (public API only) | `expand_path(path_with_QuadTo, Diagonal2::new(0.0, 5.0), Join::Miter, 4.0, 1e-3)`, release build | Reproduced: `MoveTo((0.0, NaN))` / `CurveTo((NaN, NaN), (NaN, NaN), (10.0, NaN))` in the returned `BezPath` - no panic, no error |
| Repeat-run check (5x, release build) | same input as above | 5/5 identical NaN output - deterministic |
| Random search, `do_quad`-only paths, 1,500,000 trials (`xx=0`, `yy=0`, and `xx=yy=0`) | release build, `u64` xorshift PRNG, coordinates in [-20, 20] | Every non-finite-input config reproduces a non-finite result somewhere in the search space; used to confirm the guard fix (see below) generalizes, not just to the one hand-picked input |
| Random search, `do_cubic`-only single-segment paths, 1,500,000 trials (`xx=0`) | release build | 0 non-finite results found - see "What's not done" below for why `do_cubic`'s call site did not leak NaN in this search despite being equally unguarded |

## Status

**Confirmed**, 5/5 repeat runs (deterministic, no dependency on timing/memory layout/seed) plus a
1.5M-trial random search across other `do_quad` inputs and `Diagonal2` configurations, all release
build, no debug-assertions/sanitizer needed (this is a straight floating-point logic bug, not
memory-unsafety).

**Fully reproducible**: **Yes**. This is a single-participant logic bug (an ordinary library call
from one caller's own code, not a multi-participant protocol), so `CONFIRMATION.md` section 11's
"real second participant" requirement does not apply here; the repro calls only kurbo's own public
API (`expand_path`, `Diagonal2::new`, `Join::Miter`, `BezPath`) end-to-end, exactly as any real
caller would.

## Class

logic

## Root cause

`Diagonal2::inv()` computes `1.0 / self.xx` / `1.0 / self.yy` unconditionally (its own doc:
"Will of course produce infinities if a component is zero"). `ExpandCtx::do_quad` and
`ExpandCtx::do_cubic` both call `self.expand.inv()` with no check that `expand.xx`/`expand.yy` are
nonzero (and finite). For `do_quad`, the resulting `einv` (containing `f64::INFINITY` when a
component is zero) is multiplied into `q0`/`q1` (a coordinate difference); when that coordinate
component is exactly `0.0`, IEEE-754 gives `0.0 * INFINITY = NaN`, which `.normalize()` then
carries into `utan0`/`utan1`. The subsequent guard, `if det.abs() < 1e-10 { self.do_line(p2); return; }`,
is meant to catch degenerate/collinear tangents and fall back to a straight line, but a NaN
`det.abs()` makes `NaN < 1e-10` evaluate to `false` (per IEEE-754, every comparison with NaN except
`!=` is false), so the fallback is skipped and the NaN propagates straight into the emitted
`CurveTo`/`MoveTo` control points - the public function returns a silently NaN-poisoned `BezPath`
with no panic or error signal at all.

`do_cubic` has the identical unguarded `self.expand.inv()` call at line 244, but a 1.5M-trial random
search (single-cubic-segment paths, `xx=0`, coordinates spanning both zero and nonzero x-values)
did not find an input where the resulting NaN/infinity reaches the final output. Reading the code:
`do_cubic`'s own routing check, `if p1xp0 * p2xp1 >= 0.0 { try_one_point(&cx) } else { None }`, has
the same "comparison with NaN is false" property - but here it happens to work in the bug's favor:
when `p1xp0`/`p2xp1` are NaN (as they generally are once `einv` is infinite), `p1xp0 * p2xp1 >= 0.0`
is `false`, which routes to `None` (skipping the also-unguarded `try_one_point`) and then into
`two_point_linear`, whose own determinant check is explicitly NaN-safe
(`det.abs().partial_cmp(&1e-12) != Some(Ordering::Greater)`, which is `true` for NaN) and correctly
returns `None`, falling back to `self.do_line(p3)`. This appears to be incidental self-protection
from the specific arrangement of NaN-unsafe/NaN-safe checks in that call path, not a deliberate
guard against a zero expansion component - see "What's not done" below.

## Introduced by

c907a3a "Add anisotropic expansion methods for paths (#575)" (2026-05-07) - both unguarded
`self.expand.inv()` call sites (`do_quad` line 214, `do_cubic` line 244) are present verbatim in
this commit's original version of `expand.rs`; the one later commit touching this file (6836244,
"Close subpaths when computing area for expansion (#580)") only changed `expand_path`'s own area
computation, not `ExpandCtx`.

## The fix

Added an explicit finite-check guard right after each `self.expand.inv()` call, in both `do_quad`
and `do_cubic`: if either resulting `einv` component is non-finite (which is exactly the symptom of
a zero, or otherwise degenerate, `expand.xx`/`expand.yy`), fall back to `self.do_line(...)` the same
way the existing degenerate/collinear-tangent checks already do, instead of continuing into
computations that produce NaN.

```rust
let einv = self.expand.inv();
if !einv.xx.is_finite() || !einv.yy.is_finite() {
    // A zero (or non-finite) expansion component makes `inv()` produce an
    // infinity, which poisons all downstream computation with NaN. Fall back
    // to a straight line rather than emitting NaN control points.
    self.do_line(p2); // (p3 in do_cubic)
    return;
}
```

Verified:
- `repro/` re-run against the patched tree: no longer reproduces - `expand_path` now returns
  `MoveTo((0.0, 5.0)) / LineTo((10.0, 5.0)) / LineTo((10.0, -5.0)) / LineTo((0.0, -5.0)) /
  LineTo((0.0, 5.0)) / ClosePath`, all finite, for the exact input that previously produced NaN.
  5/5 repeat runs.
- Same 1.5M-trial random search re-run against the patched tree (`do_quad`, all three of
  `xx=0`/`yy=0`/`xx=yy=0`): 0 non-finite results out of 1,500,000 trials (previously this same
  search reliably found non-finite results before the fix - not re-quantified as a raw count since
  the point was only to confirm the fix generalizes beyond the single hand-picked input, which it
  does).
- Project test suite (`cargo test --release` in `kurbo/`, this candidate's own worktree):
  **185 passed, 2 failed both before and after the patch** - `bezpath::tests::test_elements_to_segments_starts_on_closepath`
  and `bezpath::tests::test_must_not_start_on_quad` fail identically on the pristine, unmodified
  `ca27349` checkout (verified separately against the main project's own working tree, no relation
  to `expand.rs`/this patch). All 9 of `expand::tests::*` pass both before and after.

## What's not done / open questions

- `do_cubic`'s own `self.expand.inv()` call (line 244) is equally unguarded in principle - the fix
  above guards it too, defensively, even though the 1.5M-trial search never found a live input that
  leaks NaN through that specific path in the *unpatched* code (see "Root cause" above for why the
  existing NaN-unsafe/NaN-safe check ordering happens to self-protect there). A more targeted
  fuzz/property search (e.g. via `fuzz-bug-repro`, run separately) covering multi-segment paths
  (where `do_cubic` isn't necessarily the first segment, so `self.last_pt`/`last_n` come from a
  prior possibly-NaN-tainted segment) was not attempted this session and could still turn up a
  live path through `do_cubic` that the random search here didn't hit.
- Only `Join::Miter` was exercised in the repro; `Join::Round`'s branch in `do_join` (used when the
  join's cross/dot computation involves `self.expand`/`einv`-derived quantities) was not
  specifically probed for a zero-component interaction, though `do_join` itself is only reached
  after a segment has already produced a normal vector `n`, which for a zero-component `expand`
  would itself already be `0`/NaN by the time it gets there - not separately confirmed.
- No change was made to `Diagonal2::inv()` itself, nor to `expand_path`/`expand_path_signed`'s
  public doc comment (still just says factors "should be positive", not enforced) - the bounded fix
  here only stops the two internal call sites from propagating the resulting NaN into the emitted
  path; a maintainer might instead prefer to reject/clamp non-positive `Diagonal2` values at the
  public entry point, which is a larger API-design decision left to them.
