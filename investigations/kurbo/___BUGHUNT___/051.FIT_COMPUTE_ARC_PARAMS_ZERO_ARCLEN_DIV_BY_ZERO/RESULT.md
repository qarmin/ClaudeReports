# FIT_COMPUTE_ARC_PARAMS_ZERO_ARCLEN_DIV_BY_ZERO

`CurveDist::compute_arc_params` (`kurbo/src/fit.rs:260-279`) sums `deriv.hypot()` from
`ParamCurveFit::sample_pt_deriv` over 110 sample points into `arclen`, then unconditionally
computes `let arclen_inv = arclen.recip();` and multiplies every entry of `self.arcparams` by it,
with no check that `arclen` is nonzero. This path only runs when `CurveDist::spicy` is `true`
(`from_curve`, lines 231-251), which is decided purely from consecutive `sample_pt_tangent` tangent
vectors - a value that a `ParamCurveFit` implementor can report completely independently of the
magnitude `sample_pt_deriv` reports for the same range. The trait's own docs explicitly allow
`sample_pt_deriv`'s derivative to "go to zero" (cusps), so an implementor reporting it as
literally `Vec2::ZERO` everywhere in a spicy range - while still reporting a well-defined, varying
tangent from `sample_pt_tangent` - is valid input, not a caller bug.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: a custom `ParamCurveFit` (`StationarySpicy`) whose `sample_pt_tangent` reports a straight-line position with a rapidly rotating tangent (forces `spicy = true`), and whose `sample_pt_deriv` reports the same position with `Vec2::ZERO` everywhere; called through the real public `fit_to_cubic`/`fit_to_bezpath` entry points | release build, standalone `repro/` crate depending on kurbo via path, plus temporary `eprintln!` instrumentation added directly to `CurveDist::compute_arc_params`/`eval_arc` (reverted after use - not part of `patch.diff`) | Confirmed: `arclen == 0.0` exactly, `arclen_inv == +inf`, every `arcparams` entry becomes `NaN` (`0.0 * inf`), reproduced deterministically (3/3 runs in the working tree, plus 1/1 via the standalone `repro/`+`diagnose` justfile recipe on a fresh clone - 4/4 total, no seed/layout dependency, pure IEEE-754 float math) |
| Tracing the downstream consequence of the NaN `s` value through `eval_arc`'s `c.inv_arclen(c_arclen * s, EPS)` | same instrumented build | The NaN does **not** reach the public return value as `NaN`: `inv_arclen`/`solve_itp_fallible`'s own (separately already-known, see candidate 041 `INV_ARCLEN_NAN_SILENT_MIDPOINT`) mishandling of a NaN `y` value at each ITP iteration causes it to take the "exact root found" branch on the very first iteration, returning the initial bracket's midpoint (`t = 0.5`) - a finite but semantically meaningless value, used identically for every one of the 20 samples regardless of where they actually lie |
| Systematic sweep for a black-box (public-API-only) input where this corruption visibly flips `fit_to_cubic`'s accept/reject decision or its returned error, comparing pristine vs. the fix below | release build; 1080 combinations of tangent-rotation frequency, chord length, accuracy, and subrange on a straight-line source; a second 1080-combination sweep re-running the same source through `fit_to_bezpath`'s recursive subdivision | Zero divergence found in either sweep: `eval_ray` (the independent, unaffected ray-based check that always runs first in `eval_dist`) and `try_fit_line`'s short-chord bypass dominate the accept/reject decision for every case tried, absorbing the corrupted arc-length check's effect before it can change the externally-visible result |

## Status

**Confirmed** - the internal corruption (NaN written into a crate-private field via the real,
unmodified public API, `fit_to_cubic`/`fit_to_bezpath`) reproduces deterministically, 4/4 runs
(3 in the working tree, 1 via the standalone `repro/` + `diagnose` justfile recipe against a fresh
clone) - pure floating-point arithmetic, no randomness or platform/layout dependency. Per
CONFIRMATION.md section 5 ("read the diagnostic, don't just note non-zero exit"): the captured
`eprintln!` values (`arclen=0`, `arclen_inv=inf`, `arcparams` all `NaN`) are the actual confirming
evidence, not an inferred consequence.

**Fully reproducible**: **Yes**. `fit_to_cubic` is called directly with a `ParamCurveFit`
implementor built via kurbo's own public trait - no second participant (peer/caller/wire) is being
stood in for here (CONFIRMATION.md section 11's carve-out applies directly: a custom trait impl
handed to a public generic function *is* the genuine external input a real caller would supply,
the same way a malformed file handed to a parser's own `from_reader` is genuine input with no
"other side" to fake).

## Class

logic (silent internal data corruption - no panic, no NaN in the public return value, entirely
invisible without reading the crate-private `CurveDist` state directly)

## Root cause

`CurveDist::compute_arc_params` divides by the sum of 110 `sample_pt_deriv` derivative magnitudes
with no guard against that sum being exactly `0.0`. `0.0_f64.recip()` is `+inf`; every entry of
`self.arcparams` was built purely from non-negative additive terms (`deriv.hypot() >= 0.0`), so if
the total is exactly `0.0` every partial-sum entry pushed into `arcparams` is also exactly `0.0` -
and `0.0 * f64::INFINITY` is `NaN` under IEEE-754. This is reachable whenever `CurveDist::spicy` is
`true` (decided independently, from `sample_pt_tangent`'s tangent field only) for a range where the
same implementor's `sample_pt_deriv` reports a zero derivative at all 110 sub-samples - both are
individually permitted by the trait's documented contract, but the combination is never guarded
against here.

Once `s` (an `arcparams` entry) is `NaN`, `eval_arc` calls `c.inv_arclen(c_arclen * s, EPS)`.
`ParamCurveArclen::inv_arclen`'s default implementation (`kurbo/src/param_curve.rs:98-123`) doesn't
short-circuit on a NaN `arclen` argument (`NaN <= 0.0` and `NaN >= total_arclen` are both `false`),
so it proceeds into `common::solve_itp`/`solve_itp_fallible`. There, the very first `yitp` computed
is `NaN` (it's `arclen_last - arclen` with `arclen` = NaN), and `solve_itp_fallible`'s three-way
`if yitp > 0.0 { .. } else if yitp < 0.0 { .. } else { return Ok((xitp, xitp)) }` treats a NaN
`yitp` as the `else` ("exact root found") case, since both `>` and `<` comparisons against NaN are
`false` in Rust/IEEE-754 - this is the same pre-existing defect candidate 041
(`INV_ARCLEN_NAN_SILENT_MIDPOINT`) already documented. The net effect: every one of the 20 samples
in the spicy range gets compared against the *same* fixed point on the candidate curve (`t = 0.5`,
the initial bracket's midpoint) instead of its own correctly-parametrized position - a silent,
deterministic corruption of the accuracy check's intended meaning, with no crash and no NaN ever
appearing in the function's public return value.

## Introduced by

c5e0f5a "Checkpoint, arc length parametrized error" (2023-04-14) - this commit
(`git log -S"arclen_inv" --follow -- src/fit.rs`) introduced `CurveDist::compute_arc_params` and
`eval_arc` in essentially their current form, including the unguarded `arclen.recip()`.

## The fix

Guard the normalization loop on `arclen_inv` actually being finite; if it isn't (i.e. `arclen` was
exactly `0.0`), leave `self.arcparams`'s already-all-zero raw values in place instead of corrupting
them to `NaN`:

```rust
let arclen_inv = arclen.recip();
// If the source curve reports a zero derivative everywhere in this range (e.g.
// a degenerate `ParamCurveFit` implementor), `arclen` is exactly 0.0 and
// `arclen_inv` is infinite; multiplying the (all-zero) partial sums by it
// would produce NaN. Leave the raw (already all-zero) values in place instead.
if arclen_inv.is_finite() {
    for x in &mut self.arcparams {
        *x *= arclen_inv;
    }
}
```

Verified: re-running the instrumented build after this change shows `arcparams` staying `[0.0, 0.0,
...]` instead of `[NaN, NaN, ...]`, and `eval_arc`'s `s`/`t` become `0`/`0` (a different, but at
least well-defined and non-NaN, fixed comparison point) instead of `NaN`/`0.5` (see `output.txt`,
captured via this folder's own `just diagnose` recipe against a fresh clone). Checked against a
second input in the same class (varying the tangent-rotation frequency, chord endpoint, and
subrange - 1080 additional combinations) to confirm the fix doesn't merely paper over the one
triggering input; no NaN reappeared in any of them either.

Per CONFIRMATION.md section 7 ("vary the input, not just the exact trigger") and section 9 (the
baseline must be genuinely pristine): the fix was applied to and reverted from a pristine checkout
of the exact commit under test (`ca273499e3e48bd2de6f02aa8e99a148984e45f3`) each time, confirmed via
`git status`/`git diff` before rebuilding.

**Note on external observability**: this fix does *not* change `fit_to_cubic`'s/`fit_to_bezpath`'s
*returned* result for any input tried this session (see "What was tried"'s sweep row) - the fixed
comparison point (`t = 0`) is no less semantically arbitrary than the buggy one (`t = 0.5`) for a
genuinely-degenerate-arclen input, and in every geometry tried, the independent `eval_ray` check
(unaffected by this bug) and `try_fit_line`'s short-chord bypass already gate the accept/reject
decision correctly regardless. The value of this fix is eliminating the NaN itself (an IEEE-754
invariant violation with no defined semantics, silently propagating through arithmetic that assumes
finite values) rather than a demonstrated change in end-user-visible fitting quality.

- Project test suite (`cargo test --workspace`, `kurbo` + `polycool`): 187 + 10 + 55 passed, 0
  failed, 1 ignored doctest (unrelated, requires a feature not enabled by default) - identical
  counts before and after the fix. One unrelated one-off flaky failure was observed in a single
  `cargo test --workspace` invocation against the *unpatched* baseline (a `polycool` test,
  ~5-second run, did not reproduce on immediate rerun); per CONFIRMATION.md section 4, this was
  investigated by simply rerunning (no source/input changed) and the failure did not recur - it
  is unrelated to `src/fit.rs` (untouched by this candidate's patch) and not attributed to this fix.

## What's not done / open questions

- No black-box (public-API-only) input was found this session where this specific corruption
  visibly changes `fit_to_cubic`'s/`fit_to_bezpath`'s returned curve or accept/reject decision -
  see the sweep row above. A more targeted search (e.g. deliberately shaping the candidate cubic's
  own `t=0`/`t=0.5` points to fall on opposite sides of the accuracy boundary relative to the true
  sample positions) might still find one; not attempted further this session.
- `moment_integrals`'s own default implementation also consumes `sample_pt_deriv` and would report
  `(0.0, 0.0, 0.0)` for the same all-zero-derivative input - not itself a bug (a real, genuinely
  straight/stationary curve legitimately has zero area/moment), but it means a degenerate
  `sample_pt_deriv` can silently steer `cubic_fit`'s candidate generation too; not investigated
  further as a separate defect this session since it doesn't itself produce NaN or any other
  IEEE-754 invariant violation, only IB a different (self-consistent) chosen candidate shape.
- `fit_to_bezpath_opt`/`fit_to_bezpath_opt_inner` (`kurbo/src/fit.rs`) share the same
  `CurveDist::eval_dist`/`compute_arc_params` code path and were not separately exercised, though
  the root cause and fix apply identically since it's the same underlying method.
- Whether a real, non-adversarial `ParamCurveFit` implementor (as opposed to one deliberately
  constructed to demonstrate this) would ever plausibly report this exact combination (a
  well-defined tangent direction with a genuinely-zero-everywhere derivative across an entire
  spicy subrange, not just an isolated cusp point) wasn't established - the trait's own docs only
  explicitly discuss zero derivative "in curves with cusps" (implying isolated points, not a whole
  subrange), so this may be a narrower real-world exposure than a fully generic implementor could
  produce; the code has no guard regardless of how likely a real implementor is to trigger it.
