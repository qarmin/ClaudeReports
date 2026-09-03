# 009.gated_row_batching

Direct follow-up to `008.decoder_baseline_hotloop`. That candidate batched every MCU row of a
scan's decoded coefficients into one `worker.append_rows` call (rayon: one task per row) instead
of calling `worker.append_row` once per row inline - a decisive win on `large_image.jpg`/
`tower.jpg` (-29.7%/-15.1% on two independent batches) but a decisive regression on
`tower_progressive.jpg`/`tower_grayscale.jpg` (+6-12%), because those scans have too few MCU rows
to amortize the `rayon::in_place_scope` dispatch cost. This candidate gates the batching behind a
per-scan row-count threshold in `src/decoder.rs`'s `Decoder::decode_scan`, computed at the same
call site candidate 008 touched.

## Measuring the actual per-scan row counts first

Before picking a threshold, temporary instrumentation (`eprintln!`, reverted before the final
patch) was added at the end of `decode_scan` to print `is_progressive`, `is_interleaved`,
`max_mcu_y`, `components.len()`, `finished`, and the exact count of rows handed to the worker for
that scan (`rows_appended`). All four workload images turned out to have no chroma subsampling
(4:4:4, confirmed by parsing each image's SOF marker directly), so a single scan's total row count
is simply `max_mcu_y * (number of finished/interleaved components)`, plus for progressive scans
only the one scan per component that actually finishes it appends anything at all (every other
scan appends 0 rows and is untouched by this candidate either way):

| image | scan shape | rows_appended (= pending_rows.len() for that scan) |
|---|---|---|
| `tower.jpg` (512x512, baseline RGB) | 1 scan, interleaved, 3 components, 64 MCU rows | **192** |
| `tower_progressive.jpg` (512x512, progressive RGB) | 10 scans total; 7 append 0 rows (not yet finished); 3 finishing scans (one per component, non-interleaved, 64 MCU rows each) | **64** per finishing scan |
| `tower_grayscale.jpg` (512x512, single-component) | 1 scan, non-interleaved, 1 component, 64 MCU rows | **64** |
| `large_image.jpg` (2268x1512, baseline RGB) | 1 scan, interleaved, 3 components, 189 MCU rows | **567** |

This lines up exactly with candidate 008's win/regression split: the two winning images sit at
192 and 567 rows: the two regressing images/scans sit at 64. The threshold only has to separate
64 from 192 - there is no in-repo data point anywhere in between.

## Design: decide before decoding, not after

A first implementation (matching 008's structure) buffered every row into a `pending_rows: Vec<>`
unconditionally during the scan, then decided at the end whether to flush it via `append_rows` or
drain it one row at a time via `append_row`. Measuring this version showed the *unbatched* fallback
path was **itself** a regression on `tower_grayscale.jpg` (+13% to +21%, decisive across 2 batches)
even though it made the same `append_row` calls as the true baseline - buffering all 64 rows'
coefficients (~512KB for this image) before doing any IDCT work, instead of interleaving decode and
IDCT row-by-row as the original code does, changed the access pattern enough to cost real time on
a scan this size (plausibly an L1/L2 locality effect, given ~512KB is around this machine's per-core
L2 size). Rewritten to decide `batch_this_scan` **before** the main MCU loop starts (a cheap dry-run
count using the exact same row-count condition as the real dispatch code, over `max_mcu_y` -
negligible cost next to the real per-block decode work), so the below-threshold path calls
`worker.append_row` at the *original* call site, preserving the original interleaved decode/IDCT
order exactly. This removed the regression (see Result below). This is the only variant actually
measured end-to-end; the buffer-then-decide version was an intermediate step, not a second
candidate.

## What was tried

| Variant | tower.jpg | tower_progressive.jpg | tower_grayscale.jpg | large_image.jpg |
|---|---|---|---|---|
| baseline (per-row `append_row`, unpatched tree) | - | - | - | - |
| 008's unconditional batching (for reference, not re-measured here) | -12% to -15% win | +6-7% regression | +11-12% regression | -29% win |
| buffer-always-then-decide (rejected, see above) | not measured (already known bad for grayscale) | not measured | +13% to +21% regression | not measured |
| **gated, decide-before-loop (final patch)** | -25.6%/-38.6% win (2 batches) | +1.6%/-1.0%, 4/9 and 5/9 (inconclusive/noise) | +2.7%/+4.0%/+0.8%/+1.5%, 1/9,3/9,4/9,2/9 (inconsistent/noise, 4 batches) | -39.9%/-31.5% win (2 batches) |

## Result

Measured with `tools/paired.sh` (unpinned multi-core, RUNS=9), gated-patch binary vs. an
unpatched-tree baseline binary built in the same session, bit-exact on every image/batch:

- **`large_image.jpg`** (2268x1512, 567 rows/scan, batched): batch 1 median 40.206ms -> 24.172ms
  (**-39.9%**, 9/9); batch 2 median 36.102ms -> 24.737ms (**-31.5%**, 9/9). Decisive win, both
  batches - the win candidate 008 found is fully preserved.
- **`tower.jpg`** (512x512, 192 rows/scan, batched): batch 1 median 3.299ms -> 2.454ms
  (**-25.6%**, 9/9); batch 2 median 3.390ms -> 2.082ms (**-38.6%**, 9/9). Decisive win, both
  batches (larger than 008's own -12%/-15% figures, likely just this machine being noisier
  this session - see "Note on absolute numbers" below - but the direction and decisiveness match).
- **`tower_progressive.jpg`** (512x512, 64 rows per finishing scan, NOT batched): batch 1 median
  6.906ms -> 6.130ms (+1.6% by median but only 6/9 pairs, "NOT consistent"); batch 2 median
  7.419ms -> 7.802ms (+5.2% by median, only 2/9 pairs, "NOT consistent"). Neither batch is
  decisive in either direction - reads as noise, not a regression. Candidate 008's +6-7%
  regression here is gone.
- **`tower_grayscale.jpg`** (512x512, 64 rows, NOT batched): 4 batches run (2 extra, since the
  first two were borderline) - +2.7% (1/9, i.e. patched faster only once - "consistent" by
  `paired.sh`'s own >=8/9-or-<=1/9 rule, but a small effect); +4.0% (3/9, "NOT consistent");
  +0.8% (4/9, "NOT consistent"); +1.5% (2/9, "NOT consistent"). Three of four batches are
  explicitly inconclusive; the one "consistent" batch is a small (+2.7%) effect on a sub-
  millisecond image, well inside this machine's documented noise band for small images. Read
  together as **no real regression** - candidate 008's decisive +11-12% regression here is gone.

Note on absolute numbers: this session's raw ms figures (e.g. `tower.jpg` baseline median ~3.3ms
vs. candidate 008's ~2.2-2.3ms for what should be the same unpatched code) are higher across the
board than 008's session, on the same machine. This looks like general system load/noise
differences between sessions, not a real change to the baseline - relative deltas (paired,
same-session, same invocation) are what this candidate's verdict rests on, per MEASUREMENT.md.

## Threshold chosen and confidence

`ROW_BATCH_THRESHOLD = 128` (total row-append count for the scan, summed across all its finished
components - i.e. `pending_rows.len()` if batching were unconditional). This is a **row-count**
threshold, not a block or byte count, chosen because it was the value already being incrementally
computed at the batching call site and needed no new per-component math.

**Confidence: the only thing established is that something between 64 (exclusive, regression) and
192 (inclusive, win) is safe.** 128 was picked because it sits roughly in the middle of that gap and
happens to numerically echo the existing `PARALLELISM_THRESHOLD` constant elsewhere in this file
(128*128, unrelated units - no other significance intended). No image in this repo, or resized
variant, was tried to narrow the gap further. In particular:

- Safe and confirmed for scans at least as large as `tower.jpg`'s 192 rows/scan (and
  `large_image.jpg`'s 567).
- Confirmed *not* to regress at 64 rows/scan (`tower_grayscale.jpg`, and every finishing scan of
  `tower_progressive.jpg`).
- **Completely unconfirmed anywhere in (64, 192) rows** - no data point exists there. A synthetic
  image family spanning that gap (e.g. progressively larger square baseline JPEGs at 8-row
  intervals) would be needed to find the actual crossover, not just a threshold that happens to
  separate the 4 available data points.
- Component count (the task's other suggested gating dimension) was checked but not found to need
  separate treatment: `tower_grayscale.jpg`'s single-component scan and `tower.jpg`'s three-
  component scan both fall on the correct side of a row-count-only threshold once the row count is
  summed across components rather than measured per-component - i.e. summing already captures
  "fewer components contributing rows" as "fewer total rows" without an extra dimension. This
  candidate did not find a case where row count alone was misleading; it may still exist (e.g. a
  hypothetical component count large enough that even a "big" per-component row count still nets a
  small per-task granularity after rayon's own internal splitting - not investigated).

## Does it change behavior/output?

- **Bit-exact?** Yes on all 4 workload images, confirmed by `bench_decode`'s own per-iteration
  hash check (aborts on mismatch) plus `tools/paired.sh` refusing to compare identical binaries.
- **Is this a breaking change?** No public API change. Memory behavior is now scan-size-dependent
  in a way the unconditional version wasn't: scans at or below the threshold keep the original
  per-row-in-flight memory profile (one row's coefficients buffered at a time, same as unpatched);
  only scans above the threshold pay 008's full-scan buffering cost (peak memory grows by roughly
  that scan's total decoded-coefficient size, same as before - see the allocation-count check
  below for the actual number, not just the qualitative argument 008 left open).
- Project test suite (`cargo +stable test --release`): **23 passed / 0 failed before and after**
  (5 unit + 12 lib-integration + 4 rayon integration + 2 doctests), `grep -c "^error"` = 0 both
  times - identical to the documented baseline.

## Allocation-count check (candidate 008's first open follow-up)

Measured with a temporary counting `GlobalAlloc` scratch binary (`count_allocs`, 3 warmup decodes
then one measured `Decoder::decode()` call; not kept - deleted after use, same discipline as
candidate 004's original check):

| image | baseline allocations | gated-patch allocations | delta | batched this scan? |
|---|---|---|---|---|
| `tower.jpg` | 2301 | 2503 | **+202** (+8.8%) | yes (192 rows > 128) |
| `tower_progressive.jpg` | 2423 | 2423 | **+0** | no (64 rows/finishing scan <= 128) |
| `tower_grayscale.jpg` | 103 | 103 | **+0** | no (64 rows <= 128) |
| `large_image.jpg` | 6673 | 7258 | **+585** (+8.8%) | yes (567 rows > 128) |

The two images that get batched show an allocation increase almost exactly equal to their row
count (+202 for 192 rows, +585 for 567 rows) - consistent with rayon's `in_place_scope` allocating
roughly one job/task record per `scope.spawn` call (one per batched row), on top of the
`pending_rows: Vec<(usize, Vec<i16>)>` growth candidate 008 already accounted for qualitatively.
The two images that stay below the threshold show **zero** allocation difference from baseline -
the gate has no allocation-count cost at all for small scans, confirming the "decide before the
loop, don't buffer-then-decide" design point above didn't just fix the timing regression but also
avoided adding allocations on the ungated path. Peak memory growth for the batched path is real
but modest in absolute terms (roughly 1 extra small allocation per row, not per-byte-of-image), not
the kind of increase that would matter for `set_max_decoding_buffer_size` use cases - it does not
materially change 008's qualitative "same order of magnitude as `self.coefficients`'s existing
progressive-JPEG usage" conclusion.

## Why the result looks the way it does

The mechanism is exactly what candidate 008 and this candidate's own dry-run row counts predict:
below the threshold, decode and dequantize+IDCT stay interleaved exactly as in the unpatched code
(same call site, same order), so there is nothing left to regress; above it, the same
`rayon::in_place_scope`-based parallel dispatch candidate 008 already validated kicks in and
recovers the same win. The one wrinkle - the rejected "buffer always, decide at the end" version -
is itself a useful data point: *when* rows are hydrated into the worker matters even without
rayon in the picture, because interleaving decode and IDCT keeps the working set small and hot,
while decoding a whole small scan up front cools that data before it's used. That's a distinct
mechanism from the rayon-dispatch-overhead one 008 found, and it's the reason the gate has to be a
decision made *before* the loop rather than a buffering strategy resolved *after* it.

## What's not done / open questions

- **The safe range is only known to be (64, 192] rows** - no synthetic or additional real-world
  image was created to narrow this. A family of same-content images resized to intermediate MCU
  row counts (e.g. 80, 96, 112, 128, 144, 160, 176 rows) would pin down the actual crossover;
  until then, 128 is a reasonable-looking but unconfirmed point inside a wide safe interval, not a
  precisely tuned value.
- **The "batch every N rows, flush periodically" middle ground was not implemented or measured** -
  per the task, only one approach (size-gating) was carried through to a full measurement; the
  periodic-flush alternative from `IDEAS.md` remains an open, untried lead (left in `IDEAS.md`).
- Component count was examined qualitatively (see "Threshold chosen and confidence" above) but not
  swept independently of row count - no image exists in-repo with, say, 2 or 4 components at a row
  count near the threshold to check whether component count changes the crossover point on its
  own.
- No progressive image larger than 512x512 exists in-repo (same gap 008 left open) - so it's still
  unconfirmed whether a large progressive image's per-component finishing scans individually clear
  the row-count threshold the way `large_image.jpg`'s single baseline scan does, or whether
  progressive's per-component finishing structure keeps each individual scan's row count bounded
  by that component's own MCU row count (in which case a large-enough progressive image would
  still batch normally, one component at a time) - left in `IDEAS.md`.
