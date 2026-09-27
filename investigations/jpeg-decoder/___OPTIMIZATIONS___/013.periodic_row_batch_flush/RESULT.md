# 013.periodic_row_batch_flush

Direct follow-up to `009.gated_row_batching`, per the task's own explicit lead in `IDEAS.md`
("batch every N MCU rows, flush periodically" as a middle ground between 008's whole-scan
buffering and 009's size-gated whole-scan buffering). Implements periodic flushing in
`src/decoder.rs`'s `Decoder::decode_scan`: instead of gating batching behind a per-scan row-count
threshold, rows are buffered into `pending_rows` as they're decoded and flushed via one
`worker.append_rows` call every `ROW_FLUSH_N` rows (plus a final flush for any remainder), with
**no size gate at all** - every scan, however small, goes through at least one `append_rows` call.

## What was tried

Three values of `ROW_FLUSH_N` (16, 32, 64), each a one-line `const` change to the same patch
shape (see `patch.diff`, captured for N=64, the best-performing of the three). All measured with
`tools/paired.sh` (unpinned multi-core, RUNS=9), bit-exact on every run (aborts on hash mismatch,
none occurred).

### vs. unpatched baseline

| Image (rows/scan) | N=16 (1 batch) | N=32 (2 batches) | N=64 (2 batches) |
|---|---|---|---|
| `tower.jpg` (192) | +26.1%, 0/9 | +15.0%, 0/9 / +15.3%, 0/9 | +5.2%, 3/9 (NOT consistent) / -22.2%, 7/9 (NOT consistent) |
| `tower_progressive.jpg` (64/finishing scan) | +16.1%, 0/9 | +9.2%, 0/9 / +7.6%, 0/9 | +3.7%, 0/9 (consistent) / -5.5%, 4/9 (NOT consistent) |
| `tower_grayscale.jpg` (64) | +30.7%, 0/9 | +18.8%, 0/9 / +15.6%, 0/9 | +8.8%, 0/9 (consistent) / +11.0%, 0/9 (consistent) |
| `large_image.jpg` (567) | +3.0%, 5/9 (NOT consistent - win gone) | -6.1%, 9/9 / -9.0%, 9/9 | -11.7%, 8/9 / -11.3%, 9/9 |

("X%, W/9" = median delta, patched-faster win count out of 9 paired runs; `paired.sh`'s own rule
is `wins>=8` or `wins<=1` out of 9 for "consistent", else explicitly flagged NOT consistent.)

Pattern across all three N values: smaller N (more frequent flushes = more `rayon::in_place_scope`
dispatches per scan) makes the small-scan regression *worse* (N=16's +26-31% vs N=64's +5-11%) and
erodes the large-image win (N=16 loses it entirely, 5/9 - not consistent). Larger N=64 is the best
of the three tried, but still: a decisive, repeatable regression on `tower_grayscale.jpg` (+8.8%/
+11.0%, both batches consistent) and noisy-but-never-winning results on `tower.jpg`/
`tower_progressive.jpg` (one batch each landed "NOT consistent" rather than a clean win), while
`large_image.jpg`'s win (-11.3%/-11.7%) is real but roughly a third the size of candidate 009's
(-31.5%/-39.9%) on the same image.

### vs. candidate 009's gated whole-scan approach (N=64, best periodic variant, 2 batches)

| Image | batch 1 | batch 2 |
|---|---|---|
| `tower.jpg` | 009 faster by 20.2%, 9/9 | 009 faster by 16.6%, 8/9 |
| `tower_progressive.jpg` | 009 faster by 2.6%, 9/9 | 009 faster by 2.8%, 9/9 |
| `tower_grayscale.jpg` | 009 faster by 6.1%, 9/9 | 009 faster by 6.6%, 9/9 |
| `large_image.jpg` | 009 faster by 18.3%, 9/9 | 009 faster by 19.7%, 9/9 |

Candidate 009's gated approach beats this candidate's best variant (N=64) decisively, on every
one of the 4 workload images, in both independently-run batches (8/9 or 9/9 both directions,
every time) - there is no image or batch where periodic flushing came out ahead.

## Result

Periodic flushing at every N value tried (16, 32, 64) is a real, repeatable regression on the
small/single-scan images (`tower.jpg`, `tower_progressive.jpg`, `tower_grayscale.jpg` - all under
candidate 009's `ROW_BATCH_THRESHOLD=128`) and a smaller win than candidate 009 on the one image
where 009's approach wins big (`large_image.jpg`, -11%something vs 009's -30-something%/-40%). A
direct paired comparison against a rebuilt copy of candidate 009's own patch shows 009 winning on
all 4 images, both measured batches, with no exceptions.

## Does it change behavior/output?

- **Bit-exact?** Yes on all 4 workload images and all 3 N values, confirmed by `bench_decode`'s
  own per-iteration hash check (aborts on mismatch; none occurred) plus `tools/paired.sh` refusing
  to compare identical binaries (never triggered here, since every binary compared genuinely
  differs).
- **Is this a breaking change?** No public API change. Memory behavior changes uniformly for
  every scan (not just large ones, unlike 009): peak in-flight coefficient buffer is capped at
  `ROW_FLUSH_N` rows instead of one row (as in the pre-patch/009-gated-off path) or the whole scan
  (as in 009's gated-on path) - lower peak memory than 009 for large scans, but every scan now
  pays at least one `rayon::in_place_scope` dispatch it wouldn't have paid at all under either the
  unpatched baseline or 009's gate.
- Project test suite (`cargo +stable test --release`), for the N=64 variant kept as `patch.diff`:
  **23 passed / 0 failed before and after** (5 unit + 12 lib-integration + 4 rayon integration + 2
  doctests) - identical to the documented baseline, `grep -c "^error"` = 0 both times.

## Allocation-count check (candidate 004's technique, per the task)

Measured with a temporary counting `GlobalAlloc` scratch binary (`count_allocs.rs`, added
temporarily to `harness/bench_decode/src/bin/` and deleted after use - same discipline as
candidates 004/009's original checks; 3 warmup decodes then one measured `Decoder::decode()`
call):

| image | baseline | periodic-flush N=64 | delta | 009-gated | delta |
|---|---|---|---|---|---|
| `tower.jpg` | 2301 | 2497 | **+196 (+8.5%)** | 2503 | +202 (+8.8%) |
| `tower_progressive.jpg` | 2423 | 2628 | **+205 (+8.5%)** | 2423 | **+0** |
| `tower_grayscale.jpg` | 103 | 169 | **+66 (+64.1%)** | 103 | **+0** |
| `large_image.jpg` | 6673 | 7250 | **+577 (+8.6%)** | 7258 | +585 (+8.8%) |

On the two images 009 batches anyway (`tower.jpg`, `large_image.jpg`), periodic flushing's
allocation increase is about the same magnitude as 009's (both ~+8.5-8.8%, expected - both add
roughly one extra allocation per row via rayon's per-`scope.spawn` task bookkeeping). The
difference is on the two images 009 leaves completely untouched: periodic flushing still adds
+205 (+8.5%) allocations to `tower_progressive.jpg` and +66 (+64.1%, since the baseline count is
tiny) to `tower_grayscale.jpg`, because every scan flushes through `append_rows` at least once
regardless of size - there is no way for periodic flushing to reach zero allocation overhead on a
small scan the way 009's gate does. **On the allocation-count axis, periodic flushing is strictly
worse than 009's gated approach**, not better - it does not achieve the "lower allocation-count
increase than 009's whole-scan buffering" the task asked to check for.

## Why the result looks the way it does

`worker.append_rows` unconditionally sets up a `rayon::in_place_scope` and spawns one task per row
in the batch it's given (see `src/worker/rayon.rs`), regardless of how many rows that is - there is
no cheap way to make a single flush of a handful of rows behave like the direct, synchronous
`worker.append_row` calls the unpatched code uses. Periodic flushing therefore cannot "naturally
degrade toward the old per-row behavior at small N" the way the task's framing hoped: even at
N=16, a 64-row scan still pays 4 full `in_place_scope` dispatches (one per flush) instead of 0 (the
unpatched/009-gated-off path) or 1 (008's/009's-gated-on whole-scan dispatch). Smaller N means
*more* dispatches per scan, not fewer - which is exactly why N=16 was the worst variant tried and
N=64 (fewest dispatches per scan among the three) was the best, but still never as good as 009's
literal zero-dispatches-below-threshold design on small scans, or 009's single whole-scan dispatch
on large ones. The cache-locality trap 009's own RESULT.md flagged (buffering cools decoded data
out of L1/L2 before IDCT touches it) was checked for directly - it does not appear to be the
dominant mechanism here, since N=64's regression on `tower_grayscale.jpg` (a 64-row, ~512KB scan,
buffered in exactly one flush, i.e. structurally identical to 008's original unconditional
whole-scan buffering for that image) closely matches 008's own historical finding for the same
image (+11-12%) - the regression is explained by rayon dispatch overhead alone, the same
mechanism 009 already identified and fixed by gating it away entirely below the threshold, not by
a distinct cache-cooling effect from periodic buffering itself.

## What's not done / open questions

- Only N in {16, 32, 64} was tried, all multiples chosen up front per the task; a value between 64
  and 128 (or above 128) was not tried and might reduce the small-image regression further, but
  cannot change the fundamental mechanism above (a scan of R rows always pays at least
  `ceil(R/N)` `in_place_scope` dispatches - which is `>=1` even for a scan smaller than N - so no N
  value makes a small scan pay zero dispatch overhead the way 009's gate does).
- A hybrid design (009's row-count gate to decide whether to batch at all, *and* periodic flushing
  instead of whole-scan buffering only once gated on) was not tried - this would combine 009's
  "zero overhead below threshold" with periodic flushing's lower peak memory above it, and is a
  more promising direction than periodic flushing alone if peak memory above the threshold is a
  real concern; left as a new lead in `IDEAS.md`.
- The temporary `count_allocs.rs` scratch binary and the periodic-flush patch variants (N=16, 32)
  not chosen as the final `patch.diff` were not kept; only the N=64 patch (best of the three) is
  saved here, per the "one fix per candidate folder" convention - the N=16/32 numbers above are
  fully recorded in this file's tables even though their patches are not separately saved.
