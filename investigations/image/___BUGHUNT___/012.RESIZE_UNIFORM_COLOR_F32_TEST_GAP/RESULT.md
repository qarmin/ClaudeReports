# `image::imageops::resize`'s test suite has no f32/`Rgb32FImage` uniform-color-preservation test

Backlog item from `IDEAS.md` (top row, sourced from candidate 009): candidate 009 added a
uniform-color-preservation regression test for `Rgb<u8>` (`test_resize_uniform_color_preserved`)
but the crate still has no sibling test for the `f32` pixel type (`Rgb32FImage`/`Rgba32FImage`).
Candidate 003's `RESULT.md` noted the confirmed `+1.0` `vertical_sample` accumulator-seed bias
applies to `f32` output "unconditionally and unclamped" (no `u8` saturation to mask it) - this
candidate checks whether that gap is real and, if so, what the bug actually looks like for `f32`
output and closes the gap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/primitive_sealed.rs`'s `NearestFrom<f32>` impls and `horizontal_sample`'s final conversion (`S::clamp_nearest_from(tc)`, `src/imageops/sample.rs:331`) to check candidate 003's "unclamped" claim | Manual code read | Correction to candidate 003's wording: `f32`'s own `clamp_nearest_from` *does* clamp to `[0.0, 1.0]` (`value.max(0.0).min(1.0)`) - there is a clamp step for f32 output too, applied in `horizontal_sample` (the *second* sampling pass; `vertical_sample`'s own intermediate `Rgba32FImage` buffer is unclamped, but that's an internal detail, not the final output). The practical effect for typical normalized `[0,1]` f32 image data is still severe: `true_value + 1.0` almost always exceeds `1.0`, so the bug saturates nearly every f32 output pixel to exactly `1.0` (pure white) rather than the +1/255 the `u8` case showed - a much larger, more visible corruption, but still not caught by any existing test. |
| Reran the crate's existing suite on the current (still-buggy) `main` working tree, in-process | `cargo test --lib`, debug build, no changes | 331 passed, 0 failed - confirms the gap is still live on current `main` (`44ce9226`), the same result candidate 009 found for the `u8` case. |
| Added a candidate regression test (`test_resize_uniform_color_f32_preserved`, resizes a uniform 6x6 `Rgb32FImage` (value `0.4`) to 3x3 under 5 filter types and asserts every output channel is within `1e-5` of the source value) to `src/imageops/sample.rs`'s existing `mod tests`, ran it against the unmodified (buggy) source, in-process | `cargo test --lib test_resize_uniform_color_f32_preserved`, debug build, 3 repeat runs | Deterministic failure, 3/3 runs: `resize with Nearest did not preserve a uniform source color: got 1 expected 0.4` - confirms the saturation-to-1.0 prediction and that the new test genuinely catches this bug class for f32. |
| Applied candidate 003's already-verified one-line accumulator fix (`Rgba([1.0; 4])` -> `Rgba([0.0; 4])`) on top of the new test, in-process | `cargo test --lib`, debug build, 3 repeat runs | 332 passed, 0 failed, 3/3 runs - the new test passes once the underlying bug is fixed, and no other test regresses. |
| Standalone `repro/` (own `Cargo.toml`, real public API `image::imageops::resize` on a real `Rgb32FImage`, mirrors the added test's exact check) run via `just test` end-to-end against a pristine local clone pinned at `base_commit`, plus the clone's own `cargo test --lib` run before/after `patch.diff` | `just test` (full cycle: clone -> reproduce-before -> test-only-gap -> apply -> reproduce-after), release build | Matches the in-tree runs exactly: pristine clone -> repro prints `pixel(0,0) = [1.0, 1.0, 1.0]` (expected `[0.4, 0.4, 0.4]`) for all 5 filters and `GAP DEMONSTRATED`, and `cargo test --lib` still reports 331/331 green; test-only patch (no fix) -> new test fails deterministically (`got 1 expected 0.4`); full `patch.diff` (test + fix) applied -> repro prints all 5 filters within `1e-5` of `0.4` and `Gap closed`, `cargo test --lib` reports 332/332 green. See `output.txt` for the real captured run. |

This is a manual boundary-value/differential-invariant check (per `HEURISTICS.md`'s "Logic bugs"
guidance - no sanitizer/miri/fuzzing needed to confirm a missing assertion, same tool class as
candidate 009), not a source-code bug hunt. Soft cap of 3 was not needed; the gap was evident from
the first rerun of the existing suite and the code read explained the exact failure mode.

## Status

**Confirmed** (as a test-suite gap, not a new source bug - the underlying `vertical_sample` bias
itself was already confirmed by candidate 003; this candidate confirms the gap is real for the
`f32` pixel type specifically and characterizes its distinct symptom - saturation to white rather
than a small offset). Deterministic: the gap (existing suite passes 331/331 despite the known bug
saturating every f32 output pixel) and the fix (new test + candidate 003's accumulator fix
together produce 332/332) both reproduce 3/3 direct in-tree runs, plus a further 3/3 through the
standalone `repro/`'s `just test` cycle (clone's own `cargo test --lib`, matching in-tree results
exactly) - 6/6 total for each side, byte-identical test-result summaries every time.

**Fully reproducible: Yes.** `repro/`'s `main.rs` and the added `src/imageops/sample.rs` test both
call only `image::imageops::resize`, the crate's real public API, on a real `Rgb32FImage` built
through its own public constructor - the same real entry point a genuine caller (or the crate's own
CI) exercises. There is no second participant/peer/wire input in this finding (a single-process
question of whether the test suite covers an invariant for a specific pixel type), so
`CONFIRMATION.md` section 11's "real second participant" requirement does not apply.

## Class

test-coverage

## Root cause

`src/imageops/sample.rs`'s `mod tests` (as of `base_commit`, i.e. still not fixed by candidate
009) checks the uniform-color-preservation invariant only for `Rgb<u8>`
(`test_resize_uniform_color_preserved`). No test exercises `resize`/`vertical_sample`/
`horizontal_sample` with an `f32`-backed pixel type (`Rgb32FImage`/`Rgba32FImage`), even though
`resize<I: GenericImageView>` is fully generic over the pixel type and the crate explicitly
supports `f32` images (`Rgb32FImage`/`Rgba32FImage` type aliases, HDR/OpenEXR codec support). This
gap let candidate 003's confirmed `vertical_sample` accumulator-seed bug
(`crate::Rgba([1.0; 4])` instead of `[0.0; 4]`, `src/imageops/sample.rs:544`) ship with a
particularly severe, unnoticed symptom for this pixel type: `horizontal_sample`'s final conversion
step (`S::clamp_nearest_from(tc)`, line 331) does clamp `f32` output to `[0.0, 1.0]`
(`src/primitive_sealed.rs:135-137`, contrary to candidate 003's "no clamping step" wording for
f32, which this candidate corrects), but for ordinary normalized `[0,1]` image data, adding a flat
`+1.0` bias before that clamp pushes almost every value above `1.0`, so the clamp saturates nearly
every resized pixel to exactly `1.0` (pure white) instead of preserving the source color -
verified empirically (`got 1 expected 0.4` for all 5 filter types).

## Introduced by

20a23082 "Remove Pixel channels4/from_channels" (2025-12-30)

## The fix

Same one-line fix candidate 003 already verified for the `u8` case (`Rgba([1.0; 4])` ->
`Rgba([0.0; 4])` at `src/imageops/sample.rs:547`), applied together with the new
`test_resize_uniform_color_f32_preserved` test - see `patch.diff`. The fix is sufficient because
it corrects the shared root cause (`vertical_sample`'s accumulator seed) that both candidate 009's
`u8` test and this candidate's `f32` test independently exercise; verified against 5 filter types
(`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`) and both pixel-type tests together,
not just the one triggering case.

- Project test suite: 331 passed / 0 failed before the fix (with only the new test added, the new
  test itself fails: 0 passed / 1 failed for `test_resize_uniform_color_f32_preserved` alone) ->
  332 passed / 0 failed after (test + fix applied together). No other test regressed. Matches
  candidate 009's own before/after counts for the sibling `u8` test exactly (same underlying fix,
  now two regression tests guarding it instead of one).

## What's not done / open questions

- This candidate does not apply or leave candidate 003's fix in the main project's working tree -
  per protocol, the tree was reverted (`git checkout -- src/imageops/sample.rs`) immediately after
  verification, same as every other candidate touching this bug. The underlying `vertical_sample`
  bug (`crate::Rgba([1.0; 4])`) is still present in the real `main` branch as of `base_commit`
  `44ce9226c541dd2b11897b9ef07156a28871cdb7` - candidates 003, 009, and this one (012) all confirm
  and re-verify the same fix without ever landing it, by design (this skill applies/verifies/
  reverts; a maintainer decides whether to actually merge `patch.diff`).
- `Rgba32FImage`/`Rgba<f32>` (with a real, non-constant alpha channel) was not separately tested -
  only `Rgb32FImage` (no alpha). Candidate 010 already swept `Rgba32FImage` (opaque+alpha) broadly
  for the *pic-scale-safe*-backed `DynamicImage::resize` path (a different implementation) and
  found no bug there; this candidate's `vertical_sample`/`horizontal_sample` path is untested with
  `Rgba<f32>` specifically, though the accumulator-seed mechanism is per-channel and
  alpha-agnostic, so the same bug/fix is expected to apply identically.
- `f64` output (`ImageBuffer<Rgb<f64>, Vec<f64>>`) has the identical `NearestFrom<f32>::
  clamp_nearest_from` clamp-to-`[0,1]` shape (`src/primitive_sealed.rs:144-146`) and would show
  the same saturation-to-1.0 symptom, but wasn't empirically checked - no `f64` type alias/codec
  currently exists in this crate to make it a natural target, so this is a lower-value gap than
  the `f32` one closed here.
- Per `IDEAS.md`'s framing (candidate 009's own precedent): this closes a test-coverage gap for an
  *already-confirmed* bug (candidate 003), not a newly-discovered source bug - the real value here
  is a second, independent regression guard plus a corrected understanding of exactly how the bug
  manifests for float pixel types (full saturation, not a small offset), which is more actionable
  for anyone deciding whether/how urgently to land candidate 003's fix.
