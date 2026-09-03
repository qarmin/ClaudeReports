# FIT_TO_CUBIC_GENERIC_COARSE_ACCURACY_SWEEP

A systematic pass over every `fit_to_cubic`/`cubic_fit`/`fit_to_bezpath_opt_inner` call site in
`kurbo/src/fit.rs`, as an explicit follow-up to candidate 026's own broader (but not declared
exhaustive) sweep, using a *different* class of source curve (a smooth parametric curve, not a
polyline) than any prior candidate (008/009/026) fuzzed. It confirms and demonstrates candidate
026's own explicitly-flagged open question - "a candidate whose control arm is even shorter [than
026's `MAX_ARM_LEN = 3.0`] but still manages to cheat the discrete sample check... is not ruled out
by either fix and was not found or specifically searched for this session" - with a concrete,
reproducible trigger.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual read of every `fit_to_cubic`/`cubic_fit`/`fit_to_bezpath_opt_inner` call site (the same four sites candidate 026 already enumerated: `fit_to_bezpath_rec` line 194, the initial single-cubic check and the (candidate-008-fixed) `.unwrap()` build loop in `fit_to_bezpath_opt_inner`, and `measure_one_seg` used by the search-pass helpers), specifically re-checking whether the two-pass "tightened accuracy `x` during search vs. final `accuracy` during build" pattern from 008/009 recurs anywhere not yet fixed | Manual code reading | Confirmed no *additional distinct call site* has that exact two-pass shape beyond the two already fixed by 008/026. This matches 026's own conclusion and motivated broadening the search to a different *source curve class* instead, per 026's own stated open question |
| Short property-fuzz harness (`kurbo/examples/fuzz045.rs`, deleted before finalizing; debug then release build) generating random smooth degree-4-polynomial `ParamCurveFit` sources (`y = amp * P((x/xscale) - 0.5)`, `x` linear in `t` - the same public-API shape as `kurbo/examples/fit_poly.rs`'s own `MyPoly`, not a polyline), fed through the public `fit_to_bezpath_opt`/`fit_to_bezpath` entry points, checking for panics and for a returned path control point straying far from a dense-sampled-and-locally-refined nearest-point oracle against the true source curve | Release build, `cargo run --example`, 2,000-300,000 iterations against a tree with 008's and 026's patches pre-applied (to avoid rediscovering those two already-recorded defects) | An early version (curves with both `x(t)` and `y(t)` as arbitrary degree-4 polynomials directly in `t`) produced a hang/huge-memory run and, after fixing that, many spurious "BAD_FIT" reports - both were the harness's own artifacts (see next two rows), not kurbo bugs. After correcting the harness: 30/200,000 genuine `BAD_FIT` reports, e.g. seed 172491: control point at x = -0.411 against a source curve whose entire valid x-domain is [0, 0.445] - a control point escaping by ~92% of the curve's own domain width, for an accuracy of 0.000546 (escape ~750x the requested accuracy) |
| Diagnosed the early hang: an earlier curve generator let `x(t)` itself be an arbitrary degree-4 polynomial in `t`, which for large `xscale` made the curve's own natural coordinate range blow up past 1e9; `fit_to_bezpath_opt`'s subdivision search then needed enormous segment counts to meet a (relatively) tight accuracy against that blown-up range, taking minutes/many GB and never being a real kurbo defect | Process inspection (`ps aux`, RSS ~12GB after several minutes) confirmed the hang was real CPU/memory use in the fuzz binary, not a harness bug proper - fixed by keeping `x(t)` linear in `t` (matching `fit_poly.rs`'s own `MyPoly` shape) and scaling `accuracy` relative to the curve's own scale, not independently | Not a kurbo defect - a self-inflicted test-generator pathology in this session's own harness, corrected before continuing |
| Diagnosed the early spurious `BAD_FIT` reports: the harness's original oracle (dense 400-point uniform sampling of the source curve's own `t` range to find the nearest point to a given output-path point) under-resolved the source curve's own fine structure whenever the curve had a large dynamic range, producing distances that were sampling-resolution artifacts, not real fitting errors | Rebuilt the oracle as coarse-dense-sample-then-local-ternary-search refinement around the best coarse sample; re-ran the same 2,000-iteration batch that previously showed dozens of near-threshold "BAD_FIT"s (ratios ~10-13x the reporting threshold, consistent with sampling-resolution noise) | 0 failures after the refinement, on the same random seeds - confirms those were oracle artifacts, not real bugs, before any of the 30 genuine failures below were trusted |
| Standalone repro of the smallest/most striking genuine failure (seed 172491) as its own binary (`repro/`, kept), run 3x against a fresh clone of vanilla HEAD (no 008/026 patches needed or involved) | Release build, real clone at ca27349 | Reproduces identically (same escape, same coordinates) 3/3 runs on plain vanilla HEAD |
| Instrumented `fit_to_cubic`'s per-candidate loop with a temporary `eprintln!` (reverted before finalizing) printing `d0`/`d1`/`raw_err2`/`scaled_err2`/`acc2`/`spicy` for every candidate considered, re-run against the minimized repro | Same debug build | Exactly one candidate was generated for the whole `[0,1]` range and accepted immediately (no subdivision at all): `d0 = 0`, `d1 = 1.9226` (well under both candidate 009's `MAX_ARM_LEN = 10.0` and candidate 026's `MAX_ARM_LEN = 3.0`), `spicy = false`, `scaled_err2 ~= 4.8e-9` against `acc2 ~= 2.99e-7` |
| Repeat-run the minimized repro 3 times against the same pristine clone | Release build, no RNG/seed involved | Bit-for-bit identical output all 3 runs |
| Hypothesis test: since `spicy = false`, only the cheap one-sided `eval_ray` metric was ever computed for this candidate (the more robust arc-length-parametrized `eval_arc` cross-check was never invoked, because `spicy` reflects only the *source* curve's own tangent variation, not the *candidate*'s). Patched `CurveDist::eval_dist` to also force the `eval_arc` check whenever the *candidate*'s own control arm exceeds `D_PENALTY_ELBOW` (0.65), independent of the source's `spicy` flag, and re-ran the same repro | Debug then release build | Escape eliminated: the wild single-segment candidate is now rejected by the arc-length check, forcing the expected multi-segment subdivision; `repro/` reports "did not reproduce" |
| Re-ran the fix without also re-tightening `MAX_ARM_LEN` back up (testing the new `long_arm` mechanism in isolation, with `cubic_fit`'s own arm-length filter left unbounded) | Release build, `FUZZ_ITERS=20000` | Did not complete within 60s (previously ~0.6ms/iteration with `MAX_ARM_LEN` capped) - the new `eval_arc` fallback is now reached far more often (most legitimate candidates already have `d0` or `d1` above the low `0.65` threshold) and `cubic_fit`'s own quartic solver can still return roots with enormous (hundreds-of-chord-lengths) arm lengths absent a cap, for which `eval_arc`'s `arclen`/`inv_arclen` calls become considerably more expensive to converge - a genuine performance regression, not a correctness one |
| Re-added an absolute `MAX_ARM_LEN = 10.0` cap in `cubic_fit` (candidate 009's original, less aggressive value - chosen here purely as a performance bound on how wild a candidate can be before reaching the new, more expensive `eval_arc` fallback, not as the primary correctness mechanism) alongside the `long_arm` fix, and re-ran the broader fuzz harness | Release build, `FUZZ_ITERS=200000`, `FUZZ_ITERS=300000` | 0 failures in both runs; performance restored (200,000 iterations in ~26s, comparable to the pre-fix baseline) |
| Re-ran the polyline-sourced property fuzz (same shape as candidate 026's own harness, `kurbo/examples/fuzz045_poly.rs`, deleted before finalizing) both before and after this candidate's patch, to check for regressions in the already-covered polyline case | Release build, `FUZZ_ITERS=150000`, both against a tree with only 008/026's patches and against a tree with this candidate's patch added | Identical result both times: 14 failures, same seeds, same escape magnitudes (residual, much-smaller-scale cases - see "What's not done") - confirms no regression and no incidental fix from this candidate's own patch for that residual class |

## Status

**Confirmed** - reproduces 3/3 runs, deterministic, release build, on a fresh clone of vanilla
HEAD (no 008/026 patches needed or involved).

**Fully reproducible**: **Yes**. `repro/` calls the real public `fit_to_bezpath_opt` entry point
on a real, ordinary `ParamCurveFit` implementation (the same public-API shape as
`kurbo/examples/fit_poly.rs`'s own `MyPoly`) - the kind of input a genuine caller fitting a smooth
parametric curve would pass in. There is no second participant to fake here (a single-process pure
function on a specific curve/accuracy combination, not a peer/protocol interaction), so
`CONFIRMATION.md` section 11's two-participant recipe does not apply.

## Class

logic

## Root cause

Same underlying blind spot as candidates 009/026 (`CurveDist::eval_ray`'s one-sided
nearest-ray-intersection check can be fooled by a candidate whose control arm swings away from the
chord and back between sample points), but reached through a different, previously-unexamined
gate: `CurveDist::eval_dist` only ever falls back to the more expensive, more robust
`eval_arc` (arc-length-parametrized) check when `self.spicy` is set - and `spicy` is computed
purely from the *source* curve's own tangent variation across `N_SAMPLE` samples (see
`CurveDist::from_curve`), never from the *candidate* cubic actually being evaluated. A smooth,
non-polyline source curve (low tangent variation, `spicy = false`) can still cause `cubic_fit`'s
quartic solver to return a candidate with a control arm long enough to fold back on itself (in this
repro, `d1 ~= 1.92` chord lengths) while still crossing every one of `eval_ray`'s per-sample rays
close to the correct point - so the reported error is spuriously tiny (`~4.8e-9` against a target
of `~2.99e-7`) and the whole `[0,1]` range is accepted as a single cubic with no subdivision at
all, even though the accepted control point sits almost entirely outside the source curve's own
valid coordinate domain.

This is a genuinely different trigger from 009/026's polyline-sourced repros: this candidate's
control-arm length (~1.92) is comfortably *under* both candidate 009's `MAX_ARM_LEN = 10.0` and
candidate 026's tighter `MAX_ARM_LEN = 3.0` - neither of those two already-recorded, arm-length-cap
fixes rejects it, because both were tuned only against polyline-sourced repros where the observed
"legitimate" arm-length range happened to stay under 3. Candidate 026 itself explicitly flagged
this exact gap as an open, unsearched question ("a candidate whose control arm is even shorter...
but still manages to cheat the discrete sample check... is not ruled out by either fix"); this
candidate is that search, carried out against a different source-curve family, and it closes with
a concrete, reproducible confirmation.

## Introduced by

721d9e4 "Checkpoint hybrid approach" (2023-04-14) - this is the commit that introduced the
`spicy`/`eval_ray`/`eval_arc` split itself (`CurveDist::from_curve`'s tangent-variation-based
`spicy` computation, and `eval_dist`'s `if !self.spicy { return Some(ray_dist); }` early return),
i.e. the commit that first coupled "should the more expensive/robust check run" to a property of
the *source* curve alone, with no corresponding check on the *candidate* cubic actually being
scored. Found via `git log --follow` over `fit.rs`'s full history plus `git log -S"eval_dist"`/
`git show` on the resulting candidate commits (`34ace6c`, `c5e0f5a`, `721d9e4`), then reading each
commit's diff directly to confirm which one introduced the `spicy`-gated branch specifically (the
two earlier commits only introduce `eval_dist`/`eval_arc` themselves, unconditionally, before the
hybrid gating existed).

## The fix

Applied directly to `kurbo/src/fit.rs`, as a standalone patch against vanilla HEAD (does not
depend on, and was not combined with, candidates 008's or 026's own recorded patches - all
verification above used this candidate's own patch alone unless a row says otherwise):

1. `CurveDist::eval_dist` gains a `long_arm: bool` parameter; the early-return-to-`eval_ray`-only
   path now requires *both* `!self.spicy` (the existing source-curve check) *and* `!long_arm` (a
   new candidate-cubic check) - i.e. the more robust `eval_arc` fallback now runs whenever *either*
   the source is spicy *or* the specific candidate's own control arm is long, closing the gap
   `spicy`-alone left open.
2. In `fit_to_cubic`'s per-candidate loop, `long_arm` is computed as `d0 > D_PENALTY_ELBOW || d1 >
   D_PENALTY_ELBOW` - reusing the same `D_PENALTY_ELBOW = 0.65` threshold the existing penalty
   mechanism already uses to start penalizing long arms, so no new tuning constant is introduced
   for the correctness-relevant threshold.
3. `cubic_fit`'s own long-flagged "We could implement a maximum d value here" comment is finally
   acted on with an absolute `MAX_ARM_LEN = 10.0` cap (candidate 009's original, less aggressive
   value, chosen here purely as a **performance** bound - see "What was tried" above for why
   omitting it made the new `eval_arc`-triggering mechanism considerably slower on pathological
   candidates, not incorrect). The actual correctness fix is (1)/(2); this cap only bounds how
   expensive evaluating a rejected-anyway candidate can get.

Verified:

- `repro/` (the minimized seed-172491 case): before the fix, a control point at x = -0.411 escapes
  the source curve's own valid x-domain of `[0, 0.445]` by ~0.411 units, ~92% of the domain's own
  width (exit code 1); after the fix, the same call produces a well-behaved multi-segment fit with
  every control point inside `[0, 0.445]` (exit code 0). See `output.txt`.
- Varied beyond the one triggering input: the same property-fuzz harness (random smooth degree-4
  polynomials, accuracy scaled relative to each curve's own magnitude), re-run with only this
  candidate's own patch applied to plain vanilla HEAD, found 0 failures in 300,000 iterations - both
  the one specific repro and the broader class it represents are addressed, at restored
  performance (~26s for 200,000 iterations, comparable to the pre-fix baseline).
- No regression on the already-covered polyline case: the same 14 residual (much-smaller-scale)
  failures appear identically in the polyline-sourced property fuzz both with and without this
  candidate's patch (150,000 iterations each) - this patch neither fixes nor worsens that separate,
  already-flagged-as-still-open class (see "What's not done").
- Project test suite (`cargo test --workspace`, run in this worktree, before and after `git apply
  patch.diff`):
  - Before: kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0 failed;
    combined doctests 55 passed / 0 failed (1 ignored, pre-existing/unrelated).
  - After: kurbo lib tests 187 passed / 0 failed; polycool lib tests 10 passed / 0 failed;
    combined doctests 55 passed / 0 failed (1 ignored) - identical counts, no regressions.

## What's not done / open questions

- **This candidate's `patch.diff` re-introduces a `MAX_ARM_LEN` constant at the same location in
  `cubic_fit` that both candidate 009 (`MAX_ARM_LEN = 10.0`) and candidate 026 (`MAX_ARM_LEN =
  3.0`) already independently proposed** - by this workspace's design none of these three patches
  are ever applied for real to the main project, so this isn't a conflict for the record itself,
  but a maintainer taking this candidate's patch should treat its `MAX_ARM_LEN = 10.0` purely as
  the performance-bound companion to the `long_arm`/`eval_dist` mechanism (which is this
  candidate's actual, new correctness contribution), not as an alternative recommendation for
  026's own tighter `3.0` value - the two constants are answering different questions (arm-length
  cap as the *sole* correctness gate, vs. arm-length cap as a cost bound on top of a
  candidate-aware robustness check) and a maintainer combining all three candidates' ideas would
  reasonably want 026's tighter cap *and* this candidate's `long_arm` mechanism together, not this
  candidate's own `10.0` choice specifically.
- **A residual, smaller-magnitude class of the same `eval_ray` blind spot still exists on
  polyline-sourced input, unaffected by this candidate's patch**: the same polyline property fuzz
  used by candidate 026 (`n=3..40` vertices, scale up to ~1e7) found 14/150,000 failures with
  escapes on the order of 100-1000 units against bounding boxes of ~100,000-400,000 units (roughly
  0.1-0.3% of bbox scale, ~1000x the requested accuracy) - both with and without this candidate's
  patch, i.e. genuinely unaddressed by any of 008/009/026/045's fixes combined. These are
  substantially smaller-magnitude escapes than the ones already fixed (candidate 009's ~558-chord
  arm, this candidate's ~92%-of-domain escape), and were not minimized, root-caused, or fixed this
  session - flagged here as a concrete lead for a future candidate rather than investigated
  further, to keep this candidate's own scope to the one bug it found and fixed.
- **The broader "is this sweep now exhaustive" question remains open**, exactly as candidate 026
  left it: this candidate closes one more instance of the same general blind-spot class (found via
  a different source-curve family than 008/009/026 used), and the residual polyline-side failures
  noted above show the class is still not fully closed even with all four candidates' fixes
  combined. A more targeted structural fix to `CurveDist` itself (checking the *candidate*'s own
  shape unconditionally, e.g. always computing `eval_arc` as a cross-check regardless of any
  threshold, or adding intermediate sample points between the existing `N_SAMPLE` ray-cast
  locations) was not attempted this session; a longer, dedicated fuzzing campaign (via
  `fuzz-bug-repro`, run separately) against `fit_to_bezpath_opt` with a wider variety of
  `ParamCurveFit` source shapes (not just polylines and low-degree polynomials - e.g. curves with
  actual cusps, or curves derived from stroke/offset expansion as `fit.rs`'s own module doc
  mentions as a major motivation) would be a reasonable next step if a maintainer wants stronger
  assurance before considering this class of defect closed.
- The `long_arm` threshold reuses `D_PENALTY_ELBOW = 0.65` rather than introducing a new constant;
  this was a deliberate choice to avoid an additional untuned magic number, but it does mean
  `eval_arc` (the more expensive check) now runs for a substantial fraction of ordinary candidates
  (most legitimate fits have at least one arm above `0.65`), not just pathological ones - the
  ~26s/200,000-iterations figure above reflects that cost; it was not benchmarked against the
  project's own stroke-expansion/offset-curve workloads (`fit.rs`'s own doc comment names these as
  a "major motivation" for `ParamCurveFit`), which were not exercised by this session's fuzzing at
  all.
