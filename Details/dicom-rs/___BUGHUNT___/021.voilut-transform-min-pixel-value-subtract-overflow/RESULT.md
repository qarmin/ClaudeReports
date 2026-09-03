# VoiLutTransform::apply's i32 subtraction overflows for extreme rescaled values

Found during a discovery round (no seeded lead): after `009`/`010` uncovered two bugs in
`pixeldata/src/lut.rs`'s LUT-table construction, and `003` uncovered a bug in
`pixeldata/src/attribute.rs`'s VOI LUT Descriptor parsing, I read the third piece of that same
pipeline, `pixeldata/src/transform.rs`'s `VoiLutTransform::apply` (the function that actually
looks up a rescaled sample value in the explicit VOI LUT table). Its index computation,
`(value_rounded - self.lut.min_pixel_value) as usize`, subtracts two `i32` values without
widening first - `value_rounded` is clamped to the full `i32` range (`value.round().clamp(i32::MIN
as f64, i32::MAX as f64) as i32`), while `min_pixel_value` comes from the VOI LUT Descriptor's
second value and can be an arbitrary signed 16-bit value (down to `-32768`) once encoded with
VR=SS. `i32::MAX - (-32768)` exceeds `i32::MAX`, so the subtraction overflows.

`value_rounded` reaching an extreme is easy to trigger without any other malformed input: it is
`rescale.apply(v) = v * RescaleSlope + RescaleIntercept` rounded and clamped, and `RescaleSlope`/
`RescaleIntercept` are ordinary DS-valued attributes with no bounds enforced anywhere in this
crate - an DS value like `"1.0e30"` is valid DICOM and pushes `value_rounded` straight to
`i32::MAX`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `pixeldata/src/transform.rs:238`, `VoiLutTransform::apply`; cross-referenced against `pixeldata/src/attribute.rs`'s `parse_voi_lut_entry` (`min_pixel_value: i32` populated straight from the LUT Descriptor's second value with no extra range restriction) and `pixeldata/src/lut.rs`'s call sites (`new_rescale_and_lut`, which feeds `rescale.apply(v)`'s output straight into `voi.apply(v)` with no clamping other than `apply`'s own `.clamp(i32::MIN as f64, i32::MAX as f64)`). Concluded the subtraction can overflow whenever `value_rounded` and `min_pixel_value` have opposite extreme signs. |
| Standalone repro (release build with `overflow-checks = true` set explicitly in the repro's own `Cargo.toml`, using the crate's public API: `PixelDecoder::decode_pixel_data`, `DecodedPixelData::to_dynamic_image`, feature `image`) | `___BUGHUNT___/021.../repro`. Built a minimal 1x1 MONOCHROME2, 8-bit DICOM object with `RescaleSlope = "1.0e30"`, `RescaleIntercept = "0"`, and a VOI LUT Sequence item whose LUT Descriptor is `[2, -32768, 8]` (VR=SS, forcing signed interpretation of `min_pixel_value`), then called `.to_dynamic_image(0)` (the default conversion pipeline: Modality LUT followed by the first VOI LUT table found, matching `VoiLutOption::Default`). | Reproduced deterministically: panicked with `attempt to subtract with overflow` at `pixeldata/src/transform.rs:238:22`. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically (deterministic arithmetic on fixed input; the panic additionally appears once per rayon worker thread, since `Lut::new_with_fn` parallelizes table construction with `into_par_iter()` when the `rayon` feature is enabled - only one panic is needed to fail the `.collect()` and abort). |
| Cross-check: plain release build (Cargo default `overflow-checks = false`) against the same unmodified source | same repro, temporarily without the `Cargo.toml` profile override | Did not panic - the subtraction silently wraps in two's-complement arithmetic. In this specific crafted input, the wrapped index happens to still land out of the LUT data's 2-entry bounds, so `.get(...).unwrap_or(self.lut.data.last().unwrap())` falls back to the last table entry - the same output the fixed code also produces for this particular input. This does not make the release-mode wraparound harmless in general: for other `(min_pixel_value, value_rounded)` pairs the wrapped index can land *inside* the table's bounds by coincidence, silently returning the wrong LUT entry - that specific diverging-output scenario was not independently reproduced with a concrete input in the time available; only the debug/overflow-checked panic half of the bug was fully confirmed end-to-end. |
| Reachability check | `pixeldata/src/lib.rs`'s `build_monochrome_image` (used by the public `to_dynamic_image`/`to_dynamic_image_with_options`) constructs `VoiLutTransform::new(...)` and calls `Lut::new_rescale_and_lut(8, signed, rescale, transform)` whenever a VOI LUT Sequence is present and `VoiLutOption::Default`/`First` is requested (the default) - an ordinary caller decoding pixel data to an image, on any DICOM object that happens to combine a VOI LUT Sequence with an out-of-range `RescaleSlope`/`RescaleIntercept`, hits this exact path. | N/A (confirms the public entry point matches the repro's call sequence). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build (with `overflow-checks = true`, which is how
the panic half of this bug is reliably observed; many projects enable this in their own release
profile as a hardening measure, and it is always on in debug builds).

## Class

`crash` (an unhandled arithmetic-overflow panic, reachable through the ordinary public pixel data
-> image conversion pipeline on a crafted-but-well-formed DICOM object; the sibling "release
silently wraps instead of panicking" half of the same defect is a `logic` concern but was not
independently confirmed with a diverging concrete output).

## Root cause

`VoiLutTransform::apply` (`pixeldata/src/transform.rs`) computes the VOI LUT table index as
`(value_rounded - self.lut.min_pixel_value) as usize`, entirely in `i32`. `value_rounded` is
explicitly clamped to the full `i32` range just one line above, but `min_pixel_value` (an
attacker-controlled 16-bit signed/unsigned value from the VOI LUT Descriptor, stored as `i32`) is
never bounded relative to it. When `value_rounded` is very large (positive) and `min_pixel_value`
is negative, or vice versa, the subtraction exceeds the `i32` range - this is only prevented from
causing visible harm by luck (the two operands both being small in ordinary, non-adversarial
inputs), not by any actual bound in the code. `value_rounded` itself is easy to push to an extreme
via `rescale.apply()`, since `RescaleSlope`/`RescaleIntercept` are unrestricted DS attributes.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Widened the subtraction to `i64` before converting to `usize`, so no `i32` overflow can occur; the
existing `.get(...).unwrap_or(self.lut.data.last().unwrap())` fallback already handles any
out-of-range index gracefully (matching the function's pre-existing "clamp to last entry" design
for indices beyond the table). See `patch.diff` (`pixeldata/src/transform.rs`, one function).

Verified: `repro/` re-run against the patched tree no longer panics (`to_dynamic_image` completes
successfully, `pixel(0,0) = 255`, an in-bounds, well-defined result matching the fallback-to-last-
entry semantics already documented for out-of-range values).

- Project test suite: `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical
  both before and after the patch (no regressions, no newly-passing tests - no existing test
  exercised a VOI LUT table lookup with an out-of-i32-range index).

## What's not done / open questions

- Did not independently reproduce a concrete input where the release-mode (overflow-checks off)
  silent wraparound produces a *different, wrong* pixel value rather than happening to fall back
  to the last table entry - the code-reading argument for why this is possible (a wrapped index
  landing inside the table's bounds by chance) is sound but unconfirmed with an actual repro.
- Only the explicit-VOI-LUT-table path (`VoiLutTransform`) was checked; `WindowLevelTransform`
  (the window/center-width VOI path, `pixeldata/src/transform.rs`'s `window_level_linear` and
  siblings) uses only `f64` arithmetic with no integer casts of untrusted magnitudes, so it does
  not share this specific overflow mechanism.
- Not fuzzed - found by direct code reading of a `HEURISTICS.md`-style "untrusted-value arithmetic
  in a type too narrow for its actual range" pattern, the same class candidate `010` found in the
  sibling `Lut::new_with_fn`'s shift computation.
