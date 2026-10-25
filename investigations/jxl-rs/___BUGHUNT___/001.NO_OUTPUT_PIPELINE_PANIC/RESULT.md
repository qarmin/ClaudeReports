# Render pipeline panics when the caller requests no pixel output

`JxlDecoder::set_pixel_format` documents `None` ("ignore") for `color_data_format` and each extra channel. If every format is `None` the low-memory render pipeline is built with zero stages and `LowMemoryRenderPipeline::new_from_shared` indexes `border_pixels_per_stage[0]`. Found by a cargo-fuzz harness over pixel-format/option combinations (`_reference/fuzz_formats`), which the repo's own fuzz targets do not cover (they only use the default f32 format).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Custom cargo-fuzz harness, fuzzed pixel format + options + codestream | release + debug-assertions, ~3 min, 8 forks, seeded from `jxl/resources/test` | 24+ crash files, all the same panic site |
| Standalone repro via public API | `repro/`, 3x3 sRGB lossless test image, `color_data_format: None`, all extra channels `None` | panic 5/5 |
| Replay of all 24 fuzz crashes against the patched tree | debug-assertions build | none crash |

## Status

**Confirmed** - `repro/` panics 5/5 runs (release build, deterministic, no seed).

**Fully reproducible**: **Yes** - the repro drives the real public decoding API on a real (66 byte) JXL file; no second participant is involved.

## Class

crash

## Root cause

With no output requested, `Frame::build_render_pipeline` adds no Save stage, so `shared.stages` is empty. `new_from_shared` then does `assert!(border_pixels_per_stage[0].0 <= MAX_BORDER)` (`jxl/src/render/low_memory_pipeline/mod.rs:209`) on an empty `Vec`, panicking with "index out of bounds: the len is 0 but the index is 0". The input file is valid; only the caller-chosen pixel format triggers it.

## Introduced by

939ceec "Implement the low-memory render pipeline (#408)" (2025-10-28) - the initial version already did `border_pixels_per_stage[0]` unconditionally.

## The fix

`assert!(border_pixels_per_stage.first().is_none_or(|b| b.0 <= MAX_BORDER));` (1 line, `patch.diff`). Decoding then completes normally (`repro/` prints "completed normally"). All 24 fuzz-found crash inputs (varying color types, bit depths, endianness, orientation/premultiply options) also pass with the patch, so it is not specific to the one repro input.

- Project test suite (`cargo test --release -p jxl`): 1561 passed / 0 failed before -> 1561 passed / 0 failed after.

## What's not done / open questions

- `fuzz_crash_inputs/` holds 3 of the fuzzer-found inputs (first 9 bytes = pixel format/options for `harnesses/fuzz_formats` target `formats`).
- Only the low-memory pipeline was exercised (it is the one the API uses); the simple pipeline (tests only) was not checked.
- Whether a "no output at all" request should rather return `ApiUsageError` is a maintainer decision; the patch makes it a no-op decode.
