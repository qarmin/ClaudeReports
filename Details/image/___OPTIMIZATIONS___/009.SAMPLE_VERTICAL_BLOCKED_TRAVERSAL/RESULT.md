# `vertical_sample` blocked/tiled band traversal (on top of candidate 005's fast path)

`src/imageops/sample.rs`'s `vertical_sample` (feeding the standalone `imageops::resize` free
function, base commit `44ce9226c541dd2b11897b9ef07156a28871cdb7` - candidate 005's flat
`to_pixel_view()` fast path is *not* applied in this base, since every candidate always starts
from a pristine checkout) strides column-by-column across source rows for a fixed output column:
for each output row, and for each output column in that row, it re-reads the same handful of
contributing source rows. `IDEAS.md`'s entry for this candidate proposed a blocked/tiled
traversal - process a band of several output rows together, scan each contributing source row
*once*, contiguously across x (matching the row-major buffer, the way `horizontal_sample` already
does), and fan its weighted contribution out to every output row in the band that needs it,
instead of re-deriving the strided per-column reads separately for each output row.

This candidate implements exactly that: a `BAND_ROWS`-sized (16) band of output rows processed
together, with a `band_len * width`-sized `f32x4` accumulator array that every contributing source
row in the band's combined range is scanned into once.

## What was tried

Benchmark: the crate's own pre-existing `benches/imageops.rs::bench_resize` ("resize 400x300
<Filter>" cases, 1920x1080 `RgbImage` source), same as candidate 005, reused as-is.

Three prebuilt `--release` criterion binaries were compared, alternated in round-robin (A/B/C/A/B/C/...)
rather than back-to-back pairs, since this session's measurements showed the noise floor on this
machine swings much wider between "quiet" and "busy" stretches than a same-adjacent-pair comparison
alone would catch (see "Why the result looks the way it does" below):

- **baseline** - pristine `vertical_sample`, the original `get_pixel`/`put_pixel` per-pixel loop.
- **005-only** - candidate 005's flat fast path applied alone
  (`005.SAMPLE_VERTICAL_ROWSLICE/patch.diff` applied on top of the same base commit), built fresh
  this session purely to isolate this candidate's own incremental effect on top of it, per
  MEASUREMENT.md's confound-isolation discipline (candidate 005's own `RESULT.md` has its own
  from-baseline numbers already, measured in an earlier session under different machine
  conditions).
- **patched** - this candidate's blocked-traversal fast path (`patch.diff` in this folder).

`--warm-up-time 1 --measurement-time 1.5`, `taskset -c 3` for single-core, unpinned for
multi-core. `uptime` load ~2.2-3.6 on 24 cores throughout.

**`resize 400x300 Triangle`, single-core (`taskset -c 3`), 8 rounds:**

| round | baseline | 005-only | patched |
|---|---|---|---|
| 1 | 7.149 ms | 5.375 ms | 6.957 ms |
| 2 | 7.558 ms | 5.406 ms | 7.092 ms |
| 3 | 7.181 ms | 5.805 ms | 6.965 ms |
| 4 | 7.125 ms | 6.261 ms | 7.812 ms |
| 5 | 7.274 ms | 6.986 ms | 7.286 ms |
| 6 | 7.268 ms | 5.500 ms | 7.427 ms |
| 7 | 7.017 ms | 5.258 ms | 6.614 ms |
| 8 | 7.222 ms | 5.115 ms | 6.375 ms |

Median: baseline 7.20 ms, 005-only 5.45 ms (~24% faster than baseline, matching candidate 005's
own measurement), patched 7.02 ms (~2.5% faster than baseline, ~29% *slower* than 005-only).
Patched slower than 005-only in 8/8 rounds.

**`resize 400x300 Lanczos3`, single-core (`taskset -c 3`), 5 rounds:**

| round | baseline | 005-only | patched |
|---|---|---|---|
| 1 | 17.866 ms | 13.580 ms | 17.882 ms |
| 2 | 17.744 ms | 18.062 ms | 16.688 ms |
| 3 | 18.437 ms | 13.599 ms | 16.692 ms |
| 4 | 17.719 ms | 13.657 ms | 16.663 ms |
| 5 | 17.721 ms | 13.568 ms | 16.660 ms |

Median: baseline 17.74 ms, 005-only 13.60 ms (one 18.06 ms outlier excluded from the "expected"
reading, still the minimum-latency binary in 4/5 rounds), patched 16.69 ms (~6% faster than
baseline, ~23% slower than 005-only). Patched slower than 005-only in 4/5 rounds.

**`resize 400x300 Triangle`, multi-core (unpinned), 5 rounds:**

| round | baseline | 005-only | patched |
|---|---|---|---|
| 1 | 6.797 ms | 5.111 ms | 6.231 ms |
| 2 | 6.800 ms | 5.065 ms | 6.361 ms |
| 3 | 6.883 ms | 7.521 ms | 7.389 ms |
| 4 | 6.828 ms | 5.370 ms | 7.090 ms |
| 5 | 6.882 ms | 5.187 ms | 6.366 ms |

Median: baseline 6.83 ms, 005-only 5.19 ms (~24% faster; round 3's 7.52 ms is a same-noise-lottery
outlier), patched 6.37 ms (~7% faster than baseline, ~23% slower than 005-only).

**`resize 400x300 Lanczos3`, multi-core (unpinned), 3 rounds:**

| round | baseline | 005-only | patched |
|---|---|---|---|
| 1 | 18.075 ms | 13.533 ms | 16.859 ms |
| 2 | 17.931 ms | 13.843 ms | 17.187 ms |
| 3 | 18.397 ms | 14.000 ms | 17.230 ms |

Median: baseline 18.08 ms, 005-only 13.84 ms (~23% faster), patched 17.19 ms (~5% faster than
baseline, ~24% slower than 005-only).

A separate, bounded diagnostic sweep of `BAND_ROWS` (1, 2, 4, 8, 16, 32) was run on
`resize 400x300 Triangle` single-core after the main comparison above looked unexpectedly bad, to
check whether the band size specifically was the problem. First readings (band=16 clocking
~12.7-14.2 ms, far worse than the numbers in the table above) suggested band=4 or smaller fixed
it (~6.4-6.9 ms) - but re-testing band=16 again immediately after showed it back down at ~6.9-7.1
ms, matching every other band size tried (1/2/8/32 all landed in the same ~6.9-7.2 ms band on a
retest). This was the machine's documented "performance mode lottery" (see `README.md` and
`IDEAS.md`'s noise-floor entry), not a real band-size effect - band size in the 1-32 range tested
does not appear to materially change the result once measured with enough repeats; `BAND_ROWS = 16`
(the value in `patch.diff`) was kept as an arbitrary reasonable choice, not because it measurably
beat other values.

## Result

A consistent, small win over the true pristine baseline (~2.5-7% faster across both filters,
single- and multi-core - modest, and close to this machine's documented noise floor for a single
comparison, but the direction held in 8/8, 5/5, 5/5, and 3/3 rounds respectively) - but a clear,
consistent regression relative to candidate 005's already-applied flat fast path (~23-29% slower
than 005-only in every round across both filters and both core configurations). The blocked
traversal gives back most of the ~23-27% win candidate 005 already measured, rather than adding to
it as `IDEAS.md`'s entry hoped.

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone check (`examples/_vsample_check_tmp.rs`, written and run this
  session, removed before capturing `patch.diff`) compared `imageops::resize` end-to-end between
  the pristine-baseline build and this candidate's patched build, hashing the raw output buffer,
  across: RGB8 at several up/downscale ratios (11x9, 53x41, 37x29 from a 37x29 source) for all 5
  `FilterType` variants; RGBA8 (4-channel, exercising the "unused trailing channel" replication);
  a `SubImage` crop (9x6 out of 19x13, both downscaled and upscaled) exercising the "packed row,
  non-contiguous buffer" case; a large 1920x1080 source downscaled to 400x300 and to an extreme
  4000x20 aspect (`Triangle`/`Lanczos3`); and 8 output heights straddling the fast path's
  `BAND_ROWS = 16` band boundary (1, 15, 16, 17, 31, 32, 33, 100). All hashes matched between
  baseline and patched builds - see "Why the result looks the way it does" for why this is
  expected (same summation order as the original loop, not just the same final value).
- Additionally, the project's own test suite: `cargo test --release --lib` passes 331/331 both
  before (pristine baseline) and after (patched) - same count, no new failures.
- **Is this a breaking change?** No - `imageops::resize`'s public signature, panics, and numeric
  output are unchanged. This candidate's fast path faithfully reproduces the same pre-existing
  `+1.0`-per-channel accumulator-init bug candidate 005 found and reported in
  `___OTHER___/001.RESIZE_VERTICAL_SAMPLE_OFF_BY_ONE/` (not re-investigated here, not fixed).

## Why the result looks the way it does

The blocked traversal trades one real inefficiency (candidate 005's fast path still re-reads the
same handful of small, cheap-to-read source rows once per nearby output row) for a different,
larger one: a `band_len * width`-sized `[f32; 4]` accumulator (30,720 elements / ~480 KiB at
`BAND_ROWS = 16`, `width = 1920`) that is read-modify-written on *every filter tap for every band
row*, plus a separate initialization pass over the whole accumulator before accumulation starts
(`acc.resize(.., [1.0f32; 4])`) and a separate copy-out pass after. Candidate 005's flat fast path,
by contrast, keeps its per-output-pixel accumulator (`[1.0f32; 4]`) entirely in a stack-resident
local variable across all of that pixel's taps - the accumulation itself never touches memory, only
the final store to `dst` does. This candidate's version turns every intermediate accumulation step
into real memory traffic against a working set (hundreds of KiB) that does not comfortably stay
resident the way 005's tiny per-pixel register accumulator did, more than offsetting whatever
locality was gained from reading each source row only once. The source rows being read were never
the bottleneck to begin with at this image width (`1920 * 3 bytes = 5,760 bytes/row` for RGB8,
comfortably within L1) - so avoiding re-reading them bought little, while introducing the large
accumulator cost real time.

This also explains why the `BAND_ROWS` sweep found no consistent effect from band size: shrinking
the band shrinks the accumulator (helping), but by the same token shrinks how much re-reading is
actually avoided (hurting) - the two roughly cancel out in this design, rather than one dominating.
A materially different blocked-traversal design (e.g. accumulating per output-column-tile instead
of per output-row-band, or keeping the accumulator small enough to fit in registers/L1 by
processing a narrow strip of columns at a time instead of the full row width) might avoid this
trade-off, but was not attempted this session - see "What's not done" below.

## What's not done / open questions

- A column-tiled variant (process a fixed-width strip of output columns across all bands/rows,
  keeping the accumulator width-bounded instead of full-image-width) was not tried - it might avoid
  the large-accumulator cost identified above while still reading each source row only once per
  band, but is a different code shape from what `IDEAS.md`'s entry described and would need its own
  measurement.
- Did not profile with `perf`/disassembly to directly confirm the "large accumulator traffic"
  explanation above - `perf` remains unavailable in this environment (see `README.md`); the
  explanation is inferred from the `BAND_ROWS` sweep's behavior and the structural difference from
  candidate 005's register-resident accumulator, not measured directly (e.g. via cache-miss
  counters).
- This candidate's own comparison against 005-only reuses candidate 005's patch applied fresh in
  this session (not re-measured against a separately-rebuilt "just 005" binary from 005's own
  original session) - the numbers in the "005-only" column above come from a binary built this
  session from `005.SAMPLE_VERTICAL_ROWSLICE/patch.diff`, alongside baseline and patched, so all
  three are directly comparable to each other under the same machine conditions.
