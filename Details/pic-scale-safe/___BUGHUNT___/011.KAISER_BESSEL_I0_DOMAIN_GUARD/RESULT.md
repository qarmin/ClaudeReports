# kaiser / bessel_i0 asymmetric domain guard and unbounded loop

`kaiser` in `src/math/kaiser.rs:55-77` guards its domain with `if x > 1f32.as_()` instead of
`if x.abs() > 1f32.as_()`, so a negative `x < -1` falls through to
`bessel_i0(6.33 * (1.0 - x*x).sqrt())` with a negative `sqrt()` argument. Separately,
`bessel_i0` (`src/math/kaiser.rs:33-52`)'s `while t > 1e-12` convergence loop has no iteration
cap and no finite-check on `t`. Both looked suspicious from a code-audit pass (see
`___BUGHUNT___/IDEAS.md`'s prior entry) but were flagged as "not confirmed reachable via current
callers (`generate_weights` always passes `dx.abs()`)" - this candidate determines whether that
invariant actually holds and, separately, whether the two defects are real at the function level.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via temporary `#[cfg(test)]` probes added directly to `src/math/kaiser.rs` (reverted after) | debug build, `cargo test --lib math::kaiser::bughunt_probe`, each test run individually with an 8s `timeout`, `--test-threads=1` | `kaiser(-2.0)` returns `0.010997608` instead of `0.0` (wrong value, reproduces 2/2); `bessel_i0(NaN)` returns `1.0` immediately, 0 loop iterations (does **not** hang, refuting that half of the original suspicion, reproduces 2/2); `bessel_i0(1.0e30_f32)` hangs (times out at 8s both times it was run, 2/2 - genuine unbounded loop, confirmed by direct measurement); `kaiser(-1e20)` returns the same wrong `0.010997608` (routes through the NaN path above, not the hang path) |
| Static call-graph analysis (`grep`) + reading `src/compute_weights.rs`'s `generate_weights` (the sole caller of any `ResamplingFilter.kernel`, including `kaiser`) and `src/sampler.rs` (where `kaiser` is installed as a filter) | Read-only, no build needed | `kaiser` has exactly one call site in the whole crate (`ResamplingFilter::new(kaiser, 2f32)` at `src/sampler.rs:281`, `window: None`). `generate_weights`'s `window_func == None` branch always calls `resampling_function(dx.abs() * filter_scale)` with `filter_scale = 1.0/scale.max(1.0) ∈ (0,1]` - always non-negative, always finite for any input that already passed the crate's own size validation. `bessel_i0` has exactly two call sites, both inside `kaiser` itself, with the second call's argument mathematically confined to `[0, 6.33]` (valid `\|x\|<=1`) or exactly `NaN` (invalid `\|x\|>1`, `sqrt` of a negative) - never a large finite value of the shape that was shown to hang |

## Status

**Not reproducible** (through the crate's actual public API / any real call path). The two
underlying function-level defects were directly confirmed (2/2 repeat runs each, debug build,
in-crate unit tests exercising the `pub(crate)` functions directly - see `output.txt`), but
neither is reachable from any public entry point: `kaiser`'s only caller always passes
`dx.abs()` (non-negative) scaled by a strictly positive `filter_scale`, and `bessel_i0`'s only
caller (`kaiser` itself) never produces an argument large enough to overflow to `+inf` - the
argument is either bounded to `[0, 6.33]` or becomes exactly `NaN` (which, contrary to the
original suspicion, does not hang the loop - `NaN` comparisons are always `false` in Rust, so the
loop exits after 0 iterations).

No fuzz harness or sanitizer was used - this is a pure boundary-value/domain-invariant question,
which HEURISTICS.md's tool-selection guidance covers with manual construction, not a
fuzzer/sanitizer (there's no adversarial byte stream here, just a scalar float domain).

## Class

N/A (not reproducible through the public API - see below for the internal-only findings that
would apply if this invariant were ever changed).

## Root cause

Two independent, real defects exist in the code as written, but the crate's only call path
through `generate_weights` structurally prevents either from firing:

1. `kaiser`'s guard `x > 1f32.as_()` only rejects large positive `x`; for `x < -1`,
   `1.0 - x*x` is negative and `.sqrt()` produces `NaN`, and `bessel_i0(NaN)` (see point 2)
   returns `1.0` rather than propagating `NaN` or panicking, so `kaiser` silently returns
   `i0a` (a nonzero constant, `~0.011` for the default `alpha=6.33`) instead of the correct `0.0`
   for any `x` outside `[-1, 1]` on the negative side. This is a real, demonstrated wrong-value
   bug at the function level.
2. `bessel_i0`'s `while t > 1e-12` loop has no iteration cap and no finiteness check. If `t`
   ever becomes `+inf` (rather than `NaN`) - which happens when the *squared* input overflows the
   float type's range while the input itself is still finite - the loop never terminates, because
   `+inf` compares as `> 1e-12` forever and multiplying/dividing `+inf` by finite positive values
   keeps it `+inf`. This was directly measured to hang (2/2 timeouts). It is a real, demonstrated
   unbounded-loop bug at the function level, distinct from the `NaN` case (which, contrary to the
   original suspicion, self-terminates immediately).

Neither defect is reachable today because `generate_weights` (`src/compute_weights.rs`,
`window_func == None` branch, the only code path that ever calls `kaiser`) always passes
`dx.abs() * filter_scale` with `filter_scale` strictly positive and both operands finite for any
input that already passed the crate's own size/overflow validation - so `kaiser` only ever
receives `x >= 0`, and by the formula, `bessel_i0`'s second call site only ever receives a value
in `[0, 6.33]` or exactly `NaN`, never the large-finite shape that was shown to hang. This
"abs-before-call" invariant is genuinely load-bearing and structural (enforced by
`generate_weights`'s own code, not by convention or a caller-side check that could be skipped),
not accidental - but it depends entirely on `kaiser` continuing to have no other caller and on
`generate_weights`'s formula continuing to pass a bounded, non-negative argument, which a future
refactor could silently break without touching `kaiser.rs` itself.

## The fix

No fix was applied - see "Status": there is nothing to demonstrate is fixed via the crate's
public API, since nothing public can trigger either defect today. A defensive, no-behavior-change
hardening (changing `x > 1f32.as_()` to `x.abs() > 1f32.as_()` in `kaiser`, and adding an
iteration cap / finite-check to `bessel_i0`'s loop) would cost nothing on the only live call path
(where `x` is already `>= 0` and bounded) and would remove the latent hazard for any future
caller - but per this skill's workflow, a fix is only written into `patch.diff` for a `Confirmed`
finding reached through the actual candidate loop; this candidate is not writing one since the
bug does not reproduce through any real call path today. (No project test suite run was needed
since no source change was made.)

## What's not done / open questions

- The sibling `pic-scale` repo (`/home/rafal/test/pic-scale`, read-only reference) has the exact
  same `if x > 1f32.as_()` asymmetric guard in its own `kaiser` (`src/math/kaiser.rs:71-72`
  there), but delegates to a library Bessel I0 (`pxfm::f_i0`/`f_i0f`) rather than a hand-rolled
  loop, so the unbounded-loop half of this finding likely does not apply there - not
  independently verified, added to the backlog below.
- If a future change ever adds a second caller to `kaiser` or `bessel_i0` (e.g. a window function
  reusing them, or exposing them as `pub`), this invariant should be re-checked rather than
  assumed to still hold - it depends on the exact shape of `generate_weights`'s current formula,
  not on any doc comment or type-level guarantee.
- Not investigated: whether making `kaiser`/`bessel_i0` defensively correct (symmetric guard +
  loop cap) would change any existing resize output for the `Kaiser` resampling function on the
  crate's current test/benchmark images - expected to be a no-op since the live input domain is
  unchanged, but not measured.
