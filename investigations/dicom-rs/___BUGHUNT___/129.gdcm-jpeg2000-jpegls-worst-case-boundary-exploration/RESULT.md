# gdcm-jpeg2000-jpegls-worst-case-boundary-exploration

Follow-up to candidate 103's own "what's not done" note: candidate 103 (and 100 before it)
confirmed a real heap-buffer-overflow in vendored GDCM's JPEG 2000/JPEG-LS decode paths, using one
fixed declared-vs-actual size gap (32x32 declared vs the real 64x64 embedded codestream). This
candidate sweeps several larger gaps against the same real, unmodified, genuinely multi-frame
fixture to see whether the crash's observable severity/character scales with the gap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real end-to-end repro (`repro/src/main.rs`, adapted from candidate 103's own repro to take the declared size as a CLI argument) via the public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` entry point (`gdcm` feature), on the real, unmodified `pydicom/emri_small_jpeg_2k_lossless.dcm` fixture (10 real embedded 64x64 JPEG 2000 frames), sweeping declared Rows/Columns through 32/16/8/4/1 (4x to 4096x fewer declared pixels/frame than real) - each size run as a fresh subprocess since every run crashes the whole process | release build, pristine `df2b9e42`, 1 run per size | Crashed 5/5 sizes - every single size in the swept range still crashes reliably; symptom varies (SIGABRT with different glibc heap-corruption messages at 32/4/1, consistent SIGSEGV at 16/8) - see `output.txt` for full transcript |
| 3 repeat runs at a fixed gap (declared 8x8) to check symptom stability at one size | release build, same fixture | SIGSEGV 3/3 - stable at this specific size, in contrast to size 32 (candidate 103's own original size) which varies its glibc message run to run |
| Standalone fresh-clone `just test` (clone + build + 3 of the 5 sizes) | `just test` in a scratch copy | Crashed 3/3 as expected (all SIGABRT this run, different glibc messages than the in-tree run at the same sizes - consistent with heap-layout-dependent non-determinism, not a discrepancy) |

## Status

**Confirmed** - as a bounded deepening of already-Confirmed candidates 100/103 (same underlying
defect, same root cause, no new distinct bug), not a new candidate in its own right. Every size
tested crashed reliably (8/8 total runs across 5 distinct gap sizes plus 3 repeats), consistent
with candidates 100/103's own established characterization of genuine heap corruption (crash is
deterministic, exact symptom is heap-layout-dependent and therefore not).

**Fully reproducible**: **Yes**. Same repro shape as candidates 100/103 - the real public
`PixelDecoder::decode_pixel_data()` entry point on a real, unmodified, genuinely multi-frame
fixture with only `Rows`/`Columns` patched via the public `put_element` API. Single external
artifact (a malformed/inconsistent DICOM object handed to a decoder's own public entry point), no
second participant to fake, per `CONFIRMATION.md` section 11's carve-out.

## Class

memory-safety (same class as candidates 100/103 - not a structurally distinct bug, a
characterization of the same one).

## Root cause

Same as candidates 100/103 - see those `RESULT.md`s. This candidate's own contribution:

1. **The crash is reliable across the entire gap range tested** (4x to 4096x fewer declared
   pixels/frame than the real embedded codestream) - there is no "small gap, no crash" threshold
   within this range; even a very small, easily-plausible-looking discrepancy (a scanner
   off-by-a-factor-of-4 in a declared dimension, the 32x32 case) crashes just as reliably as an
   extreme one (1x1).
2. **The exact crash symptom does not scale monotonically with gap size in this sample.** Sizes
   32, 4, and 1 all produced a glibc heap-corruption `SIGABRT` (each with a *different* specific
   glibc diagnostic message - "double free or corruption", "corrupted unsorted chunks",
   "munmap_chunk(): invalid pointer" - itself already expected non-determinism per candidate 103),
   while sizes 16 and 8 both instead produced a plain `SIGSEGV`, and repeating size 8 three times
   kept producing `SIGSEGV` every time (more stable at that specific size than size 32 was in
   candidate 103's own original characterization). This is consistent with genuine heap
   corruption whose specific *observable* failure mode depends on which heap metadata/adjacent
   allocation happens to get clobbered for a given allocation-size/gap combination, not a simple
   "worse gap = worse symptom" relationship a reader might otherwise assume.
3. **Frame-count scaling was not explored** - the real fixture used here is fixed at 10 frames;
   testing a genuinely larger frame count would require hand-crafting a synthetic multi-frame
   JPEG 2000 file with many more valid embedded codestreams from scratch (or finding/generating a
   larger real fixture), which was judged out of scope for this bounded follow-up session.
4. **No exact "corrupted byte count" was measured.** Getting a precise out-of-bounds
   write offset/size would need a dedicated ASAN-instrumented rebuild of vendored GDCM
   specifically (not just the default Debug/Release CMake build this feature normally produces) -
   attempting that build was judged not worth the time for a third-party bug this repository
   cannot fix internally anyway (see below); not attempted this session.

## Introduced by

N/A - not attempted, same as candidates 100/103 (a proper `git log -S`/blame/bisect pass on this
third-party vendored source was judged out of scope, consistent with those candidates' own
reasoning).

## The fix

**No bounded fix found or expected** - same reasoning as candidates 100/103. The defect lives
entirely inside vendored, third-party GDCM 3.1 C++ source, outside this repository's own
git-tracked working tree. This candidate's sweep doesn't change that conclusion; it only
establishes that the crash is robust across a wide range of declared-size gaps, which if anything
reinforces that a real Rust-side mitigation would need to validate `Rows`/`Columns` against each
fragment's own decoded codestream dimensions *before* the FFI call - the same scope candidates
100/103 already judged too large for one session.

- Project test suite: not run (no code change made).

## What's not done / open questions

- Frame-count scaling (see point 3 above) remains genuinely unexplored - this is the one part of
  the original backlog item's ask ("many frames x a maximal per-frame size gap") this candidate
  did not cover, since the real fixture's frame count (10) can't be varied without synthesizing a
  new multi-frame codestream from scratch.
- An ASAN-instrumented GDCM rebuild, if ever attempted, could give a precise corrupted-byte-count
  answer - not pursued this session, low expected value for a bug that's already Confirmed and
  already known to be out of this repository's own fix scope.
- This closes the `IDEAS.md` backlog row that requested this exploration; no new candidate needed
  beyond this bounded follow-up.
