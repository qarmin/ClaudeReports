# 008.decoder_baseline_hotloop

First dedicated read-through of `src/decoder.rs`'s baseline/progressive per-MCU decode loop
(`Decoder::decode_scan`) and a skim of `src/parser.rs`. Temporary `Instant`-based phase timers
(added to `decode_scan`/`decode_internal`, reverted before the final patch) showed Huffman entropy
decode and dequantize+IDCT each take roughly a third of wall time on `benches/large_image.jpg`,
with the rest split between per-MCU loop bookkeeping and the final upsample+color-convert stage.
Reading `decode_scan`'s row-dispatch code (`worker.append_row(...)`, right next to a
`// FIXME: additional potential work stealing opportunities for rayon case` comment already left
by a previous author) showed why: baseline/progressive scans send each decoded MCU row to the
worker one at a time via `append_row`, which the rayon worker backend
(`src/worker/rayon.rs::Scoped::append_row`) executes **synchronously on the calling thread** -
unlike `append_rows` (plural), which the same file already implements using
`rayon::in_place_scope` to spawn one task per row. Only the final progressive-render-unfinished-
components path (`Decoder::decode_planes`) and the very last upsample/color-convert stage
(`compute_image_parallel`) actually use rayon's parallelism; the dequantize+IDCT work for every
ordinary baseline or progressive scan runs fully serially, interleaved with the (inherently
sequential) Huffman decode, on a single thread.

## Phase-timing breakdown (temporary instrumentation, `benches/large_image.jpg`, 2268x1512 baseline RGB)

Measured with `Instant`-based counters wrapping (1) `decode_block`/`decode_block_successive_approximation`
plus the per-block coefficient-slice indexing (labeled `huffman` below), (2) the `worker.append_row`
call (dequantize+IDCT, labeled `idct`), (3) the per-row scratch-buffer `mem::replace` (labeled
`alloc`), and (4) the header/marker-parsing loop outside any scan (labeled `loop_other`). All
instrumentation was reverted before the final patch/measurement; the breakdown below is from a
`bench_decode ... 8` run with the instrumented build, so absolute numbers include some
instrumentation overhead (confirmed and mostly removed by moving the Huffman timer from per-block
to per-MCU granularity, see below) - the *proportions* are the useful signal, not the absolute ms:

| phase | typical share of one decode() call |
|---|---|
| Huffman decode (+ per-block coefficient indexing) | ~33-35% |
| dequantize + IDCT (`worker.append_row`, run serially on the decode thread) | ~33-36% |
| per-row scratch-buffer allocation (`mem::replace`) | ~1% |
| upsample + color convert (already parallel via `compute_image_parallel`) | ~6-12% |
| unaccounted (per-MCU loop control, restart-interval checks, `parse_sos`) | ~20-25% (includes leftover per-block `Instant::now()` overhead - see below) |
| one-time header/marker parsing (SOF/DQT/DHT/APPn, outside the scan loop) | negligible (~10-20µs) |

A first pass timed Huffman decode per-block (~81 000 calls for this image, 4:2:0 subsampling);
`scan_call_other` (the unaccounted bucket) measured ~7-8ms out of ~22-27ms total. Reducing the
timer to per-MCU granularity (~13 490 calls, 6x fewer `Instant::now()`/thread-local operations)
dropped that bucket to ~5.1-5.7ms, confirming a meaningful fraction of the original "unaccounted"
time was the timer overhead itself, not real application cost - a concrete instance of
MEASUREMENT.md's "the measurement changes what it measures" trap. The header/marker-parsing
bucket (`loop_other`, measured separately by timing the whole outer marker-loop iteration minus
the `decode_scan` call span) was negligible (~10-20µs) - ruling out `src/parser.rs`'s one-shot
segment parsing as a target for this image; that file otherwise looked unremarkable on a
straight read (small fixed-size segment parsers, no loops over per-block/per-pixel data).

## What was tried

Only one variant: batch every MCU row's decoded coefficients across the whole scan into a
`Vec<(usize, Vec<i16>)>` (`pending_rows`) instead of calling `worker.append_row((i,
row_coefficients))` immediately after each row is decoded, then call the already-existing
`worker.append_rows(&mut pending_rows.into_iter())` once after the scan's entropy-decode loop
finishes. `append_rows`' rayon implementation (`src/worker/rayon.rs`) already spawns one task per
row inside `rayon::in_place_scope` and writes each row's dequantized+IDCT'd output into a disjoint
slice of the shared result buffer (offsets tracked per-component-index in submission order) - the
same mechanism `decode_planes` already uses for progressive re-render. This candidate does not
change the dequantize/IDCT math or the `mem::replace`-based scratch buffer at all, only *when* and
*how* (one call vs many) the already-decoded rows are handed to the worker.

| Variant | large_image.jpg p50 (multi-core) | tower.jpg p50 | tower_progressive.jpg p50 | tower_grayscale.jpg p50 |
|---|---|---|---|---|
| baseline (`worker.append_row` per MCU row) | ~25-26ms | ~2.2-2.3ms | ~4.4ms | ~0.67-0.71ms |
| patched (batched `worker.append_rows` once per scan) | ~17.7-19ms | ~1.9-2.0ms | ~4.4-4.7ms | ~0.75-0.76ms |

(All single-run min/p50 figures above; see `tools/paired.sh` output below for the actual paired
comparison used for the verdict - this table is context, not the evidence.)

## Result

Measured with `tools/paired.sh` (unpinned multi-core, RUNS=9), two independent batches per image
except `tower_progressive.jpg` (three batches, see below), bit-exact on every image/batch:

- **`benches/large_image.jpg`** (2268x1512, baseline RGB, primary/best-signal workload): batch 1
  median 25.184ms -> 17.701ms (**-29.7%**, 9/9 pairs); batch 2 median 25.059ms -> 17.852ms
  (**-28.8%**, 9/9 pairs). Decisive win, both batches, far above this machine's ~4.5% documented
  noise floor for this image.
- **`benches/tower.jpg`** (512x512, baseline RGB): batch 1 median 2.248ms -> 1.908ms (**-15.1%**,
  9/9 pairs); batch 2 median 2.233ms -> 1.958ms (**-12.3%**, 8/9 pairs). Decisive win, both
  batches.
- **`benches/tower_progressive.jpg`** (512x512, progressive): batch 1 median 4.384ms -> 4.647ms
  (**+6.0%**, "B faster" 1/9 pairs, i.e. patched slower 8/9); batch 2 median 4.426ms -> 4.428ms
  (+0.0%, 5/9 - inconclusive); batch 3 (run to break the tie) median 4.378ms -> 4.685ms (**+7.0%**,
  patched slower 7/9). Net: a real regression, two of three batches decisive, the middle one
  merely inconclusive rather than contradicting.
- **`benches/tower_grayscale.jpg`** (512x512, single-component, non-interleaved): batch 1 median
  0.674ms -> 0.751ms (**+11.4%**, patched slower 9/9); batch 2 median 0.673ms -> 0.756ms
  (**+12.3%**, patched slower 9/9). Decisive regression, both batches.

Overall: **Mixed**. Large win on the two baseline/interleaved images (one of which,
`large_image.jpg`, is this workspace's designated "best default all-rounder" workload); real,
decisive regression on the two small and/or single-component images.

## Does it change behavior/output?

- **Bit-exact?** Yes - FNV-1a hash of the decoded output is identical to the unpatched build on
  every image tested (`large_image.jpg`, `tower.jpg`, `tower_progressive.jpg`,
  `tower_grayscale.jpg`), confirmed by `bench_decode`'s own per-iteration hash check (aborts on
  mismatch) as well as by `tools/paired.sh` refusing to compare identical binaries (ruling out a
  no-op patch).
- **Is this a breaking change?** No public API change. The only behavior-adjacent change is
  memory: the patched code buffers every decoded MCU row's coefficients for the *entire scan*
  before handing any of them to the worker (previously one row's worth was in flight at a time,
  immediately consumed). Peak memory during a scan grows by roughly the size of that scan's
  quantized-coefficient data (same order of magnitude as the final decoded image size for that
  scan) - comparable to what `self.coefficients` already holds for progressive JPEGs today, so
  this isn't a new *class* of memory usage for this crate, just a larger constant for the baseline
  path specifically. Not measured precisely (no allocation-count check was run this candidate);
  worth a quick check before taking this patch if peak memory on very large images matters.
- Project test suite: 5 unit + 12 integration (`tests/lib.rs`, includes reftest/crashtest image
  suites) + 4 rayon integration (1 each) + 2 doctests, **23 passed / 0 failed before and after** -
  identical pass count, `grep -c "^error"` = 0 both times.

## Why the result looks the way it does

The win on `large_image.jpg`/`tower.jpg` matches the phase-timing breakdown directly: dequantize+
IDCT was consistently ~33-36% of wall time and ran serially, one MCU row at a time, on the same
thread as the (unavoidably sequential) Huffman decode; batching it into one `append_rows` call
lets rayon spread that ~33-36% across this machine's 12 cores instead of paying it fully on the
critical path. The regression on `tower_progressive.jpg`/`tower_grayscale.jpg` is the mirror image
of the same mechanism: both are 512x512 (small - `tower_grayscale.jpg` also has only one
component, so no cross-component parallelism to exploit either), meaning relatively few MCU
rows/blocks and correspondingly little real IDCT work to amortize the `rayon::in_place_scope`
dispatch overhead against - the existing `Decoder::select_worker`'s
`PARALLELISM_THRESHOLD = 128*128` heuristic (used to pick the Immediate vs. Multithreaded worker
*kind* for a whole decode) doesn't help here because both these 512x512 images are already well
above that threshold and already use the rayon worker; the new cost this candidate introduces is
a *per-scan* batching decision inside `decode_scan`, which the existing coarse-grained,
whole-decode threshold was never calibrated against. This mirrors candidate 007's finding
(rayon dispatch overhead dominating for small per-task work) in a different part of the same
worker pipeline.

## What's not done / open questions

- No attempt was made to gate the batching behind a size heuristic (e.g. only batch if the scan
  has more than some number of MCU rows/blocks, falling back to the old per-row `append_row` call
  otherwise) - that would plausibly capture the `large_image.jpg`/`tower.jpg` win while avoiding
  the `tower_progressive.jpg`/`tower_grayscale.jpg` regression, but is a second design decision
  (where to put the threshold, and whether to reuse/extend `select_worker`'s existing constant or
  add a new one) deliberately left for a follow-up candidate rather than bundled into this one -
  see `IDEAS.md`.
- Allocation-count impact of buffering the whole scan's rows (vs. one row at a time) was not
  measured; only the qualitative memory-order-of-magnitude argument above was made.
- Batching in smaller groups (e.g. every N MCU rows) instead of the whole scan at once - a middle
  ground between "one row at a time" (current baseline) and "the whole scan at once" (this
  patch) - was not tried; it might recover most of the large-image win with less peak memory and
  less small-image regression, at the cost of a periodic-flush threshold to tune. Left as a
  follow-up idea.
- Only 4 workload images were measured (2 wins, 2 regressions); no progressive image larger than
  512x512 was available in-repo to check whether the progressive-path regression is purely a
  small-image effect or also affects large progressive images differently from large baseline
  ones.
