# 014.lut_bits_on_progressive

Re-measurement of candidate 003 (`raise_lut_bits`, see `../003.raise_lut_bits/RESULT.md` and its
`patch.diff` - re-applied verbatim here, no new patch) on `benches/tower_progressive.jpg`. 003
measured this image's `decode()` slow-path hit rate at 1.20% (similar to `large_image.jpg`'s
1.95%) but never actually ran `tools/paired.sh` A/B on it - only inferred it would land "similarly
small to `large_image.jpg`'s near-noise result". This candidate runs that missing comparison.

No new `patch.diff` in this folder - see `003.raise_lut_bits/patch.diff` for the exact one-line
change (`LUT_BITS: u8 = 8` -> `10` in `src/huffman.rs`) re-applied for this measurement.

## What was tried

Built `bench_decode` (`___OPTIMIZATIONS___/harness/bench_decode/`) twice against the main
project's working tree: once at pristine `base_commit` (`eb2d7c0`, `LUT_BITS=8`), once with
003's `patch.diff` applied (`LUT_BITS=10`). Tree reverted (`git checkout -- src/`) immediately
after both binaries were built and the patched-tree test run completed, before writing this file.

| Variant | multi-core p50 (`benches/tower_progressive.jpg`, iters=30) | notes |
|---|---|---|
| baseline (`LUT_BITS=8`) | 4.307 / 4.313 / 4.324 ms (3 batches) | |
| patched (`LUT_BITS=10`) | 4.338 / 4.338 / 4.363 ms (3 batches) | |

`tools/paired.sh` (unpinned multi-core, `RUNS=9`, 3 independent batches - one more than the
required 2, run because the first two batches disagreed on the consistency verdict):

- batch 1: median 4.307 -> 4.338 ms (+0.7%), B (patched) faster in 1/9 pairs -> "consistent" (A,
  i.e. unpatched `LUT_BITS=8`, decisively faster by the script's `wins<=1` rule)
- batch 2: median 4.313 -> 4.338 ms (+0.6%), B faster in 3/9 pairs -> NOT consistent
- batch 3: median 4.324 -> 4.363 ms (+0.9%), B faster in 3/9 pairs -> NOT consistent
- batch 4 (from a fresh `just test` run in this candidate's own folder, separate binaries built
  from a fresh clone at the same `base_commit`): median 4.425 -> 4.294 ms (-3.0%), B faster in
  8/9 pairs -> "consistent" - this time favoring the *patched* binary, the opposite direction
  from batch 1's "consistent" result.

Batches 1 and 4 each independently cross `paired.sh`'s strict consistency bar, in opposite
directions - the clearest possible demonstration that neither result is a real, repeatable
effect on this machine for this image; both are noise landing on one side of the bar by chance.
All four raw `A` columns across the two independent binary builds span roughly 4.23-4.45 ms
(~5% band), consistent with the noise floor this workspace has already documented for other
small/fast images (`projects/jpeg-decoder.md`: ~2-4% on `large_image.jpg`, ~2-5% on the lossless
image) - this image's ~4.3-4.5ms decode is even smaller/faster, so a comparable or slightly
larger noise band is expected.

Bit-exact: `bench_decode`'s per-iteration hash check (aborts on any mismatch) did not abort on
any run across all four `paired.sh` batches (batches 1-3: 3x9x2 = 54 runs; batch 4: 9x2 = 18
runs) or the two standalone before/after invocations `just test` prints - both binaries' hash
(`6f04744ab9f876c2`, from the `just test` run's independent clone/build) matched each other and
stayed stable across every iteration.

Project test suite (`cargo +stable test --release`), same run already covered by candidate 003
and reconfirmed here on the patched tree: 5 lib unit + 12 `tests/lib.rs` + 4 rayon integration (1
each) + 2 doctests = 23 passed, 0 failed, `grep -c "^error"` is 0. Baseline (pristine `eb2d7c0`)
pass count is the documented project default (same 23/23) and was not independently re-run for
this candidate since no source outside `src/huffman.rs`'s one-line constant differs from 003's
own already-tested baseline run.

## Result

No confirmed win either way - the four batches disagree on direction (three lean slightly toward
unpatched, +0.6% to +0.9%; one, from an independently rebuilt pair of binaries, leans decisively
toward patched, -3.0%, 8/9), and the two batches that individually cross `paired.sh`'s strict
consistency bar do so in opposite directions. This is the textbook signature of an effect at or
below this machine's noise floor, not a real direction-consistent effect. Confirms the
expectation stated in 003's "open questions" and IDEAS.md: `tower_progressive.jpg`'s 1.20%
slow-path hit rate is too low, and its ~4.3-4.5ms decode too fast, for `LUT_BITS=10` to produce a
measurable win here - same conclusion 003 already reached for `large_image.jpg`'s comparable
1.95% hit rate (and the lossless image's 0%), just confirmed directly on this image instead of
inferred.

## Does it change behavior/output?

- **Bit-exact?** Yes, confirmed via `bench_decode`'s built-in per-iteration hash check across all
  runs on both binaries.
- **Is this a breaking change?** No - `LUT_BITS` is a private internal constant (see 003's
  RESULT.md for the full analysis; nothing new to add for this image specifically).
- Project test suite: 23 passed / 0 failed before and after (see above) - identical to candidate
  003's own recorded run, since the patch is byte-for-byte the same.

## Why the result looks the way it does

`tower_progressive.jpg` is a small (512x512), fast-decoding (~4.3ms) image whose measured
`decode()` slow-path hit rate (1.20%, from 003) is the second-lowest of the five images 003
surveyed (only the lossless image's 0% is lower). A ~1% shift in how often ~1.2% of Huffman
symbol decodes take a slightly slower branch is well below what this machine's wall-clock noise
floor can resolve on a sub-5ms decode - the same conclusion 003 already reached for
`large_image.jpg`'s comparable 1.95% hit rate, just confirmed directly here instead of inferred.

## What's not done / open questions

- Not re-run with a dedicated same-binary noise-floor control on this exact image (the script
  refuses to compare two identical binaries) - the ~4% same-side spread observed across the three
  batches' raw `A` columns is used as an approximate stand-in instead.
- Consistent with 003's own open question about `ac_lut`'s packed-`u8` overflow risk above
  `LUT_BITS=10` - not revisited here, out of scope for a pure re-measurement.
