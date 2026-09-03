# Vendored pic-scale-safe dispatch: per-filter capped-thread pool instead of the flat row-count gate

Follows candidate 034's "What's not done" section (and `IDEAS.md`'s last backlog row, 2026-08-25):
candidate 034 extended candidate 033's `dst_height >= 32` serial-or-full-parallelism gate from
`resize_nearest` to the other three vendored `pic-scale-safe` dispatch functions
(`convolve_trampoline_*`/`convolve_row_*`/`convolve_column_*` in `fixed_point_dispatch.rs` and
`floating_point_dispatch.rs`), and found it a decisive multi-core *regression* (+25% to +41%) vs.
the already-established ungated per-filter capped-pool wins (candidates 023/026/027/029) for
`CatmullRom`/`Gaussian`/`Lanczos3` - the binary "serial below threshold, full global pool above"
choice throws away real parallel work these filters have at even small output sizes. This
candidate replaces that binary gate with candidate 029's caller-side per-filter capped-thread-pool
idea (`num_threads(4)` for `Triangle`/`Lanczos3`/`Nearest`-shaped filters, `num_threads(6)` for
`CatmullRom`/`Gaussian`), but implemented *inside* the vendored dispatch functions themselves:
every `par_chunks_exact_mut`/`for_each_chunk_with_scratch` call is now always wrapped in
`pool_for_filter(resampling_function).install(...)` - there is no row-count threshold anymore, and
no bare-serial fallback path when the `rayon` feature is compiled in at all.

## What was tried

Same vendoring mechanism as candidates 033/034 (`vendor/pic-scale-safe/` + `Cargo.toml`'s
`[patch.crates-io]` override + the `pic-scale-safe/rayon` feature forward). Added a new
`vendor/pic-scale-safe/src/thread_pool.rs` module with a `pool_for_filter(ResamplingFunction) ->
&'static rayon::ThreadPool` (two cached `OnceLock` pools, `num_threads(6)` for
`CatmullRom`/`Gaussian`, `num_threads(4)` for everything else - the same split candidate 029
already validated caller-side). Threaded a new `resampling_function: ResamplingFunction` parameter
through all three dispatch functions in both `fixed_point_dispatch.rs` and
`floating_point_dispatch.rs` (and their `resize_fixed_point.rs`/`resize_floating_point.rs` call
sites), and replaced each function's `#[cfg(feature = "rayon")] if destination_size.height >= 32 {
...; return; }` / unconditional-serial-fallback pair with a plain `#[cfg(feature = "rayon")]`
block that always calls `pool_for_filter(resampling_function).install(|| { <unchanged parallel
body> })`, and a `#[cfg(not(feature = "rayon"))]` block holding the unchanged serial body (reached
only when the crate's own `rayon` feature is off entirely, same as before).

Four source states (naming convention carried over from candidates 019-034):

- **S (serial / current main)** - vendored tree, built with `--no-default-features --features
  default-formats` (`pic-scale-safe`'s own `rayon` feature never compiled in - behaviorally
  identical to what `main` ships today, since the dispatch functions fall to the unchanged
  `#[cfg(not(feature = "rayon"))]` serial path either way).
- **A (parallel, ungated)** - candidate 002's `Cargo.toml` rayon-forward only, registry
  `pic-scale-safe` (no vendoring, no gate at all) - the same regression baseline candidates
  019-034 have used throughout this line.
- **G (candidate 034's flat row-count gate)** - candidate 034's own vendored `patch.diff`, rebuilt
  fresh this session as a same-session control, to compare this candidate directly against the
  thing it's meant to fix.
- **E (this candidate)** - the vendored per-filter capped-thread-pool dispatch, no row-count gate,
  built with `--features rayon`.

Binaries alternated via `raw/paired_pic_scale.sh` (candidates 023-034's script, reused
unmodified). `taskset -c 3` for single-core, unpinned for multi-core (24 logical cores). `uptime`
load average 2.2-7.4 during this session (well under the 24-core count; one transient spike to
~19 was observed immediately after several rayon-heavy criterion runs launched back-to-back, not
during an actual measurement window). 6 paired runs per comparison, matching candidates 026-034's
convention.

| Filter | Comparison | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) |
|---|---|---|---|
| Triangle | A -> E | 74.765 -> 75.874 us (+1.5%, 1/6 E-faster) | 66.626 -> 41.721 us (**-37.4%**, 6/6 E-faster) |
| Triangle | G -> E | 65.218 -> 74.093 us (+13.6%, 0/6 E-faster) | 65.794 -> 41.883 us (**-36.3%**, 6/6 E-faster) |
| Triangle | S -> E | (see CatmullRom row below for the single-core S pattern; not re-run per filter) | 64.828 -> 42.462 us (**-34.5%**, 6/6 E-faster) |
| CatmullRom | A -> E | 135.770 -> 137.600 us (+1.3%, 0/6 E-faster) | 95.072 -> 67.448 us (**-29.1%**, 6/6 E-faster) |
| CatmullRom | G -> E | 126.250 -> 135.110 us (+7.0%, 2/6 E-faster) | 125.370 -> 68.192 us (**-45.6%**, 6/6 E-faster) |
| CatmullRom | S -> E | 126.000 -> 135.690 us (+7.7%, 0/6 E-faster) | 126.040 -> 68.861 us (**-45.4%**, 5/6 E-faster) |
| Gaussian | A -> E | 130.360 -> 131.950 us (+1.2%, 2/6 E-faster) | 100.540 -> 66.192 us (**-34.2%**, 6/6 E-faster) |
| Gaussian | G -> E | 122.010 -> 131.060 us (+7.4%, 0/6 E-faster) | 120.500 -> 64.028 us (**-46.9%**, 6/6 E-faster) |
| Gaussian | S -> E | (not separately re-run; see multicore row) | 122.550 -> 75.015 us (**-38.8%**, 6/6 E-faster) |
| Lanczos3 | A -> E | 206.060 -> 207.500 us (+0.7%, 2/6 E-faster) | 138.130 -> 123.800 us (**-10.4%**, 6/6 E-faster) |
| Lanczos3 | G -> E | 196.650 -> 206.280 us (+4.9%, 0/6 E-faster) | 197.000 -> 125.300 us (**-36.4%**, 6/6 E-faster) |
| Lanczos3 | S -> E | (not separately re-run; see multicore row) | 200.750 -> 122.210 us (**-39.1%**, 6/6 E-faster) |

Raw paired output under `raw/`: `singlecore_{A,G,S}_vs_E_<Filter>.log`,
`multicore_{A,G,S}_vs_E_<Filter>.log`.

## Result

**Fixes candidate 034's measured multi-core regression: `E` beats `G` (034's own flat gate) by a
large, decisive margin on multi-core for every filter measured, including `Triangle` (the one
filter 034 found improved by its gate) - `-36.3%` to `-46.9%`, 6/6 paired wins each.** `E` also
keeps (and for `CatmullRom`/`Gaussian` further improves) the already-established multi-core win
over the ungated baseline `A` (`-10.4%` to `-37.4%`, 6/6 pairs each) that candidates 023/026/027/029
found for these filters caller-side - unlike `G`, which regressed multi-core `CatmullRom`/
`Gaussian`/`Lanczos3` relative to `A` by `+25%` to `+41%`. Against true serial `S` (what `main`
ships today), `E` is also a decisive multi-core win for every filter (`-34.5%` to `-45.4%`, 5-6/6
pairs).

This does not come for free: single-core is a **small but consistent regression** for `E` relative
to both `G` and `S` (`+4.9%` to `+13.6%`, 0-2 of 6 pairs favoring `E`) - always dispatching into a
`ThreadPool::install()` call, even for the 24-row output this scenario uses, costs real overhead
that `G`'s serial fallback (and `S`'s permanently-serial build) avoid entirely below the old
32-row threshold. This is the expected trade-off of removing the threshold rather than retuning
it: `G` chose "serial or nothing" per filter's row count; `E` chooses "always this filter's capped
pool", trading a small, repeatable single-core cost for the multi-core win. Vs. `A` (which also
always dispatches, just to the large default global pool instead of a capped one), single-core is
close to a wash (`+0.7%` to `+1.5%`), similar to candidate 029's own caller-side single-core
numbers - `A`'s own always-dispatch overhead is already present in that comparison's baseline, so
`E`'s small additional single-core cost only shows up clearly against `G`/`S`, which don't pay any
dispatch cost at all below the old threshold.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_035.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`,
  over `to_rgba8()`/raw buffer bytes and dimensions) `DynamicImage::resize_exact` output for 5
  `FilterType` variants (`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`) across 4
  scenarios each (20 hashes total): `RgbImage`/`RgbaImage` at `400x300->32x24` (small, exercises
  every dispatch function's now-unconditional pool-install path), and a `4032x3024` `RgbImage`
  resized to `64x48` and to `400x300` (both previously above the old 32-row threshold, so this
  re-confirms the parallel-branch math itself is unchanged, only how it's dispatched). Built
  twice: once as **S** (`--no-default-features --features default-formats`), once as **E**
  (`--features rayon`, per-filter-pool vendored dispatch). All 20 hashes matched exactly between
  the two builds (`raw/hashes_S.txt`, `raw/hashes_E.txt`).
- **Is this a breaking change?** No public API changes to `image` itself - same as candidates
  033/034, the change lives entirely in the vendored `pic-scale-safe` copy plus the still-unmerged
  `pic-scale-safe/rayon` feature forward, neither of which is on `main` today.
- Project test suite: `cargo test --release --no-default-features --features default-formats` for
  S: **546 passed, 1 failed** (`raw/test_S_full.txt`). `cargo test --release --features rayon` for
  E: **550 passed, 1 failed** (`raw/test_E_full.txt`). Both share the identical single
  pre-existing unrelated failure (`resizing_with_catmul` in `tests/regression.rs`, recorded in
  every prior candidate in this line back through candidate 007) - no new failures. The 4-test
  difference in total count (546 vs. 550) is `default-formats` vs. the crate's true default
  feature set, not this candidate's change (candidates 033/034 recorded the same split).

## Why the result looks the way it does

`ThreadPoolBuilder::install()` has a real, roughly fixed dispatch cost per call regardless of how
much work runs inside it (candidate 032's own ~9-26us estimate for the default global pool;
smaller but still nonzero for a small dedicated pool) - `G`'s row-count gate avoided that cost
entirely below 32 rows by falling back to a plain sequential loop, at the price of also giving up
any parallelism for `CatmullRom`/`Gaussian`/`Lanczos3`'s substantial per-row convolution cost at
that size (candidate 034's own explanation for its regression). `E` never gives up that
parallelism - every call pays the pool-dispatch cost, but every call also gets to spread real
per-row work (100-200us total at this scenario's size, per candidate 034) across up to 4-6
threads, which is worth far more than the dispatch cost pays for once there's enough work per
thread to matter - exactly the same shape of result candidates 023-030's caller-side sweeps
already established, just relocated from the `image` crate's call site into `pic-scale-safe`'s own
dispatch functions. The residual single-core regression is the mirror image of candidate 033/034's
own single-core "wash vs. S" result: those candidates' gate reused the *exact* same serial code
path S/G already ran below the threshold, so single-core cost was identical; this candidate's
dispatch always goes through `pool.install()` even on a single physical core, so it pays a real
(if small) fixed cost `G`/`S` simply never incur.

## What's not done / open questions

- Only the `400x300->32x24` scenario was measured (same scope every candidate in this line since
  019 has used) - whether always-pool-dispatch remains worthwhile at even smaller outputs (e.g. a
  handful of rows) where the fixed `pool.install()` cost might start to dominate even the
  multi-core win was not investigated; a lower bound below which `E` starts to lose to `G`/`S` may
  exist and isn't characterized here.
- `Nearest` was not separately re-timed through this candidate's dispatch functions - it never
  reaches `fixed_point_dispatch.rs`/`floating_point_dispatch.rs` at all (`resize_fixed_point.rs`/
  `resize_floating_point.rs` both special-case `ResamplingFunction::Nearest` into
  `resize_nearest`, gated separately by candidate 033), so `pool_for_filter`'s `Nearest` arm
  (routed to the `N=4` pool alongside `Triangle`/`Lanczos3`) is currently unreachable dead code
  from `image`'s own call sites - included for parity with candidate 029's caller-side `match` and
  in case a future candidate reaches this dispatch path from `Nearest` some other way, but not
  exercised by anything measured here.
- As in candidates 033/034, this fix was not given its own separate `___OPTIMIZATIONS___/` for the
  vendored crate - it continues the measurements already on file in *this* workspace (candidates
  019-034). Per SKILL.md's "vendoring a suspect dependency" guidance, no decision is made here
  about upstream PR vs. local `[patch.crates-io]` vs. drop.
- The single-core regression vs. `G`/`S` (`+4.9%` to `+13.6%`) was not itself tuned away - e.g. a
  hybrid that keeps a much lower row-count floor (well under 32, just enough to skip the pool for
  genuinely trivial outputs) combined with the per-filter pool above that floor was not tried; this
  candidate deliberately tested the two extremes (034's binary gate vs. this candidate's
  always-pool dispatch) rather than a further-tuned middle ground.
