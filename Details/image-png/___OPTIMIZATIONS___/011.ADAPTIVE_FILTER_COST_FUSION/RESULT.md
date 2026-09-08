# Adaptive-filter cost accumulation fused into the filtering pass

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

`src/filter/mod.rs`'s `adaptive_filter` (the default `Filter::Adaptive` path, used by every
`Compression` level except `NoCompression`/`Fastest`) calls `filter_internal` for each of
Up/Sub/Avg/Paeth, then does a *separate* full pass over the just-written `output` row to compute
`sum_buffer(output)` (the cost heuristic), plus a 9th re-filtering pass when the winning filter
isn't Paeth (since `output` was left holding Paeth's bytes, not the winner's). The idea: have
`filter_internal` accumulate the running `sum_buffer`-style cost *while it writes* `output`,
removing the 4 separate cost-computation passes.

## What was tried

Two variants of the same idea, both against `Filter::Adaptive` on `tests/benches/kodim02.png`
(768x512 RGB8) at `FdeflateUltraFast` compression (the encoder's fast path, where filtering cost
is a meaningful fraction of total time per this workspace's `README.md`), via
`harness/encoder_bench --mode filter --only Adaptive`:

- **v1 (`patch.diff`, the literal candidate)**: `filter_internal` became
  `filter_internal_with_cost`, taking a `cost_of: impl Fn(u8) -> u64` closure and calling it on
  every output byte *inside* the existing per-chunk write loop (`for i in 0..CHUNK_SIZE { out[i] =
  ...; cost += cost_of(out[i]); }`), returning `(RowFilter, u64)`. A new `adaptive_filter_sum`
  uses this fused form with `cost_of = |b| (b as i8).unsigned_abs() as u64` (the same per-byte
  term `sum_buffer` already computes) instead of calling `sum_buffer(output)` afterwards. The
  plain `filter_internal` (used by the non-adaptive path and by `MinEntropy`, which needs a
  256-bucket histogram, not a simple additive cost, and was left untouched) became a thin wrapper
  passing `cost_of = |_| 0`, relying on monomorphization to optimize the dead accumulator away.
- **v2 (`patch_v2_chunked_no_win.diff`, follow-up investigation, not the primary patch)**: same
  idea, but instead of interleaving `cost_of` into the same `for i in 0..CHUNK_SIZE` loop as the
  write, the cost is accumulated in a *second* `for i in 0..CHUNK_SIZE` loop immediately after the
  write loop, per chunk (so it's still touching only just-written, cache-hot data, but the two
  loops are compiled separately).

| Variant | single-core p50 (taskset -c 2) | paired result (11 runs) | notes |
|---|---|---|---|
| baseline (`sum_buffer` as a separate pass, unmodified) | 1.85-1.86 ms | - | |
| v1: cost fused into the write loop | 4.87-4.99 ms | 0/11 pairs faster, median +164-166% | reproduced 3x (2 single-core paired runs of 9-11 pairs each, plus 3 direct unpinned repeats per side) |
| v2: cost in a separate loop per chunk, right after the write loop | 1.843-1.855 ms | 6/9 pairs faster, median -0.2%, "NOT consistent" per `paired.sh` | i.e. noise-floor, no measurable difference from baseline |

Confirmed both variants weren't accidentally comparing identical binaries: baseline/v1/v2
binaries all had distinct md5sums.

## Result

v1 (the literal candidate as scoped) is a **~2.6x slowdown** (1.85 ms -> 4.89 ms p50) on the
default filter path, fully consistent across every paired run (0/11, then a repeat 0/9, all
consistent verdicts) and across unpinned repeats. This is a regression an order of magnitude
larger than the ~20% cost this candidate was trying to remove.

v2, a version of the same idea that keeps the cost-accumulation loop separate from (but adjacent
to) the write loop, measures as a clean **no change** (median -0.2%, "NOT consistent" per
`paired.sh`'s own pair-count discipline - i.e. indistinguishable from noise). No memory-count
metric was collected for either variant since none of them add or remove any allocation
(`allocs=4 reallocs=19` unchanged across baseline/v1/v2 in all three configurations' RESULT
lines) - this candidate is a pure-compute question, not an allocation one.

## Does it change behavior/output?

- **Bit-exact?** Yes, for both variants. `verify=OK` every iteration for baseline/v1/v2, identical
  `out=663722B ratio=1.78x` in all three - the fused cost computation selects the exact same
  winning filter per row as the two-pass version (expected: it computes the identical sum, just
  interleaved differently with the write).
- **Is this a breaking change?** No - internal-only refactor of `src/filter/mod.rs`, no public API
  change either way.
- Project test suite, `cargo test --release`: baseline 91 passed/1 ignored (unit) + 1 passed
  (`bugfixes`) + 4 passed (`check_testimages`) + 3 passed (`partial_decode`) + 8 passed (doctests)
  = 107 passed/1 ignored. v1: identical counts, identical result. v2: identical counts, identical
  result. No new failures in either variant.

## Why the result looks the way it does

v1's per-chunk write loops (`Sub`/`Up`/`Avg`/`Paeth`, all `for i in 0..CHUNK_SIZE { out[i] = ...
}`) rely on LLVM auto-vectorizing that loop - the file's own comment above `filter_internal`
explicitly credits the 32-byte chunk size to "what achieved the best performance" via
auto-vectorization. Interleaving `cost += cost_of(out[i])` (a horizontal-reduction, scalar
dependency chain across loop iterations, plus an `i8`-cast-and-`abs` per element) into that exact
loop gives the optimizer a single loop body that mixes a vectorizable elementwise write with a
non-trivially-vectorizable running reduction; in practice it stopped vectorizing the write itself
(observed effect: ~2.6x slower, consistent with a vectorized-to-scalar regression on a
memory/ALU-bound loop, not inspected at the assembly level beyond that inference). This matches a
known pattern: fusing a scalar reduction into an otherwise-SIMD-friendly hot loop can cost far
more than the extra traversal it was meant to remove.

v2 avoids that by keeping the reduction in its own loop, immediately after the write loop but
still within the same `CHUNK_SIZE`-sized chunk (so cache locality is unaffected either way - a
32-byte chunk is far smaller than L1). That recovers vectorization for the write loop, but the
result is a wash: `sum_buffer` computed as a whole-row second pass afterwards was already reading
data that had just been written by `filter_internal` moments earlier in the *same* function call,
so it was already cache-hot for any row that fits in L1/L2 (which every realistic PNG row does -
even the corpus's largest file's row is a few KB, far under typical L2 sizes). The theorized
saving (removing 4 full row traversals) assumed those traversals cost real memory bandwidth; they
don't, on this machine, for row-sized buffers - the cost the original code pays is compute (the
`abs`+sum arithmetic, already vectorized in `sum_buffer`'s own separate 32-byte-chunked loop), not
memory traffic, and v2 doesn't reduce that compute, it just relocates it next to the write it no
longer needs to help cache for.

## What's not done / open questions

- `Filter::MinEntropy`'s cost (`entropy`, a 256-bucket histogram over the whole row) was
  deliberately left untouched in both variants - it's a whole-row statistic, not a per-byte
  additive one, and it isn't the default (`Adaptive` is), so the candidate's stated ~20%-of-encode
  claim doesn't apply to it. Fusing entropy's histogram accumulation into `filter_internal`'s
  write loop (incrementally updating a `[u32; 256]` counts table, then computing entropy from the
  final histogram in O(256) instead of a full second row pass) is a plausible but unexplored
  follow-up, gated on whether `MinEntropy` is worth optimizing at all (per this workspace's
  `README.md`, it is both slower and often produces a larger file than `Adaptive`, so this was not
  pursued here).
- The "9th pass" (re-running `filter_internal` when the winning filter isn't Paeth, since `output`
  holds Paeth's bytes at loop end) was not addressed by either variant - removing it would need
  buffering all four candidate rows (3x extra row-sized scratch memory) rather than one shared
  `output` buffer, which is a materially different, unexplored change from what was scoped here.
- No corpus-wide check was run (single real image only, `kodim02.png`) - given v1's regression is
  ~2.6x (far outside plausible corpus-to-corpus variance) and v2 is a clean noise-floor null, a
  wider corpus run is unlikely to change either conclusion, but wasn't done to confirm that.
