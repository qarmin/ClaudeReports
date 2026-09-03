# 004.upsampler_row_buffer_reuse

`Upsampler::upsample_and_interleave_row` in `src/upsampler.rs` allocated a fresh
`vec![vec![0u8; self.line_buffer_size]; component_count]` on every call - this runs once per
output row (called from the per-row loop in both `src/worker/rayon.rs`'s parallel path and
`src/worker/mod.rs`'s single-threaded fallback path), so a tall image pays one heap allocation of
`component_count` inner `Vec`s per row for the whole decode. Idea from `IDEAS.md` (code-read
heuristic).

## What was tried

`Upsampler` is used as a shared `&Upsampler` inside `rayon`'s `par_chunks_mut(...).for_each(...)`
closure (`src/worker/rayon.rs:200-216`), which is the **default** build's code path (`rayon` is a
default-on feature) - each row is dispatched as its own work item (`.with_max_len(1)`), and that
closure runs concurrently across the rayon thread pool. This ruled out the two most obvious
fixes:

- **`&mut self`** on `upsample_and_interleave_row`, hoisting the buffer to an `Upsampler` field:
  works fine for the sequential fallback loop in `src/worker/mod.rs`, but is a non-starter for the
  rayon path since `&upsampler` is shared across concurrently-executing closures - would need a
  `Mutex` (contention on every row, likely worse than the allocation it replaces) or per-task
  cloning (defeats the purpose).
- **Caller-owned scratch buffer threaded in via `rayon::iter::ParallelIterator::for_each_with`**
  (clone-per-split "template" buffer): measured empirically before writing any patch, using a
  standalone scratch crate (`rayon = "1.5"`) reproducing this exact call shape
  (`par_chunks_mut(..).with_max_len(1).enumerate().for_each_with(Buf(vec![...]), |buf, ..| ..)`,
  2268 simulated rows). Result: **2267 clones for 2268 rows** - with `with_max_len(1)` forcing the
  work to be split down to individual rows, `for_each_with`'s init value gets cloned at
  (essentially) every split point, i.e. about as often as a fresh per-row allocation already
  happens today. This is a real, structural obstacle at this call granularity, not a borrow-checker
  inconvenience to code around - changing it would mean changing the chunking granularity
  (`with_max_len`), a separate, riskier change with its own load-balancing tradeoffs, out of scope
  for this candidate.

The fix actually applied instead: keep the `&self` signature (no call-site changes needed at all,
in either worker), and replace the per-call `vec![vec![0u8; ..]; ..]` with a `std::thread_local!`
`RefCell<Vec<Vec<u8>>>` cache, resized only when the required shape (`component_count`,
`line_buffer_size`) changes from what's cached (e.g. first row of a decode, or a new image with
different dimensions reusing the same worker thread). Since `Upsampler`'s only shared/concurrent
use is across OS threads (never within one), and each of rayon's worker threads keeps its own
thread-local slot for the whole life of the process-wide thread pool, this reuses the buffer across
every row after the first row seen by a given thread - not just within one image, but across every
subsequent image decoded by that thread for the life of the process.

Correctness check done before writing the patch (not just after): every `Upsample::upsample_row`
impl (`UpsamplerH1V1/H2V1/H1V2/H2V2/Generic`) writes exactly the leading `output_width` bytes of
its output buffer before returning, and every `color_convert_line_*` function in `decoder.rs` reads
exactly `output_width` bytes per component (bounded by `output.chunks_exact_mut(N)`, which has
`output_width` chunks) - so bytes beyond `output_width`, and any stale bytes from a previous
row/image on the same thread, are never read. `output_width` is constant across all rows of one
image (same value passed on every call in the loop), so there's no risk of an under-sized reused
buffer being read past what was (re)written on the current call.

## Result

Allocation-count sanity check (temporary counting-`GlobalAlloc` scratch binary, path-dependency on
this tree, 3 warmup decodes then one measured `Decoder::decode()` call):

| image | baseline allocations (post-warmup, 1 decode) | patched allocations (post-warmup, 1 decode) |
|---|---|---|
| `benches/large_image.jpg` (1512 rows, 3 components) | 6673 | 625 |
| `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` (2048 rows) | 70 | 70 |

The mechanism is confirmed: on `large_image.jpg`, patched eliminates essentially all of the
~6048 allocations attributable to the old per-row `vec![vec![...]]` (1512 rows x (3 inner Vecs + 1
outer Vec) = 6048, matching the 6673 -> 625 delta almost exactly). On the lossless reftest image
the count is **identical, 70 = 70** - because lossless-mode decoding (`src/decoder.rs`'s
`compute_image_lossless`, `mod lossless`) never calls `compute_image_parallel`/`Upsampler` at all;
it's a structurally different code path. This designated "best signal-to-noise" workload image
(per `README.md`) turns out to give **zero** information about this candidate - noted here so a
future session doesn't re-run it expecting a signal.

`tools/paired.sh` (unpinned multi-core, RUNS as noted), `benches/large_image.jpg`, iters=30:

- RUNS=9: median 23.690 -> 23.633 ms (-0.2%), B faster 4/9, NOT consistent
- RUNS=9 (repeat): median 24.610 -> 24.452 ms (-0.6%), B faster 8/9, consistent
- RUNS=9 (repeat): median 25.894 -> 24.763 ms (-4.4%), B faster 7/9, NOT consistent
- RUNS=15: median 24.499 -> 24.479 ms (-0.1%), B faster 10/15, NOT consistent
- RUNS=20: median 24.227 -> 24.031 ms (-0.8%), B faster 16/20, NOT consistent

Aggregated across all 5 runs above: **45 of 62 paired comparisons (72.6%) favored the patched
binary** - well above the 50% a pure-noise effect would produce, but no single run crossed
`paired.sh`'s strict `wins >= n-1` bar except one. Same pattern as candidate 002's DC LUT result:
direction-consistent, small in magnitude (roughly 0-1%, one outlier run at -4.4%), and mostly at or
below this machine's documented same-binary noise floor for this image (~4.5%, from
`projects/jpeg-decoder.md`).

`tools/paired.sh`, `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg`, iters=30:

- RUNS=9: median 256.756 -> 257.654 ms (+0.3%), B faster 4/9, NOT consistent
- RUNS=9 (repeat): median 256.640 -> 255.998 ms (-0.3%), B faster 6/9, NOT consistent

No signal either direction, as expected once the allocation-count check showed this path doesn't
touch `Upsampler` at all.

Bit-exact output confirmed: both binaries' FNV-1a hash for `large_image.jpg`
(`2f4deafc37e4ec2b`) and the lossless image (`0d970236b43aedad`) are identical to the
pre-existing baseline hashes recorded in `002`/`003`'s `RESULT.md`, and stable across all
iterations within each build.

## Does it change behavior/output?

- **Bit-exact?** Yes - confirmed via `bench_decode`'s built-in hash check on both workload images
  (see above), and via `cargo +stable test --release`.
- **Is this a breaking change?** No - internal-only change to `src/upsampler.rs`, no public API,
  file format, or documented-behavior change. One nuance worth flagging for a maintainer: the
  `thread_local!` buffer now persists for the lifetime of each rayon/worker OS thread, not just for
  one `decode()` call - so a long-lived process that decodes very large images on many threads will
  retain a small amount of per-thread memory (`line_buffer_size` bytes x `component_count`, a few
  KB at most for realistic images) between decodes, rather than freeing it immediately after each
  image. This is a deliberate memory-for-allocations tradeoff, not a leak (bounded by the largest
  image size seen per thread), but is a change from "no retained state between decode() calls".
- Project test suite (`cargo +stable test --release`), main working tree:
  - Before (pristine, base_commit `eb2d7c0`): 5 passed (`src/lib.rs` unit tests) + 12 passed
    (`tests/lib.rs`) + 1 passed each for `tests/rayon.rs`/`rayon-0.rs`/`rayon-1.rs`/`rayon-2.rs` + 2
    passed doctests - all passing, matches `projects/jpeg-decoder.md`'s documented baseline.
  - After (patched): identical - 5 + 12 + 1+1+1+1 + 2 = 23 tests, **all passing, 0 failed**, 7
    `test result` lines, 0 lines matching `^error` in the captured log.

## Why the result looks the way it does

The allocation-count evidence is unambiguous - this candidate genuinely removes ~91% of all
allocations happening during a `large_image.jpg` decode, and 100% of the ones attributable to
`Upsampler`. But the *wall-clock* effect on this machine is small and mostly buried in noise,
because each individual allocation removed is tiny (a handful of small `Vec<u8>` allocations per
row, freed almost immediately) relative to the rest of the per-row work (IDCT, color conversion,
and - on the default rayon path - the parallel dispatch overhead itself). The aggregate
paired-run win rate (72.6% across 62 independent comparisons) is well above chance and consistent
with a real small effect, but by this workspace's stated bar (`wins >= n-1` per individual `RUNS`
batch) it is not a *confirmed* win on this machine - the same caveat candidate 002 already
documented for a comparably-small per-row/per-block fix.

## What's not done / open questions

- Not measured with `--no-default-features` (the single-threaded fallback in
  `src/worker/mod.rs`), where the buffer reuse has no threading caveats at all and the allocation
  count should drop just as much per-call; the rayon path is the default and was the priority for
  this session.
- Not re-measured with `perf stat -r N` / a cycle-count tool - still not installed on this
  machine (see `projects/jpeg-decoder.md`); would likely resolve the sub-noise-floor ambiguity in
  fewer runs than repeated wall-clock `paired.sh` batches.
- The `for_each_with` clone-count experiment (2267 clones / 2268 rows) was done in a standalone
  scratch crate reproducing the call shape, not by actually patching `rayon.rs` and measuring - the
  conclusion (no improvement over the status quo at `with_max_len(1)` granularity) is based on that
  clone count being the same order of magnitude as today's per-row allocation, not on a further
  wall-clock comparison; changing `with_max_len` itself was intentionally left out of scope.
