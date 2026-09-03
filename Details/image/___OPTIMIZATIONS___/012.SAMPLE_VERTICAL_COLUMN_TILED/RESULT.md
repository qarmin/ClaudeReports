# `vertical_sample` column-tiled blocked traversal (candidate 009's alternative approach)

`src/imageops/sample.rs`'s `vertical_sample` (base commit `44ce9226c541dd2b11897b9ef07156a28871cdb7`
- candidate 005's flat fast path is *not* applied in this base, every candidate starts from a
pristine checkout) feeds the standalone `imageops::resize` free function. `IDEAS.md`'s entry for
this candidate proposed tiling across output *columns* (narrow strips, e.g. 32-256 pixels wide)
rather than candidate 009's row-banding, on the theory that a much smaller
`band_rows * tile_width`-sized accumulator (rather than 009's `band_rows * width`) might avoid the
large-accumulator memory-traffic regression 009 found, while keeping the "read each contributing
source row once" property that motivated both variants.

This candidate implements that: output-row filter windows/weights are precomputed once up front
(they don't depend on `x`), then the output is processed in `TILE_WIDTH = 64`-wide column tiles,
each further split into `BAND_ROWS = 16`-row bands, with a `band_len * tile_w`-sized (at most
`16 * 64 = 1024` `[f32; 4]` elements, 16 KiB) accumulator per (tile, band) pair - about 30x smaller
than candidate 009's `band_rows * width`-sized accumulator (491 KiB at `width = 1920`).

## What was tried

Benchmark: the crate's own pre-existing `benches/imageops.rs::bench_resize` ("resize 400x300
<Filter>" cases, 1920x1080 `RgbImage` source), same as candidates 005/009, reused as-is.

Three prebuilt `--release` criterion binaries were compared, alternated in round-robin
(A/B/C/A/B/C/...) per candidate 009's documented noise-floor lesson (`IDEAS.md`'s "machine-wide
slow/fast mode" entry) rather than back-to-back same-binary reps:

- **baseline** - pristine `vertical_sample`, the original `get_pixel`/`put_pixel` per-pixel loop.
- **candidate005** - candidate 005's flat fast path applied alone
  (`005.SAMPLE_VERTICAL_ROWSLICE/patch.diff`), built fresh this session to isolate this
  candidate's own incremental effect on top of it (candidate 005's own `RESULT.md` numbers were
  measured in a different session under different machine conditions).
- **patched** - this candidate's column-tiled fast path (`patch.diff` in this folder).

`--warm-up-time 1 --measurement-time 2.5` per invocation (criterion's own ~100-sample internal
average, chosen larger than 009's 1-1.5s after an initial 1.2s sweep showed too much run-to-run
noise), `taskset -c 3` for single-core, unpinned for multi-core. `uptime` load swung 2.8-17 during
the session (this machine's documented mode-lottery noise, see `README.md`/`IDEAS.md`) - round-robin
alternation and reporting the full spread (not just a median) is how this is handled here, per
`IDEAS.md`'s existing entry on this.

**`resize 400x300 Triangle`, single-core (`taskset -c 3`), 10 rounds** (`patched`/`baseline` and
`patched`/`candidate005` ratios per round):

| round | baseline | candidate005 | patched | patched/baseline | patched/candidate005 |
|---|---|---|---|---|---|
| 1 | 14.677 ms | 10.353 ms | 17.153 ms | 1.169 | 1.657 |
| 2 | 13.292 ms | 10.461 ms | 14.451 ms | 1.087 | 1.382 |
| 3 | 13.300 ms | 10.400 ms | 14.268 ms | 1.073 | 1.372 |
| 4 | 13.697 ms | 10.750 ms | 13.056 ms | 0.953 | 1.215 |
| 5 | 14.045 ms | 10.924 ms | 13.074 ms | 0.931 | 1.197 |
| 6 | 14.173 ms | 7.693 ms | 6.721 ms | 0.474 | 0.874 |
| 7 | 6.800 ms | 5.109 ms | 6.522 ms | 0.959 | 1.276 |
| 8 | 6.777 ms | 5.101 ms | 13.291 ms | 1.961 | 2.606 |
| 9 | 19.379 ms | 10.533 ms | 19.426 ms | 1.002 | 1.844 |
| 10 | 20.576 ms | 16.066 ms | 20.071 ms | 0.975 | 1.249 |

Median ratio: patched/baseline = 0.989 (patched faster in 5/10 rounds - no consistent direction);
patched/candidate005 = 1.324 (patched faster in only 1/10 rounds - consistently slower).

**`resize 400x300 Lanczos3`, single-core (`taskset -c 3`), 6 rounds:**

| round | baseline | candidate005 | patched | patched/baseline | patched/candidate005 |
|---|---|---|---|---|---|
| 1 | 53.498 ms | 29.685 ms | 33.977 ms | 0.635 | 1.145 |
| 2 | 35.942 ms | 28.232 ms | 33.692 ms | 0.937 | 1.193 |
| 3 | 44.648 ms | 40.219 ms | 51.096 ms | 1.144 | 1.270 |
| 4 | 45.438 ms | 28.079 ms | 35.386 ms | 0.779 | 1.260 |
| 5 | 38.528 ms | 28.045 ms | 33.751 ms | 0.876 | 1.203 |
| 6 | 39.148 ms | 26.394 ms | 36.560 ms | 0.934 | 1.385 |

Median ratio: patched/baseline = 0.905 (patched faster in 5/6); patched/candidate005 = 1.232
(patched faster in 0/6).

**`resize 400x300 Triangle`, multi-core (unpinned), 6 rounds:**

| round | baseline | candidate005 | patched | patched/baseline | patched/candidate005 |
|---|---|---|---|---|---|
| 1 | 6.795 ms | 5.131 ms | 6.502 ms | 0.957 | 1.267 |
| 2 | 6.788 ms | 5.723 ms | 7.636 ms | 1.125 | 1.334 |
| 3 | 7.063 ms | 5.572 ms | 7.018 ms | 0.994 | 1.259 |
| 4 | 7.381 ms | 5.465 ms | 7.325 ms | 0.992 | 1.340 |
| 5 | 7.133 ms | 5.434 ms | 7.423 ms | 1.041 | 1.366 |
| 6 | 7.152 ms | 5.336 ms | 7.758 ms | 1.085 | 1.454 |

Median ratio: patched/baseline = 1.017 (patched faster in 3/6, essentially a wash);
patched/candidate005 = 1.337 (patched faster in 0/6).

**`resize 400x300 Lanczos3`, multi-core (unpinned), 5 rounds:**

| round | baseline | candidate005 | patched | patched/baseline | patched/candidate005 |
|---|---|---|---|---|---|
| 1 | 18.036 ms | 13.815 ms | 17.593 ms | 0.975 | 1.273 |
| 2 | 18.140 ms | 14.125 ms | 17.447 ms | 0.962 | 1.235 |
| 3 | 18.813 ms | 13.882 ms | 18.152 ms | 0.965 | 1.308 |
| 4 | 18.226 ms | 14.517 ms | 17.578 ms | 0.964 | 1.211 |
| 5 | 18.133 ms | 13.932 ms | 17.517 ms | 0.966 | 1.257 |

Median ratio: patched/baseline = 0.965 (patched faster in 5/5, the most consistent scenario
measured, but a small ~3.5% margin); patched/candidate005 = 1.257 (patched faster in 0/5).

## Result

Consistent across all four scenarios (32 rounds total): **patched vs. candidate005 is a clear,
consistent regression** (~20-46% slower than candidate005's flat fast path in every scenario's
median, patched faster than candidate005 in only 1 of 32 rounds total). **Patched vs. the true
pristine baseline is roughly a wash** - close to break-even in Triangle (median ratio 0.989
single-core, 1.017 multi-core, no consistent direction round-to-round) and a modest, fairly
consistent ~3.5-10% win in Lanczos3 (median ratio 0.905 single-core, 0.965 multi-core, faster in
5/6 and 5/5 rounds respectively). Column-tiling did not recover candidate 005's ~23-27% win the
way `IDEAS.md`'s entry hoped - the regression relative to candidate005 is comparable in magnitude
to candidate 009's row-banded variant (~23-29% slower there vs. ~20-46% slower here).

## Does it change behavior/output?

- **Bit-exact?** Yes. A standalone check (`examples/_vsample_check_tmp.rs`, written and run this
  session, removed before capturing `patch.diff`) compared `imageops::resize` end-to-end between
  the pristine-baseline build and this candidate's patched build, hashing the raw output buffer,
  across: RGB8 at three ratios (11x9, 53x41, 37x29 from a 37x29 source) for all 5 `FilterType`
  variants; RGBA8 (4-channel, exercising the "unused trailing channel" replication); a `SubImage`
  crop (9x6 out of 19x13, both downscaled and upscaled) exercising the "packed row,
  non-contiguous buffer" case; a 1920x1080 source downscaled to 400x300 and to an extreme 4000x20
  aspect for `Triangle`/`Lanczos3`; 8 output heights straddling the `BAND_ROWS = 16` band boundary
  (1, 15, 16, 17, 31, 32, 33, 100); and 8 output widths straddling the `TILE_WIDTH = 64` tile
  boundary (1, 63, 64, 65, 127, 128, 129, 200). All 39 hashes matched exactly between the baseline
  and patched builds.
- Additionally, the project's own test suite: `cargo test --release --lib` passes 331/331 both
  before (pristine baseline) and after (patched) - same count, no new failures.
- **Is this a breaking change?** No - `imageops::resize`'s public signature, panics, and numeric
  output are unchanged. This candidate's fast path faithfully reproduces the same pre-existing
  `+1.0`-per-channel accumulator-init bug candidate 005 found and reported in
  `___OTHER___/001.RESIZE_VERTICAL_SAMPLE_OFF_BY_ONE/` (not re-investigated here, not fixed).

## Why the result looks the way it does

Shrinking the accumulator from candidate 009's `band_rows * width` (491 KiB at this benchmark's
1920px width) down to `band_rows * tile_width` (16 KiB at `TILE_WIDTH = 64`) did make the
accumulator itself small enough to plausibly stay in L1 - but it did not recover 009's lost ground
relative to candidate005's flat fast path, and the regression is if anything slightly larger in the
worst case (up to ~46% slower in one Triangle round vs. 009's reported ~23-29% band). The most
likely explanation, based on the code shape rather than direct profiling (`perf` remains
unavailable in this environment, see `README.md`): the column-tiling introduces its own new
overhead that the row-banding-only variant didn't have. Each of the ~30 column tiles (1920 / 64) at
this benchmark's width re-touches every output row it needs (up to `ceil(300/16) = 19` bands each),
so the *number of band-processing passes* (tile-count x band-count) is roughly 30x more here than
in candidate 009 (which only had one "column pass" - the full width - per band). Each pass carries
fixed overhead independent of the accumulator's size: the `acc.clear()`/`acc.resize()` calls,
looking up `row_infos[outy]` for every row in the band, and the branchy
`if src_y < info.left || src_y >= info.right { continue; }` skip-check inside the innermost
source-row loop. With a narrow `tile_w = 64`, that per-band fixed overhead is amortized over far
fewer accumulated pixels than with 009's full-width band, so it looms proportionally larger. In
effect, this candidate traded "large accumulator memory traffic" (009's identified cost) for "many
more small passes, each paying a fixed per-band setup/branch cost" - a different bottleneck, not a
clear net improvement, at the tile/band sizes tried.

Against the *true* baseline (rather than candidate005), the picture is different: the fast path
still removes the same per-pixel `get_pixel`/`put_pixel` trait-dispatch overhead that candidate 005
did, which is enough to land it close to breakeven-or-slightly-ahead of the untouched original loop
in every scenario measured. It just does this while giving back all of - and in most rounds
somewhat more than - the specific efficiency candidate 005's flat, register-resident accumulator
already had.

## What's not done / open questions

- No `TILE_WIDTH`/`BAND_ROWS` sweep was run for this variant (unlike candidate 009's dedicated
  `BAND_ROWS` sweep) - `TILE_WIDTH = 64` and `BAND_ROWS = 16` were chosen by the same "L1-sized
  accumulator" reasoning `IDEAS.md`'s entry suggested, not measured against alternatives. Given
  the "many small passes" explanation above, a *wider* tile (trading some of the L1-locality
  benefit back for fewer, larger passes - i.e. moving back toward candidate 009's row-band-only
  shape) or a *combined* single-pass-per-band design that processes the full row width per band
  but only materializes a bounded window into it at a time (rather than fully separate tile-loop
  and band-loop nesting) might land between 009's and this candidate's regressions, or might not -
  not measured.
  - This candidate already hoists per-row weight computation once for all output rows ahead of
    the tile/band loops (unlike candidate 009, which recomputed weights per band) - worth noting
    that this hoist alone was not sufficient to offset the tiling overhead identified above.
- Did not profile with `perf`/disassembly to directly confirm the "many small passes, each paying
  fixed per-band overhead" explanation above - `perf` remains unavailable in this environment (see
  `README.md`); the explanation is inferred from the code shape and comparison with candidate 009's
  structurally similar but untiled variant, not measured directly (e.g. via instruction or branch
  counters).
- Same round-to-round noise caveat as candidate 009: this machine's load swung 2.8-17 during this
  session's measurement (well beyond the "quiet" range), handled here the same way (round-robin
  alternation, full per-round table, median-of-ratios rather than a single aggregate) but individual
  rounds (e.g. Triangle single-core round 8, patched at 1.96x baseline) are visibly noisy outliers
  even with that discipline - the *median* direction across each 5-10 round group is the trustworthy
  signal, not any single round.
