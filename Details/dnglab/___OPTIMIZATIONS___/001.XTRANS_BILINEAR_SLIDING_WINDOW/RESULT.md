# X-Trans bilinear demosaic: sliding-window column accumulator

`interpolate_bilinear` (`rawler/src/imgop/sensor/xtrans/bilinear.rs:78-100`) recomputed a fresh
5x5 (up to 25-cell) neighborhood sum from scratch for every output pixel, even though adjacent
pixels' windows overlap by 4/5, and called `CFA::color_at` (two `%48` modulos) once per cell -
up to 50 modulos per output pixel. This was the single largest bucket seen in this session's
callgrind profiling (~54% of total instructions on a full decode+develop run of a Fujifilm X-T5
sample). Patched to compute each column's vertical-window contribution once per row and slide a
small ring-buffer accumulator across columns instead of resumming the full window per pixel.

## What was tried

One variant: replace the per-pixel from-scratch 5x5 sum with (a) a `col_contribution` closure
that sums the (up to 5-row) vertical window for one column once, and (b) a fixed-size (5-slot)
ring buffer of column contributions that is incrementally updated (subtract the column leaving
the window on the left, add the column entering on the right) as `x` advances across the row.
The vertical window is already row-constant in the original code (`y_lo`/`y_hi` depend only on
`y`), so this turns the per-row cost from O(25*width) cell visits into roughly O(5*width) (column
sums) + O(width) (O(1)-amortized horizontal slide) - about a 4-5x reduction in raw cell visits
and `CFA::color_at` calls (and thus `%48` modulos). Row-level parallelism (`par_chunks_exact_mut`
over rows, via rayon) is unchanged; the new state (ring buffer, running sums) is local to each
row's closure invocation, so it doesn't affect the parallel structure.

Measured with `rawler/benches/demosaic.rs`'s existing `full_image_fuji_xt5` case (Fujifilm X-T5,
7872x5196, criterion, `--features rawdb`), plus a small scratch example
(`rawler/examples/xtrans_dump.rs`, added and removed for this candidate only - not part of the
patch) that times just `RawDevelop::develop_intermediate` in a loop (1 warmup + N timed
iterations) for finer single-core control than criterion's own warmup/sampling gives.

| Variant | single-core p50 (taskset -c 0, RAYON_NUM_THREADS=1) | multi-core p50 (unpinned, 24 cores) | notes |
|---|---|---|---|
| baseline (criterion, run 1) | - | 474.60 ms [458.26, 491.44] | `cargo bench --bench demosaic -- full_image_fuji_xt5` |
| baseline (criterion, run 2) | - | 535.84 ms [464.78, 618.53] | repeat run - noise floor is wide (~±15%) on this 24-core, currently-loaded (`uptime` load avg ~4.5) machine |
| baseline (scratch example, run 1) | 1.924 s (min 1.922 s) | 428.84 ms (min 413.0 ms) | 2 single-core iters, 5 multi-core iters, 1 discarded warmup each |
| baseline (scratch example, run 2) | 2.471 s (min 1.964 s) | 434.60 ms (min 407.6 ms) | repeat |
| patched (criterion) | - | 346.08 ms [344.42, 348.22] | tight interval, no overlap with either baseline criterion run |
| patched (scratch example, run 1) | 846.71 ms (min 846.04 ms) | 331.44 ms (min 329.20 ms) | |
| patched (scratch example, run 2) | 823.65 ms (min 818.50 ms) | 343.31 ms (min 325.63 ms) | |

## Result

**Single-core**: ~1.92-2.47 s baseline -> ~0.82-0.85 s patched, roughly **2.2-2.4x faster**
(consistent across 2 baseline and 2 patched runs, no overlap between the two clusters).

**Multi-core (unpinned, 24 cores)**: criterion baseline median 474.60 ms and 535.84 ms across two
runs (noise floor ~±15% on this machine) vs. patched median 346.08 ms with a tight [344.42,
348.22] ms interval - patched is below both baseline runs' lower bound. The scratch example
agrees: ~408-435 ms baseline vs. ~325-343 ms patched. Multi-core speedup is smaller in relative
terms than single-core (~20-30% vs. ~2.2-2.4x) because the bench's timed span also includes
per-call fixed overhead (thread pool dispatch, output buffer allocation) that doesn't shrink with
this change, and because with 24 rayon workers the machine may already be running close to
memory-bandwidth-bound for this step.

## Does it change behavior/output?

- **Bit-exact?** Yes, on the one real-world sample checked. Full-image output buffer (7872x5196,
  3 f32 channels = 490,834,944 bytes) hashed identically (md5) before and after the patch, and
  the baseline binary itself was confirmed deterministic (two runs, same hash) before comparing
  against the patched one. This was not mathematically guaranteed up front - the patch changes
  the floating-point summation order (per-column vertical sums added left-to-right across the
  window, plus incremental subtract/add as the window slides, vs. the original's flat row-major
  25-term accumulation) - but for this sample it produced identical bits everywhere.
- **Is this a breaking change?** No public API change. Purely an internal implementation change
  to `interpolate_bilinear`, a private function.
- Project test suite: `cargo test -p rawler --lib` - 66 passed, 0 failed, both before and after
  (no existing unit test exercises `XTransBilinearDemosaic` directly; this crate has no
  dedicated bilinear-demosaic test). No `rawdb`-gated integration test was run against the
  patched tree specifically (see "What's not done" below).

## Why the result looks the way it does

The vertical window (`y_lo`/`y_hi`) only depends on the output row `y`, not on `x` - so it was
always redundant to recompute a fresh 25-cell sum for every pixel in a row. Hoisting the
per-column vertical sum out of the inner pixel loop and only doing O(1)-amortized work per pixel
horizontally removes most of the redundant `input.at()`/`CFA::color_at()` calls directly, which
matches the profiling finding that this function's cost was dominated by many cheap-but-frequent
cell visits and modulo lookups rather than by a few expensive operations.

## What's not done / open questions

- No `cargo test -p rawler --features rawdb` (integration tests reading real raw files) was run
  against the patched tree - only the lib unit tests and the dedicated bit-exact full-image hash
  comparison via the scratch dump example. The latter is a stronger, more targeted signal for
  this specific code path than the general integration suite would be.
- Bit-exact result is confirmed on one sample (Fujifilm X-T5) only; the reordering could in
  principle produce a 1-ULP difference on a different image/pattern shift where the specific
  values summed happen to round differently. Not observed here, but not exhaustively proven
  impossible either - see "Does it change behavior/output?" above.
- The `CFA::color_at` modulo-48 cost itself was reduced only as a side effect (fewer calls
  overall, ~5x), not eliminated per-call - see `IDEAS.md`'s remaining entry about using a
  period-sized (6-wide) lookup instead of the full 48x48 table, which was not attempted here and
  is a candidate for a follow-up.
- Multi-core numbers were collected on a machine with background load (`uptime` load average
  ~4.5 out of 24 cores) rather than a fully idle one; the criterion baseline noise floor (~±15%
  across two repeat runs) reflects that. The patched result's interval did not overlap either
  baseline run despite this, which is why the multi-core win is reported as real rather than
  "within noise", but a fully idle re-run would tighten the confidence further.
- `critcmp` (referenced in `projects/rawler.md` as installed) was not actually found on `PATH` or
  under `~/.cargo/bin` this session - comparisons here use criterion's own printed confidence
  intervals directly instead. Worth fixing the stale note in `projects/rawler.md`.
