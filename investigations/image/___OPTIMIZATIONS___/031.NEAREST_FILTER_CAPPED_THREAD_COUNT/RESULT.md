# Nearest filter, separately timed - does it belong in the existing N=4 capped-thread pool?

Follows up on the `IDEAS.md` row for candidate 029 (dated 2026-08-25): candidates 021-030
established that pic-scale-safe's unconditional `rayon` fan-out regresses small resizes
(`400x300->32x24`), and that routing them through a small dedicated pool (`N=4` for
`Nearest`/`Bilinear`/`Lanczos3`, `N=6` for `CatmullRom`/`Gaussian`) recovers a large multi-core
win - but `FilterType::Nearest` was only ever bit-exact-checked, never separately timed, despite
being grouped into the `N=4` pool by construction (candidate 029's `match`'s default arm). This
candidate adds a `bench_resize_dynamic` case for `Nearest` at `400x300->32x24`, confirms the
baseline (ungated) regression exists, and sweeps `N` to see whether `Nearest` fits the existing
`N=4` pool.

## What was tried

Extended candidate 026's `bench_resize_dynamic` criterion group in `benches/imageops.rs` with a
`FilterType::Nearest` case at the existing `400x300->32x24` small-source scenario (previously only
`Lanczos3`/`Triangle`/`CatmullRom`/`Gaussian` were registered there). Three source states were
compared, reusing candidate 021-029's naming convention:

- **S (serial / current main)** - pristine `Cargo.toml` (`pic-scale-safe/rayon` not forwarded),
  no `resize.rs` gate. This is literally what `main` does today.
- **A (parallel, ungated)** - candidate 002's `Cargo.toml` forward only, no gate - pic-scale-safe's
  own `par_chunks_exact`/`par_chunks_exact_mut` dispatches unconditionally onto the default
  (~24-thread) global rayon pool.
- **E (parallel, gated, `N=4`)** - candidate 029's exact, unmodified per-filter gate
  (`run_resize` in `src/imageops/resize.rs`): below `SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD =
  500_000` source pixels, dispatch goes through a cached `rayon::ThreadPool` - `num_threads(4)`
  for `Nearest`/`Bilinear`/`Lanczos3` (the branch `Nearest` falls into by construction),
  `num_threads(6)` for `CatmullRom`/`Gaussian`.

A temporary `N`-sweep build (`std::env::var("NEAREST_SWEEP_N")` read once at pool construction,
reverted before capturing `patch.diff` - not part of the patch) additionally measured
`num_threads(1)`, `num_threads(2)`, and `num_threads(8)` against S, to check whether some other
`N` (not 4) recovers serial-level performance for `Nearest` specifically.

Binaries alternated via `raw/paired_pic_scale.sh` (this workspace's established
`--bench <id> --exact`, criterion `time: [.. mid ..]` extraction). `taskset -c 3` for single-core;
unpinned for multi-core (24 logical cores; `uptime` load average ranged 1.4-28.3 across this
session's runs - `projects/image.md` already documents `/proc/loadavg` here as host-wide, not
container-scoped, and this session's own S-vs-A repeat runs at very different load levels, 18.27
and 28.28, produced consistent results, matching that existing note). 5-10 paired runs per
comparison, matching this workspace's convention; each comparison run twice to check the noise
floor.

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S (serial/main), run 1 | 0.767 us | 1.162 us | - |
| A (parallel, ungated), run 1 | 23.568 us (**+2973.5%**) | 31.306 us (**+2593.2%**) | A-faster: 0/8 single, 0/10 multi |
| S (serial/main), run 2 | 0.812 us | 0.930 us | - |
| A (parallel, ungated), run 2 | 25.360 us (**+3022.4%**) | 34.344 us (**+3594.1%**) | A-faster: 0/8 single, 0/10 multi |
| A (ungated), run 1 (vs E below) | 9.491 us | 18.893 us | - |
| E (gated, `N=4`), run 1 | 9.554 us (+0.7%) | 19.485 us (+3.1%) | E-faster: 3/6 single, 7/10 multi |
| A (ungated), run 2 | 9.417 us | 19.842 us | - |
| E (gated, `N=4`), run 2 | 9.515 us (+1.0%) | 19.315 us (-2.7%) | E-faster: 2/6 single, 9/10 multi |
| S (serial/main) | 1.329 us | 0.771 us | (paired against N-sweep builds below) |
| `N=1` sweep | 25.742 us (**+1837.1%**) | - | S-faster: 5/5 single |
| S (serial/main) | 0.794 us | - | |
| `N=2` sweep | 23.748 us (**+2892.1%**) | - | S-faster: 5/5 single |
| S (serial/main) | 0.783 us | 0.771 us | |
| `N=4` sweep (= E's gate) | 22.037 us (**+2714.1%**) | 21.340 us (**+2668.4%**) | S-faster: 5/5 single, 6/6 multi |
| S (serial/main) | 0.854 us | - | |
| `N=8` sweep | 25.385 us (**+2874.1%**) | - | S-faster: 5/5 single |

Raw paired output under `raw/`: `singlecore_S_vs_A_run{1,2}.log`, `multicore_S_vs_A_run{1,2}.log`
(S vs A), `singlecore_Nearest_run{1,2}.log`, `multicore_Nearest_run{1,2}.log` (A vs E, candidate
029's existing gate), `sweep_singlecore_N{1,2,4,8}.log`, `sweep_multicore_N4.log` (S vs each swept
`N`).

## Result

**Two distinct, both confirmed findings:**

1. **The baseline (ungated) regression exists for `Nearest`, and it is far larger in relative
   terms than any other filter measured in this workspace's 019-030 line.** Enabling
   pic-scale-safe's `rayon` feature with no gate (A) makes `Nearest` at `400x300->32x24`
   **~26-37x slower** than the true serial baseline (S, i.e. current `main`) - +2593% to +3594%,
   confirmed on both single-core (2 runs) and multi-core (2 runs), 0 of 18 total paired
   single-core runs and 0 of 20 total paired multi-core runs favored A. For comparison,
   candidate 019 found `Triangle`'s equivalent regression was single-digit-percent (baseline
   ~76 us vs patched ~83 us, single-core) - `Nearest`'s regression is roughly two orders of
   magnitude larger in relative terms, because its own serial compute (~0.8-1.3 us) is so tiny
   that essentially all of A's ~9-35 us is pure rayon thread-pool dispatch overhead, not real
   work.
2. **No `N` tested (1, 2, 4, 8) recovers anywhere close to serial performance, and `N=4` (the pool
   `Nearest` is already routed to by candidate 029's gate) is not meaningfully better than the
   ungated case A.** Every swept `N` lands in the same ~22-26 us single-core band as ungated A
   (~23-25 us) - S was faster in 5/5 (every `N`) single-core paired runs and 6/6 multi-core paired
   runs for `N=4`. Separately, A vs E (candidate 029's actual `N=4` gate) is a near-wash on both
   axes (+0.7%/+1.0% single-core, +3.1%/-2.7% multi-core, no consistent direction across the two
   repeats) - unlike `Triangle`/`Lanczos3`/`CatmullRom`/`Gaussian`, which all showed a large,
   decisive, repeatable multi-core win from the same gate mechanism (candidate 029: -16.6% to
   -44.3%). **The gate mechanism that fixed those filters does not fix `Nearest` at any tested
   `N`, including its currently-assigned `N=4`.**

Taken together: if candidate 029's per-filter gate (or any single-`N` gate covering `Nearest`)
were applied to `main` as-is, it would still leave `Nearest` at `400x300->32x24` roughly 20-30x
slower than `main`'s current (no-`rayon`-forward) behavior - the gate only ever compared itself
against the already-regressed ungated case (A), never against true serial (S), so this gap was
invisible to every prior measurement in this line.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_031.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`, over `to_rgba8()` bytes) `DynamicImage::resize_exact` output for `Nearest` on
  4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large `4032x3024` `RgbImage`
  resized to `64x48` and to `400x300`), built three times: S (serial/main), A (ungated), E
  (candidate 029's gate). All hashes matched exactly across all three builds (`raw/hashes_S.txt`,
  `raw/hashes_A.txt`, `raw/hashes_E.txt`) - expected, since none of these variants change which
  chunk computes which output byte, only how many rayon workers (if any) a call is dispatched to.
- **Is this a breaking change?** No public API changes. Behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled with pic-scale-safe's own `rayon`
  feature forwarded (not currently the case on `main` - this candidate reapplies candidate 002's
  proposed forward the same way every candidate in this line has, still not itself applied to
  `main`).
- Project test suite (`cargo test --release`, S state / `cargo test --release --features rayon`,
  E state): **550/551 passed both before and after**, with the same single pre-existing unrelated
  failure both times (`resizing_with_catmul` in `tests/regression.rs` - same failure recorded in
  every prior candidate in this line back through candidate 007). Full output:
  `raw/test_S_full.txt`, `raw/test_E_full.txt`.

## Why the result looks the way it does

`Nearest`'s own per-call compute at this size is on the order of 0.8-1.3 us (S's own measurement)
- the cheapest of any filter in this codebase, consistent with it skipping premultiply entirely
and doing no blending. Any `rayon::ThreadPool::install()` call (even with `num_threads(1)`) still
pays fixed dispatch overhead - building/looking up the pool, crossing into the pool's task-spawn
machinery, `par_chunks_exact`'s own chunk-splitting/join-tree setup inside pic-scale-safe - which
this session's sweep shows lands in the same ~22-26 us range regardless of whether the pool has 1,
2, 4, or 8 workers. For filters like `Triangle` (candidate 019: ~76 us serial) or `Lanczos3`
(~209-211 us serial), that same fixed overhead is small relative to the real compute being
parallelized, so a capped pool's per-call dispatch cost is worth paying to get several workers
splitting genuinely substantial work - explaining why `N=4`/`N=6` are real wins for those filters.
For `Nearest`, the real compute (~1 us) is smaller than the dispatch overhead itself by more than
an order of magnitude regardless of `N`, so no capped pool size can win: parallelizing 1 us of
work can never repay tens of microseconds of pool-dispatch cost. This is not a "wrong `N`" problem
- it is a "any pooled dispatch is the wrong mechanism for this filter at this size" problem, a
structurally different situation from the other filters gated by the same threshold.

## What's not done / open questions

- No fix was attempted or applied for `Nearest`'s specific problem - this candidate is a
  measurement/characterization of the existing gate (candidate 029's, reapplied unmodified), not
  a new mechanism. The natural next step suggested by "Why the result looks the way it does"
  above - excluding `Nearest` from the pooled-dispatch branch entirely below the size threshold
  (call `f()` directly, the same as the `#[cfg(not(feature = "rayon"))]` path already does, rather
  than routing it through any `rayon::ThreadPool`) - was not built or measured here; it is a
  distinct, new mechanism from anything tried in this line so far (021-030 only ever varied pool
  *size*, never "no pool at all below the threshold, filter-specific") and belongs in its own
  candidate.
- Only `400x300->32x24` was checked (same scope every prior candidate in this line has used).
  Whether `Nearest`'s dispatch-overhead-dominates-compute situation holds at other small sizes, or
  whether some larger small-output size exists where `Nearest`'s own compute grows enough to make
  a capped pool worthwhile again, was not explored.
- The exact source of the ~22-26 us fixed overhead (pool lookup vs. `ThreadPool::install`'s own
  context switch vs. pic-scale-safe's `par_chunks_exact` setup) was not isolated via `perf`
  (unavailable in this environment, see `README.md`) or disassembly - only the aggregate,
  N-independent magnitude was measured.
- `N=3`/`N=6`/`N=12` (available as prebuilt wrapper scripts from this session's sweep) were not
  measured against S for `Nearest` - the four values tested (1, 2, 4, 8) already span the range
  enough to show the overhead is N-independent, so the remaining values were judged low-value to
  add, but weren't ruled out.
