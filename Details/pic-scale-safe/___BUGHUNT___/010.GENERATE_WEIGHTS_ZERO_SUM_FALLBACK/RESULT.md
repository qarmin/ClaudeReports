# generate_weights general (non-area) branch missing zero-sum fallback

`src/compute_weights.rs`'s general (non-area) branch of `generate_weights` only writes normalized
weights `if weights_sum != 0.0`, with no `else` fallback - unlike the sibling area-filter branch a
few lines below (`else { weights[filter_position] = 1.0; }`). Flagged from a prior
`code-audit-structured` pass as the cheapest backlog entry to confirm.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + a temporary internal `#[cfg(test)]` scan (removed before finalizing this candidate) calling `generate_weights` directly across all 39 `ResamplingFunction` variants x 23 `(in_size, out_size)` pairs (1x1 up to 10000x1) | `cargo test --lib`, debug build | Found 4365 destination rows across many filters/sizes whose entire normalized-weights slice was exactly `0.0` (nonempty `bound.size`, e.g. `HaasnSoft` at `in_size=1, out_size∈{100,1000,10000}`, `in_size∈{2,3,5}` with various `out_size`; `Lanczos6Jinc` at `in_size=1, out_size=1`) - live signal, kept digging |
| Public-API differential check: does the fixed-point path (`resize_plane8`, `u8`/`i32`) also manifest this? | `cargo run --example`, debug build, 1x1->100x1, `HaasnSoft` | Did **not** manifest - `FilterWeights::numerical_approximation_i16`'s residual-redistribution step (`quantize_kernel` in `src/filter_weights.rs`) incidentally forces every row's quantized weights to sum to `precision_scale` regardless of the input row being all-zero, masking the bug for every 8/16-bit integer resize entry point |
| Public-API reproduction on the floating-point path (`resize_plane_f32`, `f32`, no quantization step) | `cargo run --release`, 1x1->100x1 and 3x1->10x1, `HaasnSoft` | Reproduced directly - destination samples silently became `0.0` despite an all-nonzero source; re-ran 3/3 times, bit-for-bit identical output and exit code every time |

## Status

**Confirmed** - reproduces 3/3 runs, release build, fully deterministic (no seed, no threading
involved). See `output.txt` for the actual captured output of all 3 runs plus the raw scan
output.

## Class

logic

## Root cause

In `src/compute_weights.rs`'s general (non-area) branch of `generate_weights`, each destination
row's kernel weights are summed into `weights_sum`, then normalized *only* `if weights_sum !=
0.0`. When a destination row's entire window evaluates the resampling kernel (and, for windowed
filters, the window function) to exactly `0.0` at every sample point in `start..end` - which
happens for some `(in_size, out_size)` combinations with `HaasnSoft`, `Lanczos6Jinc`, and other
kernels whose value can legitimately be `0.0` away from `x=0` - the `if` body never runs, and the
row's slice of `weights` is left at its `Vec::default()`-initialized `0.0` (from
`vec![T::default(); kernel_size * out_size]` a few lines above). Every subsequent convolution that
reads this row (`convolve_row_floating_point`/`convolve_column_floating_point` in
`floating_point_dispatch.rs`) then computes `sum(weight[k] * src[k]) = 0.0` for that destination
sample, regardless of the actual source pixel values - a real, nonzero source pixel is silently
replaced with black/zero output.

The sibling area-filter branch a few lines below (`is_area == true`) has exactly this fallback
already: `else { weights[filter_position] = 1.0; }`, which is precisely the nearest-sample
fallback the general branch is missing.

This is masked on the 8/16-bit integer resize entry points (`resize_plane8`, `resize_rgba8`,
`resize_plane16`, etc.) only as an unintended side effect of a *different* piece of code:
`FilterWeights::<f32>::numerical_approximation_i16` (`src/filter_weights.rs`) quantizes each row to
fixed-point and then redistributes a residual to make the row's quantized weights always sum to
exactly `precision_scale` - for an all-zero input row this residual redistribution pushes the
entire `precision_scale` onto the first tap (index 0 after the stable sort, since all taps are
tied at `0`), which happens to reconstruct a working nearest-sample fallback by coincidence, not by
design. Only the floating-point resize entry points (`resize_rgba_f32`, `resize_rgb_f32`,
`resize_plane_f32`, `resize_luma_alpha_f32`), which use `FilterWeights<f32>` directly with no
quantization step, actually manifest the bug end-to-end.

## Introduced by

f417be2 "Loop unroll for common cases, fix kernel generation, added `Area` resampling method, aarch64 benchmark updates" (2024-10-26) - best-effort: this is the commit that added the `is_area` branch (which has always had its own zero-sum nearest-sample fallback) alongside the general branch (which never got one); `git log -S"weights_sum != 0"` doesn't cleanly isolate the general branch's specific omission since both branches use the same condition text, so this is inferred from the Area-branch addition rather than a direct hit.

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) added an `else if size > 0 { ... }` nearest-sample fallback to the general branch, mirroring the Area branch's - the current working tree no longer reproduces this bug.

## The fix

Added the same fallback the area-filter branch already uses, guarded by `size > 0` (there is
nothing to write for an empty window, and writing anyway would be harmless but pointless since no
source pixel is ever read for a zero-size bound):

```rust
if weights_sum != 0f32.as_() {
    let recpeq = 1f32.as_() / weights_sum;
    for (dst, src) in weights.iter_mut().skip(filter_position).take(size).zip(local_filters.iter().take(size)) {
        *dst = *src * recpeq;
    }
} else if size > 0 {
    weights[filter_position] = 1f32.as_();
}
```

This mirrors the area-filter branch's own fallback exactly (falls back to using the first source
sample in the window with full weight, i.e. a nearest-sample approximation for that one
destination pixel, rather than 0).

Verified:
- Re-ran `repro/` against the patched tree: both `HaasnSoft 1x1->100x1` and `HaasnSoft 3x1->10x1`
  now produce all-nonzero output matching (within float rounding) the source pixel values; exit
  code `0` ("did not reproduce").
- Re-ran the same exhaustive internal scan (39 filters x 23 `(in_size, out_size)` pairs) against
  the patched tree: `0` zero-weight-sum rows found, down from `4365` before the fix - confirms the
  fix generalizes across the whole bug class, not just the two specific inputs `repro/` uses (per
  `CONFIRMATION.md` section 7).
- Project test suite: `cargo test --release` both before and after the patch: `0 passed; 0
  failed` (unit tests) and `0 passed; 0 failed` (doc-tests) in both cases - this package currently
  has no tests at all (`tests/` is an empty directory), so this is the honest "N/A-shaped" result
  for this project, not an omission (consistent with candidate 009's finding on the same repo).

## What's not done / open questions

- The scan found the same zero-sum-row class for several other kernels beyond `HaasnSoft` and
  `Lanczos6Jinc` (visible in `output.txt`'s sample of raw hits) - the fix is structural (applies
  to every kernel that goes through this one branch), so it is not kernel-specific, but only
  `HaasnSoft` and `Lanczos6Jinc`/`(3,10)` were exercised end-to-end through the public API in
  `repro/`.
- Not re-checked whether the *fixed-point* entry points' accidental masking (via
  `quantize_kernel`'s residual redistribution) is itself reliable across every zero-sum case, or
  whether some other input could produce a different quantization outcome that isn't a clean
  nearest-sample fallback - it was only spot-checked for the one `HaasnSoft 1x1->100x1` case in the
  "what was tried" table above, not proven in general. Worth a dedicated follow-up if the
  fixed-point path is considered load-bearing here.
- The sibling `pic-scale` repo (checked out locally at `/home/rafal/test/pic-scale`, used as a
  differential-test oracle by earlier candidates) was not checked for the same missing-fallback
  pattern in its own `generate_weights` equivalent.
