# TRIANGLE_CIRCUMSCRIBED_INSCRIBED_CIRCLE_DEGENERATE_DIV_BY_ZERO

`Triangle::inscribed_circle` (kurbo/src/triangle.rs, lines 100-117) divides by
`perimeter_recip = 1. / (ab + bc + ac)` and `Triangle::circumscribed_circle` (lines 119-136)
divides by `d_recip = 0.5 / b.cross(c)`, both with no guard for a degenerate triangle (collinear
or coincident vertices), silently producing NaN/Inf `Circle` fields instead of erroring/panicking
or being documented as doing so - unlike e.g. `Affine::inverse`, which explicitly documents
"Produces NaN values when the determinant is zero."

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, direct public-API call | `cargo run` (debug) and `cargo run --release`, three `Triangle::new(...)` inputs: three distinct collinear points, three coincident points, two coincident + one distinct point | Reproduced: `circumscribed_circle()` returns NaN/Inf fields for all three degenerate inputs; `inscribed_circle()` returns NaN fields when the perimeter itself is exactly zero (fully coincident points) |
| Repeatability check | Same 3 inputs, 4x `cargo run --release` + 1x `cargo run` (debug) in a row, same process/build each time | Identical output every time (5/5) - deterministic pure floating-point arithmetic, no layout/seed dependency, no debug-vs-release difference (no `debug_assert!` exists in either function to catch it in debug either) |
| `cargo test --release` on the workspace before vs. after the fix | Full `cargo test --release` at `kurbo/` | 185 passed / 2 failed both before and after (the 2 failures are pre-existing, in `bezpath.rs` tests, unrelated to `triangle.rs` - identical failure set before and after, confirming the change didn't affect them) |

No crash/panic/UB tool (miri, ASAN, debug-assertions) was needed beyond a debug build check above,
since this is pure `f64` arithmetic with no `unsafe` and no existing assertion to trip - the bug
class here is "silently produces NaN/Inf", which a fuzz harness would find no faster than the
direct boundary-value construction that already found it deterministically on the first try.

## Status

**Confirmed**, reproduces 5/5 runs across both debug and release builds - deterministic (no
seed/layout dependency, pure floating-point arithmetic).

**Fully reproducible**: Yes. `repro/` calls `Triangle::new` (kurbo's public constructor) directly
with a degenerate point set and calls the public `inscribed_circle()`/`circumscribed_circle()`
methods on the result - this is the genuine, single external artifact (a degenerate point set is
itself the "adversarial input", no second participant is being impersonated), so
`CONFIRMATION.md` section 11's single-process-repro allowance applies directly; no exception
needed.

## Class

logic (silently wrong/non-finite output, no crash, no panic)

## Root cause

`inscribed_circle`'s `perimeter_recip = 1. / (ab + bc + ac)` is `1./0.` (i.e. `inf`) when all
three vertices coincide (`ab = bc = ac = 0`), and the incenter/radius computed from it become
`NaN` (since `self.area()` is also `0.0` there, giving `0.0 * inf = NaN`, and `0*inf` for each
coordinate term). For collinear-but-distinct points the perimeter itself stays nonzero/finite, so
`inscribed_circle` alone actually stays finite there (it returns a degenerate zero-radius circle
correctly) - but `circumscribed_circle`'s `d_recip = 0.5 / b.cross(c)` divides by the cross
product of `(self.b - self.a)` and `(self.c - self.a)`, which is exactly `0.0` whenever the three
points are collinear (cross product of two parallel vectors) or when any two points coincide
(one of the two subtracted vectors becomes the zero vector). That `1/0.0` division yields
`Inf`, which then propagates through the `x`/`y`/`r` computations into `Inf`/`NaN` `Circle`
fields with no panic and no documented contract for the caller.

## Introduced by

5a4b2d9 "add triangle shape (#350)" - the commit that first introduced both
`inscribed_circle`/`circumscribed_circle` with this same unguarded-division shape. Two later
commits (da9bc17 "Correctly calculate incenter in `Triangle::inscribed_circle` (#387)" and
dc50b30 "Reduce number of operations in `Triangle::circumscribed_circle` (#390)") rewrote the
arithmetic for correctness/efficiency but neither added a degenerate-triangle guard or
documentation; the missing-documentation gap (the specific defect this candidate targets) has
been present continuously since 5a4b2d9.

## The fix

Documentation-only, matching the project's own existing precedent for the same class of issue
(`Affine::inverse`'s doc comment: "Produces NaN values when the determinant is zero."). Added a
doc-comment paragraph to both `inscribed_circle` and `circumscribed_circle` stating explicitly
that a degenerate (collinear or coincident-vertex) `Triangle` produces a `Circle` with NaN (and,
for `circumscribed_circle`, potentially infinite) fields, so callers know this without having to
read the implementation. See `patch.diff` (+6/-0 lines in `kurbo/src/triangle.rs`).

This does not change the numeric behavior (still NaN/Inf for a degenerate triangle) - a
behavior-changing fix (e.g. returning `Option<Circle>`, or a sentinel `Circle`) would be a
breaking public-API change requiring a project-maintainer decision, out of scope for a bounded
session fix; see "What's not done" below.

Verified: re-ran the `repro/` binary (both inputs already covered - three separate degenerate
cases: collinear, fully coincident, and two-coincident-one-distinct, i.e. more than the single
input class the candidate description called out) against the patched tree - output unchanged
(still reproduces, as expected for a documentation-only fix; see `output.txt`'s before/after).
`cargo doc --no-deps` builds cleanly with the new doc comments (pre-existing, unrelated doc
warnings elsewhere in the crate are unchanged).

- Project test suite (`cargo test --release` at `kurbo/`): 185 passed / 2 failed before -> 185
  passed / 2 failed after (same 2 pre-existing failures both times:
  `bezpath::tests::test_elements_to_segments_starts_on_closepath` and
  `bezpath::tests::test_must_not_start_on_quad`, unrelated to `triangle.rs`).

## What's not done / open questions

- No behavior change was made (no guard, no `Option`/`Result` return, no sentinel value) - only
  documentation. Whether kurbo's maintainers want an actual behavior change (and what shape:
  `Option<Circle>`, a documented sentinel, or leaving it as NaN/Inf-on-degenerate like several
  other kurbo functions already do, e.g. `Affine::inverse`) is a project-level API decision, not
  made here.
- `Triangle::is_zero_area()` already exists and could be checked by a caller before calling either
  method to avoid the NaN/Inf case, but nothing in either method's own doc comment pointed callers
  at it before this fix - the new doc comments don't yet cross-reference `is_zero_area()` either;
  a follow-up doc improvement could do so.
- Not fuzzed - the degenerate cases here are a small, easily enumerable boundary-value space
  (collinear / coincident-vertex), not a large adversarial input space, so a fuzz harness was not
  judged worth the setup cost for this candidate.
