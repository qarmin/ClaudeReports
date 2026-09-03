# rotate90_in / rotate270_in: blocked-transpose `BLOCK` tile-size sweep

Candidate 007 added a 64x64-tile blocked-transpose fast path to `rotate90_in`/`rotate270_in`
(`src/imageops/affine.rs`) and measured a much larger win for `rotate270_in` (~37-42%) than for
`rotate90_in` (~11%) with a fixed `BLOCK = 64`, chosen by analogy to a typical L1-friendly tile
without measuring alternatives. This candidate reuses that exact patch unchanged
(`patch.diff` here is identical to `007.AFFINE_ROTATE90_270_BLOCKED_TRANSPOSE/patch.diff`) and
sweeps `BLOCK` in `{32, 64, 128, 256}` to see whether either function's win grows, or whether the
asymmetry narrows, at a different tile size.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidate 007). Benchmark: the
same existing `benches/imageops.rs` `rotate90`/`rotate270` criterion cases candidate 007 used - no
bench file changes needed. Four release criterion binaries were built from the identical patch,
each with only the `const BLOCK: usize = N;` literal (both occurrences, `rotate90_in` and
`rotate270_in`) changed to 32/64/128/256, then compared pairwise against the 64 binary via
alternating A/B runs (same paired-measurement discipline as candidate 007; done by hand with a
small helper script in `tmp/run_pair.sh`, not the repo's `tools/paired.sh` which is adapted for a
different bench's stdout shape).

This machine's load average climbed from ~1.2 at session start to ~4-7 during this sweep (a
noisier window than candidate 007's session, consistent with `IDEAS.md`'s existing note that this
host's `/proc/loadavg` is host-wide, not container-scoped) - absolute per-run times here are
therefore not comparable to candidate 007's own absolute numbers, only the ratios and pair-win
counts within this session's own paired runs are meaningful.

**`rotate90`, single-core (`taskset -c 3`), 6 paired runs, ms - BLOCK=64 (A) vs alternative (B):**

| B value | median A (64) | median B | ratio B/A | A(64) faster in |
|---|---|---|---|---|
| 32 | 2.191 | 5.068 | +131.3% | 6/6 |
| 128 | 2.255 | 3.513 | +55.8% | 6/6 |
| 256 | 2.312 | 2.464 | +6.6% | 4/6 |

**`rotate90`, multi-core (unpinned), 4 paired runs, ms:**

| B value | median A (64) | median B | ratio B/A | A(64) faster in |
|---|---|---|---|---|
| 32 | 1.064 | 1.189 | +11.8% | 4/4 |
| 128 | 1.063 | 1.121 | +5.4% | 4/4 |
| 256 | 1.089 | 1.226 | +12.6% | 4/4 |

**`rotate270`, single-core (`taskset -c 3`), 6 paired runs, ms:**

| B value | median A (64) | median B | ratio B/A | A(64) faster in |
|---|---|---|---|---|
| 32 | 1.123 | 1.744 | +55.4% | 6/6 |
| 128 | 1.133 | 2.280 | +101.2% | 6/6 |
| 256 | 2.228 | 2.232 | +0.2% | 3/6 |

**`rotate270`, multi-core (unpinned), 4 paired runs, ms:**

| B value | median A (64) | median B | ratio B/A | A(64) faster in |
|---|---|---|---|---|
| 32 | 1.175 | 2.831 | +141.0% | 4/4 |
| 128 | 1.175 | 1.527 | +30.0% | 4/4 |
| 256 | 1.123 | 1.281 | +14.1% | 4/4 |

("ratio B/A" > 0% means the alternative is slower than BLOCK=64; A(64) faster in N/M is the raw
paired-win count, the decisive number per this workspace's `paired.sh` convention.)

## Result

`BLOCK = 64` (candidate 007's original choice) is at least as fast as, and in most comparisons
substantially faster than, every alternative tested (32/128/256), for both `rotate90_in` and
`rotate270_in`, on both single-core and multi-core measurements:

- `BLOCK = 32` is dramatically worse for both functions (+11.8% to +141.0% slower than 64,
  6/6 or 4/4 paired losses in every case) - too small a tile multiplies per-tile loop overhead
  relative to the work done per tile.
- `BLOCK = 128` is consistently worse for both functions (+5.4% to +101.2% slower, 6/6 or 4/4
  paired losses in every case).
- `BLOCK = 256` is a wash on single-core (`rotate90`: 4/6 favor 64 but only +6.6%; `rotate270`:
  3/6, +0.2%, i.e. no measurable difference) but consistently worse on multi-core for both
  functions (+12.6% and +14.1%, 4/4 paired losses each) - likely 256x256 tiles (256 * 256 *
  channel-count bytes) starting to exceed a cache level that 64x64 fits inside more comfortably
  with other cores' traffic also competing for it, an effect the single-core measurement doesn't
  capture.

No alternative narrowed the `rotate90_in` (~11%) vs. `rotate270_in` (~37-42%) asymmetry candidate
007 found - if anything, the two functions responded in the same direction and in similar relative
proportion to a worse `BLOCK` (e.g. `BLOCK=128` cost `rotate90` +55.8% and `rotate270` +101.2%
single-core - both got worse, `rotate270` proportionally more so, same asymmetry direction as
007's original finding, not a different one), consistent with the asymmetry being about which axis
gets held fixed during the scan (007's own explanation) rather than about the tile size chosen.

## Does it change behavior/output?

- **Bit-exact?** Yes, for all four `BLOCK` values tested. `cargo test --release --lib
  imageops::affine` (8 tests: `test_rotate90`, `test_rotate180`, `test_rotate270`,
  `test_rotate180_in_place`, plus 4 more in the same module) passes with 0 failures at
  `BLOCK` = 32, 128, and 256, same as candidate 007 already confirmed for 64 - the boundary-tiling
  logic (`(bx + BLOCK).min(w0u)` / `(by + BLOCK).min(h0u)`) handles a tile size larger or smaller
  than the image itself correctly in every case tried.
- Full project test suite (`cargo test --release`, all suites): 550 passed, 1 failed both before
  (unpatched) and after (patched, `BLOCK = 64` - the value that stays in `patch.diff`) - identical
  single pre-existing failure both times (`tests/regression.rs::resizing_with_catmul`, unrelated to
  `affine.rs`, already noted in candidate 007's `RESULT.md`). The full suite was not re-run
  separately for BLOCK=32/128/256 since the module-level `imageops::affine` tests already establish
  those variants are behaviorally identical to BLOCK=64 (same code, only the tiling literal
  differs) and don't touch anything else in the crate.
- **Is this a breaking change?** No - this candidate makes no source change beyond what candidate
  007 already characterized; `patch.diff` here is identical to 007's.

## Why the result looks the way it does

64x64 pixels is small enough that a tile's write-side footprint (64 destination rows x 64 pixels x
channel-count bytes, e.g. 64*64*3 = 12 KiB for `Rgb<u8>`) comfortably fits in a typical L1 data
cache (32-48 KiB on most current x86 cores) alongside the read-side source row also being scanned,
while still large enough to amortize per-tile loop overhead (bounds computation, slice indexing
setup) over a reasonable number of pixels. `BLOCK=32` shrinks the per-tile work below what's needed
to amortize that overhead; `BLOCK=128`/`256` grow the tile's working set past a cache level (L1,
then plausibly L2 sharing effects under multi-core contention for 256) faster than the reduced
loop-overhead fraction can compensate. This is consistent with - not a new finding beyond -
candidate 007's own reasoning for why blocking helps at all; this sweep just confirms 64 sits at or
near the sweet spot for both functions rather than at an accidental one-size measurement.

## What's not done / open questions

- Only 4 discrete values were tried (32/64/128/256, each a power of two) - a finer sweep (e.g.
  48, 96) was not attempted; given 64 already beats or ties both of its power-of-two neighbors in
  every scenario measured, a finer search seems unlikely to find a meaningfully better value, but
  this wasn't confirmed.
- Still no `perf`/disassembly available in this environment to directly confirm the cache-occupancy
  explanation above (same limitation noted throughout this workspace's `README.md`) - it remains an
  inference from the measured shape of the results, not an independently confirmed mechanism.
- This session's load average was higher and more variable (~1.2 to ~4-7) than candidate 007's own
  session - the paired-win counts (6/6, 4/4, etc.) are the trustworthy signal per this workspace's
  established noise discipline, but the absolute millisecond figures here should not be directly
  compared to candidate 007's own `RESULT.md` numbers (see `README.md`'s note on host-wide, not
  container-scoped, load).
- Did not test non-square tiles (e.g. a wider-than-tall tile matched to `rotate90_in`'s or
  `rotate270_in`'s specific fixed-vs-varying axis) - the sweep only varied the single square
  `BLOCK` constant both functions share, not their tile shape.
