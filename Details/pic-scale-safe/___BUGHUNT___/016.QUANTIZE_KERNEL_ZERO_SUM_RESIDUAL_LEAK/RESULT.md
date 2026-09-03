# `quantize_kernel` zero-sum residual redistribution follow-up

`src/filter_weights.rs`'s `quantize_kernel` (used by `FilterWeights::<f32>::numerical_approximation_i16`,
the sole bridge from float weights to the `i16` fixed-point weights the 8/16-bit integer resize
entry points use) redistributes leftover quantization residual onto the largest-magnitude taps.
Candidate 010's session only spot-checked one input (`HaasnSoft 1x1->100x1`) and observed this
happening to reconstruct a working nearest-sample fallback for an all-zero-weight row - flagged as
unproven in general, since a stable sort of all-tied-at-zero taps landing the whole residual on
tap 0 was only demonstrated once.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: a temporary in-crate `#[cfg(test)]` probe calling `quantize_kernel` directly with an all-zero 4-tap row at the crate's real `PRECISION = 15` constant (`precision_scale = 32768.0`) | `cargo test --lib`, debug build | Live signal found immediately: `scratch` ends up `[32767.0, 1.0, 0.0, 0.0]`, not `[32768.0, 0.0, 0.0, 0.0]` as candidate 010 assumed - `precision_scale` (32768) exceeds `i16::MAX` (32767) by exactly 1, so the redistribution loop caps tap 0 at `upper_bound` and is forced to spill the leftover 1 unit onto the *second* tap (`order[1]`) on its next iteration. Confirmed this is deterministic and independent of tap count (re-derived analytically: residual is always exactly `precision_scale` for a fully zero-sum row regardless of how many taps there are, so the leak is always exactly 1 unit, always onto exactly one extra tap) - kept digging on whether this is externally observable |
| Manual boundary-value construction, second probe: found the real zero-sum rows `generate_weights` produces for the public-API-reachable case `HaasnSoft` `in_size=3, out_size=10` (same case family as candidate 010's), then drove the real public `resize_plane8`/`resize_plane16` (`bit_depth=12`) entry points with the most divergent boundary source values possible at each bit depth (`0`/`255` for 8-bit, `0`/`4095` for 12-bit) around those rows | `cargo test --lib` (2x debug) + `cargo test --release --lib` (1x release), same probe, `--nocapture --test-threads=1` | No divergence in any of the 3 runs (`mismatches = []` every time): every zero-sum-row output byte/u16 exactly equalled `src[bounds.start]` (the nearest-fallback expectation), even at the most extreme boundary values available at these bit depths |
| Exact arithmetic derivation of the observability bound (see "Root cause" below), cross-checked against every public entry point that ever reaches the fixed-point/`i16`-quantization path | Code reading of `src/resizer.rs` (`resize_plane8`/`resize_plane8_with_alpha`/`resize_rgba8`/`resize_rgb8`/`resize_luma_alpha8` hardcode `bit_depth=8`; `resize_plane16`/`resize_rgba16`/`resize_rgb16`/`resize_luma_alpha16` only route to `resize_fixed_point` when `bit_depth == 10 \|\| bit_depth == 12`, otherwise falling through to `resize_floating_point`) plus `src/saturate_narrow.rs`/`src/fixed_point_horizontal.rs`/`src/fixed_point_vertical.rs` for the exact rounding formula | Confirmed the fixed-point/`i16`-quantization path this bug lives in is **only ever reached with `bit_depth` 8, 10, or 12** through the crate's public API - `resize_fixed_point` itself is `pub(crate)`, not exported, so there is no way to force it with a wider bit depth from outside the crate |

## Status

**Not reproducible** through the public API as a wrong output value - reproduces 3/3 runs (2
debug + 1 release, deterministic, no seed) at the *function level* (`quantize_kernel` does not
achieve the "whole residual on tap 0" semantics candidate 010 assumed), but every public
entry point that reaches this code is provably bounded below the input magnitude the leak would
need to ever flip an output byte/u16, and this was also checked empirically (see output.txt)
with no divergence found at the most extreme boundary inputs available at the reachable bit
depths.

## Class

n/a (not confirmed as an externally-observable bug)

## Root cause (of why the suspected escalation to a visible bug does not hold)

`FilterWeights::<f32>::numerical_approximation_i16::<PRECISION>` is instantiated with the crate's
one real constant, `PRECISION = 15` (`src/definitions.rs`), so `precision_scale = 1i64 << 15 =
32768.0` always. `quantize_kernel`'s redistribution loop caps each tap's quantized value at
`upper_bound = i16::MAX as f64 = 32767.0`. For a fully zero-sum row, every tap's initial quantized
value is `0.0`, so `residual = precision_scale - 0.0 = 32768.0` exactly - the *maximum possible*
residual this function can ever see. Applying `delta = residual.min(upper_bound - scratch[order[0]])
= min(32768.0, 32767.0) = 32767.0` to the first (stable-sort-selected) tap leaves exactly
`32768.0 - 32767.0 = 1.0` unit of residual, which the loop's next iteration hands to the *second*
tap (`order[1]`) rather than clamping/discarding it. This is a structural, unconditional
consequence of `precision_scale` (32768) being 1 greater than what a single `i16` tap can hold
(32767) - it is not a coincidence of any particular input, so it is a real function-level defect
in `quantize_kernel`, not merely an untested edge case.

However, the *magnitude* of this leak is fixed at exactly 1 part in 32768 regardless of tap count,
row shape, or filter (re-derived analytically, not just observed for one input - see "What was
tried"). Each row is convolved as `output = (ROUNDING_CONST + sum(weight_i * pixel_i)) >>
PRECISION` (`ROUNDING_CONST = 1 << 14 = 16384`, `src/fixed_point_horizontal.rs`/
`src/fixed_point_vertical.rs`), then clamped/narrowed to the target integer type
(`src/saturate_narrow.rs`). Comparing the buggy quantization (`[32767, 1, 0, ...]`) against the
"intended" one (`[32768, 0, 0, ...]`, which would have reproduced `pixel_0` exactly), the two only
diverge in their rounded integer result when `|pixel_1 - pixel_0| >= 16384` (derived by direct
substitution: `floor((16384 + 32767*p0 + p1) / 32768) = p0 + floor((16384 + p1 - p0) / 32768)`,
which only differs from `p0` once `p1 - p0 >= 16384` or `p0 - p1 > 16384`). That threshold exceeds
the maximum representable value at every bit depth (8, 10, 12) the fixed-point/`i16`-quantization
path is ever reached with through the crate's public API (`resize_fixed_point` itself is
`pub(crate)`, unreachable with a wider bit depth from outside the crate) - `2^12 - 1 = 4095 <
16384`. The single-tap-window case (`bounds.size == 1`, no second tap to absorb the leak) behaves
similarly: the row is left permanently 1 unit short of `precision_scale`, a relative shortfall of
`1/32768 ≈ 0.003%`, which by the same substitution never changes the floored integer result for
any 8/10/12-bit pixel value either.

## The fix

No fix applied - there is nothing externally broken to fix. The `quantize_kernel` residual cap
(`upper_bound = i16::MAX`) is intrinsic to using a signed 16-bit accumulator with a
`precision_scale` of `2^15`; widening `upper_bound` isn't possible without also widening the
output type or lowering `PRECISION`, either of which is a design decision beyond the scope of one
zero-sum-row edge case, especially since no reachable input actually surfaces a wrong result. If a
future change ever raises the bit depths reachable through this path (e.g. exposing
`resize_fixed_point` publicly, or wiring `resize_plane16` to route 14/16-bit through the
fixed-point path instead of floating-point), this bound needs to be re-derived - see "What's not
done" below.

- Project test suite: not run before/after since no source change was made (N/A - this candidate
  made no fix).

## What's not done / open questions

- Only `HaasnSoft in_size=3, out_size=10` was checked end-to-end through the public API for actual
  output-byte divergence; the function-level leak itself (`quantize_kernel` capping tap 0 at
  32767 and spilling 1 unit onto tap 1, or leaving a 1-unit shortfall for single-tap rows) was
  shown analytically to be independent of filter/tap-count/row shape, so this is not expected to
  matter, but no exhaustive sweep across every `ResamplingFunction` variant's own zero-sum rows was
  run (candidate 010's own scan already catalogued which `(filter, in_size, out_size)` triples
  produce zero-sum rows, if a future session wants to re-verify with a wider sample).
- The observability bound (`pixel delta >= ~16384` needed to flip the result) is specific to
  `PRECISION = 15` and `i16` as the fixed-point accumulator type - if either constant changes in a
  future version of this crate, this candidate's conclusion needs to be re-derived, not assumed to
  still hold.
- Multi-channel/RGB(A) entry points (`resize_rgba8`, `resize_rgb16` at bit_depth 10/12, etc.)
  were not separately exercised - they share the exact same per-channel convolution math and
  bit-depth ceiling as `resize_plane8`/`resize_plane16` (checked by reading `src/resizer.rs`), so
  this is not expected to differ, but wasn't independently run.
- Not checked whether the sibling `pic-scale` repo (`/home/rafal/test/pic-scale`) has an
  equivalent `quantize_kernel`-style fixed-point residual redistribution step with the same
  `precision_scale > i16::MAX` mismatch, or a different accumulator width that avoids it entirely.
