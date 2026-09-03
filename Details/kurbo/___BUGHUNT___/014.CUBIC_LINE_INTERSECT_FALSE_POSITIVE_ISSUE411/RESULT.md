# CUBIC_LINE_INTERSECT_FALSE_POSITIVE_ISSUE411

`PathSeg::intersect_line`'s cubic/line case (`kurbo/src/bezpath.rs`) can report a false-positive
intersection for a cubic and line segment that clearly don't cross. Sourced from upstream issue
#411 ("Funniness with cubic/line intersection (when negative values present?)"), open, with a
concrete test case and exact coordinates already provided by the reporter; the issue's own
comment thread (from a project maintainer) already narrows the cause to `solve_cubic`'s numerical
robustness, not literally a sign-handling bug as the reporter's own title guessed - this candidate
independently reproduces and confirms that analysis rather than re-deriving it from scratch.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reproduction: the exact `CubicBez`/`Line` coordinates from issue #411, run through the real public `PathSeg::intersect_line` entry point via a `cargo run --example` in the crate | debug build, single run | Reproduced immediately: `intersect_line` returns one `LineIntersection` (`segment_t ≈ 0.2045`) for a cubic and line that do not cross |
| Repeat-run confirmation | debug build, 3 consecutive runs, no RNG/threading involved (pure deterministic floating-point arithmetic) | Identical output all 3 runs - bit-for-bit repeatable |
| Root-cause read of `solve_cubic` in `kurbo/src/common.rs`, cross-checked against the already-open, unmerged PR #593 ("Improve solve_cubic accuracy when the cubic coefficient is negligible"), which independently diagnoses the exact same `solve_cubic` weakness (for a *different* entry point, `Shape::winding`) and explicitly lists #411 as "possibly related" | code reading, no tool | Confirmed: PR #593's fix (a magnitude-relative threshold before the `1/c3` scaling) also fixes this candidate's repro when applied - see "The fix" below |

Given the maintainer's own comment thread on #411 already pinpointed the mechanism (a spurious
huge third root from scaling by `1/c3` when `c3` is floating-point noise near zero, corrupting
the other two roots' precision) and an independently-submitted, unmerged PR already proposed and
tested a fix for the same underlying function, no further escalation (miri/ASAN/fuzzing) was
needed or would add signal here - this is a pure floating-point numerical-robustness bug in
deterministic code, not memory-safety/UB.

## Status

**Confirmed** - reproduces 3/3 runs, deterministic (no flakiness possible: pure floating-point
computation on fixed inputs, no RNG, no threading, no environment dependence).

**Fully reproducible**: **Yes**. `repro/` calls the real, public `PathSeg::intersect_line` entry
point directly with the exact `CubicBez`/`Line` values from the upstream issue - this is not a
finding about how one participant reacts to another's wire input (no untrusted-peer/parser angle
here), so `CONFIRMATION.md` section 11's second-participant requirement does not apply; the "real
end-to-end trigger path" is simply calling the documented public API with valid, finite,
in-range `f64` coordinates, exactly as any caller (e.g. a boolean-path-operations or
hit-testing library built on kurbo) would.

## Class

logic (silently wrong result - a false-positive intersection reported for geometry that does not
actually intersect; no panic, no UB).

## Root cause

`PathSeg::intersect_line`'s cubic branch (`kurbo/src/bezpath.rs`, the `PathSeg::Cubic(c) => { ... }`
arm) builds cubic coefficients `c0..c3` for the signed-distance-from-line polynomial and calls
`common::solve_cubic(c0, c1, c2, c3)` to find where the curve crosses the probe line.

`solve_cubic` (`kurbo/src/common.rs`) only falls back to `solve_quadratic` when `c3.recip()`
makes the scaled coefficients non-finite (i.e. `c3` is *exactly* zero or so close to it that
`1/c3` overflows). For the issue #411 test case, the cubic coefficient `c3` (`dy*px3 - dx*py3`,
mathematically driven near zero for this particular cubic/line pair but not exactly zero due to
floating-point rounding in the input coordinates) is small but not small enough to overflow
`1/c3` - so the general cubic-solving branch runs, scaling `c0`, `c1`, `c2` by `1/c3`. This
amplifies their rounding error enormously: one of the three returned roots is an enormous,
physically meaningless value (~8e15, per the issue's own maintainer analysis), and this
amplification corrupts the precision of the other two roots enough that one of them
(`segment_t ≈ 0.2045`, within `intersect_line`'s accepted `[0,1]` range) is reported as a root of
the original (unscaled) equation even though it isn't actually close to being one - it's a
numerical artifact of the near-singular scaling, not a real curve/line crossing.

This is the same underlying weakness in `solve_cubic` as the one described by the open, unmerged
PR #593 (a *different* trigger: `Shape::winding`/`contains` misclassifying points for a rotated
circle built from symmetric cubics, where the cubic term cancels mathematically but not in
floating point) - PR #593's own description explicitly lists #411 as "possibly related", which
this candidate confirms directly: the same fix resolves both.

## Introduced by

The vulnerable pattern in `solve_cubic` (scaling by `c3.recip()` with only an `is_finite()` guard,
no magnitude-relative check against the other coefficients) has been present since the crate's
very first commit: 92feb79 "Start curves crate" (this repo's history begins with `solve_cubic`
already implemented this way; `git log -S"pub fn solve_cubic"` finds no earlier revision).

The specific entry point that makes this reachable via `PathSeg::intersect_line`'s cubic case was
introduced by e5a1c11 "Path segment intersection against line" (`git log -S"fn intersect_line"`),
which added the `intersect_line` method (including its `PathSeg::Cubic` arm calling
`solve_cubic`) to the crate.

## The fix

Applied the same fix proposed and already tested (for a different call site) by the open,
unmerged PR #593: before the `1/c3`-scaling branch in `solve_cubic`, check whether `c3` is
negligible *relative to the other coefficients' magnitudes* (not just whether `1/c3` itself
overflows), and if so, fall back to `solve_quadratic` directly:

```rust
if c3.abs() <= 1e-12 * c0.abs().max(c1.abs()).max(c2.abs()) {
    return solve_quadratic(c0, c1, c2).iter().copied().collect();
}
```

This is sufficient for the #411 repro: `intersect_line` on the exact issue coordinates now
returns an empty `ArrayVec` (no false-positive hit) instead of the spurious `segment_t ≈ 0.2045`
result. See `patch.diff` (against `kurbo/src/common.rs` only) and `output.txt` for the actual
before/after `repro/` run.

- Project test suite: 252 passed, 0 failed, 1 ignored (187 kurbo unit tests + 10 polycool unit
  tests + 55 kurbo doctests + 1 ignored polycool doctest) both **before** and **after** the fix
  (`cargo test --workspace`) - no regressions, no new failures. The fix only changes behavior for
  the near-degenerate-cubic-coefficient case, which no pre-existing test in the suite exercised.

## What's not done / open questions

- This fix only guards `solve_cubic`'s own entry point. `PathSeg::intersect_line`'s `PathSeg::Quad`
  arm uses `solve_quadratic` (already has its own near-linear guard) so is not affected by this
  class of bug. Other `solve_cubic` call sites in the crate (e.g. `winding_inner`/`Shape::winding`,
  `CubicBez::extrema`-adjacent code, curve-fitting) inherit the same fix automatically since it's
  applied inside `solve_cubic` itself, not at the `intersect_line` call site - this was a deliberate
  choice (matching PR #593's own scope) so the fix isn't papering over just the one call site that
  happened to be reported.
- Did not independently re-verify PR #593's own reported repro (the rotated-circle
  `Shape::winding` case) in this session - relied on reading its description and diff; the fix
  applied here is textually identical to PR #593's proposed change, so it should carry the same
  effect, but this candidate's own confirmed reproduction is scoped to issue #411's
  `intersect_line` case only.
- No attempt was made to construct a more general/adversarial fuzz search for other near-degenerate
  `solve_cubic` inputs beyond the two already-known cases (#411, PR #593's circle case) - the
  `1e-12` relative threshold is PR #593's own choice, carried over as-is rather than independently
  re-derived or stress-tested against a wider input space this session.
