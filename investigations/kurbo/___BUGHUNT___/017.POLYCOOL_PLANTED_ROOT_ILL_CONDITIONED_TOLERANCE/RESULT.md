# POLYCOOL_PLANTED_ROOT_ILL_CONDITIONED_TOLERANCE

`polycool`'s `poly::tests::planted_root_deg5` arbtest is flaky in CI (kurbo issue #594, CI seed
0x3180bc82000019a5). PR #599 (open, unmerged) "fixes" it by widening the root-separation used when
generating the test polynomial and loosening the acceptance tolerance to a fixed
`magnitude * 1e-6`, without touching the actual solver (`Poly::roots_between`/`yuksel::find_root`).
This candidate investigates the CI-reported seed, and whether the flakiness is a genuine
robustness gap in the degree-5 solver versus an unrealistic test tolerance, per the task brief.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct repro with CI-reported seed | `ARBTEST_SEED=0x3180bc82000019a5 cargo test --release poly::tests::planted_root_deg5` | Reproduces exactly the panic message reported in issue #594. |
| High-precision root/derivative check (mpmath, 60 decimal digits) of the exact failing polynomial | Independent of kurbo/polycool entirely | The polynomial has only **one real root** (the other 4 are two complex-conjugate pairs); the derivative of the polynomial at that real root is only ~6.3e-7 (poly magnitude ~8). The solver's found root has a function-value residual of ~2e-16 (f64 noise floor) but a *position* error of ~3.2e-10 from the true root - exactly explained by residual/derivative, i.e. the fundamental Newton-refinement precision limit for an ill-conditioned root, not a solver defect. |
| Applied PR #599's diff (`gh pr diff 599`) as-is, re-ran the exact CI seed | Same seed | Passes for that specific seed, but only because PR #599 also raised `root_separation` from 1e-6 to 1e-3, which changes how many bytes `poly_with_planted_root` consumes before succeeding, so the *same* seed now generates a *different* polynomial than before - this doesn't establish PR #599 actually fixes the underlying flakiness class. |
| Stress-test PR #599's patched test with `.budget_ms()` temporarily raised to 60_000 (12x CI's real 5_000), 6 back-to-back runs (~6 min of fuzzing) | `cargo test --release poly::tests::planted_root_deg5`, PR #599 applied, no fixed seed (real search/fuzz mode, same mode CI runs) | **2 of 6 runs still failed** with fresh seeds. PR #599 does not fix the flakiness, it only reduces its frequency. |
| Root-caused one of PR #599's own new failures (seed 0x5759368900010000) with the same mpmath cross-check | - | Derivative at the true root here is ~1.18e-11 (poly magnitude ~5.15) - **worse conditioned** than the original CI seed - producing a construction/refinement error of ~1.27e-5, which exceeds even PR #599's loosened `magnitude * 1e-6 ≈ 5.15e-6` tolerance. Confirms PR #599's fixed-tolerance approach is not conditioning-aware and can still fail for sufficiently ill-conditioned generated polynomials. |
| Root-caused a second PR #599 failure (seed 0x301077ae00010000) that also broke an initial 8-iteration-Newton-refinement fix attempt | - | `planted_root` here sits so close to a local extremum of the constructed polynomial (derivative ~1.4e-15, i.e. *at* f64 noise) that unconstrained Newton refinement from it diverges to a completely unrelated root ~0.02 away - a distinct failure mode from the derivative-too-small-but-not-degenerate case above, requiring the fix to bound refinement to a single, conditioning-gated step rather than iterate freely. |

## Status

**Confirmed** (as a test-only defect, not a defect in `polycool`'s public root-finding solver -
see Root cause). Reproduces 100% of the time for the CI-reported seed against un-patched `main`
(and, separately, 2/6 times in ~6 minutes of extended-budget fuzzing against PR #599's patched
version of the test, and once more with a fresh, non-cherry-picked seed against un-patched `main`'s
default 5s budget during this session - see `output.txt`).

**Fully reproducible**: **Yes**. `repro/` runs the actual, unmodified `cargo test -p polycool
poly::tests::planted_root_deg5` (the literal thing CI runs) against a real clone of the repository,
with `ARBTEST_SEED` pinned to three seeds known to reproduce the failure; this is a single-process,
single-participant test (a deterministic function of one arbitrary-generated input, no peer/caller
relationship to fake), so CONFIRMATION.md section 11's two-participant bar doesn't apply here -
running the crate's real test entry point against a real checkout already is the genuine trigger
path.

## Class

logic (test-only: a property-test's own construction/assertion is not conditioning-aware, not a
defect in the solver code it's testing)

## Root cause

`crate::arbitrary::poly_with_planted_root` builds the test polynomial by repeatedly multiplying
out linear factors `(x - root)` in `f64`. Because `Poly::eval`'s Horner evaluation, and this
repeated-multiplication construction, both operate in double precision, the polynomial's actual
(rounded) coefficients can have a *true* root measurably displaced from the nominal `planted_root`
value used to build it - by an amount that grows without bound as the polynomial's conditioning
worsens (i.e. as the derivative at that root shrinks, which happens when other generated roots -
real *or*, after rounding, spuriously complex - end up close to it). `float_in_unit_interval`'s and
`another_finite_float`'s bias towards near-repeated values makes this a common occurrence, not an
edge case (arbtest hit it within the CI-reported seed's very first few thousand search
iterations, and independently found 2 more instances in ~6 minutes of fuzzing PR #599's patched
version).

The original test's assertion (`(root - planted_root).abs() <= magnitude * 1e-12`) compares the
solver's output against this *nominal, unrefined* `planted_root` rather than against the
polynomial's actual true root, so any construction-time displacement directly counts against the
solver even though the solver isn't responsible for it. Independently verified against the
CI-reported seed's exact polynomial with 60-decimal-digit arbitrary precision (mpmath): the
solver's returned root has a function-value residual right at the f64 noise floor (~2e-16) and is
within ~3.2e-10 of the polynomial's *true* algebraic root - this is essentially the best any
double-precision Newton-type method (including the solver's own Newton-refinement step) can do
once the root's derivative is as small as ~6.3e-7 (residual/derivative ≈ position error, matching
observation almost exactly). **The solver is not missing or mislocating the root; it is finding it
about as precisely as f64 arithmetic allows.**

PR #599 (`gh pr diff 599`) partially addresses this by (a) raising `root_separation` from 1e-6 to
1e-3 (keeping *other* generated roots away from the *planted* one specifically) and (b) loosening
the acceptance check to a fixed `magnitude * 1e-6` as a secondary, value-residual-gated criterion.
This session found two independent ways it still isn't enough: (1) `root_separation` only bounds
distance from the planted root, not the distance between the *other* generated roots themselves,
so a cluster of non-planted roots can still make the true root near the planted one arbitrarily
ill-conditioned, producing errors bigger than the fixed `magnitude * 1e-6` bound (observed: 1.27e-5
against a 5.15e-6 bound); (2) even the *reference point* itself (a naive planted_root or a naive
multi-step Newton refinement of it) can become unreliable/divergent when the derivative at the
planted root is itself near the f64 noise floor.

## Introduced by

07860801016e492972292d1597de1dad570dbcb6 "Bring poly-cool into the kurbo repo (#502)" (2025-10-18)
- this commit is when `poly::tests::planted_root_deg5` and `arbitrary::poly_with_planted_root`
entered the kurbo repository's history verbatim (from the separately-developed `polycool` crate);
`git log -S` on both function names inside this repo finds no earlier or later change to their
core logic before this session's own patch.

## The fix

Applied directly to `polycool/src/poly.rs`'s `planted_root_deg5` test (see `patch.diff`):

1. Compute the derivative of `poly` at the *nominal* `planted_root` (`dy0`) and the corresponding
   intrinsic position uncertainty `noise_floor / |dy0|`, where `noise_floor = magnitude * 16 *
   f64::EPSILON` approximates the evaluation noise floor for `poly.eval`/`deriv.eval`. If `dy0` is
   zero, non-finite, or this intrinsic uncertainty exceeds `1e-6`, discard the generated input via
   `Err(arbitrary::Error::IncorrectFormat)` instead of asserting something floating-point
   arithmetic cannot deliver - this is the same idiom the test already uses for "polynomial
   coefficients too large" a few lines above. Gating on the derivative *at the un-refined
   `planted_root`* (rather than iterating first) is what avoids the divergence failure mode found
   in seed 0x301077ae00010000 (case 2 in "What was tried").
2. Otherwise, take a single Newton step from `planted_root` against `poly` itself
   (`true_root = planted_root - poly.eval(planted_root) / dy0`) to get a conditioning-appropriate
   estimate of the polynomial's *actual* nearby root, and compare the solver's roots against
   `true_root` instead of the raw `planted_root`.
3. Scale the acceptance tolerance to `(magnitude * 1e-12).max(intrinsic_uncertainty * 4.0)` - the
   original tight bound for well-conditioned cases, widened only as much as the conditioning at
   this specific root actually requires, rather than PR #599's fixed `magnitude * 1e-6` for every
   case.

Verified:

- All 3 known-bad seeds (the CI seed plus the 2 found in this session) pass or are legitimately
  rejected against the patched tree, and still reproduce against the un-patched tree - see
  `output.txt` and `repro/`.
- Stress-tested with `.budget_ms()` temporarily raised to 60_000 twelve times in a row (12 minutes
  of fuzzing, 10x CI's real per-run budget) against the patched tree: **12/12 passed, 0
  failures** (vs. 2 failures in 6 such runs for PR #599's version, and a fresh unprompted failure
  for un-patched `main` at the real 5s budget during the same session).
- Checked the fix doesn't make the test vacuous: instrumented the rejection branch temporarily
  (reverted before finalizing) and counted over one real 3-second search-mode run: 455 rejected /
  43,155 total generated inputs (~1.1%), so the assertion still meaningfully exercises ~99% of
  generated cases.
- `cargo test -p polycool --release`: **before, un-patched**: flaky (fails intermittently, as
  documented in issue #594 and reproduced fresh in this session); **after, patched**: 10/10
  passed, 0 failed across 10 consecutive runs at the real default 5s budget (`10 passed; 0
  failed` each time).
- `cargo test --workspace --release` (patched tree): 2 pre-existing, unrelated failures in the
  `kurbo` crate (`bezpath::tests::test_elements_to_segments_starts_on_closepath` and
  `bezpath::tests::test_must_not_start_on_quad`) - confirmed present identically against a clean,
  un-patched `git stash` of the same tree (i.e. not introduced by this patch; this failure
  corresponds to already-tracked candidate `004.SEGMENTS_LEADING_CLOSEPATH_PANIC` in this
  workspace). All other 185 kurbo tests, and all polycool tests, passed both before and after.

## What's not done / open questions

- This fix only addresses `planted_root_deg5` (`Poly<6>`). `polycool` doesn't currently have an
  equivalent planted-root arbtest for other degrees, so this class of issue (construction rounding
  outpacing a fixed test tolerance) wasn't checked elsewhere - if a similar planted-root test is
  added for another degree in the future, it should use the same conditioning-aware approach from
  the start rather than reintroducing this bug.
- The fix is scoped to the test's own assertion logic; it deliberately does not touch
  `Poly::roots_between`/`yuksel::find_root` (the actual solver), because this session's own
  high-precision cross-check found no evidence the solver itself is inaccurate beyond the
  irreducible f64-arithmetic limit for the given conditioning - see Root cause.
- Not raised upstream as a PR comment on #599 or a new issue - this workspace's own record is the
  deliverable per this skill's workflow; a maintainer reading this would still need to decide
  whether to take this approach over PR #599's simpler fixed-tolerance one (this fix is more
  code, but empirically closes 2 of the flaky-failure modes that outlived PR #599 in this
  session's testing).

## Later independent hits (cross-reference)

**2026-09-02** - hit again, incidentally, by a `perf-optimize` session
(`___OPTIMIZATIONS___/014.INLINE_SWEEP_CROSS_MODULE_HOT/`). No new candidate folder was created:
same test, same mechanism, same class as the confirmed root cause above - recorded here per
`perf-optimize/SKILL.md`'s duplicate rule.

Two details worth keeping from that hit:

- **One more failing seed for the un-patched tree**: `0xce849f7300010000` (time-derived, not
  cherry-picked) at the default 5s budget - `planted_root = -0.8650278750640445`, solver returned
  `-0.865027875706912`, i.e. a position error of **7.1e-10** against the original test's allowed
  `magnitude * 1e-12` = **7.06e-12**. Same shape as the CI seed (residual at the f64 noise floor,
  position error set by a tiny derivative at the root), three orders of magnitude over the bound.
  It failed on the very first, *un-patched* baseline run of that session and passed on re-run.
- **Operational consequence not previously written down**: because this is an `arbtest` with a
  time-derived seed, it makes `cargo test --workspace --all-features` **nondeterministic**, and
  without `--no-fail-fast` a failure *aborts the whole run*, so only 2 of the expected 4
  `test result` lines get printed. Any workflow that compares a before/after test summary (every
  `perf-optimize` candidate does exactly this) can therefore see a spurious "tests changed" signal
  that has nothing to do with its own patch. Until this is fixed upstream, run the workspace test
  suite with `--no-fail-fast` and compare the full set of 4 `test result` lines.
