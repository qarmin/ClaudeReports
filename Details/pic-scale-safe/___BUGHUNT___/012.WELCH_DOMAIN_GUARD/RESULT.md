# welch missing abs() in domain guard

`welch` in `src/math/welch.rs:34-45` guards its domain with `else if x >= 1f32.as_()` (with no
preceding `x <= -1` check) instead of `if x.abs() >= 1f32.as_()`, so any `x <= -1` falls through to
the `1f32.as_() - x * x` branch and returns a large negative value instead of the expected `0.0`
outside the window's support. Flagged by a `code-audit-structured` pass
(`___BUGHUNT___/IDEAS.md`'s backlog) as mirroring the same `dx.abs()` pattern already fixed in the
sibling `kaiser` function (candidate 011, `Not reproducible`), with reachability via
`generate_weights`/resize call paths explicitly left as an open question for this candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction via temporary `#[cfg(test)]` probes added directly to `src/math/welch.rs` (reverted after) | debug build, `cargo test --lib math::welch::bughunt_probe -- --nocapture --test-threads=1`, run twice | `welch(-2.0)` returns `-3.0` instead of `0.0` (wrong value, reproduces 2/2); `welch(-1.0)` happens to coincidentally return `0.0` (`1 - (-1)*(-1) = 0`), so the exact boundary alone would not have exposed the bug - `-2.0` demonstrates it unambiguously |
| Static call-graph analysis (`grep`) + reading `src/compute_weights.rs`'s `generate_weights` (the sole caller of any `ResamplingFilter.kernel`, including `welch`) and `src/sampler.rs` (where `welch` is installed as a filter) | Read-only, no build needed | `welch` has exactly one call site in the whole crate (`ResamplingFilter::new(welch, 2f32)` at `src/sampler.rs:271`, `window: None`, `is_area_filter: false`). `generate_weights`'s `window_func == None` branch (the only branch ever reached for Welch, since `is_area_filter` is false) always calls `resampling_function(dx.abs() * filter_scale)` with `filter_scale = 1.0/scale.max(1.0)` - `scale` is a ratio of two `usize`-cast-to-`f32` values, which can never be negative, so `filter_scale` is always strictly positive (`(0,1]`), and `dx.abs()` is always non-negative - the product is always `>= 0.0`, for any `in_size`/`out_size`/scale-factor combination |

## Status

**Not reproducible** (through the crate's actual public API / any real call path). The
function-level defect was directly confirmed (2/2 repeat runs, debug build, in-crate unit test
exercising the `pub(crate)` function directly - see `output.txt`), but it is not reachable from
any public entry point: `welch`'s only caller (`generate_weights`) always passes `dx.abs()`
(non-negative) scaled by a strictly positive `filter_scale`, so `welch` never receives an `x <= -1`
through any real resize call.

No fuzz harness or sanitizer was used - this is a pure boundary-value/domain-invariant question,
which `HEURISTICS.md`'s tool-selection guidance covers with manual construction, not a
fuzzer/sanitizer (there's no adversarial byte stream here, just a scalar float domain), and the
soft 3-approach cap was not needed since two clean, structurally airtight approaches already
settled the question (no partial/ambiguous signal to keep digging on).

## Class

N/A (not reproducible through the public API - see below for the internal-only finding that would
apply if this invariant were ever changed).

## Root cause

`welch`'s guard is asymmetric: `x >= 1f32.as_()` only rejects large positive `x`. For `x <= -1`
(e.g. `x = -2.0`), execution falls through to `1f32.as_() - x * x`, which for `|x| > 1` is
negative (`1 - 4 = -3` at `x = -2.0`) - the opposite of the intended `0.0` outside the Welch
window's `[-1, 1]` support. This is a real, demonstrated wrong-value bug at the function level,
structurally identical to candidate 011's `kaiser` finding.

It is not reachable today because `generate_weights` (`src/compute_weights.rs`, `window_func ==
None` branch, the only code path that ever calls `welch`) always passes `dx.abs() * filter_scale`
with `filter_scale` strictly positive - and unlike candidate 011's `kaiser` case, this does not
even rely on the crate's own size/overflow validation already having run: `filter_scale`'s sign is
determined entirely by `scale = (in_size as f32) / (out_size as f32)`, and a `usize -> f32` cast
can never produce a negative value regardless of the magnitude of `in_size`/`out_size`, so
`filter_scale_cutoff = scale.max(1.0) >= 1.0` always holds and `filter_scale` is always in
`(0.0, 1.0]`. So `welch` only ever receives `x >= 0.0` through this call path - the `x <= -1`
branch that triggers the bug is unreachable independent of any input-validation edge case.

## The fix

No fix was applied - see "Status": there is nothing to demonstrate is fixed via the crate's public
API, since nothing public can trigger this defect today. A defensive, no-behavior-change hardening
(changing `else if x >= 1f32.as_()` to `else if x.abs() >= 1f32.as_()` in `welch`) would cost
nothing on the only live call path (where `x` is already `>= 0`) and would remove the latent hazard
for any future caller - but per this skill's workflow, a fix is only written into `patch.diff` for
a `Confirmed` finding reached through the actual candidate loop; this candidate is not writing one
since the bug does not reproduce through any real call path today. (No project test suite run was
needed since no source change was made.)

## What's not done / open questions

- The sibling `pic-scale` repo (`/home/rafal/test/pic-scale`, read-only reference, per
  `IDEAS.md`'s earlier note) was not checked for the same asymmetric guard in its own `welch`
  implementation.
- If a future change ever adds a second caller to `welch` (e.g. exposing it as `pub`, or a new
  window function reusing it directly rather than through `generate_weights`), this invariant
  should be re-checked rather than assumed to still hold.
- Not investigated: whether defensively fixing `welch` (symmetric `x.abs() >= 1.0` guard) would
  change any existing resize output for the `Welch` resampling function on the crate's current
  test/benchmark images - expected to be a no-op since the live input domain is unchanged, but not
  measured.
