# Reuse the encoder's per-row scratch buffers across animation frames

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

`Writer::write_image_data` (`src/encoder.rs`) allocated two fresh row-sized scratch buffers -
`prev` (`vec![0; in_len]`, the "previous row" used for the first row of the image) and `current`
(the per-row filter output buffer) - on every single call. For a still image this runs once; for
an APNG it runs once *per frame*, on the same `Writer` instance, so these buffers were being
allocated from zero on every frame instead of being reused. This candidate adds two `Vec<u8>`
fields to `Writer` (`zero_row_scratch`, `current_row_scratch`) and reuses them via `.resize(..)`
(reuses existing capacity when large enough, one realloc only when growing beyond a previous
frame's size) instead of `vec![0; ...]` (always a fresh allocation).

## What was tried

Measured with `harness/encoder_bench --mode apng` (writes an N-frame animation via
`write_image_data`, the real per-frame call path), `taskset -c 0`, paired via `tools/paired.sh`.

| Config | baseline allocs / reallocs | patched allocs / reallocs | baseline p50 | patched p50 | paired (RUNS=15) |
|---|---|---|---|---|---|
| 8 frames, 512x512 RGBA | 25 / 120 | 11 / 120 | 8.709ms | 8.697ms | -0.0%, B faster 7/15 - NOT consistent |
| 32 frames, 256x256 RGBA | 97 / 394 | 35 / 394 | 9.716ms | 8.221ms (single-shot, see below) | -0.6%, B faster 9/15 - NOT consistent |

The 32-frame single-shot numbers show an apparent ~15% speedup, but the paired/alternating
comparison on the same two binaries does **not** reproduce it (9/15, inconsistent) - another
instance of the cross-process timing confound flagged in candidate 001's `RESULT.md` and
`projects/image-png.md`. The honest reading is the paired number, not the single-shot one.

## Result

**Time: no change** (both frame counts, paired/`taskset`-pinned: -0.0% and -0.6%, both
inconsistent/within noise). Total encode time here is dominated by deflate compression of several
MB of pixel data; two small (row-sized, at most a few KB) per-frame allocations are far below that
cost.

**Allocation count: real, consistent win.** 8 frames: 25 -> 11 allocations (-56%). 32 frames: 97 ->
35 (-64%), and the win widens with frame count as expected (removing ~2 allocations/frame with a
roughly fixed number of one-time allocations remaining). `reallocs` and `alloc_MiB` are unchanged
(120 reallocs / ~13 MiB both configs, both variants) - those come entirely from the compressed
output `Vec` growing from zero (see candidate 001, a separate, larger allocation-shaped cost in the
same function that this candidate does not touch).

## Does it change behavior/output?

- **Bit-exact?** yes - `verify=OK` every iteration, both configs, before and after (round-trip
  decode + determinism check).
- **Is this a breaking change?** No - `Writer`'s public API and `write_image_data`'s output are
  unchanged; only how two internal scratch buffers are allocated.
- Project test suite (`cargo test --release`): **107 passed, 1 ignored, 0 failed**, identical
  before and after. `cargo clippy --release` shows the same 3 pre-existing warnings (unrelated
  files/lines) both before and after - no new warnings introduced.

## Why the result looks the way it does

Deflate compression of the actual pixel data dominates `write_image_data`'s cost by a wide margin
(candidate 001's `README.md`/`IDEAS.md` calibration: ~97% of encode time on the default
compression level), so removing two small per-frame allocations - real and measurable in allocation
*count* - doesn't move wall-clock time outside this machine's noise floor. This mirrors candidate
002's finding for small ancillary chunks: allocation-count wins don't automatically imply
wall-clock wins when the removed cost was never the bottleneck to begin with.

## What's not done / open questions

- Not combined with candidate 001 (compressed-output `Vec` capacity) in the same patch, per this
  skill's one-fix-per-candidate rule - the two are independent and could be applied together, but
  each is measured and reported on its own.
- The `current_row_scratch` buffer is sized to `in_len + 1` uniformly (reusing the same buffer
  shape as the `FdeflateUltraFast` path, with the `Level(level)` path just using `current[1..]`
  and ignoring index 0) to keep one field instead of two differently-shaped ones - this is a code
  simplification, not something separately measured, since both paths already needed the same
  `in_len`-sized filter output either way.
- Not measured: whether these buffers could be shared/reused across *separate* `Writer` instances
  (e.g. encoding many independent single images in a batch, not just frames of one animation) -
  out of scope, since `Writer` is per-encode and doesn't currently support that.
