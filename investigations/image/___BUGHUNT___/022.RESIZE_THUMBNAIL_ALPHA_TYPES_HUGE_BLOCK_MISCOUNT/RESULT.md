# RESIZE_THUMBNAIL_ALPHA_TYPES_HUGE_BLOCK_MISCOUNT

Backlog item from `IDEAS.md` (top row): re-check candidate 020's `thumbnail_sample_block` huge-block
(`pixel_count > 256*256`) STEP_SIZE-splitting divisor bug (floor-division sub-block-count estimate
instead of the actual number of `step_by`-iterated sub-blocks) specifically for the alpha-bearing
pixel types (`LumaA8`, `Rgba8`, `Rgba16`, `Rgba32F`), which candidate 020 itself did not empirically
exercise (it only covered the 5 no-alpha types). The buggy divisor computation is generic over
`P: Pixel<Subpixel = S>` with no type-specific branching, so it was expected to reproduce
identically - this candidate verifies that expectation rather than leaving it as inference.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: temporary `#[test]` fn in `src/imageops/sample.rs`'s `mod tests`, uniform-color 600x600 sources (not a multiple of 256, > 65536 pixels) through the real public `DynamicImage::thumbnail_exact`/`thumbnail` entry points, for `LumaA8`, `Rgba8`, `Rgba16`, `Rgba32F` (plus a non-square 300x1000 `Rgba8` case through `thumbnail`) | `cargo test --lib scratch_thumbnail_uniform_color_probe_alpha_types -- --nocapture`, debug build, 3 repeat runs | 5/5 cases wrong every run (uniform color not preserved after thumbnailing); 3/3 identical runs, bit-for-bit repeatable. |
| Standalone `repro/` crate (own `Cargo.toml`, bare `[workspace]`) depending on the real public `image` crate API, run via `cargo run --release` against the pristine source and again after applying the fix | `cargo run --release`, release build | Before fix: REPRODUCED, same wrong values as the in-tree probe (exit code 1). After fix: Not reproduced, all 5 cases exact (exit code 0). |

Only 1 escalating approach was needed (well under the soft cap of 3): the manual boundary-value
construction from candidate 020 reproduced the bug immediately and deterministically for every
alpha-bearing type tried, mirroring candidate 020's own result for the no-alpha types exactly, so
no fuzzing/sanitizer escalation was needed - this is a pure integer-arithmetic logic bug with no
crash/UB component.

## Status

**Confirmed**, 3/3 identical runs (debug build, in-tree probe) plus a standalone release-build
`repro/` run before and after the fix, matching CONFIRMATION.md's bit-for-bit-repeatable bar.

**Fully reproducible**: **Yes**. `repro/` runs the real public `image::DynamicImage::thumbnail`/
`thumbnail_exact` API end-to-end against a normal `ImageBuffer` built via the crate's own public
`ImageBuffer::from_pixel` - the same path any real caller thumbnailing a >256x256-pixel image would
take. No second participant/peer/wire input is involved (CONFIRMATION.md section 11 does not
apply): a uniform-color source image handed to the crate's own public thumbnail API is itself the
genuine external input.

## Class

logic

## Root cause

Identical mechanism to candidate 020, confirmed to affect the alpha-bearing types too:
`thumbnail_sample_block` (`src/imageops/sample.rs`) is generic over `P: Pixel<Subpixel = S>` with
no per-type branching in its huge-block-splitting path. When a sampled block has more than
256*256 pixels, it splits into `STEP_SIZE = 256` sub-blocks via
`(bottom..top).step_by(STEP_SIZE)` / `(left..right).step_by(STEP_SIZE)`, sums each sub-block's own
average with `sum.add_pixel(k)`, then divides by
`(right - left) / STEP_SIZE * (top - bottom) / STEP_SIZE` (integer floor division). `step_by`
yields one extra, smaller trailing sub-block per dimension whenever the span isn't an exact
multiple of 256, so the real number of `sum.add_pixel` calls is `ceil(span/STEP_SIZE)` per
dimension, not `floor(span/STEP_SIZE)`. The floor-based divisor undercounts, inflating (and often
saturating) the resulting average - for a 600x600 uniform-color source (`ceil(600/256) = 3` per
dimension, `floor(600/256) = 2`), the true divisor is 9 sub-blocks but the buggy code divides by 4,
roughly doubling the result before per-channel saturation kicks in. This affects every channel of
every pixel type identically, alpha channels included - there is nothing alpha-specific about the
bug or the fix; candidate 020's fix was simply never empirically re-checked against these 4 types.

## Introduced by

61c7622d "Fix overflow in `thumbnail`" (2026-05-28)

## The fix

Same fix as candidate 020, unchanged (`patch.diff`): track the actual number of `sum.add_pixel`
calls in a `block_count: u64` counter incremented once per loop iteration, and use that counter
(instead of the floor-division estimate) as the final divisor. Verified sufficient across all 4
alpha-bearing types plus a non-square shape, not just the single triggering input.

- Project test suite (`cargo test --lib`): 331 passed, 0 failed before the fix -> 331 passed, 0
  failed after the fix. The existing `thumbnail_huge_block` test only checks for absence of
  overflow panics, not pixel-value correctness, so it does not catch this bug either way and is
  unaffected by the fix.

## What's not done / open questions

- This closes out the alpha-type gap candidate 020 itself flagged; the bug class (huge-block
  divisor miscount) is now believed checked across all 9 `ColorType` variants image-rs ships
  (5 no-alpha types via candidate 020, 4 alpha-bearing types here).
- A separate open backlog item (candidate 020's own follow-up suspect, still in `IDEAS.md`) asks
  whether the bug's relative error shrinks meaningfully at much larger sub-block counts (e.g. a
  10000x10000 source) - not investigated here, out of scope for this candidate.
