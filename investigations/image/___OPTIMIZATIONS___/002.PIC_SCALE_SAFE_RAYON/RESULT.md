# pic-scale-safe rayon feature forwarding

`Cargo.toml`'s `rayon` feature enables multi-threading for `ravif` and `exr` but, per a standing
`# TODO: add pic-scale-safe rayon in the next breaking release` comment, never forwards into
`pic-scale-safe` (the crate `DynamicImage::resize`/`resize_exact`/`resize_to_fill` delegate to).
Adding `"pic-scale-safe/rayon"` to the feature list is a one-line change; pic-scale-safe 0.1.11
already gates its horizontal/vertical convolution dispatch on `#[cfg(feature = "rayon")]` and
requires no other code change to pick it up.

**Harness note:** the crate's existing `benches/imageops.rs::bench_resize` calls the standalone
`imageops::resize` free function, a separate/older implementation that never touches
pic-scale-safe at all (confirmed by grepping for `pic_scale_safe` in `src/`, and by hashing
outputs before/after this patch - `imageops::resize`'s output is byte-identical because it isn't
even in the call graph this patch touches). Only `DynamicImage::resize`/`resize_exact` go through
`src/imageops/resize.rs` -> pic-scale-safe. `DynamicImage::thumbnail`/`thumbnail_exact` also do
**not** go through pic-scale-safe (they use the separate `imageops::thumbnail` fast
nearest-neighbor-ish integer algorithm) - the candidate's premise about "resize/thumbnail" is only
half accurate; only `resize`/`resize_exact`/`resize_to_fill` are affected. A new
`bench_resize_dynamic` criterion group was added to `benches/imageops.rs`, calling
`DynamicImage::resize_exact` on a 4032x3024 synthetic RGB8 image (matching the size used in the
callgrind profiling that originally flagged the vertical-convolution hot spot) down to 400x300,
for `Lanczos3` and `Triangle`, using `iter_batched(.., BatchSize::LargeInput)` to clone the source
per iteration (consistent with candidate 001's established convention for in-place-mutating
benchmarks) so the clone doesn't pollute the timed portion. Bundled into this candidate's
`patch.diff` since it's needed to reach the patched code path at all.

## What was tried

Two prebuilt `--release` `imageops` bench binaries (baseline vs. patched Cargo.toml), same
binary alternated per MEASUREMENT.md/README.md's established practice on this noisy machine
(load average 4.5-9.3 during this session, well under 24 cores). `taskset -c 3` for single-core
pinning; unpinned for multi-core/realistic wall-clock.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline, Lanczos3 400x300 from 4032x3024 | ~23.5-24.0 ms (run1/run2 medians) | ~21.0-22.5 ms (run1-3 medians) | pic-scale-safe rayon feature off |
| patched, Lanczos3 400x300 from 4032x3024 | ~22.4-22.7 ms | ~5.45-5.67 ms | pic-scale-safe rayon feature on |
| baseline, Triangle 400x300 from 4032x3024 | not measured | ~8.15-8.27 ms | |
| patched, Triangle 400x300 from 4032x3024 | not measured | ~3.31-3.81 ms | |

Raw paired runs (2 pairs single-core, 3 pairs multi-core for Lanczos3, 2 pairs multi-core for
Triangle - `cargo bench --bench imageops -- "dynamic_resize/DynamicImage::resize_exact ..."`,
`--noplot`, sample_size 10, 1s warmup per the bench's own `benchmark_group` settings):

Single-core (`taskset -c 3`), Lanczos3:
- baseline run 1: [22.638 22.638 24.476] ms; patched run 1: [22.223 22.363 22.538] ms (change -3.0%)
- baseline run 2: [22.331 23.967 26.189] ms; patched run 2: [22.377 22.690 23.251] ms (change -11.2%)

Multi-core (unpinned), Lanczos3:
- baseline run 1: [21.573 22.472 23.924] ms; patched run 1: [5.4374 5.5346 5.7304] ms
- baseline run 2: [20.823 20.974 21.101] ms; patched run 2: [5.4036 5.4496 5.5109] ms
- baseline run 3: [20.961 21.893 22.838] ms; patched run 3: [5.5833 5.6729 5.8422] ms

Multi-core (unpinned), Triangle:
- baseline run 1: [8.1534 8.1959 8.2382] ms; patched run 1: [3.6777 3.7305 3.8126] ms
- baseline run 2: [8.0613 8.1488 8.2667] ms; patched run 2: [3.1975 3.3106 3.3828] ms

24 logical cores available on the test machine.

## Result

**Multi-core (unpinned, realistic wall-clock): a clear, large win.** `DynamicImage::resize_exact`
with `Lanczos3` on a 4032x3024 -> 400x300 downscale drops from ~21-22.5 ms to ~5.4-5.7 ms, a
~74-76% reduction (~3.9x), consistent across all 3 paired runs with non-overlapping ranges (p <
0.001 each pair per criterion's own significance test). The cheaper `Triangle` filter also wins,
though by a smaller margin: ~8.1-8.3 ms -> ~3.2-3.8 ms, a ~55-61% reduction (~2.2x), consistent
across both pairs. Both filters parallelize embarrassingly well because pic-scale-safe's rayon
path is an unconditional `par_chunks_exact`/`par_chunks_exact_mut` split of independent output
rows (see "Why the result looks the way it does" below) - the speedup isn't specific to Lanczos3,
though the more convolution-heavy Lanczos3 kernel benefits more (more work to hide the
thread-dispatch overhead behind).

**Single-core (`taskset -c 3`, for ranking): a small, borderline-noise difference, not the
regression one might expect from thread-pool overhead.** Baseline ~23.5-24.0 ms vs. patched
~22.4-22.7 ms across 2 pairs (~3-11% "improvement", overlapping ranges between the two pairs).
Pinning to one core does not disable the rayon thread pool - the patched binary still spawns
(and, with the `par_iter` calls active for every resize, uses) a thread pool sized to the
machine's full core count, all scheduled onto the single pinned core. This should, if anything,
add contention/scheduling overhead relative to true single-threaded code, but no such regression
showed up outside noise in this session's runs; it wasn't chased further with `perf`/disassembly
(not installed on this machine, see README.md) since the multi-core number is what actually
matters for this feature's real-world usage (a from-source build enabling `rayon` implies wanting
to use more than one core).

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp.rs`, built and
  run, not committed - removed before the working tree was reverted) hashed
  (`DefaultHasher`) the raw output bytes of `DynamicImage::resize_exact(400, 300, filter)` for
  all 5 `FilterType` variants (`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`) plus
  `DynamicImage::thumbnail(400, 300)`, on the real `examples/scaledown/test.jpg` (4032x3024 JPEG)
  test image, built and run once against the baseline Cargo.toml and once against the patched
  Cargo.toml. All 6 hashes matched exactly between the two builds:
  - Lanczos3: `06cc80396dcc222e`
  - Triangle: `9c9cf96afbc7db1e`
  - CatmullRom: `f7c8b8141be830c1`
  - Gaussian: `c6048ceaa14672ef`
  - Nearest: `74b80bcccb154c6e`
  - thumbnail: `0aeb358949e2af17` (unaffected either way - `thumbnail`/`thumbnail_exact` never
    call into pic-scale-safe at all, see the harness note above; included as a control)

  This is expected, not a lucky coincidence: pic-scale-safe's rayon path splits the vertical and
  horizontal convolution passes into independent per-row/per-chunk closures with no shared
  mutable state and no floating-point reduction across chunks (each output row/chunk is computed
  from the same fixed-point weight tables regardless of which thread computes it), so there's no
  execution-order-dependent rounding to introduce nondeterminism.
- **Is this a breaking change?** No. It only changes what runs when the crate's own already-public
  `rayon` feature is enabled (already the crate's default); no public API, file format, or
  documented behavior changes. It does add a compile-time requirement that `pic-scale-safe`
  actually ships a `rayon` feature at whatever version is resolved - true for the currently
  resolved 0.1.11, confirmed via `cargo metadata`, but a project-level `Cargo.toml` pin of
  `pic-scale-safe = "0.1.5"` (looser than what's actually resolved) means a `Cargo.lock` regen
  against an old-enough 0.1.x that predates the `rayon` feature could fail to resolve; not checked
  in this session (see "What's not done" below).
- Project test suite: not run as a full suite for this candidate (the change is Cargo.toml-only
  plus a new, additive bench function - no existing source logic changed). `cargo build --release
  --bench imageops` succeeded cleanly both before and after the patch with no warnings introduced.

## Why the result looks the way it does

pic-scale-safe's dispatch code (`fixed_point_dispatch.rs`/`floating_point_dispatch.rs` in the
resolved 0.1.11 source) is written as two code paths behind `#[cfg(feature = "rayon")]`/`#[cfg(not(feature
= "rayon"))]`: without the feature, plain `chunks_exact`/`chunks_exact_mut` loops; with it,
`par_chunks_exact`/`par_chunks_exact_mut` over the same slices, unconditionally (no size
threshold gating parallelism on and off). A 400-row output split across up to 24 worker threads on
this test machine has ample independent per-row work (each row/chunk's convolution is
computationally nontrivial, especially for Lanczos3's wider kernel), so the thread-dispatch
overhead is a small fraction of the total - hence the ~4x multi-core win. The forwarding was
simply never wired up in `image`'s own `Cargo.toml`; the TODO comment removed by this patch names
the exact PR discussion (image-rs/image#2639) where this was flagged as deferred to "the next
breaking release" for reasons not stated in the comment itself (not investigated further here -
see "What's not done").

## What's not done / open questions

- Why the original PR deferred this to a "breaking release" rather than doing it immediately
  wasn't investigated (the linked PR discussion may have version/MSRV/semver reasons for pinning
  `pic-scale-safe` at 0.1.5 while relying on a `rayon` feature only present in later 0.1.x - this
  session's `cargo metadata` only reflects the currently-resolved 0.1.11, not what a fresh
  `cargo update` against the loose `"0.1.5"` requirement would resolve to on an arbitrary future
  date).
- Single-core thread-pool contention (pinning the whole rayon pool onto one core) was measured as
  "no clear regression" but not root-caused with `perf`/disassembly (unavailable on this machine).
- Larger/smaller image sizes, other filters (`CatmullRom`, `Gaussian`, `Nearest`), and the 16-bit /
  f32 pixel-format resize paths (`resize_plane16`, `resize_rgba_f32`, etc., also declared in
  `src/imageops/resize.rs`) were correctness-checked (bit-exact, `Nearest`/`CatmullRom`/`Gaussian`
  included in the hash comparison above) but not separately performance-measured.
- `resize_to_fill` and the aspect-ratio-preserving `resize` wrapper both call `resize_exact`
  internally, so the same speedup applies to them without separate measurement, but this wasn't
  independently re-confirmed.
- No attempt was made to gate the parallelism behind a size threshold (e.g. skip spawning the
  thread pool for very small images where dispatch overhead could exceed the work) - that would be
  a change inside pic-scale-safe itself, not something fixable from image's Cargo.toml alone.
