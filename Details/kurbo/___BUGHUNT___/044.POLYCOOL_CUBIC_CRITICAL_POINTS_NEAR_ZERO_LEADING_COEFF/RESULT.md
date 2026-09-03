# POLYCOOL_CUBIC_CRITICAL_POINTS_NEAR_ZERO_LEADING_COEFF

`polycool/src/cubic.rs:72-90` (`Cubic::critical_points`) computes `a = 3.0 * self.coeffs[3]`,
`disc_4 = b_2*b_2 - a*c`, and (when `disc_4 > 0.0`) `r0 = q/a`. The only overflow guard
(`!disc_4.is_finite()`, falling back to `rescaled_critical_points`) doesn't fire when `a` is
merely tiny-but-nonzero rather than causing `disc_4` itself to overflow, so `r0` can become huge
but still finite. This looked suspicious as a separate instance of the same "near-zero leading
coefficient amplifies floating-point error" bug class already confirmed in candidates 014/016/037
(`common::solve_cubic`'s `1/c3` scaling), reached here via a different path: `CubicBez::nearest`'s
quintic -> quartic -> cubic derivative recursion, where the innermost cubic's own leading
coefficient (`a`) is driven towards zero whenever the input bezier's cubic term is small (a
"nearly quadratic" cubic bezier).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value/asymptotic reasoning: traced `a`'s exact algebraic origin back through `CubicBez::nearest` (`c5 = 3.0 * q3.hypot2()` -> quartic's `coeffs[4] = 5*c5` -> cubic's `coeffs[3] = 4*coeffs[4]` -> `a = 3*coeffs[3] = 60*q3.hypot2()`), and worked out the exact-quadratic-degeneration limit of `a*x^2 + 2*b_2*x + c = 0` as `a -> 0` | Code reading only, no build | `a` is a non-negative scaled sum of squares (`q3.hypot2()`) computed with only constant-factor multiplications - no subtraction/cancellation is involved in deriving it, unlike `solve_cubic`'s `c3` (which arises from genuinely unrelated terms that can cancel). Separately, as `a -> 0` a quadratic `a*x^2+2*b_2*x+c=0` mathematically degenerates towards the linear equation `2*b_2*x+c=0`: one root correctly converges to the linear equation's `-c/(2*b_2)`, the other correctly diverges towards +/-infinity. `r0 -> infinity` in this regime is the documented, intended behavior (`critical_points`'s own doc comment: "If the derivative is linear or close to it, we might return +/- infinity as one of the roots"), not an error-amplification artifact |
| Direct differential fuzz of the public `polycool::Cubic::roots_between` (which `critical_points`/`first_root` back) against a brute-force sign-change ground truth (20,000-sample scan per trial) on `[0.0, 1.0]` (the actual bound `CubicBez::nearest` uses), checking for (a) any returned root outside `[lower, upper]`, (b) any returned root whose polynomial value isn't actually near zero (spurious root), and (c) any brute-force-detected sign change with no corresponding returned root (missed root) | Release build; `c0`/`c1`/`c2` magnitudes spanning `1e-20` to `1e2`, leading coefficient `c3` spanning `1e-320` to `1e2` (both signs, log-uniform exponent); 1,000,000 trials in the wider standalone probe, 500,000 trials in `repro/` (see `output.txt`); also re-ran 50,000 trials under a debug-assertions build (`overflow-checks = true`) | 0 mismatches of any kind, in both release and debug builds |
| Real end-to-end differential fuzz of the actual affected entry point, `kurbo::CubicBez::nearest`, comparing its reported `distance_sq` against a brute-force-sampled (20,000 points/trial) ground truth, on cubic beziers constructed to be "nearly quadratic" (control point `p3` set to the value that would make the cubic term `q3` exactly zero, then perturbed by a random epsilon spanning `1e-300` to `1e-1`) | Release build; `p0`/`p1`/`p2`/query-point magnitudes spanning `1e-2` to `1e8`; 2,000,000 trials in the wider standalone probe, 500,000 trials in `repro/` (see `output.txt`) | Worst observed ratio of `nearest()`'s reported `distance_sq` to the brute-force minimum was `1.0000000000000056` - floating-point noise, not a meaningful discrepancy indicating a missed critical point/wrong root count |

## Status

**Not reproducible.** Covered: manual algebraic/asymptotic tracing of `a`'s origin (ruling out the
cancellation mechanism that makes the analogous `solve_cubic` bug real), a direct fuzz of the
public `polycool::Cubic::roots_between` (1,000,000 release-build trials + 50,000 debug-build
trials, leading coefficient down to `1e-320`, 0 mismatches), and a real end-to-end fuzz of
`kurbo::CubicBez::nearest` (2,000,000 release-build trials on deliberately near-degenerate cubic
beziers, 0 meaningful discrepancies vs. brute-force ground truth). `repro/` (driven by `just test`)
reruns condensed versions of the second and third approaches (500,000 trials each, ~44s total)
against a fresh clone and also found 0 mismatches.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce). `repro/` demonstrates the
negative result via the real public `polycool::Cubic::roots_between` and `kurbo::CubicBez::nearest`
APIs directly (no hand-built internal state standing in for a caller) - see `CONFIRMATION.md`
section 11's "single-process, single-participant repro remains completely fine" carve-out; there
is no second participant to fake here (this is a pure numerical/logic question about how one
function's own internal recursion handles its own inputs).

## Class

N/A (Not reproducible)

## Root cause

The suspect turns out to be safe, for a different reason than the exactly-zero case's "handled
gracefully by design": unlike `common::solve_cubic`'s `c3` (candidates 014/016/037), which is a
combination of otherwise-unrelated terms that can suffer genuine catastrophic cancellation, making
its "near zero" a numerically-noisy false signal that then gets amplified by `1/c3`, this cubic's
own `a = 3*coeffs[3]` is derived (via `CubicBez::nearest`'s quintic -> quartic -> cubic derivative
chain) as `a = 60 * q3.hypot2()` - a non-negative sum of squares scaled only by small integer
constant multiplications, with no subtraction anywhere in its derivation. So `a` is only ever
"near zero" when it is genuinely, accurately near zero (the input bezier's own cubic term `q3` is
genuinely small), not as an artifact of cancellation error. In that genuine-smallness regime, `r0
= q/a` growing large (up to and including the documented +/-infinity case) is the mathematically
correct asymptotic behavior of a quadratic degenerating towards a linear equation as its leading
coefficient shrinks to zero - one root legitimately diverges, the other (`r1 = c/q`, well
conditioned since it doesn't divide by `a`) legitimately converges to the correct single root of
the limiting linear equation. The diverging root reliably falls outside `[lower, upper]`
(typically `[0, 1]` for the `CubicBez::nearest` path) whether it lands at a merely-large finite
value or at literal infinity, so `first_root`'s `x <= upper` bracketing filter excludes it
correctly either way - this is a structural consequence of the magnitude involved, not a fragile
coincidence specific to one input.

## Introduced by

N/A (Not reproducible).

## The fix

No fix - the suspected mechanism does not reproduce, so no code change is proposed.

- Project test suite: not run (no code change made; `SKILL.md`'s candidate loop step 7 only
  requires this for a verified fix).

## What's not done / open questions

- This candidate deliberately scoped itself to the exact path named in its description
  (`CubicBez::nearest`'s quintic-derivative recursion reaching `Cubic::critical_points`). Other
  direct callers of `polycool::Cubic::roots_between`/`Quartic`/`Quintic::roots_between` with bounds
  other than `[0, 1]` (e.g. a hypothetically much wider `[lower, upper]` interval, if one exists
  elsewhere in `kurbo` or a downstream consumer of `polycool` directly) were not specifically swept
  - the `[0, 1]` bound is what makes "the diverging root always ends up out of range" a structural
    guarantee in this analysis; a caller using a much wider interval (say `[-1e150, 1e150]`) could
    plausibly still have the diverging root land inside it. No such caller was found in this
    codebase during this session, but this was not exhaustively verified.
  - The `Poly<N>` macro-generated recursive `roots_between_with_buffer` (`poly.rs`, degrees 5-10)
    was not itself re-examined for an analogous near-zero-leading-coefficient concern at higher
    degrees; only the degree-3 (`Cubic`, hand-written in `cubic.rs`) case named by this candidate
    was checked.
  - A longer/dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) covering a wider
    parameter space (asymmetric `[lower, upper]` bounds, denormal-range coefficients, NaN/Inf
    injection at intermediate steps) was not attempted - this session's ~3.5M total trials across
    both probes give reasonable but not exhaustive confidence.
