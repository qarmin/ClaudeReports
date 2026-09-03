# WindowLevelTransform::apply: hoist window-width reciprocal out of the per-call division

`IDEAS.md` carried an open lead from the original session: candidate 012 fixed rayon *dispatch*
for `Lut::new_with_fn`'s table-construction loop, but never touched the per-entry math itself -
`Rescale::apply`/`WindowLevelTransform::apply`, called once per table entry via the closure
`Lut::new_with_fn` builds. `WindowLevelTransform::apply`'s three variants (`Linear`,
`LinearExact`, `Sigmoid`) each divide by `window_width` (or `window_width - 1`) inside the
function body, on every call, even though `window_width` is fixed for the lifetime of the
`WindowLevelTransform` value. This candidate re-derives whether hoisting that division into a
one-time reciprocal, computed in the constructor and multiplied in `apply`, is a measurable win -
`IDEAS.md`'s own framing flagged this as "unclear if worth pursuing given divisions aren't
fixed-coefficient here" (unlike candidate 002's YBR conversion, `window_width` varies *between*
different `WindowLevelTransform` instances, so this isn't a single global constant - but it IS
constant *within* one instance's lifetime, which is what matters for hoisting out of `apply`).

## What was tried

Added two fields to `WindowLevelTransform` (`inv_width: f64`, `inv_width_minus_1: f64`),
computed once in `new()`. Changed `window_level_linear`/`window_level_linear_exact`/
`window_level_sigmoid` to accept the relevant precomputed reciprocal as a parameter and multiply
by it instead of dividing by `window_width`(`-1`) directly - `min`/`max` bounds-check computations
(which don't divide) are unchanged.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `colorspace` mode (full decode + LUT
construction + windowing + pixel mapping) against `CT1_RLE.dcm` (a real fixture, 512x512 16-bit
CT, RLE Lossless - real acquisition, real `WindowCenter`/`WindowWidth` values, so the `Linear`
branch of `WindowLevelTransform::apply` is genuinely exercised via `VoiLutOption::Default`, not
just checked for compilation). Bit-exactness cross-checked additionally on `vlut_04.dcm`,
`mono_below.dcm`, `mono_above.dcm`.

| Variant | round 1 (RUNS=9) | round 2 (RUNS=9) |
|---|---|---|
| baseline (division per call) | 1.448 ms p50 | 1.424 ms p50 |
| patched (reciprocal, multiply per call) | 1.495 ms p50 (+3.3%, 2/9) | 1.477 ms p50 (+3.8%, 3/9) |

## Result

**No consistent change** - neither round clears this workspace's paired-comparison consistency
bar (2/9 and 3/9, both "NOT consistent"). This confirms `IDEAS.md`'s own stated uncertainty: the
mechanism is real (a division genuinely becomes a multiply), but `WindowLevelTransform::apply` is
called at most `2^bits_stored` times per `Lut` construction (up to 65,536 for a 16-bit image) -
**bounded by bit depth, not by pixel count** - unlike `convert_colorspace_u8`/`u16`
(candidates 002/003) or `convert_pixel_slice` (candidate 006), which run once per *pixel* and
scale with image size. A 512x512 image (262,144 pixels) and a hypothetical 4096x4096 image
(16.8M pixels) would exercise this exact same ≤65,536-call table-construction step identically -
the fix's total possible saving is capped at a small, fixed amount regardless of how large the
image being processed is, and that fixed amount turns out to be below this box's measurement
noise floor for the LUT-construction step's own share of `colorspace`'s total per-call cost.

## Does it change behavior/output?

- **Bit-exact?** Yes on all 4 fixtures tested (`CT1_RLE.dcm`, `vlut_04.dcm`, `mono_below.dcm`,
  `mono_above.dcm` - checksum of the decoded+converted image bytes identical between baseline and
  patched builds). This was **verified, not assumed** - multiply-by-reciprocal is not always
  bit-identical to direct division in IEEE 754 (two roundings instead of one), so this candidate's
  own source comment explicitly avoids claiming bit-exactness a priori and this section is where
  that claim is actually backed by a real check, across every fixture available with real
  `WindowCenter`/`WindowWidth` content in this workspace's `testdata/`.
- **Is this a breaking change?** No public API/type changes - `inv_width`/`inv_width_minus_1` are
  private fields of `WindowLevelTransform`, which already derives `PartialEq` but not
  `Clone`/`Copy`/`Serialize`, so no external code can observe the new fields directly.
- Project test suite: `cargo test --release -p dicom-pixeldata`: **39 passed / 0 failed / 2
  ignored** (lib), plus 1 (image_reader) + 1 (overlay) + 10 (doctests) - identical to baseline,
  both before and after.

## Why the result looks the way it does

The fix's mechanism (division -> multiply) is sound and would matter for a function called
millions of times, but `WindowLevelTransform::apply` is structurally bounded by *table size*
(`2^bits_stored`), not by *image size* - real DICOM `bits_stored` values (8, 10, 12, 16) cap this
at at most 65,536 calls per `Lut` construction, regardless of whether the image being windowed has
a thousand pixels or a hundred million. `colorspace` mode's total per-call time is dominated by
actually decoding and remapping every *pixel* (which candidates 002/003/012's already-applied
dispatch/fixed-point fixes target), not by the comparatively tiny, size-independent LUT
construction step this candidate touches - consistent with `IDEAS.md`'s own framing of this as an
open question rather than a confident lead, and now resolved as a real, if practically
inconsequential, no-op.

## What's not done / open questions

- This candidate's negative result specifically concerns `WindowLevelTransform::apply` as reached
  through `Lut`-table construction (the only call path in this codebase). If a future refactor
  ever called this function per-pixel directly (bypassing the LUT table entirely), the fix's
  mechanism would become relevant again at that scale - not a concern with the current
  architecture, noted only for completeness.
- `Rescale::apply` itself (`slope * value + intercept`, no division) was read but not modified -
  already minimal, nothing to hoist.
- Not measured: whether the `LinearExact` or `Sigmoid` branches (not exercised by any real
  fixture in this workspace's `testdata/`, which only has real `Linear`-function content) show
  a different result - unlikely given the same bounded-by-table-size reasoning applies to all
  three variants identically, but not independently confirmed.
