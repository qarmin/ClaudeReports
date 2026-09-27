# Write bit groups in bulk in the Adam7 sub-byte expansion (src/adam7.rs, `expand_pass`)

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

`expand_pass`'s 1/2/4-bpp arms carried their own TODO: "for 1, 2, 4 multiple runs through the
iteration will access the same byte in `img` ... A better strategy would write multiple bit groups
in one go." Each sample was written with its own `img[pos.byte] |= px << shift` (one bounds check
+ one read-modify-write per pixel), even on the many Adam7 passes where several samples from the
same row pack into one output byte.

Key fact used: in the Adam7 pass-constants table, `x_offset < x_sampling` always holds. That
guarantees `base_bit = x_offset * bits_pp < x_sampling * bits_pp = stride_bits`, so whenever
`stride_bits < 8` (i.e. more than one sample per byte), every group of `n = 8 / stride_bits`
consecutive samples fills exactly one output byte with no partial/spilling byte at a row's start.
This means the byte value can be built up locally (shift + OR into a local `u8`) and written to
`img[byte]` once per group instead of once per sample.

## What was tried

Added a private helper `expand_subbyte_pass_grouped` used by the `1 | 2 | 4` arms of `expand_pass`:
computes `stride_bits` for the current pass/bpp; if `>= 8` (no aliasing - some pass/bpp
combinations still map 1:1, or even sparser), falls back to the original per-sample write; if `< 8`,
combines every `n`-sample group into one local `u8` and does a single `img[byte] |= combined`
write per group. `expand_pass_splat` (the other function with the same TODO comment, for the
`Adam7Variant::Splat` output mode) was intentionally left untouched - out of scope for this
candidate (different x_repeat/y_repeat splatting logic, would be its own candidate).

Benchmark: existing `benches/adam7.rs` (criterion, `--features benchmarks`), filtered to
`bpp=[124]$` (the 3 sizes - 16, 256, 4096 - for bpp 1, 2, 4; bpp 8/16/24/32 are untouched by this
patch and were left out of the comparison). Two baseline runs (`--save-baseline` run twice,
comparing the second against the first) established the noise floor before touching the patch;
then patched runs compared against the surviving baseline via criterion's own `--baseline` A/B
comparison.

Single-core (`taskset -c 0`):

| bpp | size | baseline p50 | patched p50 | change |
|---|---|---|---|---|
| 1 | 16 | 366.86 ns | 347.76 ns | -5.3% |
| 1 | 256 | 64.05 µs | 56.29 µs | -12.9% |
| 1 | 4096 | 16.03 ms | 13.72 ms | -14.4% |
| 2 | 16 | 365.43 ns | 323.56 ns | -11.9% |
| 2 | 256 | 61.81 µs | 48.82 µs | -21.1% |
| 2 | 4096 | 15.22 ms | 12.12 ms | -20.4% |
| 4 | 16 | 308.65 ns | 308.83 ns | +0.3% (noise, p=0.07) |
| 4 | 256 | 52.21 µs | 50.43 µs | -2.8% |
| 4 | 4096 | 13.35 ms | 12.84 ms | -3.8% |

Multi-core (unpinned, realistic wall-clock):

| bpp | size | baseline p50 | patched p50 | change |
|---|---|---|---|---|
| 1 | 16 | 364.43 ns | 339.03 ns | -6.6% |
| 1 | 256 | 63.74 µs | 54.74 µs | -14.2% |
| 1 | 4096 | 15.77 ms | 13.60 ms | -13.8% |
| 2 | 16 | 362.63 ns | 321.42 ns | -11.2% |
| 2 | 256 | 60.57 µs | 48.34 µs | -20.2% |
| 2 | 4096 | 15.06 ms | 12.12 ms | -19.5% |
| 4 | 16 | 304.52 ns | 305.52 ns | +0.6% (noise) |
| 4 | 256 | 50.63 µs | 50.20 µs | -0.2% (noise) |
| 4 | 4096 | 12.67 ms | 12.51 ms | -1.3% |

Baseline-vs-baseline noise floor (2nd baseline run compared against the 1st, no code change):
mostly within ±0.5-2%, one outlier at 4096/bpp=4 showing +3.8% (p=0.02, wide CI up to 8%) - this is
why the bpp=4 patched deltas at 4096 (-3.8% single-core / -1.3% multi-core) are reported as real
but modest, not as clean as the bpp=1/2 numbers.

Independently re-run via this candidate's own `just test` (fresh clone, `cargo bench`'s built-in
before/after comparison rather than two separate saved-baseline files) - confirms the same
magnitude: bpp=1 -12% to -13%, bpp=2 -20% to -21%, bpp=4 -1.5% to -3%.

## Result

**Time.** Strong, consistent win for bpp=1 and bpp=2 across all three sizes (roughly -5% to -21%,
single- and multi-core agree closely). bpp=4 shows a small but real win at larger sizes (-2.8% to
-3.8%) and is indistinguishable from noise at the smallest size (16x16). This matches the theory:
for bpp=1 and bpp=2, most of the 7 Adam7 passes have `x_sampling * bpp < 8` and therefore benefit
from grouped writes; for bpp=4, only pass 7 (`x_sampling == 1`) aliases two samples per byte, so
only a small fraction of the full image's rows benefit, diluting the average.

## Does it change behavior/output?

- **Bit-exact:** yes. The grouped writes only change how output bytes are assembled internally
  (accumulate `n` shifted values into a local `u8`, then one `|=`), not what final bits are OR'd
  into `img` - the derivation above establishes each group maps to exactly one byte with the same
  bit positions the original per-sample loop used.
- **Is this a breaking change?** No - `expand_pass` is a public function but its signature and
  documented behavior are unchanged; the change is purely internal.
- Project test suite: **107 passed, 1 ignored, 0 failed**, identical before and after the patch
  (`cargo test --release`: 91 unit tests in `src/`, 1 in `tests/bugfixes.rs`, 4 in
  `tests/check_testimages.rs`, 3 in `tests/partial_decode.rs`, 8 doc-tests - same counts,
  same "ok" results, both runs). `test_expand_pass_subbyte` in particular directly exercises
  `expand_pass` for every one of the 7 Adam7 passes at bpp=1, including the pass-7 case where the
  most aliasing (8 samples/byte) occurs.

## Why the result looks the way it does

The win is real bounds-check/store-count reduction, not vectorization or algorithmic change: for
bpp=1 pass 7 for instance, 8 samples that used to each do their own `img[byte] |= px << shift`
(8 bounds checks, 8 read-modify-writes to the same memory location) now become 1 local
accumulation (no memory traffic until the end) + 1 bounds-checked `img[byte] |= combined`. The
benefit scales with how many samples pack per byte (`n`), which is why bpp=1's largest group
(n=8, pass 7) sees a bigger win than bpp=2 (max n=4) and bpp=4 (max n=2, only on pass 7).

## What's not done / open questions

- `expand_pass_splat` (the `Adam7Variant::Splat` output mode) has the identical TODO comment and
  the identical underlying opportunity, but a different (repeat-aware) write pattern - not
  touched here; a natural follow-up candidate.
- Not profiled with `perf stat` for actual branch-misprediction/bounds-check counts - the criterion
  timing delta is taken as sufficient evidence given how directly it matches the theoretical
  n-samples-per-byte prediction per bpp.
- Only measured square power-of-two image sizes (16, 256, 4096) from the existing bench harness;
  no reason to expect very different behavior for other sizes given the mechanism is per-row/
  per-byte, not size-dependent.
