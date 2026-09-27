# Nearest filter, skip the capped pool entirely below the small-resize threshold

Follows up on `IDEAS.md`'s row dated 2026-08-25 (candidate 031's open question): candidate 031
found `Nearest` at `400x300->32x24` is ~26-37x slower than true serial `main` when routed through
pic-scale-safe's `rayon` dispatch, and that no capped `ThreadPool` size (1/2/4/8) recovers serial
performance because `Nearest`'s own compute (~1us) is smaller than any pool's fixed dispatch
overhead. This candidate builds the concrete next step 031 identified but didn't try: adding a
third branch to `run_resize` (candidate 029's per-filter gate) that calls the resize closure `f()`
directly for `Nearest` below the size threshold, bypassing `ThreadPool::install()` entirely,
instead of routing it through any pool (capped or not) - the same as the
`#[cfg(not(feature = "rayon"))]` path already does unconditionally.

## What was tried

Same base as candidates 021-031: candidate 002's `pic-scale-safe/rayon` Cargo.toml forward +
candidate 031's `bench_resize_dynamic` `Nearest` case (`400x300->32x24`, already added, no new
bench work needed), combined with a new `run_resize` in `src/imageops/resize.rs`: candidate 029's
exact two-pool per-filter gate (`num_threads(4)` for `Bilinear`/`Lanczos3`, `num_threads(6)` for
`CatmullRom`/`Gaussian`), plus a third `match` arm - `ResamplingFunction::Nearest => f()` - that
calls the resize closure directly with no `ThreadPool::install()` call at all, still gated by the
same `SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000` source-pixel threshold from candidate 021.

Three source states, reusing candidates 029-031's naming convention:

- **S (serial / current main)** - pristine `Cargo.toml` (`pic-scale-safe/rayon` not forwarded),
  no gate. Literally what `main` does today.
- **A (parallel, ungated)** - candidate 002's `Cargo.toml` forward only, no gate - pic-scale-safe's
  own `par_chunks_exact`/`par_chunks_exact_mut` dispatches unconditionally onto whatever the
  ambient rayon context is (the default ~24-thread global pool, since nothing installs a custom
  one).
- **E (this candidate's gate)** - `Nearest` skips the pool and calls `f()` directly below the
  threshold; `Bilinear`/`Lanczos3`/`CatmullRom`/`Gaussian` unchanged from candidate 029's gate.

Binaries alternated via `raw/paired_pic_scale.sh` (this workspace's established
`--bench <id> --exact`, criterion `time: [.. mid ..]` extraction, reused unmodified from
candidates 023/026-031). `taskset -c 3` for single-core; unpinned for multi-core (24 logical
cores; `uptime` load average ranged 1.7-11.8 across this session's runs, re-checked before each
run, stayed well below the 24-core threshold throughout). 6-8 paired runs for single-core, 10 for
multi-core, each comparison run twice to check the noise floor, matching this workspace's
established convention.

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S (serial/main), run 1 | 0.788 us | 0.718 us | (paired against E) |
| E (this candidate's gate), run 1 | 26.721 us (**+3293.0%**) | 17.272 us (**+2306.4%**) | E-faster: 0/8 single, 0/10 multi |
| S (serial/main), run 2 | 0.719 us | 0.722 us | (paired against E) |
| E, run 2 | 9.703 us (**+1250.0%**) | 17.034 us (**+2258.5%**) | E-faster: 0/6 single, 0/10 multi |
| A (ungated), run 1 (vs E below) | 9.237 us | 17.029 us | - |
| E, run 1 (vs A above) | 9.322 us (+0.9%) | 17.341 us (+1.8%) | E-faster: 3/6 single, 3/10 multi |
| A (ungated), run 2 | 9.226 us | - | - |
| E, run 2 | 9.137 us (-1.0%) | - | E-faster: 5/6 single |

Raw paired output under `raw/`: `singlecore_S_vs_E_run{1,2}.log`, `multicore_S_vs_E_run{1,2}.log`
(S vs E), `singlecore_A_vs_E_run{1,2}.log`, `multicore_A_vs_E_run1.log` (A vs E).

## Result

**The hypothesis in this candidate's own name did not hold: skipping `ThreadPool::install()` for
`Nearest` does not recover serial performance - it lands in the same overhead band as the
already-measured ungated case (A), not anywhere near true serial (S).** E vs S: **+1250% to
+3293%** single-core, **+2258% to +2306%** multi-core, 0 of 14 total single-core and 0 of 20 total
multi-core paired runs favored E - essentially identical in magnitude to candidate 031's
already-measured S-vs-A regression (+2593% to +3594%). E vs A (the actual mechanism-comparison
this candidate's hypothesis rests on): a near-wash on both axes (+0.9%/-1.0% single-core,
+1.8%/no-second-run multi-core, no consistent direction), the same near-wash shape candidate 031
already found for A vs candidate 029's `N=4` gate. **Calling `f()` directly instead of
`pool.install(f)` changes nothing measurable** - `Nearest` at this size costs the same whether
dispatched through a capped pool, the ungated global pool, or with no pool wrapper at all on the
caller's side.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_032.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`,
  over `to_rgba8()` bytes and dimensions) `DynamicImage::resize_exact` output for `Nearest` on 4
  scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large `4032x3024` `RgbImage`
  resized to `64x48` and to `400x300`), built twice: once with the gate (this candidate's `E`
  state, `--features rayon`), once with the gate/forward reverted (`S` state, no `rayon` feature).
  All 4 hashes matched exactly (`raw/hashes_S.txt`, `raw/hashes_E.txt`) - expected, since none of
  these variants change which chunk computes which output byte, only how (if at all) rayon is
  invoked to compute it.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate reapplies candidate 002's proposed forward, still not
  itself applied to `main`).
- Project test suite (`cargo test --release` for S, `cargo test --release --features rayon` for
  E): **550/551 passed both before and after**, same single pre-existing unrelated failure both
  times (`resizing_with_catmul` in `tests/regression.rs`, recorded in every prior candidate in
  this line back through candidate 007). Full output: `raw/test_S_full.txt`, `raw/test_E_full.txt`.

## Why the result looks the way it does

The candidate's premise - that `ThreadPool::install()` itself is the fixed dispatch cost - was
wrong. Reading `run_resize`'s actual effect: `pool.install(f)` only changes *which* rayon thread
pool is the ambient one while `f` runs; it does not change *whether* pic-scale-safe's own
`resize_rgb8`/etc. internally parallelize. pic-scale-safe's own implementation performs its
`par_chunks_exact`/`par_chunks_exact_mut` calls unconditionally whenever its own `rayon` Cargo
feature is compiled in (forwarded by this workspace's `Cargo.toml` change), regardless of whether
the *caller* wraps the outer call in `ThreadPool::install()` or not - if the caller does nothing
special, pic-scale-safe's internal rayon calls simply run against whatever the ambient/default
global pool is (the same ~24-thread pool candidate 031's "A" state measured). So "skip the pool"
for `Nearest` doesn't mean "skip rayon" - it means "use the default global pool instead of a
custom-sized one", which is exactly what candidate 031's "A" (ungated) state already was. This
explains why E's numbers track A's almost exactly (near-wash, no consistent direction) while both
remain enormously slower than true serial S: the actual fixed cost candidate 031 identified is
inherent to pic-scale-safe's own internal `rayon::join`/chunk-splitting/join-tree setup for this
tiny amount of work, not to the caller-side `ThreadPool::install()` wrapper this candidate removed.
Removing the wrapper cannot remove a cost that was never coming from the wrapper in the first
place. The only way to actually avoid this overhead for `Nearest` at small sizes would be to avoid
calling into pic-scale-safe's `rayon`-enabled code path entirely below the threshold (e.g. a
separate non-rayon-feature build of the relevant pic-scale-safe function, or a hand-written serial
nearest-neighbor fallback in `image` itself) - not explored here, see below.

## What's not done / open questions

- No working fix for `Nearest`'s regression was found by this candidate. The next steps identified
  by this "why" section - calling a non-`rayon`-feature-gated code path for `Nearest` specifically
  (would require pic-scale-safe to expose one, or a hand-written serial nearest-neighbor
  implementation in `image` itself below the threshold, bypassing pic-scale-safe for this one
  filter/size combination entirely) - were not built or measured here.
- Whether pic-scale-safe's own `rayon`-feature-gated code has an internal size gate of its own
  (i.e. whether it already tries to avoid spawning parallel work for very small chunk counts) was
  not checked by reading pic-scale-safe's source in this candidate - if it has none, that would be
  the more targeted upstream fix (an issue/PR against pic-scale-safe) rather than working around it
  entirely from the `image` crate's call site.
- Only `400x300->32x24` was checked (same scope as every prior candidate in this line,
  021-031). Whether pic-scale-safe's fixed per-call rayon overhead for `Nearest` shrinks relative
  to compute at some larger "small" output size was not explored.
- `perf`/disassembly were not used, but the "internal pic-scale-safe rayon dispatch, not the
  caller's `ThreadPool::install()`" explanation was confirmed directly by reading pic-scale-safe
  0.1.11's own source after the fact: `src/resize_nearest.rs`'s `resize_nearest` calls
  `dst.par_chunks_exact_mut(dst_stride)` unconditionally whenever its own `rayon` Cargo feature is
  compiled in (`#[cfg(feature = "rayon")]`, no minimum-row/size gate of any kind before choosing
  the parallel iterator) - so calling `f()` directly instead of `pool.install(f)` only changes
  which ambient rayon context `par_chunks_exact_mut` runs against (the default global pool instead
  of a caller-installed one), never whether it parallelizes at all.
