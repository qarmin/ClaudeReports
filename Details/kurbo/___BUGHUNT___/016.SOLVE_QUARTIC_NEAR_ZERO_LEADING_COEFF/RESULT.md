# SOLVE_QUARTIC_NEAR_ZERO_LEADING_COEFF

Follow-up to candidate 014.CUBIC_LINE_INTERSECT_FALSE_POSITIVE_ISSUE411 and the open, unmerged PR
#593: does `common::solve_quartic` (used by `CubicBez::tangents_to_point`, the only real caller in
the crate) inherit the same unguarded "scale by 1/leading-coefficient when the leading coefficient
is floating-point cancellation noise, not exactly zero" weakness as `common::solve_cubic`? Answer:
yes, via `solve_quartic`'s own `c0 == 0.0` (exact-equality) delegation branch, which calls
`solve_cubic(c1, c2, c3, c4)` directly with no magnitude-relative guard - `c4` (the quartic's own
leading coefficient) becomes `solve_cubic`'s "c3" argument, reproducing candidate 014's exact
mechanism one level up.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: hand-derive the delegation condition (`solve_quartic`'s `c0 == 0.0` branch calls `solve_cubic(c1,c2,c3,c4)` unguarded) and directly probe `common::solve_quartic`/`common::solve_cubic` with synthetic coefficients mimicking a near-degenerate cubic (`(x-1)(x-2)(x-3)` plus tiny `c4*x^4`) | debug build, one-off `cargo run --example`, single run | Confirmed the mechanism in isolation: at `c4` in the `1e-16`-`1e-18` range, `solve_cubic`'s scaled-coefficient branch returns roots with large relative residual (the two roots near the true cubic roots drift completely off, e.g. `74.01`/`-111.02` instead of `~5`/`~7`) - but this used raw coefficients, not a real geometric entry point |
| Real end-to-end entry point: `CubicBez::tangents_to_point` (the crate's only `solve_quartic` caller). Constructed a query where `p == curve.p0` exactly (forces `c0 == 0.0` bit-for-bit, since `c0 = d.cross(c)` with `d = p0 - p = (0,0)`), then directed-searched control points for near-parallel `a`/`b` cross-product vectors (the quartic's own leading coefficient `c4 = b.cross(a)`) built via genuine `Point`/`Vec2` arithmetic (not a manually-added epsilon, so any near-zero `c4` is genuine floating-point rounding noise, the same way issue #411's near-zero `c3` arose) | debug + release build, `cargo run --example`, ~20000-iteration directed search (release, <1s) | Found 1238 inputs (out of the search space) where `tangents_to_point` returns a `t` in `[0,1]` whose tangent line is nowhere near passing through the query point (`sin(angle) > 0.5`, up to `~0.9999` i.e. ~89.9° off) - confirmed via the real public API, not raw coefficients |
| Repeat-run confirmation of the two selected repro cases (`curve1`, `curve2`) against a genuinely fresh `git clone` at the pinned base commit | release build, 3 consecutive runs, no RNG/threading (pure deterministic floating-point arithmetic) | Identical output all 3 runs - bit-for-bit repeatable |
| Root-cause read of `solve_quartic`/`solve_cubic` in `kurbo/src/common.rs`, cross-checked against candidate 014's own root-cause writeup and the open, unmerged PR #593 | code reading, no tool | Confirmed: PR #593's fix (the same magnitude-relative threshold already applied and verified in candidate 014, inside `solve_cubic` itself) also fixes this candidate's repro, since `solve_quartic`'s degenerate branches call `solve_cubic` directly and inherit any guard placed inside it |

No further escalation (miri/ASAN/fuzzing) was needed - like candidate 014, this is a pure
floating-point numerical-robustness bug in deterministic code, not memory-safety/UB, and the
mechanism was already fully understood from candidate 014's own investigation.

## Status

**Confirmed** - reproduces 3/3 runs against a fresh clone, deterministic (pure floating-point
computation on fixed inputs, no RNG, no threading, no environment dependence). Also independently
reproducible on demand via the directed search (1238/~14000 valid iterations hit the same failure
mode in one run of `probe10.rs`, not saved into `repro/` since two fixed cases are sufficient to
demonstrate and fix the bug, but the search itself is trivial to rerun).

**Fully reproducible**: **Yes**. `repro/` calls the real, public `CubicBez::tangents_to_point`
entry point directly with valid, finite, in-range `f64` control points and a valid query point -
this is not a finding about how one participant reacts to another's wire/peer input (no
untrusted-peer/parser angle here, same as candidate 014), so `CONFIRMATION.md` section 11's
second-participant requirement does not apply; the "real end-to-end trigger path" is simply
calling the documented public API exactly as any caller (e.g. an interactive vector-editing tool
offering "show tangent lines from this point" or a boolean-path-operations library built on
kurbo) would.

## Class

logic (silently wrong result - a false-positive "tangent point" reported for a curve/point pair
that isn't actually tangent there; no panic, no UB).

## Root cause

`CubicBez::tangents_to_point` (`kurbo/src/cubicbez.rs`) builds quartic coefficients `c0..c4` from
the cubic's Bernstein-form derivative-related vectors and the query point, then calls
`common::solve_quartic(c0, c1, c2, c3, c4)`.

`solve_quartic` (`kurbo/src/common.rs`) has two exact-equality fast paths that delegate straight to
`solve_cubic`:

```rust
if c4 == 0.0 {
    return solve_cubic(c0, c1, c2, c3).iter().copied().collect();
}
if c0 == 0.0 {
    return solve_cubic(c1, c2, c3, c4).iter().copied().chain(Some(0.0)).collect();
}
```

The `c0 == 0.0` branch is reached whenever the query point exactly equals the curve's own start
point `p0` (a completely ordinary case - `c0 = d.cross(c)` where `d = p0 - p`, so `d` is the exact
zero vector and `d.cross(c) == 0.0` bit-for-bit regardless of `c`). In that branch, `solve_cubic`
is called with the quartic's own leading coefficient `c4` passed as `solve_cubic`'s "c3" (leading)
argument - `c4` is exactly candidate 014's `c3`: for a curve whose tangent-count problem
mathematically degenerates from quartic to cubic (i.e. `c4` "should" be exactly zero), `c4` is
instead floating-point cancellation noise on the order of `1e-16`-`1e-18` relative to the other
coefficients - small enough that `1/c4` doesn't overflow (so `solve_cubic`'s only existing guard,
an `is_finite()` check, doesn't trigger), but large enough that scaling `c0'/c1'/c2'` (the
"c1,c2,c3" arguments received) by `1/c4` amplifies their rounding error catastrophically. The
resulting "roots" are numerical artifacts, not real solutions - confirmed directly by residual
checking (e.g. for the primary repro's curve, `solve_cubic(c1,c2,c3,c4)` returns `[1.4957e16,
0.10702, -2.10702]`, none of which satisfies the cubic equation to any reasonable tolerance,
versus the mathematically-correct near-answer `[-1.71739, 0.0]` obtained by solving the quadratic
that ignoring the negligible `c4` term gives). `tangents_to_point` then reports the spurious
`t ≈ 0.107` (within its `[0,1]` filter) as a genuine tangent point, even though the tangent line
there is nearly perpendicular (not parallel) to the direction toward the query point.

This is the identical mechanism to candidate 014
(`014.CUBIC_LINE_INTERSECT_FALSE_POSITIVE_ISSUE411/RESULT.md`), one level up: `solve_cubic` itself
has no magnitude-relative guard on its own leading coefficient, only an `is_finite()` check on the
post-scaling values, so any caller that hands it a "should be zero but isn't quite" leading
coefficient - whether directly (candidate 014's `PathSeg::intersect_line`) or via
`solve_quartic`'s degenerate-branch delegation (this candidate's `tangents_to_point`) - inherits
the same amplification bug.

## Introduced by

The vulnerable pattern in `solve_cubic` itself (no magnitude-relative guard, only `is_finite()`)
predates tracked history per candidate 014's own lookup (92feb79 "Start curves crate" - `solve_cubic`
already implemented this way in the crate's first commit).

The specific entry point that makes this reachable at the quartic level via
`CubicBez::tangents_to_point` was introduced by 987b6ff "Find tangents from point to cubic Bézier
(#288)" (`git log --all -S"fn tangents_to_point"`), which added `tangents_to_point` (including its
call into `solve_quartic`) to the crate. `solve_quartic`'s own `c0 == 0.0`/`c4 == 0.0` delegation
branches (which call `solve_cubic` directly, unguarded) were present when `solve_quartic` was
introduced and were not specifically examined for an earlier introduction point this session (not
the bug's root cause - the root cause is `solve_cubic`'s own missing guard, already dated above).

## The fix

Applied the identical fix already verified in candidate 014 (itself carried over from the open,
unmerged PR #593), inside `solve_cubic` - a magnitude-relative threshold before the `1/c3`-scaling
branch:

```rust
pub fn solve_cubic(c0: f64, c1: f64, c2: f64, c3: f64) -> ArrayVec<f64, 3> {
    let mut result = ArrayVec::new();
    if c3.abs() <= 1e-12 * c0.abs().max(c1.abs()).max(c2.abs()) {
        return solve_quadratic(c0, c1, c2).iter().copied().collect();
    }
    let c3_recip = c3.recip();
    ...
```

Because the fix lives inside `solve_cubic` itself, `solve_quartic`'s degenerate-branch delegation
inherits it automatically with no separate change needed at the `tangents_to_point`/`solve_quartic`
call sites - this candidate did not need its own patch, applying candidate 014's patch again here
confirms the same fix covers both call sites.

This is sufficient for both repro cases: `tangents_to_point` on both `curve1` and `curve2` now
returns only the trivial `t = 0.0` root (the query point itself, which is always a correct,
uninteresting root since `p == p0`) instead of the spurious `t ≈ 0.107`/`t ≈ 0.059`. See
`patch.diff` (against `kurbo/src/common.rs` only, identical to candidate 014's) and `output.txt`
for the actual before/after `repro/` run.

- Project test suite (`cargo test --workspace`, in a fresh clone at the pinned base commit): 252
  passed, 0 failed, 1 ignored (187 kurbo unit tests + 10 polycool unit tests + 55 kurbo doctests +
  1 ignored polycool doctest), identical **before** and **after** the fix - no regressions, no new
  failures.
- Fix verified against a second, independently-found input in the same class (`curve2`, different
  control points from a different iteration of the directed search) per `CONFIRMATION.md` section
  7, not just the one input that first triggered the finding - both cases show the spurious root
  before the fix and both show only the trivial root after.

## What's not done / open questions

- Same scope note as candidate 014: the fix is applied inside `solve_cubic` itself, so it
  automatically covers every call site in the crate (both `solve_quartic`'s degenerate branches and
  `solve_cubic`'s own direct callers like `PathSeg::intersect_line`), not just this candidate's
  `tangents_to_point` path - this was checked by construction (the patch is identical to candidate
  014's and lives in the shared function), not by separately re-testing every other call site.
- `solve_quartic`'s *general* (non-degenerate) path - `factor_quartic_inner`'s own
  Orellana-De Michele algorithm, used whenever neither `c0` nor `c4` is exactly `0.0` - was
  separately probed (see "What was tried" in candidate 014's sibling investigation notes / this
  session's `probe.rs`/`probe2.rs`/`probe3.rs` scratch files, not preserved in `repro/`) by feeding
  it a genuinely near-degenerate quartic (`(x-1)(x-2)(x-3)` plus a tiny quartic term, `c4` down to
  `1e-18`) and checking residuals; that path did **not** show the same amplification - it correctly
  found a large-but-real fourth root and kept the other three roots accurate to typical
  double-precision residuals. This suggests `factor_quartic_inner`'s own algorithm is materially
  more robust than `solve_cubic`'s naive `1/c3`-scaling approach, and the vulnerability is
  specifically confined to the two exact-equality delegation branches (`c0 == 0.0` / `c4 == 0.0`),
  not the general quartic solver - not re-verified with a broader fuzz search this session.
- No attempt was made to search for a real caller/scenario that reaches `solve_quartic`'s
  `c4 == 0.0` branch (the other delegation branch, `solve_cubic(c0, c1, c2, c3)` - symmetric to the
  one exercised here) with a near-zero-but-nonzero `c3`; by the same mechanism it should be
  equally vulnerable and equally fixed by the same patch, but this wasn't independently constructed
  as a third repro case this session.
- Did not check whether any other, non-kurbo-crate downstream consumer of `tangents_to_point`
  (e.g. a vector-graphics editor built on kurbo) has already observed this as a user-visible bug -
  no existing GitHub issue was found referencing `tangents_to_point` specifically during this
  session's discovery pass.
