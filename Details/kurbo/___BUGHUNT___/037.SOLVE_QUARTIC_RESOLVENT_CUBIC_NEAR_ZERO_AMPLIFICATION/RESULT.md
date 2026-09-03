# SOLVE_QUARTIC_RESOLVENT_CUBIC_NEAR_ZERO_AMPLIFICATION

Follow-up to 016.SOLVE_QUARTIC_NEAR_ZERO_LEADING_COEFF, which confirmed a bug in `solve_quartic`'s
two exact-equality degenerate fast paths (`c0 == 0.0` / `c4 == 0.0` delegating to `solve_cubic`)
but explicitly left the *general* (non-degenerate) path - `factor_quartic_inner`'s own
Orellana-De Michele algorithm, driven through `g_prime`/`h_prime` and `depressed_cubic_dominant`
at common.rs:386-410 - unchecked, per candidate 016's own "What's not done" notes. This candidate
targets exactly that path: does it inherit the same "near-but-not-exactly-zero denominator/
discriminant amplifies floating-point noise" weakness, for coefficients engineered to avoid the
`c0`/`c4` exact-zero fast paths?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + arbtest-driven differential fuzzing: build monic quartics `(x-r0)(x-r1)(x-r2)(x-r3)` from 4 chosen real roots (plain `f64` arithmetic, exact rounding a real caller would incur), call the real public `kurbo::common::solve_quartic` directly, and check the returned roots against the roots the quartic was actually constructed from (residual + nearest-true-root distance). 4 root-generation modes: fully random, "two near-double-root pairs" (`(x-p)(x-p-e)(x-q)(x-q-e)`, `e` down to ~1e-9), "near-symmetric about zero", "three roots clustered + one far". A local copy of the `g_prime`/`h_prime` computation (matching common.rs:401-405) was kept alongside purely for instrumentation (to correlate near-zero `g'`/`h'` with observed errors), never used to compute the actual answer. | debug build sanity pass + release-build `arbtest` runs, `budget_ms(20_000)` per run, ~1,000,000-1,040,000 iterations per run across several runs | Found a real, repeatable defect: **mode "two near-double-root pairs" hit `solve_quartic` returning fewer roots than exist in ~9.6% of its own draws** (24,363-25,208 of ~252,000-257,000 iterations per run, across several runs) - `found.len() < 2` (frequently `found = []`, i.e. *zero* roots returned) for quartics provably constructed from 4 real roots. The "three clustered + one far" mode hit the same failure far more rarely (6-18 per ~257,000). The fully-random and near-symmetric modes never hit it in ~250,000+ iterations each. Root-value accuracy itself (residual of returned roots, for cases that *did* return roots) stayed at normal double-precision noise levels (worst relative residual ~2.7e-13 to 3.4e-13 even at `g'+h'` relative magnitude down to ~1e-22-1e-25) - so candidate 037's originally-suspected failure mode ("root accuracy silently degrades") was **not** observed; the **"wrong root count is returned"** alternative the candidate description explicitly flagged as possible **was**. |
| Root-cause drill-down: called `kurbo::common::factor_quartic_inner` directly (also `pub`) on the failing cases to inspect the returned quadratic factors `(alpha, beta)`, and `kurbo::common::solve_quadratic` on those factors' own discriminant | same release build, hand-picked failing cases from the fuzz run above | `factor_quartic_inner` correctly returns two quadratic factors with **real** coefficients in every failing case (e.g. `x^2 + (-12.93...)x + 41.82...`) - but the *computed* discriminant `alpha^2 - 4*beta` for at least one factor is small and **negative** (e.g. `-1.9e-12`, `-2.8e-13`, up to `-4.4e-6` in the worst observed cases) even though the true discriminant (from the actual root gap) is small and **positive**. `solve_quadratic`'s own `if arg < 0.0 { return result; }` check (common.rs:267-268) has no tolerance for this - it silently returns zero roots for that quadratic factor, which is exactly where the real (near-equal) root pair gets dropped. |
| False-positive risk check for a candidate fix: built a second `arbtest` harness constructing quartics `= (x^2 - 2·re·x + (re^2+im^2)) * (x - p)(x - q)` - a genuine complex-conjugate pair (imaginary part `im` from ~1e-4 to ~1e-1, deliberately *not* down at the machine-noise floor) times a real pair - and checked whether a candidate fix's discriminant-snapping threshold ever misreports the complex pair's real part as a spurious real double root | release build, `arbtest`, `budget_ms(8_000)`, ~410,000-413,000 iterations per threshold tested | Confirmed the unpatched code has **zero** false positives here (`bad=0`), as expected. A first fix attempt using a `1e-8`-relative snap threshold introduced **1,664/412,974** false positives (genuinely-complex root pairs with `im` as large as ~1e-3 wrongly reported as a real double root) - too loose. Tightening to `1e-12`-relative (matching the threshold style already used in candidates 014/016's `solve_cubic` fix) reproduced **zero** false positives in ~409,000 iterations while still reducing the original wrong-root-count rate by ~94% (24,372/1,031,512 -> 1,420/1,031,512) - not a full elimination (see "What's not done" below), but a clear, verified improvement with no observed regression. |

No further escalation (miri/ASAN) was needed or applicable - this is pure floating-point
numerical-robustness logic in deterministic, safe code, not memory-safety/UB.

## Status

**Confirmed** - reproduces 5/5 runs (both debug and `--release` builds) against a fixed,
hand-picked input, deterministic (pure floating-point computation on fixed inputs, no RNG, no
threading). Also independently reproducible on demand via the `arbtest` differential-fuzzing
harness described above (24,363-25,208 hits per ~252,000-257,000-iteration run, consistently
across several separate runs - not a one-off).

**Fully reproducible**: **Yes**. The standalone `repro/` calls the real, public
`kurbo::common::solve_quartic` entry point directly with valid, finite, in-range `f64`
coefficients constructed from 4 genuine real roots - this is not a finding about how one
participant reacts to another's wire/peer input (no untrusted-peer/parser angle: the coefficients
*are* the external input, exactly as any real caller such as `CubicBez::tangents_to_point` would
hand them to this function), so `CONFIRMATION.md` section 11's second-participant requirement does
not apply.

## Class

logic (silently wrong result - too few roots returned, sometimes zero, for a quartic that
genuinely has 4 real roots; no panic, no UB).

## Root cause

`solve_quartic`'s general (non-degenerate) path factors the quartic into two quadratics via
`factor_quartic_inner` (`kurbo/src/common.rs`), which internally computes `phi` (a resolvent-cubic
root, via `depressed_cubic_dominant(g_prime, h_prime)`), then derives `l_1`/`l_3`/`d_2`/`l_2` and
finally `alpha_1`/`beta_1`/`alpha_2`/`beta_2` (the two quadratic factors' own coefficients),
refined by up to 8 Newton-Raphson iterations. Candidate 037's original hypothesis was that
`g_prime`/`h_prime` themselves, being near-but-not-exactly zero, would amplify through a division
the way `solve_cubic`'s `1/c3` amplifies in candidates 014/016. That specific mechanism was
**not** found: `depressed_cubic_dominant`'s own algorithm (choosing among algebraically-equivalent
formulas via `eps_rel`-based candidate selection, plus its own Newton refinement) turns out to be
considerably more robust than `solve_cubic`'s naive `1/c3` scaling - root *values*, when returned,
stayed accurate to normal double-precision residuals even at `g'+h'` magnitudes many orders of
magnitude smaller than anything tested for `solve_cubic`.

Instead, the real defect is one step further downstream, in the same general code path: when the
quartic's true roots include a pair that is real but very close together (the geometrically
ordinary case of a "near-double root" / near-tangency), the accumulated floating-point error from
the `phi`/`l_1`/`l_3`/`d_2`/`l_2`/Newton-Raphson derivation chain (each step is exact in principle
but not in `f64`) can be comparable to or larger than the *true* discriminant of that quadratic
factor (which is proportional to the square of the tiny root gap, so it shrinks quadratically as
the gap shrinks while the derivation's own absolute error does not). When the computed discriminant
`alpha^2 - 4*beta` for that factor comes out negative purely from this accumulated noise,
`solve_quadratic`'s existing `if arg < 0.0 { return result; }` check (common.rs, no tolerance) does
exactly what it's supposed to for a *genuinely* complex-root quadratic - silently return zero
roots for that factor - except here the roots are genuinely real, not complex, and both get
silently dropped from `solve_quartic`'s final result. This is the general-path analog of
candidates 014/016's underlying theme (a shared helper's strict, non-tolerant sign/finiteness
check being fed inputs whose sign is dominated by upstream floating-point noise rather than the
true mathematical value) - reached via a materially different mechanism (a discriminant sign flip
via multi-step derivation error, not a `1/near-zero-c3` blowup) than originally hypothesized for
this candidate.

## Introduced by

Two contributing commits, from different eras of the crate:

- `solve_quadratic`'s strict, non-tolerant `if arg < 0.0 { return result; }` check: 8d0c810
  "Make quadratic solver numerically robust" (2019-09-03) - `git log --all -S"if arg < 0.0" --
  '*common.rs'`. This check is reasonable in isolation (a genuinely negative discriminant does
  mean no real roots) - the bug only manifests because of the second commit below.
- The general (non-degenerate) quartic-factoring path that feeds `solve_quadratic` coefficients
  derived through the multi-step `phi`/`l_1`/`l_3`/`d_2`/`l_2`/Newton-Raphson chain described
  above, making a near-double-root quadratic factor's discriminant vulnerable to that
  accumulated error: eb658ab "Quartic solver" (2022-08-18) - `git log --all -S"pub fn solve_quartic"
  -- '*common.rs'`, the commit that introduced `solve_quartic`/`factor_quartic_inner` (and
  `solve_quartic_inner`'s direct call into `solve_quadratic` on the factored quadratics) in the
  first place.

## The fix

Added a small, narrowly-scoped wrapper `solve_quadratic_factor` used *only* at the one call site in
`solve_quartic_inner` that solves the two quadratic factors `factor_quartic_inner` returns (does
**not** touch the widely-used general-purpose `solve_quadratic` function itself, to avoid changing
behavior for its many other callers in the crate - `fit.rs`, `cubicbez.rs`'s `inflections`/
`extrema`, `offset.rs`, `quadbez.rs`, `bezpath.rs`, `stroke.rs`). The wrapper snaps a discriminant
that is negative only within a `1e-12`-relative noise floor (matching the threshold already used
and verified in candidates 014/016's `solve_cubic` fix) to zero before delegating to
`solve_quadratic`, reporting the (numerically indistinguishable-from-real) double root instead of
silently dropping it:

```rust
fn solve_quadratic_factor(a: f64, b: f64) -> ArrayVec<f64, 2> {
    let disc = a * a - 4. * b;
    if disc < 0.0 && disc.abs() <= 1e-12 * (a * a).abs().max((4. * b).abs()).max(1.0) {
        let mut result = ArrayVec::new();
        result.push(-0.5 * a);
        return result;
    }
    solve_quadratic(b, a, 1.0)
}
```

`solve_quartic_inner` now calls `solve_quadratic_factor(*a, *b)` instead of
`solve_quadratic(*b, *a, 1.0)` directly. See `patch.diff` (against `kurbo/src/common.rs` only) and
`output.txt` for the actual `just test` run (fresh clone, before/after).

Verified:
- The primary repro (`repro/`) now returns 2 roots (`[6.466862768980423, 8.551950870236292]`,
  the two double-root midpoints, residual ~1.8e-12/~5.0e-12 against coefficients of magnitude
  ~O(1e3), i.e. relative residual ~6e-16-1.6e-15, machine-epsilon level) instead of 0, on all 5
  re-runs against the patched tree.
- Per `CONFIRMATION.md` section 7 (checked against more than the single triggering input): re-ran
  the full `arbtest` differential harness against the patched tree - the "two near-double-root
  pairs" mode's wrong-root-count rate dropped from 24,372/1,031,512 (~2.4%) to 1,420/1,031,512
  (~0.14%), a ~94% reduction, with the residual of roots that *are* returned staying at the same
  normal double-precision noise level as before (worst ~3.0e-13 relative).
- The dedicated false-positive check (genuinely-complex root pairs, `im` from ~1e-4 to ~1e-1)
  found **zero** cases (out of ~409,000) of the fix wrongly reporting a real double root for an
  actually-complex pair, at the chosen `1e-12` threshold.
- Project test suite (`cargo test --workspace`, in the isolated worktree at the pinned base
  commit): **252 passed, 0 failed, 1 ignored** (187 kurbo unit tests + 10 polycool unit tests + 55
  kurbo doctests + 1 ignored polycool doctest) - **identical before and after** the fix, no
  regressions, no new failures.

The fix is a genuine, verified improvement but is explicitly **not a complete fix** - see "What's
not done" below for the residual ~0.14% failure rate and why a fully adaptive (per-instance
error-bound) approach would be needed to close it entirely.

## What's not done / open questions

- The chosen `1e-12`-relative threshold is a fixed constant, not an adaptive per-instance error
  bound - the accumulated error from the `phi`/Newton-Raphson derivation chain can occasionally
  exceed it (observed disc magnitudes in remaining failures ranged up to ~4e-6 absolute against a
  scale of ~O(100), i.e. relative ~4e-8, well above `1e-12`). A tighter fix would use
  `factor_quartic_inner`'s own already-computed `eps_l`/`eps_t` residual-quality metrics (computed
  internally during quadratic-factor selection and Newton refinement, but currently not returned
  to callers) as a per-instance adaptive threshold instead of one fixed constant - this would
  require widening `factor_quartic_inner`'s or `solve_quartic_inner`'s return type/internal
  plumbing, judged out of scope for a single bounded-fix session. A threshold loose enough to
  eliminate the residual ~0.14% failure rate entirely (tested at `1e-8`) was measured to introduce
  real false positives (misreporting genuinely-complex root pairs as fake real double roots, at
  `im` as large as ~1e-3) - see the false-positive check above - so simply widening the constant
  further is not a safe option.
- Candidate 016's own scope note still applies in reverse here: this candidate's fix is scoped to
  the general (non-degenerate) path only (`solve_quartic_inner`'s own call site) - it does not
  touch `solve_quartic`'s `c0 == 0.0`/`c4 == 0.0` degenerate delegation branches, which candidate
  016 already covers via its own fix inside `solve_cubic` itself.
- Did not check whether any real caller (`CubicBez::tangents_to_point`, the crate's only
  `solve_quartic` caller) can be driven, via actual curve control points and a query point, into
  the specific "two near-double-root pairs" coefficient shape that triggers this - the repro uses
  directly-constructed coefficients (a legitimate repro per `CONFIRMATION.md` section 11, since
  `solve_quartic` is itself public and the coefficients are the genuine external input), but a
  concrete curve/point pair reproducing the same failure through `tangents_to_point` was not
  searched for this session (candidate 016's own directed search for its distinct mechanism took
  a nontrivial, dedicated search effort; doing the same for this mechanism's much narrower trigger
  condition - specifically two independently-near-tangent root pairs - was judged to not fit this
  session's remaining scope).
- The rarer "three clustered + one far" root-construction mode also triggers the same wrong-count
  defect (6-18 times per ~257,000 iterations, i.e. roughly 1,000x rarer than the "two pairs" mode)
  - not separately root-caused or minimized into its own repro this session, but almost certainly
  the same discriminant-sign-flip mechanism (one of the two quadratic factors ends up representing
  a near-double root from the three clustered values).
- The stray `factor=Some([(a, NaN), (a, NaN)])` output noticed once in the fuzz harness's own
  diagnostic printouts (a `beta` coefficient becoming exactly `NaN` for one input, unrelated to
  this candidate's own discriminant-sign mechanism - likely from the `beta_2 = d / beta_1` /
  `beta_1 = d / beta_2` rebalancing step in `factor_quartic_inner` dividing by an exact zero) was
  not investigated further this session - added to the backlog as a new, separate suspect.
