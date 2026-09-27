# EXPAND_TWO_POINT_LINEAR_NORMALIZE_NAN

`two_point_linear` in `kurbo/src/expand.rs` (called from `do_cubic` when `try_one_point` returns
`None`) computes `cx.q.eval(t).to_vec2().normalize()` at t=0.35 and t=0.65 with no explicit check
that the evaluated derivative vector is non-degenerate, unlike its sibling `try_one_point`, which
guards the analogous t=0.5 evaluation with `if tan_hypot2 < 1e-12 { return None; }`. This looked
suspicious because a cusp (an exact zero-length derivative) located at t=0.35 or t=0.65, rather
than at the midpoint, appeared able to make `normalize()` produce `(NaN, NaN)` and propagate it
unconditionally into the emitted `curve_to` control points via the public
`expand_path`/`expand_path_signed` API.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: solved for cubic Bezier control points whose derivative curve is exactly `(0,0)` at t=0.35 (verified independently via the crate's own public `CubicBez::deriv()`/`ParamCurve::eval`), fed through the real public `expand_path_signed` entry point | release build, single collinear-tangent case | No NaN in output; `two_point_linear` returned `None` and `do_cubic` fell back to `do_line` |
| Source-level instrumentation: temporary `eprintln!` added to `two_point_linear` in the working checkout (reverted after) to print the per-sample `TwoPointSample{a_n,b_n,c_n}` and the final `det` for the same case | release build, single case | Confirmed the t=0.35 sample really is all-`NaN` internally (`normalize()` did divide 0.0/0.0), but `det` (which combines `a_n`/`b_n` from *both* samples) becomes `NaN` too, and the existing `det.abs().partial_cmp(&1e-12) != Some(Ordering::Greater)` check (its own comment: "Consider both near-zero and NaN determinants to be failures") catches it and returns `None` |
| Broad boundary-value sweep: ~1000 cusp locations (t from 0.02 to 0.98, excluding a 1e-6 neighborhood of 0.5) x 4 distinct non-parallel endpoint-tangent-direction pairs (collinear and non-collinear), each fed through the real public `expand_path_signed` API; 367 of the 3836 combinations were independently verified (again via the public `CubicBez::deriv()` API) to be genuine bit-exact-zero-derivative cusps at the target t | release build, ~1-2s runtime | 0 of the 3836 combinations (0 of the 367 genuine-cusp combinations) produced any NaN in the output |

## Status

**Not reproducible.** Covered: a single hand-constructed exact cusp at t=0.35 (re-run twice with
identical output), plus a broader sweep of 3836 (cusp location x endpoint-tangent-pair)
combinations, 367 of which were independently confirmed to be genuine bit-exact-zero-derivative
cusps at the sampled t (i.e., cases where `normalize()` really does compute `0.0/0.0 = NaN`
internally). In every single case, no NaN reached the output of `expand_path_signed`. This is not
an exhaustive search of the whole input space, but it directly targets the exact failure mode
described in the candidate (a cusp away from the guarded midpoint) across a wide range of cusp
locations and tangent configurations, and it is backed by a mechanistic explanation (below), not
just an absence of observed failures.

**Fully reproducible**: N/A (Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause (of why the suspected bug turns out to be safe)

The candidate's premise - that `two_point_linear` normalizes a possibly-zero vector with no
guard - is correct as far as it goes: when `cx.q.eval(t)` is exactly `(0, 0)` for one of the two
samples (t=0.35 or t=0.65), `Vec2::normalize()` (`self / self.hypot()`) does compute `0.0/0.0`,
producing a `(NaN, NaN)` `utan` for that sample. But every downstream quantity computed from that
sample's `utan` (`n = utan.turn_90()`, then `utan0_n`/`utan1_n`/`utan0xn`/`utan1xn`, then
`a_n`/`b_n`/`c_n`) also becomes NaN, because dot products and scalar multiplication involving a
NaN operand always produce NaN in IEEE-754 arithmetic (there is no code path here that multiplies
a NaN component by an exact `0.0` coefficient before it reaches `det` - both `a_n` and `b_n` for
the degenerate sample are already NaN by the time `det` is formed).

`two_point_linear`'s own `det = s[0].a_n * s[1].b_n - s[1].a_n * s[0].b_n` therefore always
becomes `NaN` whenever either sample is degenerate this way (a `NaN` operand in a product makes
the whole product `NaN`, regardless of the other operand's value), and the existing check

```rust
// Consider both near-zero and NaN determinants to be failures.
if det.abs().partial_cmp(&1e-12) != Some(core::cmp::Ordering::Greater) {
    return None;
}
```

explicitly treats an unordered (NaN) comparison as failure (`NaN.partial_cmp(&x)` is `None` for
any `x`, and `None != Some(Greater)` is `true`), so the function returns `None` before `a`/`b` are
ever computed from the NaN branch. `do_cubic` then falls back to `self.do_line(p3)`, which never
touches the degenerate `utan`/`cx.q` values at all.

This mechanism does not depend on which of the two samples (t=0.35 or t=0.65, or both) is
degenerate, nor on whether the cusp is collinear with the endpoint tangents or not - it is a
structural property of how `det` is built from `a_n`/`b_n` of both samples. The broad sweep above
(367 genuinely NaN-triggering internal cases, 0 NaN reaching the output) is consistent with this
being a robust, not merely incidental, guard - unlike `try_one_point`'s guard, which is an
explicit early check, `two_point_linear`'s protection is an emergent (but reliable, given the
arithmetic above) consequence of its own final determinant check already being written to reject
NaN.

## Introduced by

N/A (Not reproducible - no bug to attribute).

## The fix

No fix needed; nothing to patch. `two_point_linear` does not need an explicit early
`tan_hypot2 < 1e-12`-style guard analogous to `try_one_point`'s, because its own final `det` check
already subsumes it for the reasons above.

- Project test suite: N/A - no change was made to the source (the temporary debug
  instrumentation used during investigation was reverted; `git status`/`git diff` in the working
  checkout show no changes).

## What's not done / open questions

- This does not prove no input can ever make `two_point_linear` return a bad (non-`None`,
  NaN-tainted) `(a, b)` pair - only that the specific "exact zero derivative at t=0.35/0.65"
  failure mode named in the candidate is caught by the existing `det` check. A different kind of
  degeneracy (e.g. one that makes `a_n`/`b_n` finite-but-wrong rather than NaN) is a different
  suspect, not covered here.
- The sibling function `try_one_point` was not re-examined here beyond confirming it already has
  its own explicit guard at t=0.5 (untouched by this candidate).
- No fuzzing campaign was run against `do_cubic`/`two_point_linear` as a whole (only targeted
  boundary-value construction) - a longer, separate `fuzz-bug-repro` campaign against
  `expand_path`/`expand_path_signed` with cubic-heavy inputs could still be worth pointing at this
  file for unrelated degeneracies, but that is out of scope for this candidate's own soft 3-tool
  cap.
