# 003.raise_lut_bits

`LUT_BITS` (src/huffman.rs:11) gates both the generic `HuffmanTable::lut` used by
`HuffmanDecoder::decode()` and the AC fast-path `ac_lut` used by `decode_fast_ac()`. Any Huffman
code longer than `LUT_BITS` bits (or, for `ac_lut`, whose code-length + magnitude-category exceeds
`LUT_BITS`) falls through to a slower bit-by-bit scan (`decode()`'s `for i in LUT_BITS..16` loop).
Candidate 002's investigation flagged this as worth measuring before deciding whether to raise it.

## Slow-path hit rate (measured first, before deciding on a patch)

Added a temporary atomic counter in `decode()`'s slow branch (incremented whenever
`table.lut[..].size == 0`, i.e. the code did not fit in `LUT_BITS` bits) and one in
`decode_fast_ac()`'s `ac_lut` miss branch, printed via `eprintln!` from `Decoder::decode()` when
`BENCH_COUNT` is set. Reverted before measuring/patching (not part of `patch.diff`).

| image | `decode()` calls | slow-path hits | slow-path rate | `ac_lut` miss rate |
|---|---|---|---|---|
| `benches/large_image.jpg` | 374,026 (x3 iters shown, rate stable) | 7,300 | 1.95% | 28.9% |
| `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` | 18,874,368 | 0 | 0.00% | n/a (lossless mode never builds/uses `ac_lut`) |
| `benches/tower.jpg` | 32,637 | 1,440 | 4.41% | 23.5% |
| `benches/tower_progressive.jpg` | 81,989 | 986 | 1.20% | 22.7% |
| `benches/tower_grayscale.jpg` | 14,679 | 1,049 | 7.15% | 17.0% |

`ac_lut` "miss" mostly means the symbol's magnitude category was 0 (EOB/ZRL, which `ac_lut` never
handles by design, not a `LUT_BITS` shortfall) - the real `LUT_BITS`-limited slow path is the
`decode()` row above. It is small (~1-7% of symbol decodes) but non-zero on every baseline/
progressive image tried, and exactly zero on the lossless-mode image (lossless coding uses only
short DC-style tables, never exercises this path at all - a useful built-in control).

Given a real, if modest, hit rate on baseline JPEGs, raised `LUT_BITS` from 8 to 10 and measured
rather than writing this off outright. Chose 10 rather than 12+ because `ac_lut`'s packed
`(i16, u8)` entry stores `(run_length << 4) | (size + magnitude_category)` in the `u8`, which only
has headroom for `size + magnitude_category <= 15`; AC magnitude category maxes out at 10, so
`LUT_BITS = 10` (max sum 10) stays safely under that limit while `LUT_BITS = 12` (max sum 12,
still safe) to `16` would need re-deriving - see "open questions" below, this is a real latent
packing constraint independent of this candidate's choice.

## What was tried

Table sizes: `lut: [(u8,u8); 1<<LUT_BITS]` and `ac_lut: Option<[(i16,u8); 1<<LUT_BITS]>`, built
once per `HuffmanTable` (once per DHT segment, not per block/symbol). At `LUT_BITS=8`: 256 entries
(512 B + ~1024 B with padding). At `LUT_BITS=10`: 1024 entries (2048 B + ~4096 B with padding),
i.e. a few KB more per table, up to 4 tables per image (2 DC + 2 AC) - trivially small, and the
fill loop that builds them is still O(2^LUT_BITS) run once per table, not per block, so build-time
cost is also negligible (microseconds).

Measured with `tools/paired.sh` (unpinned, multi-core, `RUNS=9`), `bin_baseline_003` (LUT_BITS=8)
vs `bin_patched_003` (LUT_BITS=10):

| image | slow-path rate | median shift | paired wins | verdict |
|---|---|---|---|---|
| `benches/large_image.jpg` (run 1) | 1.95% | 23.493 -> 23.213 ms (-1.2%) | 9/9 | consistent |
| `benches/large_image.jpg` (run 2) | 1.95% | 24.161 -> 23.934 ms (-0.9%) | 7/9 | not consistent (strict bar) |
| `benches/large_image.jpg` (run 3) | 1.95% | 24.156 -> 23.999 ms (-0.6%) | 7/9 | not consistent (strict bar) |
| `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` | 0.00% | 249.491 -> 248.947 ms (-0.2%) | 4/9 | not consistent (expected - mechanism doesn't apply) |
| `benches/tower.jpg` | 4.41% | 2.110 -> 2.078 ms (-1.5%) | 9/9 | consistent |
| `benches/tower_grayscale.jpg` | 7.15% | 0.659 -> 0.621 ms (-5.8%) | 8/9 | consistent |

Same-binary noise floor on this machine (documented in `projects/jpeg-decoder.md` from candidate
002, reconfirmed here with 4 sequential same-binary runs before patching): ~2-4% spread on
`large_image.jpg`, ~2-5% on the lossless image.

## Result

The effect size tracks the measured slow-path hit rate almost exactly: essentially zero on the
lossless image (0% hit rate, medians and pair counts both indistinguishable from noise), small on
`large_image.jpg` (1.95% hit rate; -0.6% to -1.2% across three repeats, always leaning faster,
never regressing, but only 2 of 3 repeats cleared the strict 9/9-or-8/9 "consistent" bar and the
effect is at/below the machine's own noise floor), and clearly real on the two images with higher
hit rates: `tower.jpg` (4.41% hit rate, -1.5%, 9/9) and `tower_grayscale.jpg` (7.15% hit rate,
-5.8%, 8/9) - both decisive per `MEASUREMENT.md`'s pair-count rule and well above the noise floor.

Net: a real, direction-consistent, small-to-moderate win whose magnitude depends on how often a
given image's Huffman tables produce codes longer than 8 bits (or longer than 8 bits combined with
AC magnitude category) - negligible-to-small on the two designated primary workload images
(`large_image.jpg` 1.95% hit rate, the lossless image 0% hit rate), but a clear several-percent win
on other real workload images already in the repo (`tower.jpg`, `tower_grayscale.jpg`) that happen
to hit the slow path more often.

## Does it change behavior/output?

- **Bit-exact?** yes - `bench_decode`'s per-iteration output hash check (aborts on any mismatch)
  passed on every run of every image above, for every iteration, both baseline and patched.
- **Is this a breaking change?** no - `LUT_BITS` is a private internal constant, not part of the
  public API; decoded output is unchanged.
- Project test suite: 23/23 passing before (5 lib unit + 12 `tests/lib.rs` + 4 rayon integration +
  2 doctests) -> 23/23 passing after. No new failures, `grep -c "^error"` is 0 in both logs.

## Why the result looks the way it does

The AC fast path (`ac_lut`) already absorbs the large majority of AC symbols (its "miss" rate of
17-29% mostly reflects EOB/ZRL category-0 codes, which `ac_lut` never services by design, not a
`LUT_BITS` shortfall). What's left over - the genuine `decode()` slow bit-scan - is a small,
image-dependent fraction of total symbol decodes (1.2-7.2% across the five sample images), and
`decode()` itself is only one part of total decode time (parsing, IDCT, upsampling, color convert
also cost time), so the realistic ceiling on this fix is a few percent, exactly what was measured.
It is real (direction is consistent everywhere the mechanism applies, and is a clean zero on the
one image where it structurally cannot apply), just smaller than the machine's per-run noise floor
on the two images this workspace treats as primary.

## What's not done / open questions

- Only `LUT_BITS=10` was measured. A higher value (12+) might capture more of the slow-path 1.95%
  on `large_image.jpg`, but **`ac_lut`'s packed `u8` (`(run_length << 4) | (size +
  magnitude_category)`) only has 4 bits of headroom for `size + magnitude_category`, i.e. it
  silently corrupts (wraps into the run-length nibble) once that sum exceeds 15.** AC magnitude
  category maxes at 10, so `LUT_BITS` up to 10 keeps the max sum at 10 (safe); `LUT_BITS=12` keeps
  the max sum at 12 (still safe, not yet tested); anything higher would need the `ac_lut` entry
  widened (e.g. two separate `u8`/`u16` fields instead of one packed nibble split) before it could
  be raised further without a real correctness bug. Flagged as a new `IDEAS.md` entry rather than
  fixed here since it's out of scope for this candidate's chosen value.
  - Note this is a **latent bug in the existing (unpatched) code too**, not something this patch
    introduces: nothing currently stops a future edit from raising `LUT_BITS` past 15 without
    noticing the packing constraint. It just happens not to trigger at `LUT_BITS<=8` (max sum 8)
    or at this candidate's `LUT_BITS=10` (max sum 10).
  - Not filed under `___OTHER___/` since it isn't reachable at present (`LUT_BITS` is a fixed
    constant, not attacker-controlled), and this candidate's own patch does not trigger it - but a
    maintainer raising `LUT_BITS` further without reading this note would hit it.
- Did not try compile-time-deriving the smallest safe `LUT_BITS` per table (i.e. only paying the
  larger table for tables that actually have long codes) - always uses one fixed constant for
  every table.
- Did not measure `benches/tower_progressive.jpg` with `paired.sh` (only got its slow-path rate,
  1.20%, lower than `tower.jpg`'s baseline table) - expected to land similarly small to
  `large_image.jpg` given the similar hit rate, not run for time.
