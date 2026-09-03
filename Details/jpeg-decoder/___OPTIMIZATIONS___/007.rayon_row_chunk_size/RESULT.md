# rayon_row_chunk_size

`src/worker/rayon.rs`'s `compute_image_parallel` (the default parallel baseline/progressive
decode path) dispatched one rayon work item per output row
(`image.par_chunks_mut(line_size).with_max_len(1).enumerate().for_each(...)`) - confirmed by
reading the file, matching candidate 004's earlier description exactly. Candidate 004's
standalone scratch investigation suggested this granularity is near-maximal rayon dispatch
overhead relative to the per-row upsample/interleave work. This candidate batches a constant
number of rows (`ROW_CHUNK`) into each rayon work item by chunking the output buffer in units of
`line_size * ROW_CHUNK` instead of `line_size`, iterating the rows within each chunk with a plain
inner loop, and dropped the explicit `.with_max_len(1)` call so rayon's own adaptive splitting
still governs how many of those (now coarser) chunks execute in parallel.

## What was tried

Two chunk sizes, `ROW_CHUNK = 4` and `ROW_CHUNK = 16`, each measured against the same baseline
binary with `tools/paired.sh` (unpinned multi-core, RUNS=9), on all three assigned workload
images. Repeat batches were run per image/chunk-size combination to check consistency rather than
trusting a single batch, per this machine's known noise floor (projects/jpeg-decoder.md: 2.2-4.5%
same-binary spread, occasional bimodal clustering even unpinned).

| Variant | tower.jpg (512x512) | tower_progressive.jpg (512x512) | large_image.jpg (2268x1512) |
|---|---|---|---|
| baseline (`with_max_len(1)`, 1 row/task) | - | - | - |
| ROW_CHUNK=4, batch 1 | -2.9% median, 8/9 B faster (looked consistent) | -0.9%(!) i.e. B slower, 5/9 - not consistent | -1.0% median, 9/9 B faster (looked consistent) |
| ROW_CHUNK=4, batch 2 (repeat) | -1.3% median, 5/9 - not consistent | -3.3% median, 4/9 - not consistent | -0.3% median, 6/9 - not consistent |
| ROW_CHUNK=4, batch 3 (large_image repeat) | (not repeated a 3rd time) | -5.8% median, 5/9 - not consistent (this row is actually ROW_CHUNK=16's progressive repeat, see below) | -0.1% median, 3/9 - not consistent |
| ROW_CHUNK=16, batch 1 | -2.9% median, 5/9 - not consistent | -5.8% median, 5/9 - not consistent | -0.1% median, 3/9 - not consistent |

(The table above lists every batch run, in run order, per image/chunk-size - not just the
best-looking one. `paired.sh`'s own "B faster in N of 9 pairs" count is the number that actually
carries the conclusion, per its own header comment; a "consistent" verdict requires `wins >= 8/9`
in that script's own threshold check.)

Raw numbers, `ROW_CHUNK=4` vs baseline:
- tower.jpg batch 1: A `2.161 2.287 2.322 2.439 2.443 2.239 2.241 2.339 2.093`, B `2.102 2.251 2.190 2.305 2.277 2.138 2.226 2.220 2.150` -> median -2.9%, 8/9, "consistent"
- tower.jpg batch 2 (repeat): A `2.160 2.127 2.142 2.141 2.100 2.127 2.030 2.044 2.146`, B `2.093 2.086 2.104 2.100 2.121 2.132 2.120 2.096 2.060` -> median -1.3%, 5/9, "NOT consistent"
- large_image.jpg batch 1: A `24.237 24.258 25.461 25.240 25.540 26.164 24.078 24.083 23.975`, B `24.027 23.984 24.459 25.031 25.166 25.418 23.950 23.930 23.928` -> median -1.0%, 9/9, "consistent"
- large_image.jpg batch 2 (repeat): A `24.340 23.906 28.308 28.490 28.215 24.584 24.493 24.051 24.399`, B `24.078 28.138 28.933 28.641 28.037 24.317 24.126 23.930 24.522` -> median +0.1%, 5/9, "NOT consistent"
- large_image.jpg batch 3 (2nd repeat): A `29.436 28.434 24.069 24.054 24.233 24.359 24.341 28.131 28.661`, B `27.838 24.156 24.274 23.999 24.226 24.168 24.605 28.687 27.713` -> median -0.3%, 6/9, "NOT consistent"
- tower_progressive.jpg batch 1: A `5.217 4.486 4.517 5.088 4.578 4.584 4.368 4.729 4.443`, B `4.620 4.325 4.385 5.111 5.007 5.059 5.017 4.395 4.365` -> median +0.9%, 5/9, "NOT consistent"
- tower_progressive.jpg batch 2 (repeat): A `5.011 4.269 5.095 4.257 5.141 4.706 4.605 4.196 4.321`, B `5.149 5.146 4.454 4.914 4.293 5.067 4.409 4.219 4.248` -> median -3.3%, 4/9, "NOT consistent"

Raw numbers, `ROW_CHUNK=16` vs baseline (one batch per image):
- tower.jpg: A `2.425 2.189 2.141 2.417 2.154 2.339 2.140 2.340 2.111`, B `2.077 2.122 2.179 2.081 2.277 2.126 2.126 2.381 2.168` -> median -2.9%, 5/9, "NOT consistent"
- tower_progressive.jpg: A `4.444 4.417 4.975 4.399 4.695 4.826 5.194 4.800 4.563`, B `5.230 5.327 4.385 4.360 4.817 4.377 4.382 4.421 4.777` -> median -5.8%, 5/9, "NOT consistent"
- large_image.jpg: A `27.736 24.341 24.144 24.229 24.048 24.317 25.014 28.876 28.467`, B `27.899 24.141 24.192 24.097 24.139 24.323 28.399 28.450 28.469` -> median -0.1%, 3/9, "NOT consistent"

## Result

No confirmed effect in either direction, for either chunk size tried, on any of the three
workload images. Out of 8 total paired batches run (5 for `ROW_CHUNK=4`, 3 for `ROW_CHUNK=16`),
only 2 crossed `paired.sh`'s own `wins >= 8/9` consistency bar (`ROW_CHUNK=4` on tower.jpg batch 1,
8/9; `ROW_CHUNK=4` on large_image.jpg batch 1, 9/9) - both were `ROW_CHUNK=4`'s *first* batch on
each image, and neither reproduced on a same-image, same-binary repeat (tower.jpg dropped to 5/9,
large_image.jpg dropped to 5/9 then 6/9). This is exactly the shape of a spurious pass through the
consistency threshold on a noisy machine, not a real, reproducible effect - the same pattern
candidates 002 and 004 documented on this repo before. Median deltas across all batches ranged
from about -5.8% to +0.9%, straddling zero with no image or chunk size showing a directionally
stable sign across repeats.

**Small-image regression check (tower.jpg, the specific risk case named for this candidate):** no
meaningful regression was found - both chunk sizes' medians on tower.jpg leaned slightly negative
(faster) across all batches (-1.3% to -2.9%), never positive (slower) by more than noise. So while
there's no confirmed win, there's also no confirmed harm to the small image from under-parallelizing
it with a coarser chunk size, at either `ROW_CHUNK=4` or `ROW_CHUNK=16`, on this 24-thread machine.

`ROW_CHUNK=4` was kept as the value captured in `patch.diff` (arbitrarily, between the two - it
showed marginally more batches near/above the consistency bar than 16, though as documented above
neither was a real, reproducible win). This should not be read as "4 is better than 16" - the data
doesn't support ranking them, only "batching rows this way, at either size tried, did not produce
a measurable change on this machine with this harness."

## Does it change behavior/output?

- **Bit-exact?** yes - `bench_decode`'s hash check (`min`/`p50`/... output includes a `hash =
  ... (STABLE across all iters)` line, aborts on any mismatch) passed for every run above, both
  chunk sizes, all three images.
- **Is this a breaking change?** no - internal dispatch-granularity change only, no public API or
  output-format change.
- Project test suite (`cargo +stable test --release`): baseline (unpatched, HEAD eb2d7c0) - 12 lib
  tests + 4 separate rayon integration test binaries (1 test each) + 2 doctests, all passing.
  Patched (`ROW_CHUNK=4`) - identical: 12 lib + 4 rayon integration + 2 doctests, all passing, no
  new failures.

## Why the result looks the way it does

Two plausible, non-exclusive explanations, neither confirmed further this session (no `perf` on
this machine, per `projects/jpeg-decoder.md`):

1. `upsampler.upsample_and_interleave_row`'s actual per-row work (upsample + color-convert +
   interleave one output row, `output_size.width * components.len()` bytes) may already be large
   enough relative to rayon's per-task dispatch/steal overhead that reducing task count 4x-16x
   doesn't move wall time by more than this machine's own noise floor - candidate 004's
   standalone scratch crate measured the *clone-per-task* overhead of `for_each_with` at this
   granularity, but never measured whether raw dispatch overhead itself (without the clone) was
   actually a significant fraction of total time on real images.
2. This machine's measurement noise floor (documented at 2.2-4.5% same-binary spread, with
   observed bimodal clustering even unpinned, per `projects/jpeg-decoder.md`) is comparable to or
   larger than whatever real effect this change has, making it fundamentally hard to confirm
   either a win or a regression here without a cycle-counting tool (`perf stat -r N`) this machine
   doesn't have installed.

## What's not done / open questions

- Load-imbalance risk (batching too many rows per task hurting tail latency on images with
  variable per-row Huffman cost) was not directly measurable with this workspace's tooling - only
  a single wall-clock comparison per run is available, not a per-task timing breakdown. Nothing
  in the data above suggests a *tail-latency* problem specifically (no batch showed a
  much-worse-than-median max/p95 for the patched binary), but this wasn't isolated from ordinary
  system noise either - noted as a caveat, not ruled out.
- A cycle-accurate tool (e.g. `perf stat -r N` if ever installed on this machine) could resolve
  whether there's a small real effect underneath this machine's noise floor, in either direction,
  that wall-clock `Instant` timing cannot currently confirm.
- Chunk sizes between 4 and 16, or above 16, were not tried - given neither tried value produced
  a confirmed signal, there was no basis this session for guessing a better value would.
