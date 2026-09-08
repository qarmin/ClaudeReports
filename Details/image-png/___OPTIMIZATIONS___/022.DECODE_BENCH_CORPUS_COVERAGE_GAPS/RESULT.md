# DECODE_BENCH_CORPUS_COVERAGE_GAPS

`benches/decoder.rs`'s "decode" group reads every `*.png` in `tests/benches/`; reading each of the
15 files' `IHDR` shows all of them are 8-bit, non-interlaced, color type 0/2/3/6 - no 16-bit, no
Adam7 interlacing, no sub-byte (1/2/4-bit) depths, no APNG. So a future candidate touching
bit-depth expansion, Adam7 deinterlacing or animation-frame sequencing would have no bench in this
repo whose numbers actually move. This candidate is a harness/coverage change, not a change to the
library's own decode logic: it adds a new `decode-coverage` benchmark group to `benches/decoder.rs`
and the generator functions it needs to `src/test_utils.rs`, covering exactly the four gaps.

## What was tried

Not a speed optimization, so there is no "variant vs baseline" timing table - the change adds new,
previously-nonexistent benchmark cases rather than touching an existing code path. What was
verified instead: (a) the corpus gap is real (read every `IHDR` in `tests/benches/`), (b) the new
generator functions produce well-formed PNGs that decode correctly (temporary `#[cfg(test)]`
sanity tests, run and then removed - see "Why the result looks the way it does"), and (c) the new
bench group actually runs to completion and reports differentiated throughput numbers, pinned and
unpinned, machine: AMD Ryzen 9 7900 (24 threads), Linux, rustc/cargo 1.98.0, `uptime` load ~2.2/24
throughout. All new images are synthetic and generated at bench-run time (same pattern the
existing `generated-noncompressed-*` groups already use), not stored as binary test files, so
`patch.diff` is a pure text diff.

`cargo bench --bench decoder -- decode-coverage`, criterion default sampling (100 samples), p50:

| Case (256x256 unless noted) | pinned (`taskset -c 0`) run 1 | pinned run 2 | unpinned | notes |
|---|---|---|---|---|
| gray1 (1-bit grayscale) | 31.83 us / 1.91 GiB/s | 31.89 us / 1.91 GiB/s | 31.60 us / 1.93 GiB/s | sub-byte |
| gray2 (2-bit grayscale) | 33.28 us / 1.83 GiB/s | 33.62 us / 1.82 GiB/s | 33.12 us / 1.84 GiB/s | sub-byte |
| gray4 (4-bit grayscale) | 36.64-43.81 us / 1.39-1.67 GiB/s | 43.81 us / 1.39 GiB/s | 44.98 us / 1.36 GiB/s | sub-byte (see note below) |
| gray16 (16-bit grayscale) | 9.58 us / 12.7 GiB/s | 9.79 us / 12.5 GiB/s | 12.07 us / 10.1 GiB/s | 16-bit path |
| palette1 (1-bit palette) | 42.47 us / 4.31 GiB/s | 42.62 us / 4.30 GiB/s | 42.42 us / 4.32 GiB/s | sub-byte + palette expand |
| palette4 (4-bit palette) | 47.22-47.35 us / 3.87 GiB/s | 46.77 us / 3.91 GiB/s | 46.69 us / 3.92 GiB/s | sub-byte + palette expand |
| rgb16 (16-bit RGB) | 24.82-25.56 us / 14.3-14.8 GiB/s | 22.66 us / 16.2 GiB/s | 25.51 us / 14.4 GiB/s | 16-bit path |
| adam7-rgba8 (interlaced) | 190.9-215.8 us / 1.13-1.28 GiB/s | 215.7 us / 1.13 GiB/s | 192.3 us / 1.27 GiB/s | Adam7 |
| apng-rgba8-128x128x8frames | 37.7 us / 12.9 GiB/s | 39.7 us / 12.3 GiB/s | 38.1 us / 12.8 GiB/s | 8-frame APNG, all frames decoded per iteration |

The "pinned run 1" column mixes numbers from an earlier `--quick` smoke pass and the first full
run (both are shown because they were consistent); "pinned run 2" is a second full,
non-`--quick` run of the same binary, confirming the numbers are stable within a few percent -
this is the noise-floor check, not a before/after comparison, since there was no "before" for
these cases.

## Result

The 4 previously-uncovered decode paths now have benchmark cases that visibly differentiate from
each other and from the plain 8-bit non-interlaced path, confirming the harness change does what
it is meant to do (give a future candidate a number that moves):

- **Adam7 (interlaced) is the standout**: ~1.13-1.28 GiB/s vs ~1.9 GiB/s for the equivalent
  non-interlaced 1-bit-per-sample case, and vs ~10-16 GiB/s for the 16-bit paths - i.e. roughly
  10x slower per decoded byte than 16-bit paths, and noticeably slower than even the sub-byte
  grayscale/palette paths, on this machine. This is the clearest "a future candidate here would
  have something real to move" signal of the four.
- Sub-byte grayscale (gray1/gray2/gray4) and sub-byte palette (palette1/palette4) throughput
  numbers are visibly ordered by expansion cost per pixel (gray4 has to unpack more bits per byte,
  and is consistently the slowest of the grayscale cases; palette add a second lookup pass in
  `expand_paletted`, and is faster than gray4 per byte but slower than gray1).
  gray4's own two pinned numbers (36.64 us in the very first `--quick` pass vs ~43.8 us in the two
  full runs) is the one number in this table that is not simply reproducible run-to-run;
  everything else is stable within a few percent, so a future candidate benchmarking specifically
  this case should take an extra repeat run rather than trust either single number.
- 16-bit paths (gray16, rgb16) throughput is measured in GiB/**decoded** bytes, and 16-bit samples
  are 2 bytes each for the same pixel-position work, which mechanically inflates their GiB/s
  number relative to 8-bit paths - this is a unit artifact of the throughput metric, not evidence
  that 16-bit decoding is intrinsically ~7-8x faster than 8-bit. A future candidate comparing
  16-bit costs should use per-pixel or per-sample time, not GiB/s, to avoid re-deriving this.
  gray16 also shows the largest pinned-vs-unpinned spread of the four gap paths (9.6-9.8 us pinned
  vs 12.1 us unpinned, ~25% slower unpinned) - worth re-checking with a dedicated pinned/unpinned
  comparison before trusting it as a real effect rather than scheduling noise on this run.
- APNG (8 frames, 128x128): decoding all 8 frames of the animation completes in ~37-40 us total,
  confirming `bench_apng_file`'s repeated-`next_frame` loop actually exercises every frame (not
  just frame 0) - checked directly by asserting `num_frames == 5` in the temporary sanity test
  (a different frame count than the bench's own 8, to make sure the assertion wasn't vacuously
  matching a hardcoded value).

This is a coverage/measurement result, not a speed result: nothing in the library's own decode
path was changed, so there is no "faster/slower than before" claim to make for any of these cases
- Impact is N/A by design (see SUMMARY.md's legend).

## Does it change behavior/output?

This is a harness-only addition to `benches/decoder.rs` and `src/test_utils.rs` (the latter is a
`#[cfg(test)]`-gated module, `pub(crate)`, already reused by both `src/decoder/stream.rs`'s own
unit tests and the bench crate via `#[path = ...]`) - **not** a change to the library's own decode
or encode logic in `src/decoder/`, `src/encoder/`, `src/filter.rs`, etc. `patch.diff` exists (per
SKILL.md, it is mandatory whenever tracked project source changes, even for a harness-only
addition like this one) and applies cleanly against `base_commit` (`git apply --check`, verified
in a separate throwaway worktree at `05ccdb9c`).

- **Bit-exact?** N/A - no existing decode/encode behavior changed. The pre-existing `decode`,
  `generated-noncompressed-*` and `row-by-row` benchmark groups are untouched (same 15 files listed
  by `cargo bench --bench decoder -- --list`, verified before and after) and every pre-existing
  function in `test_utils.rs` (`write_noncompressed_png`, `write_png_sig`, `write_chunk`,
  `write_rgba8_ihdr_with_width`, `generate_rgba8_with_width_and_height`, `write_rgba8_idats`,
  `write_iend`) is unmodified; all new code is appended after them.
- **Is this a breaking change?** No - additive only, no public API changed (both touched files are
  bench/test-only; `test_utils` is `pub(crate)`).
- Project test suite, `cargo test --release`, baseline (`05ccdb9c`, before this patch) vs patched
  (this patch applied): **107 passed, 1 ignored, 0 failed** in both cases (91+1+4+3+8 across the
  lib unit tests, `bugfixes.rs`, `check_testimages.rs`, `partial_decode.rs`, and doc-tests) -
  identical pass/fail counts, no new failures.
- A temporary `#[cfg(test)] mod scratch_coverage_verification` block was added to `test_utils.rs`
  during this session to verify the new generators produce correctly-decodable PNGs (grayscale at
  every bit depth is non-all-zero, palette likewise, 16-bit RGB decodes, every Adam7-decoded pixel
  has the expected alpha byte - which would not hold if any of the 7 pass sub-image dimensions were
  computed wrong - and a 5-frame APNG reports `num_frames == 5` and decodes all 5 frames without
  error). All 5 passed. This block was **removed before capturing `patch.diff`** - it exists only
  as a validation record here, not as a permanent addition to the project (per this skill's own
  scope: it adds output/hash verification harnesses to confirm nothing broke, not permanent unit
  tests to the project's source tree).

## Why the result looks the way it does

This candidate's own premise was a documentation/coverage gap, not a discovered inefficiency, so
there is no "why didn't it move" question to answer for a missing speedup - the point was to make
these four paths benchmarkable at all. The one substantive finding worth recording: the crate's own
`Encoder` cannot write interlaced (Adam7) images at all (`FormatErrorKind::InterlacedEncodingUnsupported`
in `src/encoder.rs`), so the Adam7 test image could not be produced via the public encoder API the
way the 16-bit/sub-byte/APNG cases were - it required hand-rolling the 7-pass sub-image layout
directly in `test_utils.rs` (store-only compression, same technique the existing
`write_noncompressed_png` already uses, just replicated per Adam7 pass with the standard
`(x_start, y_start, x_step, y_step)` geometry). This is the same reason the task description
suggested "lift from `tests/pngsuite/basi*.png`" as a fallback - those are the only *encoder-made*
interlaced fixtures already in the repo, but at a fixed 32x32 they are far smaller than this
harness's other synthetic cases (256x256), so a hand-generated 256x256 version was used instead for
comparable throughput-measurement granularity.

## What's not done / open questions

- The gray4 pinned-timing discrepancy noted above (36.64 us `--quick` vs ~43.8 us full runs) was
  not root-caused - it may just be `--quick`'s lower sample count producing a noisier estimate
  rather than a real effect; flagged for whoever benchmarks this case next rather than chased down
  here, since it is orthogonal to this candidate's own purpose.
- No file was added to `tests/benches/` itself - all new coverage lives as bench-time-generated
  synthetic PNGs (matching the existing `generated-noncompressed-*` pattern), so there is nothing
  to review as a binary diff; if a future session wants *real-world* (not synthetic) interlaced/
  16-bit/APNG files for the `corpus`-style benches, `tests/pngsuite/` and `tests/animated/` already
  have small (mostly 32x32) examples that could be copied in - not done here since they wouldn't
  add throughput-measurement value the synthetic 256x256 cases don't already provide.
- Did not attempt to make the crate's `Encoder` support Adam7 writing - that's a separate,
  much larger feature (noted here only because it's the reason the Adam7 test image had to be
  hand-rolled instead of encoder-generated like the others), not something this candidate scoped
  to touch.
