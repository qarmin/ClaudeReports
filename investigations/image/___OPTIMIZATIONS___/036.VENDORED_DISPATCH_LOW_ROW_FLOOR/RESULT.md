# Hybrid low-row-floor + per-filter thread pool for vendored pic-scale-safe dispatch

Follows the IDEAS.md backlog row added 2026-08-25 (sourced from candidate 035's "What's not
done"): candidate 035 replaced candidate 034's flat `dst_height >= 32` serial-or-full-pool gate
with an always-dispatch per-filter capped thread pool (`pool_for_filter(...).install(...)`, no
row-count floor at all), fixing 034's large multi-core regression but introducing a small,
consistent single-core regression (`+4.9%` to `+13.6%`) vs. both 034's gate and true serial `S` at
this workspace's `400x300->32x24` scenario. The backlog proposed a hybrid: keep a much lower
row-count floor (well under 034's 32 - "just enough to skip `pool.install()` for genuinely trivial
outputs", 4-8 rows suggested) combined with 035's per-filter pool above that floor, instead of only
the two extremes already tried. This candidate implements that literally (`MIN_ROWS_FOR_POOL = 8`
per dispatch function) and measures it against 034 (`G`), 035 (`E`), and true serial (`S`).

## What was tried

Re-applied candidate 035's vendoring as the starting point (`vendor/pic-scale-safe/` +
`Cargo.toml`'s `[patch.crates-io]` override, `pic-scale-safe/rayon` feature forward,
`thread_pool.rs`'s `pool_for_filter`), then edited each of the 6 dispatch call sites
(`convolve_trampoline_*`/`convolve_row_*`/`convolve_column_*` in `fixed_point_dispatch.rs` and
`floating_point_dispatch.rs`): the previously-unconditional `#[cfg(feature = "rayon")] {
pool_for_filter(resampling_function).install(|| { ... }); }` block now reads `if
destination_size.height >= MIN_ROWS_FOR_POOL { pool_for_filter(...).install(|| { ...unchanged
parallel body... }); } else { ...same serial loop as the `#[cfg(not(feature = "rayon"))]` branch,
duplicated verbatim... }`, with `const MIN_ROWS_FOR_POOL: usize = 8;` local to each function
(chosen from the middle of the backlog's suggested 4-8 range). The `#[cfg(not(feature =
"rayon"))]` fallback (compiled only when the crate's own `rayon` feature is off entirely) is
unchanged from candidate 035.

Four source states, same naming convention as candidates 019-035:

- **S (serial / current main)** - `--no-default-features --features default-formats` (rayon
  feature never compiled into pic-scale-safe).
- **G (candidate 034's flat 32-row gate)** - candidate 034's own patch.diff, rebuilt fresh this
  session as a same-session control.
- **E (candidate 035's always-dispatch, no floor)** - candidate 035's own patch.diff, rebuilt
  fresh this session as a same-session control.
- **H (this candidate's hybrid, 8-row floor)** - this candidate's fix, `--features rayon`.

Binaries alternated via `raw/paired_pic_scale.sh` (candidates 023-035's script, reused
unmodified). `taskset -c 3` for single-core, unpinned for multi-core (24 logical cores). `uptime`
load average 2.3-7.4 during this session (well under the 24-core count). 5 paired runs per
comparison (slightly fewer than 034/035's 6, to fit 4 filters x 2 comparisons x 2 axes = 16 runs
in one session; still enough to read decisiveness via this workspace's own `wins >= n-1`/`<= 1`
bar).

| Filter | Comparison | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) |
|---|---|---|---|
| Triangle | G -> H | 64.739 -> 72.752 us (+12.4%, 0/5 H-faster) | 65.420 -> 41.928 us (**-35.9%**, 5/5 H-faster) |
| Triangle | E -> H | 72.739 -> 73.127 us (+0.5%, 3/5 H-faster) | 41.936 -> 40.987 us (-2.3%, 4/5 H-faster) |
| CatmullRom | G -> H | 126.240 -> 135.380 us (+7.2%, 0/5 H-faster) | 125.820 -> 74.180 us (**-41.0%**, 5/5 H-faster) |
| CatmullRom | E -> H | 135.070 -> 133.340 us (-1.3%, 4/5 H-faster) | 67.189 -> 66.886 us (-0.5%, 2/5 H-faster) |
| Gaussian | G -> H | 123.450 -> 131.060 us (+6.2%, 0/5 H-faster) | 121.780 -> 73.937 us (**-39.3%**, 5/5 H-faster) |
| Gaussian | E -> H | 130.600 -> 130.130 us (-0.4%, 4/5 H-faster) | 71.801 -> 71.797 us (-0.0%, 2/5 H-faster) |
| Lanczos3 | G -> H | 199.940 -> 208.770 us (+4.4%, 1/5 H-faster) | 197.260 -> 123.860 us (**-37.2%**, 5/5 H-faster) |
| Lanczos3 | E -> H | 207.520 -> 207.090 us (-0.2%, 4/5 H-faster) | 122.460 -> 122.850 us (+0.3%, 3/5 H-faster) |

Raw paired output under `raw/`: `singlecore_{G,E}_vs_H_<Filter>.log`,
`multicore_{G,E}_vs_H_<Filter>.log`. A supplementary spot-check with the floor temporarily raised
to 26 (i.e. above this scenario's `dst_height`) is in `raw/supplementary_floor26_CatmullRom.log`,
discussed under "Why the result looks the way it does" below.

## Result

**`H` is statistically indistinguishable from `E` (candidate 035) on both axes at this
workspace's established `400x300->32x24` scenario, for all four filters - the hybrid's 8-row
floor never activates at this scenario's `dst_height=24`, since `24 >= 8`.** Every `E -> H`
comparison lands inside noise (`-1.3%` to `+0.5%` single-core, `-2.3%` to `+0.3%` multi-core, no
comparison clears this workspace's `wins >= n-1`/`<=1` decisiveness bar on either axis). Every
`G -> H` comparison reproduces candidate 035's own `G -> E` pattern near-exactly: multi-core is a
large, decisive win (`-35.9%` to `-41.0%`, 5/5 pairs each) but single-core is a small, mostly
decisive regression (`+4.4%` to `+12.4%`, 0-1/5 pairs favoring `H`) - **the single-core regression
this candidate set out to close is still present, unchanged from candidate 035, at the exact
scenario this whole 019-035 line has always measured.**

This is not a partial improvement that fell short - it is a structural consequence of the chosen
floor value relative to the fixed benchmark scenario, confirmed directly by the supplementary
floor=26 spot-check: raising the floor above 24 (so it does activate at this scenario) makes
single-core match `G` exactly (wash, `raw/supplementary_floor26_CatmullRom.log`), but reintroduces
`G`'s already-measured large multi-core regression relative to `E` (`+71.5%` for CatmullRom, 0/5
pairs). See "Why the result looks the way it does" for the reasoning.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_036.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`,
  over raw pixel bytes and dimensions) `DynamicImage::resize_exact` output for 5 `FilterType`
  variants (`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`) across 4 scenarios each
  (40 hashes total): `RgbImage`/`RgbaImage` at `400x300->10x6` (`dst_height=6`, below this
  candidate's 8-row floor - exercises the *new* serial-fallback branch specifically, not reached
  by any earlier candidate's test in this line), `400x300->32x24` (this workspace's usual
  scenario, above the floor), and a `4032x3024` source resized to `64x48` and to `400x300` (both
  further above the floor). Built twice: once as **S** (`--no-default-features --features
  default-formats`), once as **H** (`--features rayon`, this candidate's hybrid dispatch). All 40
  hashes matched exactly between the two builds (`raw/hashes_S.txt`, `raw/hashes_H.txt`).
- **Is this a breaking change?** No public API changes to `image` itself - same as candidates
  033/034/035, the change lives entirely in the vendored `pic-scale-safe` copy plus the
  still-unmerged `pic-scale-safe/rayon` feature forward, neither of which is on `main` today.
- Project test suite: `cargo test --release --no-default-features --features default-formats` for
  S: **546 passed, 1 failed** (`raw/test_S_full.txt`). `cargo test --release --features rayon` for
  H: **550 passed, 1 failed** (`raw/test_H_full.txt`). Both share the identical single
  pre-existing unrelated failure (`resizing_with_catmul` in `tests/regression.rs`, recorded in
  every prior candidate in this line back through candidate 007) - no new failures. The 4-test
  difference in total count (546 vs. 550) is `default-formats` vs. the crate's true default
  feature set, not this candidate's change (candidates 033-035 recorded the same split).

## Why the result looks the way it does

The backlog's own framing contains an internal tension this candidate's numbers expose directly:
it asks for a floor "well under [candidate 034's] 32 - just enough to skip `pool.install()` for
genuinely trivial outputs" (suggesting 4-8 rows), while also setting the success criterion as
"preserving 035's multi-core win... down to `dst_height=24`" - i.e. wanting the *same* scenario
(`dst_height=24`) to both stay above the floor (to keep the multi-core win) and drop below it (to
avoid `pool.install()`'s single-core cost). Those two are mutually exclusive for a single flat
per-call floor: a floor low enough to leave `dst_height=24` above it (any value `<= 24`, which
"well under 32" and "4-8 rows" both satisfy) cannot possibly change behavior at `dst_height=24` at
all, so `H` inherits `E`'s exact single-core cost there unchanged. The supplementary floor=26
check confirms the other direction: pushing the floor just above 24 does make single-core match
`G`, but at the cost of taking the *same* code path `G` takes at this scenario - full serial, no
parallelism - which throws away exactly the multi-core win 035 introduced, reproducing 034's own
multi-core regression almost exactly (`+71.5%` vs. `E` for CatmullRom, next to 034's originally
measured `+32.5%`/`+45.6%`-magnitude regressions for the same filter). There is no floor value
that helps at `dst_height=24` specifically without also giving up the parallelism that scenario
already has enough real work to use productively (candidate 034's own explanation, still true
here) - the floor idea only pays off for outputs *smaller* than whatever scenario is being
measured, never for the scenario that defines where the floor is set.

## What's not done / open questions

- This candidate's own correctness check newly exercises the `dst_height=6` (below-floor) case
  this workspace had never separately hashed before (033/034/035 only tested scenarios at or
  above the old 32-row gate, or exactly at the fixed 24-row scenario) - worth keeping as a
  reusable correctness scenario for any future candidate that touches these dispatch functions'
  serial-fallback branch specifically.
- The `MIN_ROWS_FOR_POOL = 8` value itself was not swept (per the backlog's "4-8 rows" range) -
  given the structural finding above (any value `<= 24` behaves identically to `E` at this
  workspace's own scenario), sweeping within 4-8 would not change this candidate's conclusion;
  only crossing above `dst_height=24` (as the floor=26 spot-check did) changes anything, and that
  direction was already shown to just reproduce `G`'s regression.
- A floor genuinely useful for outputs *smaller* than 24 rows (e.g. a `dst_height=6` or
  `dst_height=10` real-world thumbnail request) was not separately benchmarked here - only
  bit-exactness was checked at `dst_height=6`, not timing. If a future candidate wants to
  characterize whether an 8-row floor helps at output sizes genuinely below it, that needs its own
  new criterion benchmark case (this workspace's `bench_resize_dynamic` only has `32x24`,
  `64x48`, and `400x300` destination sizes today - see `benches/imageops.rs`), not reuse of the
  existing `400x300->32x24` case.
- As in candidates 033-035, this fix was not given its own separate `___OPTIMIZATIONS___/` for
  the vendored crate - it continues the measurements already on file in *this* workspace
  (candidates 019-035). Per SKILL.md's "vendoring a suspect dependency" guidance, no decision is
  made here about upstream PR vs. local `[patch.crates-io]` vs. drop.
