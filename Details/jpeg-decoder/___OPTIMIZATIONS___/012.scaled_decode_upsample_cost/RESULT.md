<!--
Measurement-only investigation. No patch.diff against the tested project's own source: no fix
was applied to real source, only temporary phase-timing instrumentation (added, measured,
reverted). See "Does it change behavior/output?" below for the explicit statement of which of
the two valid states (SKILL.md) this candidate is in.
-->

# scaled_decode_upsample_cost

Direct follow-up to candidate 010's open question: does `Upsampler`/color-convert cost become a
proportionally *bigger* share of total decode time at reduced `Decoder::scale()` output sizes on
a genuinely chroma-subsampled (4:2:0/4:2:2) image, unlike `benches/large_image.jpg` (confirmed
4:4:4, no subsampling)? Answered by phase-timing entropy decode / dequant+IDCT dispatch /
upsample+color-convert separately at full, 1/2, 1/4 and 1/8 requested scale on a real subsampled
image found in-repo.

## Finding a subsampled test image

Parsed the SOF marker's component sampling factors (Hs/Vs per component) directly for every
`.jpg` under `benches/` and `tests/reftest/images/**` (43 images). **No 4:2:0 (H2V2) image
exists in this repo.** Every genuinely subsampled image found uses H2V1 (horizontal-only
2x1 subsampling, i.e. 4:2:2) or is a tiny (<=33x33 px, mostly single-digit KB) synthetic reftest
fixture unsuitable for timing (`mozilla/jpg-size-*.jpg`, `extraneous-data.jpg`,
`non-interleaved-mcu.jpg`, `16bit-qtables.jpg`, `mozilla/jpg-progressive.jpg`,
`mozilla/jpg-cmyk-2.jpg`). The only real-world-sized subsampled image is:

- **`tests/reftest/images/mjpeg.jpg`**: 960x720, 3 components, baseline (SOF0), 8-bit, sampling
  factors `[(Y: Hs=2,Vs=1), (Cb: Hs=1,Vs=1), (Cr: Hs=1,Vs=1)]` - i.e. **4:2:2, H2V1** (horizontal
  chroma subsampling only, no vertical subsampling), 122KB.

`benches/large_image.jpg` (2268x1512, 500KB) was re-confirmed 4:4:4 (`Hs=Vs=1` for all 3
components) as candidate 010 found, and used here as the non-subsampled control for a
side-by-side comparison under the exact same instrumentation.

## Harness: `--scale` + phase timers

Reused candidate 010's scratch `--scale W H` flag (calls `Decoder::scale(w, h)` before
`decode()`) in a fresh scratch copy of `bench_decode` at `/tmp/bench_decode_scaled012/` (not
committed to the canonical harness, same rationale as candidate 010 - this stays a one-off local
probe). Not reused verbatim: candidate 010's `AtomicU64` probe only timed the IDCT wrapper
function; this candidate needed a 3-way split (entropy / IDCT-dispatch / upsample+color-convert),
so new temporary instrumentation was added directly to `src/decoder.rs` (3 `AtomicU64` totals) and
exposed via `#[doc(hidden)] pub fn` accessors in `src/lib.rs` (mirroring candidate 010's pattern).
Saved as `probe_phase_timing.diff` in this folder; reverted via `git checkout -- src/` before this
file was written.

Instrumentation placement, chosen at MCU-row granularity (not per-block) per the project's own
documented "Instrumentation-overhead gotcha" in `projects/jpeg-decoder.md` (per-block `Instant`
timers at tens-of-thousands-of-calls/decode measurably inflate results; per-row is ~2-3 orders of
magnitude fewer calls and negligible overhead):

- **entropy**: wraps `decode_scan`'s per-MCU-row inner loop that calls `decode_block`/
  `decode_block_successive_approximation` (Huffman decode) - one `Instant` span per MCU row.
- **idct_dispatch**: wraps the per-MCU-row loop that calls `worker.append_row(...)` (dequantize +
  IDCT, executed synchronously on the calling thread for the default rayon backend per candidates
  008/009's findings) - one `Instant` span per MCU row.
- **upsample_colorconvert**: wraps the single `compute_image(...)`/`compute_image_lossless(...)`
  call inside `decode_planes` (upsampling + color conversion together) - one span per `decode()`
  call.

All three are cumulative `AtomicU64` nanosecond totals across all iterations of one harness
invocation; the harness prints the per-iteration average (total / iters) alongside the normal
min/p50/mean/p95/max wall-clock numbers. Hashing (FNV-1a of the decoded output, done after
`decode()` returns) is included in the wall-clock `mean` but not in any phase total, which is why
the three phase averages don't sum to the reported wall-clock mean - the phase totals are used
for relative-share comparisons below, not compared directly to wall-clock mean.

## Phase-timing breakdown

30 iterations per run (bench_decode default), unpinned, `cargo +stable build --release`. Multiple
repeats at full/eighth scale on `mjpeg.jpg` confirm the phase split is stable run-to-run (small
absolute image, so wall-clock min/p50 has more relative jitter than the phase totals, which sum
work done across all 30 iterations and are far less noisy).

### `tests/reftest/images/mjpeg.jpg` (960x720, 4:2:2 H2V1 - the subsampled image)

| Requested scale | entropy ms/30iter (3 runs) | idct_dispatch ms/30iter (3 runs) | upsample+colorconvert ms/30iter (3 runs) | core-decode share: entropy / idct / upsample |
|---|---|---|---|---|
| full (no `--scale`) | 35.691 / 32.348 / 36.692 (avg 34.91) | 26.979 / 23.749 / 29.423 (avg 26.72) | 13.424 / 11.571 / 12.443 (avg 12.48) | 47.1% / 36.1% / **16.8%** |
| 1/2 (`--scale 480 360`) | 32.588 | 21.093 | 5.267 | 55.3% / 35.8% / **8.9%** |
| 1/4 (`--scale 240 180`) | 32.676 | 8.938 | 4.328 | 71.1% / 19.5% / **9.4%** |
| 1/8 (`--scale 120 90`) | 32.497 / 32.391 / 32.300 (avg 32.40) | 3.331 / 3.348 / 3.367 (avg 3.35) | 3.695 / 3.508 / 3.785 (avg 3.66) | 82.2% / 8.5% / **9.3%** |

### `benches/large_image.jpg` (2268x1512, 4:4:4 - the non-subsampled control, same methodology)

| Requested scale | entropy ms/30iter | idct_dispatch ms/30iter | upsample+colorconvert ms/30iter | core-decode share: entropy / idct / upsample |
|---|---|---|---|---|
| full | 166.592 | 171.929 | 33.706 | 44.8% / 46.2% / **9.1%** |
| 1/2 (`--scale 1134 756`) | 165.473 | 154.189 | 17.390 | 49.1% / 45.7% / **5.2%** |
| 1/4 (`--scale 567 378`) | 164.448 | 61.970 | 10.256 | 69.5% / 26.2% / **4.3%** |
| 1/8 (`--scale 284 189`) | 165.163 | 40.225 | 4.308 | 78.8% / 19.2% / **2.1%** |

## Result

**The hypothesis is not confirmed - upsample/color-convert cost does not become a proportionally
bigger chunk of total decode time at reduced scale, even on a genuinely 4:2:2-subsampled image.**
On `mjpeg.jpg`, the upsample+color-convert share is *highest at full scale* (16.8%) and *drops* to
8.9-9.4% at every reduced scale tried - the opposite of "grows at reduced scale". On the
non-subsampled control (`large_image.jpg`), the share monotonically shrinks with scale (9.1% ->
5.2% -> 4.3% -> 2.1%), matching candidate 010's original finding exactly. In absolute terms,
upsample+color-convert cost shrinks at every reduced scale on both images (mjpeg.jpg: 12.48ms ->
5.27ms -> 4.33ms -> 3.66ms per 30 iters; large_image.jpg: 33.7ms -> 17.4ms -> 10.3ms -> 4.3ms) -
it tracks shrinking output pixel count in the same direction as IDCT, just less steeply, rather
than growing as scale decreases. Entropy decode is confirmed scale-invariant on both images
(mjpeg.jpg: ~32-37ms flat across all 4 scales; large_image.jpg: ~164-167ms flat), consistent with
candidate 010, and its share of total time still grows steeply as scale drops on both images
(mjpeg.jpg 47.1%->82.2%; large_image.jpg 44.8%->78.8%) - this remains the actual reason reduced-
scale decode doesn't shrink total time proportionally to pixel-count reduction, exactly as
candidate 010 already found; upsampling is not an additional contributor to that effect.

One real, secondary finding: at **full scale**, the subsampled image's upsample+color-convert
share (16.8%) is nearly 2x the non-subsampled image's (9.1%) - real chroma-upsampling work (H2V1
horizontal expansion) on top of color conversion does cost measurably more than color conversion
alone, as expected. But this is a full-scale-decode observation, not a reduced-scale one, and is
not what this candidate was chartered to check (see "What's not done" below).

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source. All temporary instrumentation
(3 `AtomicU64` totals + 3 phase-timing spans in `src/decoder.rs`; 3 `#[doc(hidden)] pub fn`
accessors in `src/lib.rs`) was reverted via `git checkout -- src/` before this file was written;
saved as `probe_phase_timing.diff` in this folder for reference/reproduction only, never applied
to the working tree that's left behind. `git status` in the project root shows nothing but
`___OPTIMIZATIONS___/`/`___OTHER___/` untracked. Project test suite was not re-run for this
candidate (no patch was ever left in place to test, matching candidate 010's precedent) - the
baseline 12 lib + 4 rayon + 2 doctest pass count recorded in `projects/jpeg-decoder.md` for this
same HEAD (`eb2d7c0`) stands unchanged.

## Why the result looks the way it does

Upsampling and color conversion are both `O(output pixel count)` operations - a nearest-neighbor
(or, for H2V1, single-axis linear-ish) expansion plus a fixed per-pixel color-space transform.
Requesting a smaller output size via `Decoder::scale()` directly shrinks the number of output
pixels these operations touch, in the same direction (though not always the same magnitude) as
IDCT's per-block cost shrinking with `idct_size`. Entropy (Huffman) decode, in contrast, always
visits every coefficient in the bitstream regardless of requested output scale (coefficient
positions can't be skipped without decoding the run-lengths that precede them) - it is the only
one of the three phases that doesn't shrink, so it eats an ever-larger share of an ever-shrinking
total as scale decreases. This holds whether or not the image is chroma-subsampled; subsampling
changes upsampling's *absolute* cost (there's more work per output pixel when chroma actually
needs expanding, vs. a no-op copy for 4:4:4) but not the *direction* it moves with scale.

## What's not done / open questions

- No 4:2:0 (H2V2, the most common real-world "4:2:0" JPEG format) image exists anywhere in this
  repo to check directly - only H2V1 (4:2:2). The mechanism argued above (upsample cost scales
  with output pixel count, same direction as IDCT) doesn't depend on which axis is subsampled, so
  a 4:2:0 image is not expected to behave differently, but this is reasoned, not measured.
- New lead noticed while measuring `mjpeg.jpg`: upsample+color-convert cost does not shrink in
  proportion to pixel count between 1/2 and 1/8 scale on the small image (5.27ms -> 4.33ms ->
  3.66ms, roughly flat, while output pixel count itself drops 4x between those two steps) -
  plausibly a fixed per-row dispatch/allocation overhead (thread-local buffer setup, rayon task
  dispatch per output row, similar in kind to candidates 008/009's per-row rayon dispatch cost
  finding for IDCT) dominating once row width shrinks enough. Not investigated further here since
  it doesn't change this candidate's headline answer (upsample share still doesn't grow at reduced
  scale - if anything this flattening works against the "grows at reduced scale" hypothesis even
  more, since the absolute cost stops shrinking but the *total* decode time it's a fraction of
  keeps shrinking, plateauing the share rather than growing it). Left as a new IDEAS.md entry.
- The full-scale finding that a real 4:2:2 image's upsample+color-convert share (16.8%) is ~2x a
  non-subsampled image's (9.1%) is a legitimate, separate observation about full-scale decode of
  subsampled images generally - not investigated as a fix target here since it's outside this
  candidate's chartered question (reduced-scale behavior) and candidate 004 already looked at
  `Upsampler` once (row-buffer allocation reuse, mostly noise-level wall-clock effect). Left as a
  new IDEAS.md entry for a future candidate that wants to focus on full-scale subsampled-image
  upsample cost specifically.
