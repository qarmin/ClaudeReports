# Replace the flat row-count gate with a per-filter capped-size rayon thread pool

**Ported from `image` repo's `___OPTIMIZATIONS___/035.VENDORED_DISPATCH_PER_FILTER_THREAD_CAP/`.**
Originally measured in 2026-08 via a vendored `pic-scale-safe` 0.1.11 copy inside the `image`
project. **Not re-measured in this workspace** - numbers below are copied verbatim. The patch was
rebuilt by hand against this repo's current `HEAD` (`b50e020`), including adding the
`resampling_function: ResamplingFunction` parameter this candidate threads through the dispatch
functions to the three call sites in `resize_fixed_point.rs`/`resize_floating_point.rs` (not shown
in the original diff excerpt but required for it to compile - added and verified in this session).
`git apply --check` passes, `cargo check --features rayon` compiles, `cargo test --release
--features rayon` passes 5/5. Includes candidate 001's `resize_nearest.rs` gate.

Candidate 002 found the flat `dst_height >= 32` serial-or-full-pool gate is a large multi-core
*regression* for `CatmullRom`/`Gaussian`/`Lanczos3` (these filters have enough real per-row work at
small output sizes to benefit from parallelism, unlike `Nearest`). This candidate replaces the
binary gate with a per-filter capped-size thread pool: every dispatch call always goes through
`pool_for_filter(resampling_function).install(...)` - a new `src/thread_pool.rs` module with two
cached `rayon::ThreadPool`s (`num_threads(6)` for `CatmullRom`/`Gaussian`, `num_threads(4)` for
everything else) - never falling back to plain serial when the `rayon` feature is compiled in.

## What was tried

New `src/thread_pool.rs::pool_for_filter`. Every `par_chunks_exact_mut`/
`for_each_chunk_with_scratch` call in `fixed_point_dispatch.rs`/`floating_point_dispatch.rs` now
runs inside `pool_for_filter(resampling_function).install(|| { <unchanged parallel body> })`
unconditionally (no row-count check). Measured at `400x300->32x24` (`taskset -c 3` single-core,
unpinned multi-core, 24-thread host), against three baselines: `S` (true serial), `A` (today's
ungated `rayon`), `G` (candidate 002's flat gate).

| Filter | vs. ungated (A), multi-core | vs. flat gate (G), multi-core | vs. either, single-core |
|---|---|---|---|
| Triangle | **-37.4%** (6/6) | **-36.3%** (6/6) | **+0.7% to +13.6%** regression (0-2/6) |
| CatmullRom | **-29.1%** (6/6) | **-45.6%** (6/6) | **+1.3% to +7.7%** regression (0/6) |
| Gaussian | **-34.2%** (6/6) | **-46.9%** (6/6) | **+1.2% to +7.4%** regression (0-2/6) |
| Lanczos3 | **-10.4%** (6/6) | **-36.4%** (6/6) | **+0.7% to +4.9%** regression (0-2/6) |

## Result

**Fixes candidate 002's multi-core regression decisively** - beats the flat gate by -36% to -47%
on multi-core for every filter measured, including `Triangle` (the one filter the flat gate had
improved), and also beats (or, for `CatmullRom`/`Gaussian`, further improves on) the already-known
multi-core win over ungated `rayon` dispatch. This does not come free: single-core is a **small but
consistent regression** (+0.7% to +13.6%) vs. both the flat gate and true serial, since
`pool.install()` now runs on every call (even a 24-row output) instead of being skippable below a
threshold.

## Does it change behavior/output?

- **Bit-exact?** Yes, per the original measurement (20 hashes across 5 filters x 4 scenarios,
  including both below and at/above the old 32-row threshold, matched exactly).
- **Is this a breaking change?** No public API change - `pool_for_filter` is a private module.
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. `cargo check --features rayon` compiles clean.

## Why the result looks the way it does

`ThreadPool::install()` has a real, roughly fixed dispatch cost per call regardless of how much
work runs inside it. The flat gate (candidate 002) avoided that cost entirely below 32 rows by
falling back to serial, at the price of giving up real parallelism for filters with substantial
per-row cost. This candidate never gives up that parallelism - every call pays the dispatch cost,
but every call also spreads real work across a capped 4-6 thread pool, which is worth far more than
the dispatch cost once there's enough work per thread to matter. The residual single-core
regression is the dispatch cost `G`/`S` simply never pay below the old threshold.

## What's not done / open questions

- Only the `400x300->32x24` scenario was measured; whether always-pool-dispatch remains worthwhile
  at even smaller outputs (where the fixed `pool.install()` cost might dominate even the
  multi-core win) is exactly what candidate 004 (this workspace) investigates.
- `Nearest` was not separately re-timed through these dispatch functions - it never reaches them
  (routed through `resize_nearest.rs` instead, gated by candidate 001), so `pool_for_filter`'s
  `Nearest` arm is currently unreachable from this crate's own call sites.
- The single-core regression was not itself tuned away here - see candidate 004 for a floor-based
  hybrid that leaves this candidate's own established scenario (`dst_height=24`) unchanged but
  helps at genuinely small outputs.
- Not independently re-benchmarked against current `HEAD` in this session - only correctness and
  build/test were re-verified; timing numbers are historical and directional only.
