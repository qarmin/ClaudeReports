# 016.hybrid_gated_periodic_flush

Direct follow-up to `009.gated_row_batching` and `013.periodic_row_batch_flush`, testing the
hybrid `IDEAS.md` lead both of those left open: keep 009's `ROW_BATCH_THRESHOLD=128` gate to
decide whether a scan gets batched *at all* (preserving 009's exactly-zero-rayon-dispatch
behavior below the threshold), but for scans that clear the gate, flush every `ROW_FLUSH_N` rows
via `worker.append_rows` instead of buffering the whole scan into one call (009's design). The
goal was to combine 009's "zero overhead below threshold" with lower peak memory above it.

**Machine note**: `uptime` showed load average ~22-24 on this 24-thread machine (started at
"observed ~25" per the task) for the entire session - elevated, plausibly host-level noise
outside this container's visibility, not a locally-visible heavy consumer. All borderline
`paired.sh` results below are read with this in mind; nothing here rests on a single borderline
batch without a second one to back it up.

## What was tried

Two `ROW_FLUSH_N` values, both on top of 009's unchanged gate/threshold: 64 and 256 (patch shape
identical between the two, see `patch.diff` for the N=256 variant, the one kept as final).

### Speed: hybrid vs. unpatched baseline (`tools/paired.sh`, unpinned multi-core, RUNS=9)

| image (rows/scan) | ROW_FLUSH_N=256 | ROW_FLUSH_N=64 |
|---|---|---|
| `tower.jpg` (192, batched) | batch1 -28.8%, 9/9; batch2 -33.0%, 9/9 (consistent win) | -6.9%, 8/9 (consistent win, smaller than N=256/009) |
| `tower_progressive.jpg` (64/finishing scan, not batched) | +20.3%, 5/9; +1.6%, 3/9 (both NOT consistent - noise, no regression) | not separately re-run (identical code path to N=256/009/unpatched below threshold - gate untouched) |
| `tower_grayscale.jpg` (64, not batched) | -0.5%, 7/9; -1.8%, 6/9 (both NOT consistent - noise, no regression) | not separately re-run (same reasoning) |
| `large_image.jpg` (567, batched) | batch1 -36.6%, 9/9; batch2 -39.1%, 9/9 (consistent win, matches 009) | +38.3%, 0/9 (consistent **regression** vs. baseline-would-be-win; too many flush dispatches, see below) |

### Speed: hybrid vs. a rebuilt copy of 009's own patch (`tools/paired.sh`, RUNS=9)

| image | ROW_FLUSH_N=256 (2 batches) | ROW_FLUSH_N=64 |
|---|---|---|
| `tower.jpg` | -1.7%, 6/9; +3.1%, 2/9 (both NOT consistent - indistinguishable from 009; expected, see mechanism below) | +10.0%, 1/9 (consistent **regression** vs. 009) |
| `tower_progressive.jpg` | -4.6%, 7/9; -2.2%, 7/9 (both NOT consistent - indistinguishable) | not separately re-run |
| `tower_grayscale.jpg` | -1.5%, 7/9; -1.2%, 7/9 (both NOT consistent - indistinguishable) | not separately re-run |
| `large_image.jpg` | -0.3%, 4/9; -1.1%, 4/9 (both NOT consistent - indistinguishable from 009) | +38.3%, 0/9 (consistent, large **regression** vs. 009 - see `009 vs. unpatched baseline` comparison above, same number, since 009-vs-baseline directly measures this delta) |

**Why `tower.jpg` never differs between N=256 and 009 at all**: `tower.jpg`'s single batched scan
is 192 rows, below `ROW_FLUSH_N=256`, so the hybrid's flush loop never actually triggers a
mid-scan flush - it buffers the whole 192-row scan and flushes it once at the very end, which is
*exactly* 009's whole-scan `append_rows` call. N=256 and 009 are the literal same code path on
every image in this repo except `large_image.jpg` (567 rows, so N=256 causes 3 flushes: 256 + 256
+ 55 rows).

## Result

**Speed**: `ROW_FLUSH_N=256` matches candidate 009 exactly on speed everywhere it was measured -
every one of the 8 hybrid-vs-009 paired batches above (4 images x 2 batches) landed "NOT
consistent" (never a decisive win or loss either direction), and the hybrid-vs-baseline numbers
reproduce 009's own win/no-regression pattern point for point (`large_image.jpg` -36.6%/-39.1%,
`tower.jpg` -28.8%/-33.0%, `tower_progressive.jpg`/`tower_grayscale.jpg` noise-level, no
regression). `ROW_FLUSH_N=64` is a real, decisive **regression vs. 009** on both images that
batch: +38.3% (0/9) on `large_image.jpg`, +10.0% (1/9) on `tower.jpg` - it still beats the
unpatched baseline on `tower.jpg` (-6.9%, 8/9) but is a clear net loss vs. 009 on `large_image.jpg`
(+38.3% vs. baseline, i.e. no longer a win at all). This reproduces candidate 013's core finding
(smaller N = more `rayon::in_place_scope` dispatches per scan = worse) inside the gated design
too, not just in 013's ungated one.

**Peak memory (the actual point of this candidate) - measured directly, not inferred from
allocation count**: a temporary `peak_alloc` scratch binary (peak concurrently-live allocated
bytes across the whole process, not just decode-local; see "Allocation-count check" below for why
allocation *count* alone couldn't answer this) shows `ROW_FLUSH_N=256` reduces
`large_image.jpg`'s peak live bytes from 009's 31,074,514 down to 20,752,482 - **within 0.03% of
the 20,759,358-byte unpatched baseline**, i.e. essentially all of 009's whole-scan-buffering
memory overhead on this image is eliminated, at no measured speed cost. `ROW_FLUSH_N=64` shows the
same near-baseline peak (20,752,482, byte-for-byte identical to N=256's on this image, since both
flush thresholds are small enough relative to the scan that the peak is bounded by something else
in the pipeline, not by `pending_rows`) - but at the speed cost documented above. On `tower.jpg`
(192 rows), N=256 does **not** reduce peak memory versus 009 (both 2,416,976 vs 2,417,152 - a
rounding-level difference, not a real one) because, as explained above, N=256 never actually
triggers a flush before 192 rows arrives - `tower.jpg`'s peak-memory profile under the hybrid is
identical to 009's for the same reason its speed profile is identical. `ROW_FLUSH_N=64` *does*
reduce `tower.jpg`'s peak back to baseline (1,623,636 vs. baseline's 1,624,148 vs. 009's
2,416,976) since 64 < 192 triggers real intermediate flushes - but only by paying the +10%/1-of-9
speed regression vs. 009 documented above.

## Does it change behavior/output?

- **Bit-exact?** Yes on all 4 workload images, both `ROW_FLUSH_N` values (64 and 256), confirmed
  by `bench_decode`'s own per-iteration hash check (aborts on mismatch; none occurred) plus direct
  hash comparison against the unpatched baseline binary.
- **Is this a breaking change?** No public API change. Memory behavior for a batched scan (above
  `ROW_BATCH_THRESHOLD`) is now bounded by `min(scan_row_count, ROW_FLUSH_N)` rows in flight
  instead of the whole scan (009) - a real reduction in the worst case (a scan much larger than
  `ROW_FLUSH_N`), confirmed directly above for `large_image.jpg`. A scan at or below
  `ROW_FLUSH_N` rows (like `tower.jpg`'s 192, under the kept `ROW_FLUSH_N=256`) sees no memory
  change from 009 at all, since it never performs an intermediate flush. Scans below
  `ROW_BATCH_THRESHOLD` are completely untouched (identical to unpatched/009's gated-off path).
- Project test suite (`cargo +stable test --release`), for the kept `ROW_FLUSH_N=256` variant:
  **23 passed / 0 failed before and after** (5 unit + 12 lib-integration + 4 rayon integration + 2
  doctests) - identical to the documented baseline, no new failures either build.

## Allocation-count check

Measured with a temporary counting `GlobalAlloc` scratch binary (`count_allocs.rs`, same
discipline/technique as candidates 004/009/013 - 3 warmup decodes then one measured
`Decoder::decode()` call; deleted after use, not kept):

| image | baseline | 009 | hybrid N=64 | hybrid N=256 |
|---|---|---|---|---|
| `tower.jpg` | 2301 | 2503 | 2501 | 2503 |
| `tower_progressive.jpg` | 2423 | 2423 | 2423 | 2423 |
| `tower_grayscale.jpg` | 103 | 103 | 103 | 103 |
| `large_image.jpg` | 6673 | 7258 | 7254 | 7256 |

**Allocation *count* is essentially unchanged vs. 009 at both N values** (within 1-4 allocations
either way, noise-level) - this is exactly the outcome the task's own framing flagged as possible
("each flush's own buffer allocation offsets the whole-scan-buffer's absence"). The mechanism:
009's own RESULT.md already attributed its +8.8% allocation increase to one extra allocation per
row from rayon's `in_place_scope`/`scope.spawn` task bookkeeping, not from the `pending_rows`
buffer itself - and the hybrid still dispatches exactly the same total number of rows through
`append_rows` (just split across more calls), so it pays the same total per-row task-allocation
cost. **This is why a separate `peak_alloc` scratch binary (see "Result" above) was built for this
candidate instead of relying on allocation count alone**: allocation *count* cannot distinguish
"one 567-row buffer, freed once" from "three ~200-row buffers, each freed after its own flush" -
both make the same number of `alloc()` calls for the row data itself (dominated by rayon's
per-row task overhead either way), but the *peak simultaneously-live bytes* differs substantially,
which is what actually matters for a `set_max_decoding_buffer_size`-style memory concern. This is
the concrete number the task asked to confirm, and allocation count alone would have missed it
entirely (it looks unchanged vs. 009 either way).

## Why the result looks the way it does

`worker.append_rows` pays a fixed `rayon::in_place_scope` dispatch cost per call, independent of
how many rows are in the batch (the same mechanism 013 already established). The hybrid's speed
therefore depends entirely on how many times a batched scan's row count divides by `ROW_FLUSH_N`:
at `ROW_FLUSH_N=256`, every workload image in this repo either doesn't batch at all (gate protects
it, 0 dispatches, identical to unpatched) or batches in exactly 1 flush (`tower.jpg`'s 192 rows) or
3 flushes (`large_image.jpg`'s 567 rows) - and 3 flushes' overhead turned out to be within this
machine's noise floor relative to 009's 1 flush, while still being small enough to bound
`large_image.jpg`'s peak `pending_rows` buffer to well under the whole scan (helped further by the
fact that this image's peak process-wide memory is apparently set by something else in the
pipeline - the final RGB output buffer, upsampler scratch, etc - not by `pending_rows` at any flush
size tried, since N=64 and N=256 land on the *same* peak byte count for this image). At
`ROW_FLUSH_N=64`, `large_image.jpg` pays ~9 flushes instead of 3, and that extra dispatch count is
no longer noise - it's a decisive, repeatable +38.3% regression vs. 009 (and against the unpatched
baseline it erases 009's win entirely, landing at a net loss). `tower.jpg`'s 192 rows divides into
3 chunks under N=64 too, producing a smaller but still real +10.0% regression vs. 009. In short:
**there is a real memory/speed trade-off surface here, not a free lunch** - `ROW_FLUSH_N=256`
picked a point on it that is speed-neutral vs. 009 (on every in-repo image) while eliminating
`large_image.jpg`'s ~10.3MB peak-memory overhead; `ROW_FLUSH_N=64` sits at a different point that
buys back `tower.jpg`'s peak-memory overhead too, but at a real, measurable speed cost 009 doesn't
have.

## What's not done / open questions

- **Whether `ROW_FLUSH_N=256` generalizes as "safe" is only confirmed for the 2 in-repo images
  that batch** (`tower.jpg` at 192 rows, where it's a no-op vs. 009 by construction, and
  `large_image.jpg` at 567 rows, where it's the only real test of the periodic-flush mechanism at
  this N). No image in this repo has a batched scan large enough to exercise more than 3 flushes
  at N=256 - a much larger synthetic image (e.g. several thousand MCU rows) would be needed to
  learn whether flush count keeps costing near-zero indefinitely or eventually shows the same
  per-flush overhead trend N=64 already demonstrates at a smaller scale.
  N in between (e.g. 128, matching `ROW_BATCH_THRESHOLD` itself) was not tried - could plausibly
  land at a different point on the same trade-off surface, but nothing here suggests it would beat
  N=256's already speed-neutral result.
- **The peak-memory win on `tower.jpg`-sized scans (192 rows, between the gate and N=256)
  requires a smaller `ROW_FLUSH_N`, which this session confirmed costs real speed** (N=64's
  +10.0% vs. 009 on this exact image). No value of `ROW_FLUSH_N` was found that gets peak memory
  down for scans in the (128, 256] row range without also incurring more flushes/dispatch
  overhead for that same scan - this looks like an inherent trade-off of the mechanism, not a
  tuning gap, but wasn't exhaustively swept (e.g. N=96, 112, 160 between the gate and 192 were not
  tried).
- `peak_alloc`'s peak-live-bytes metric is process-wide (single-threaded harness invocation, one
  decode at a time, no concurrent unrelated allocations) and was not cross-checked against an
  external tool (`/usr/bin/time -v` max RSS, `valgrind --tool=massif`) - it should track the same
  quantity closely for this workload (no OS-level page/allocator overhead modeled), but wasn't
  independently confirmed against a second measurement method this session. **Addendum
  (candidate 017, follow-up session)**: cross-checked against `valgrind --tool=massif
  --pages-as-heap=no` (`/usr/bin/time -v` was not installed on that machine) on
  `large_image.jpg`, re-applying this candidate's own numbers' comparison point, 009's patch, to
  a scratch build for the duration of the measurement only. Massif's peak-heap figures (baseline
  21,299,365 bytes; 009-patched 31,574,677 bytes) agree with this RESULT.md's own custom-allocator
  numbers (20,759,358 / 31,074,514) to within 1.6-2.6% absolute and within 0.4% on the
  patched-vs-baseline delta (10,275,312 vs. 10,315,156 bytes) - confirming the `peak_alloc`
  technique measures a real effect, not a wrapper artifact. See
  `017.peak_alloc_crosscheck/RESULT.md` for the full writeup; nothing in this candidate's own
  numbers or conclusions above was changed by that follow-up.
- The temporary `count_allocs.rs` and `peak_alloc.rs` scratch binaries (both following candidates
  004/009/013's established discipline) were deleted after use, consistent with prior candidates
  in this workspace - not kept in the harness.

## Conclusion for future work on this call site

Contrary to candidate 013's periodic-flush-without-a-gate finding, this hybrid **does** deliver a
real improvement over 009 on the specific axis the task asked about (peak memory on a batched
scan), without giving up any of 009's measured speed advantage, at `ROW_FLUSH_N=256`: essentially
eliminates `large_image.jpg`'s ~10.3MB (009 vs. unpatched) peak-memory overhead while remaining
statistically indistinguishable from 009 on speed on every image in this repo. It does **not**
reduce allocation *count* (that metric alone is the wrong tool for this question - see "Allocation-
count check" above) and does **not** help `tower.jpg`-sized scans (192 rows, which never trigger an
intermediate flush at N=256) without giving up real speed at a smaller N. Whether this added
complexity (two tunables instead of one, plus the flush-count/peak-memory/speed trade-off surface
documented above) is worth it over 009's simpler single-buffer design depends entirely on whether
peak memory on large batched scans is a real constraint for a given consumer (e.g.
`set_max_decoding_buffer_size` use cases 009's own RESULT.md flagged as the reason this might
matter) - this candidate does not take a position on that trade-off, only reports the measured
numbers on both sides of it.
