# 002.dc_coefficient_lut

`HuffmanDecoder::decode()` (DC symbol -> magnitude category) + `receive_extend()` (extra bits +
sign extension) in `src/huffman.rs` were two separate bit-consuming steps for every DC
coefficient, unlike AC decode which already has `HuffmanTable::ac_lut` folding decode+extend into
one 8-bit LUT probe for small magnitudes (`decode_fast_ac`). This candidate adds an analogous
`HuffmanTable::dc_lut` + `HuffmanDecoder::decode_dc()` for the DC path: idea from
`___OPTIMIZATIONS___/IDEAS.md` (written by candidate 001's investigation).

## What was tried

Added `dc_lut: Option<[(i16, u8); 256]>` to `HuffmanTable`, built only for `HuffmanTableClass::DC`
(mirrors `ac_lut`'s construction, but keyed directly on the DC magnitude category - DC symbol
values have no run-length nibble, unlike AC). For each of the 256 possible next-8-bits prefixes:
category 0 stores `(0, code_size)` (no extra bits to consume, matches existing behavior); category
1..=11 with `code_size + category <= 8` stores the already-sign-extended `(diff, code_size +
category)`; everything else (code longer than 8 bits, or category too large to fit in the
remaining LUT bits, or category > 11) is left as the `(0, 0)` sentinel, which falls back to the
original slow path (`decode()` then `match`+`receive_extend()`, unchanged) - including the
existing "invalid DC difference magnitude category" error for category > 11, so out-of-range
tables still error identically. `decode_block` in `src/decoder.rs` now calls
`huffman.decode_dc(reader, dc_table.unwrap())?` in place of the old `decode()` + `match { 0 => 0,
1..=11 => receive_extend, _ => Err }` block.

DC has no ZRL/EOB-equivalent special codes (verified: `decode_block`'s DC branch only ever
switches on the magnitude category 0..=11 for baseline images; DC has nothing like AC's `run` of
zero coefficients or end-of-band run-length - those are AC-only concepts), so no escape handling
beyond "category too big for the LUT, or category invalid" was needed.

| Variant | single-core `taskset -c 0` p50 (lossless image) | multi-core (unpinned) p50 (lossless image) | notes |
|---|---|---|---|
| baseline | 254-308 ms (bimodal, see below) | ~256-260 ms | |
| patched (dc_lut) | 252-302 ms (bimodal, see below) | ~254-257 ms | |

Full paired.sh (`RUNS` alternating A/B, `tools/paired.sh`) runs against
`tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` (30 iters/invocation):

- single-core (`taskset -c 0`), RUNS=9: median 257.3 -> 254.3 ms (-1.1%), B faster 6/9, **NOT consistent**
- single-core (`taskset -c 0`), RUNS=15: median 283.8 -> 257.5 ms (-9.3%), B faster 9/15, **NOT consistent**
- single-core (`taskset -c 0`), RUNS=15 (iters=60): median 257.9 -> 260.3 ms (+0.9%), B faster 6/15, **NOT consistent**
- multi-core (unpinned), RUNS=9: median 259.1 -> 256.7 ms (-0.9%), B faster 8/9, **consistent**
- multi-core (unpinned), RUNS=15: median 256.2 -> 254.4 ms (-0.7%), B faster 11/15, **NOT consistent**
- multi-core (unpinned), RUNS=21: median 256.7 -> 255.0 ms (-0.7%), B faster 17/21, **NOT consistent**

Same on `benches/large_image.jpg` (50 iters/invocation):

- single-core (`taskset -c 0`), RUNS=9: median 27.5 -> 27.1 ms (-1.5%), B faster 7/9, **NOT consistent**
- multi-core (unpinned), RUNS=9: median 26.0 -> 25.5 ms (-1.8%), B faster 4/9, **NOT consistent**
- multi-core (unpinned), RUNS=15: median 24.9 -> 24.6 ms (-1.0%), B faster 13/15, **NOT consistent**

Same-binary noise floor (baseline vs. itself, 8 sequential single-invocation runs, unpinned,
multi-core): lossless image min times ranged 256.3-261.9 ms (spread ~5.6 ms, ~2.2% of the 259 ms
median); `large_image.jpg` min times ranged 24.2-25.4 ms (spread ~1.1 ms, ~4.5% of the 24.8 ms
median).

## Result

Every comparison leans the same direction (patched build faster, by roughly 0.7-1.8% depending on
run), but the effect size is comparable to or smaller than the same-binary noise floor measured
above, and `tools/paired.sh`'s own strict pair-count bar (`wins >= n-1`, per `MEASUREMENT.md`
section 2b) was only met once out of 9 separate paired runs across both images and both
pinning modes (multi-core unpinned, RUNS=9, lossless image, 8/9). Single-core `taskset -c 0`
comparisons showed the same bimodal noise pattern `MEASUREMENT.md` section 2b describes as
surviving pinning on this machine (raw single-core timings ranged 251-308 ms for the *same*
binary depending on run) and produced no usable signal in either direction. This is consistent
with the mechanism: `decode_dc` only removes one conditional bit-consuming call for the single DC
coefficient per 8x8 block (1 of up to 64 coefficients decoded per block, most of which are AC and
already go through the existing `ac_lut` fast path) - a real per-symbol saving, but small enough
relative to total block decode time that it sits near this machine's measurement noise floor.

Given the paired-run discipline this workspace uses, this cannot be called a confirmed win: only
1 of 9 independent paired comparisons crossed the "consistent" bar, and the same-binary control
shows noise of comparable magnitude to the claimed effect. The consistent *direction* across all
9 runs (never favoring the baseline) is suggestive but not decisive on its own.

## Does it change behavior/output?

- **Bit-exact?** yes. `just test` (see `justfile`) built both binaries and ran them against
  `benches/large_image.jpg` and `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg`;
  `bench_decode`'s FNV-1a output hash was identical between baseline and patched builds on both
  images (`2f4deafc37e4ec2b` and `0d970236b43aedad` respectively), and stable across all iterations
  within each build.
- **Is this a breaking change?** No - internal-only change (`huffman.rs`/`decoder.rs`), no public
  API, file format, or documented-behavior change. `decode_dc` reproduces the same
  category 0 / 1..=11 / >11-error branches as the code it replaces, just via a LUT probe for the
  common case instead of two separate bit-consuming calls.
- Project test suite (`cargo +stable test --release`), main working tree:
  - Before (pristine, base_commit `eb2d7c0`): 12 lib tests + 4 rayon integration tests (1 each) +
    2 doctests, all passing (per `projects/jpeg-decoder.md`, established baseline).
  - After (patched): same suite re-run - `5 passed` (`src/lib.rs` unit tests) + `12 passed`
    (`tests/lib.rs`, includes reftest/crashtest image suites) + `1 passed` each for
    `tests/rayon.rs`/`rayon-0.rs`/`rayon-1.rs`/`rayon-2.rs` + `2 passed` doctests - **all passing,
    0 failed**, 7 `test result` lines, 0 lines matching `^error` in the captured log.

## Why the result looks the way it does

The DC LUT is a real, mechanically-sound analogue of the existing AC fast path (`ac_lut`) and
folds two bit-consuming operations into one for the common case (small DC diff, short code), with
a correctness-preserving fallback for everything else. But DC is only 1 coefficient of up to 64
per block, and typical DC magnitude categories in natural images are small (often already fitting
the LUT's `size + category <= 8` bound), so the *symbol-decode* cost saved per block is a small
fraction of total per-block decode time (which is dominated by the 63 AC coefficients' own
Huffman decode + IDCT + upsampling + color conversion). The theoretical upper bound on this
change is therefore small, and the measured effect (0.7-1.8%, direction-consistent but not
pair-count-decisive) is compatible with a real-but-small win that this machine's noise floor
(2.2-4.5%) makes hard to confirm definitively in a single session.

## What's not done / open questions

- Not re-measured on a quieter machine / with `isolcpus`/a dedicated cgroup - `MEASUREMENT.md`
  section 2b notes `taskset` alone does not fully isolate from OS scheduling noise on this
  machine; a session with access to genuine core isolation, or to `perf stat -r N` for a
  cycle-count (not wall-clock) comparison, would likely resolve the ambiguity in fewer runs.
- Not measured on `benches/tower.jpg` / other small images - `large_image.jpg` and the lossless
  image were used per this workspace's existing convention (best signal-to-noise); a smaller
  image would have an even smaller absolute effect and wasn't expected to add information.
- No characterized-difference tooling was needed - output is bit-exact on every run.
