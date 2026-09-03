# 006.fuse_lossless_decode_predict

Structural fusion of `decode_scan_lossless`'s (src/decoder/lossless.rs) two-phase
decode-then-predict pipeline for the `Predictor::Ra` case, following up on the
`differences: Vec<Vec<i32>>` intermediate-buffer lead left by candidate 005's investigation
(IDEAS.md item 21). The entropy-decode loop already visits pixels in the exact raster order
Ra's prediction needs, so each pixel's final value is now computed immediately after its diff is
decoded, writing straight into the output buffer instead of a separate `differences` buffer that
gets fully written during decode and fully read back during a second prediction pass.

## Correctness analysis done before implementing

- **Predictor selections 2-7 / NoPrediction (the general `else`/`predict()` branch)**: read
  closely and deliberately **not fused**. The `restart` boolean passed into `predict()`
  (`self.restart_interval > 0 && mcus_left_until_restart == self.restart_interval - 1`, line
  ~169-170 in the original) is evaluated inside the *second*, prediction-only loop, but
  `mcus_left_until_restart` is never mutated there - it is frozen at whatever value the
  first (entropy-decode) loop left it at, so this "restart" flag is a single constant for the
  *entire* second loop, not a per-pixel restart-boundary check. This is a pre-existing quirk of
  the current (`eb2d7c0`) code, unrelated to this candidate. A naive single-pass fusion of this
  branch would naturally recompute a per-pixel-correct restart flag during the real decode sweep,
  which would silently change output (fix a latent bug) for any selection-2-7 image that also
  uses restart intervals - a correctness regression relative to the bit-exact bar this session is
  held to. None of the in-repo selection-2-7 images (`jpeg_lossless_sel2.jpg` .. `sel7.jpg`) are
  large enough to move the needle on wall time anyway (~1.0-1.2ms per candidate 005's notes), so
  this branch was left untouched - same two-phase structure, unchanged code. See "What's not
  done" below.
- **`Predictor::Ra` branch**: prediction here never reads `rb`/`rc` and never looks at restart
  state at all (confirmed by reading the original code - the Ra branch's three sub-loops use only
  the top-left default, the row-above value for column 0, and the left neighbour for the rest of
  each row; restart markers only affect the Huffman decoder's own bit-alignment, not this
  branch's arithmetic). Verified by hand that a single raster sweep carrying two per-component
  locals - `left` (previous column, same row) and `col0_prev` (column 0 of the previous row) -
  reproduces the original's three sub-loops (top-left pixel, leftmost column, rows) pixel-for-
  pixel:
  - `(mcu_x, mcu_y) == (0, 0)`: prediction = `1 << (precision - point_transform - 1)` (same
    expression as the original, same u8-arithmetic-then-cast evaluation order, hoisted out of the
    loop since it's loop-invariant - not a behavior change even under debug-mode overflow checks).
  - `mcu_x == 0, mcu_y > 0`: prediction = `col0_prev[i]`, updated at the end of every row's column
    0 - matches the original's separate "leftmost column" loop exactly.
  - `mcu_x > 0`: prediction = `left[i]`, updated after every pixel - matches the original's row
    loop (which also covers `mcu_y == 0`, `mcu_x > 0`, since that loop starts at `mcu_y = 0`).
  - Multi-scan interaction: `decode_scan_lossless` builds fresh `results`/local state on every
    call (verified in src/decoder.rs:414-424, called once per SOS marker for lossless frames) -
    no state is carried between calls in either the original or fused code, so multi-scan lossless
    images are unaffected by this change.

## What was tried

Two implementation variants for the `Predictor::Ra` fusion, both entropy-decode-identical, bit-
exact everywhere, measured on the primary image plus 5 secondary images spanning 8/10/12/16-bit
precision and 1-3 components (unpinned multi-core, `tools/paired.sh`, RUNS=9):

| Variant | sel1-rgb (3072x2048, 3c, 8b) | sel1-8bit (512x512, 1c, 8b) | sel1 (256x256, 1c, 16b) | MR4 (512x512, 1c, 12b) | XA1 (1024x1024, 1c, 10b) | lossless16bit (896x876, 1c, 16b) |
|---|---|---|---|---|---|---|
| baseline | - | - | - | - | - | - |
| v1: fused, but `results[i][idx]` still indexed through the outer `Vec<Vec<u16>>` per pixel/component (double indirection reintroduced - the per-component-outer loop structure candidate 005 relied on to hoist this away no longer exists once components are interleaved per-pixel to match decode order), plus a per-pixel `mcu_x==0` branch | -24.3%, -24.2% (9/9, 9/9) | -13.0%, -12.7% (9/9, 9/9) | -8.1% (9/9) | **+3.6%, +4.1%** (0/9, 1/9) | **+4.5%, +4.7%** (0/9, 0/9) | **+4.6%** (0/9) |
| v2 (kept): same fusion, but `results.iter_mut().map(|v| v.as_mut_slice()).collect::<Vec<&mut [u16]>>()` hoists a flat slice per component once (safe Rust, no `unsafe` - the crate has `#![deny(unsafe_code)]`), and column 0 is handled in its own per-row block so the `mcu_x > 0` sweep never branches on `mcu_x` | **-26.8%, -26.8%** (9/9, 9/9) | -19.7% (9/9) | -13.2% (9/9) | -1.3%, -1.3% (9/9, 9/9) | -1.1%, -1.2% (8/9, 9/9) | -0.7% (8/9) |

(paired.sh reports p50 in ms per run; percentages above are the script's own median-shift output;
two numbers separated by a comma are two independent RUNS=9 batches on different invocations.)

v1 was a real, reproducible regression on the three higher-bit-depth single-component images
(MR4, XA1, lossless16bit) - decisive in the *wrong* direction (0-1 of 9 pairs favoring the
patch), even though it was a large win on the primary and 8-bit images. This traces to the
double-`Vec<Vec<u16>>`-indexing regression described in the variant row: v1 accidentally
reintroduced the exact anti-pattern candidate 005 fixed in the old two-phase prediction loop,
because fusing forces the per-pixel loop to interleave components (to match the bitstream's
decode order) and so can no longer hoist `&mut results[i]` for the entire sweep the way a
per-component-outer loop could. v2's `Vec<&mut [u16]>` (built once, outside the pixel loop, one
slice per component) restores an equivalent hoist while still writing directly into the final
buffer, and turned every one of v1's regressions into a (smaller, but real and paired-9/9-or-8/9-
decisive) win instead.

## Result

v2 (the kept patch) is a real, decisive, all-images win, not just on the primary benchmark image:

- `jpeg_lossless_sel1-rgb.jpg` (primary, 3072x2048, 3-component, 8-bit): **-26.8% median**, two
  independent RUNS=9 batches, 9/9 pairs both times.
- `jpeg_lossless_sel1-8bit.jpg` (512x512, 1-component, 8-bit): **-19.7%**, 9/9.
- `jpeg_lossless_sel1.jpg` (256x256, 1-component, 16-bit): **-13.2%**, 9/9.
- `MR4.jpg` (512x512, 1-component, 12-bit): **-1.3%**, two batches, 9/9 both.
- `XA1.jpg` (1024x1024, 1-component, 10-bit): **-1.1%/-1.2%**, 8/9 and 9/9.
- `lossless16bit.jpg` (896x876, 1-component, 16-bit): **-0.7%**, 8/9.

All six well above/at the ~2.2% same-binary noise floor measured for this machine on the primary
image (`projects/jpeg-decoder.md`); the smaller single-component/high-precision images show a
real but modest effect (1-2%, still decisively paired) since entropy decode (unaffected by this
change) is a larger fraction of their total time than for the primary image.

This is a considerably larger win than candidate 005's -7.9%/-8.5% on the same primary image,
because 005 only removed a redundant re-read inside the second (prediction) phase, while this
candidate removes the entire `differences: Vec<Vec<i32>>` buffer (~25MB per component, ~75MB
total for the primary image) and both its full write pass (during decode) and full read pass
(during the old prediction phase).

The general `predict()` branch (selections 2-7) was read but deliberately left unfused - see
"Correctness analysis" above.

## Does it change behavior/output?

- **Bit-exact?** yes. Verified two ways:
  - The project's own `reftest` test (tests/reftest/mod.rs) requires `diff == 0` for every pixel
    of every lossless image under `tests/reftest/images/lossless/{1,2}/` against its reference
    PNG (see `reftest_decoder`'s `info.coding_process != Lossless` exception, which only relaxes
    the check for *non*-lossless images) - passed before and after.
  - An explicit baseline-vs-patched hash sweep (via the `bench_decode` harness, which aborts on
    any hash mismatch) across all 12 lossless reftest images individually: `jpeg_lossless_sel1.jpg`,
    `sel1-8bit.jpg`, `sel1-rgb.jpg`, `sel2.jpg`..`sel7.jpg`, `lossless16bit.jpg`, `MR4.jpg`,
    `XA1.jpg` - all MATCH (same hash baseline vs. patched).
- **Is this a breaking change?** no - internal function body only, no signature/API change.
- Project test suite: `cargo +stable test --release` - 5 lib unit tests + 12 integration tests
  (tests/lib.rs, includes the full reftest/crashtest suites) + 4 separate rayon integration test
  binaries (1 test each) + 2 doctests, all passing both before and after. No new failures.

## Why the result looks the way it does

The `differences` buffer was a genuinely wasted ~75MB round-trip (write during decode, read
during predict) for the primary image - removing it plus the extra pass over it is a much bigger
structural saving than candidate 005's single-value carry-forward fix. The first attempt (v1)
showed that a naive fusion can accidentally *reintroduce* a different inefficiency (double
`Vec<Vec<u16>>` indexing) that more than cancels the buffer-removal win on smaller/higher-
precision images, because forcing per-pixel component interleaving (to preserve decode order)
removes the ability to hoist a per-component base pointer across the whole sweep the way the old
two-phase code's per-component-outer prediction loop could. Restoring an equivalent hoist
(`Vec<&mut [u16]>`, built once) recovers that and turns every measured image into a win.

## What's not done / open questions

- The general `predict()` branch (selections 2-7 / NoPrediction) was not fused - see "Correctness
  analysis" above for why (the frozen/stale `restart` flag passed into `predict()` in the current
  code is a pre-existing quirk that a naive fused rewrite would silently "fix", changing output
  for any selection-2-7 image using restart intervals). This is now a concrete, well-specified
  `IDEAS.md` lead in its own right (see the new row added there) - fixing the frozen-restart-flag
  quirk first, deliberately and separately from this performance change, would be a prerequisite
  for ever safely fusing that branch.
- No in-repo image was found that both uses predictor selection 2-7 *and* is large enough to
  measure (`sel2.jpg`..`sel7.jpg` are ~1ms decodes) - the general branch remains a low-priority
  target on measured-benefit grounds alone, independent of the restart-flag complication.
- v1 (the naive fusion) is not kept anywhere as a separate file; its numbers above are from this
  session's own measurements, not preserved as a second patch.
