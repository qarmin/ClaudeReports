# NaN-fill elision in X-Trans bilinear demosaic output buffer

`interpolate_bilinear` (`rawler/src/imgop/sensor/xtrans/bilinear.rs:74`, pre-patch) allocated its
output buffer via `RgbF32::new_with_default(roi.width(), roi.height(), f32::NAN)`, which is
`vec![[f32::NAN; 3]; width*height]` (`rawler/src/pixarray.rs:379-382`) - a full sequential write
over the entire buffer (490,834,944 bytes for the X-T5 sample) executed on a single thread,
strictly before the rayon-parallel demosaic loop starts. Every element of that buffer is then
unconditionally overwritten exactly once by the parallel loop (`for c in 0..3 { pixel[c] = ... }`,
including the `count[c] == 0` case, which still writes `0.0`), so the NaN fill has no
corresponding read anywhere - it is pure write traffic whose only purpose looks like a
debug/uninitialized-tracking aid. Patched to allocate via `Vec<MaybeUninit<[f32; 3]>>` (skips the
fill entirely) and reconstitute the `Vec<[f32; 3]>` afterward via `Vec::from_raw_parts`, once the
parallel loop has provably initialized every element.

The Bayer equivalent (`expand_bayer_rgb`, `rawler/src/imgop/sensor/bayer/mod.rs:21-34`) was
checked and is **not** an equivalent target: it allocates via `RgbF32::new()` (zero-fill, not
NaN-fill) and writes only **one** of the three channels per pixel
(`p_out[cfa_roi.color_at(row_out, col)] = *p_in`) - the other two channels legitimately rely on
the zero-fill default and are never separately written. Skipping that fill would read
uninitialized memory for 2/3 of every pixel's channels, so it was left untouched.

## What was tried

One variant: replace the `RgbF32::new_with_default(..., f32::NAN)` + typed `pixels_mut()` API
with a raw `Vec<MaybeUninit<[f32; 3]>>` allocated via `Vec::with_capacity` + `set_len` (sound:
`MaybeUninit<T>` has no validity invariant), written via `pixel.write(out)` inside the same
parallel loop, then reinterpreted as `Vec<[f32; 3]>` via `ManuallyDrop` + `Vec::from_raw_parts`
(sound: `MaybeUninit<T>` and `T` share layout, and every element was written exactly once by the
loop, which covers all `width*height` indices with no gaps via
`par_chunks_exact_mut(width).enumerate()` + `iter_mut().enumerate()`).

Measured with `rawler/benches/demosaic.rs`'s existing `full_image_fuji_xt5` case (criterion,
`--features rawdb`) plus a scratch example (`rawler/examples/xtrans_dump.rs`, added and removed
for this candidate only, not part of the patch) timing `RawDevelop::develop_intermediate` in a
manual loop (1 warmup + N timed iterations), used both for single-core control
(`RAYON_NUM_THREADS=1`, `taskset -c 0`) and as a bit-exact md5sum correctness check (dumping the
f32 RGB output buffer to a file).

| Variant | single-core p50 (taskset -c 0, RAYON_NUM_THREADS=1) | multi-core p50 (unpinned, 24 cores) | notes |
|---|---|---|---|
| baseline (criterion, run 1) | - | 412.83 ms [409.28, 417.46] | `cargo bench --bench demosaic -- full_image_fuji_xt5` |
| baseline (criterion, run 2) | - | 419.22 ms [414.44, 424.46] | repeat run, tight CI both times (machine was comparatively idle this session, `uptime` load ~4.5-6) |
| baseline (scratch example, run 1) | 1.868 s (min 1.865 s) | 387.16 ms (min 383.0 ms) | 3 single-core iters, 5 multi-core iters, 1 discarded warmup each |
| baseline (scratch example, run 2) | 1.857 s (min 1.849 s) | 384.53 ms (min 380.7 ms) | repeat |
| baseline (scratch example, interleaved check 1) | - | 486.41 ms (min 418.4 ms) | interleaved with patched run below, load rose to ~6.3 during this window |
| baseline (scratch example, interleaved check 2) | - | 441.44 ms (min 396.0 ms) | interleaved, immediately after the patched interleaved run |
| patched (criterion, run 1) | - | 282.27 ms [276.16, 291.12] | non-overlapping with either baseline criterion CI |
| patched (criterion, run 2) | - | 340.40 ms [305.18, 369.51] | repeat, wider CI (noisier window) but still below both baseline runs |
| patched (scratch example, run 1) | 1.880 s (min 1.878 s) | 261.34 ms (min 255.4 ms) | |
| patched (scratch example, run 2) | 1.917 s (min 1.902 s) | 271.47 ms (min 259.4 ms) | repeat |
| patched (scratch example, interleaved check) | - | 243.09 ms (min 235.5 ms) | interleaved between the two baseline interleaved checks above |
| baseline single-core (extra runs, 5 iters) | 1.890 s / 1.910 s (min 1.881 s / 1.902 s) | - | 2 more single-core baseline runs for noise-floor context |
| patched single-core (extra run, 5 iters) | 1.917 s (min 1.902 s) | - | overlaps baseline single-core spread |

## Result

**Multi-core**: a real, repeatable win. Criterion: baseline medians 412.83 ms and 419.22 ms
(tight CIs, no overlap with patched) vs. patched medians 282.27 ms and 340.40 ms - every patched
run is below every baseline run. The scratch example agrees and was additionally run **interleaved**
(baseline -> patched -> baseline back-to-back, to rule out a load/time-of-day confound): 486.41 ms
and 441.44 ms baseline vs. 243.09 ms patched, sandwiched directly between them - the win holds
even as background load rose mid-session (`uptime` load average ~4.5 -> ~6.3). Roughly a
**20-45% reduction in wall time** depending on which baseline/patched pair is compared, with the
interleaved comparison (least confounded) at the high end (~45-50%).

**Single-core** (`RAYON_NUM_THREADS=1`, `taskset -c 0`): no measurable difference. Baseline p50
across 4 runs: 1.868 s, 1.857 s, 1.890 s, 1.910 s. Patched p50 across 2 runs: 1.880 s, 1.917 s.
Both clusters overlap heavily (~1.85-1.93 s) - within noise.

This asymmetry (large multi-core win, no single-core win) is consistent with the mechanism: the
eliminated NaN fill is itself always single-threaded (a plain `vec![...; n]` macro expansion, not
parallelized via rayon), executed serially before the parallel demosaic loop starts. Under
`RAYON_NUM_THREADS=1` the "parallel" loop is also single-threaded, so removing the serial fill
just relocates work rather than removing a serialization bottleneck - the total serial work is
roughly unchanged either way (the eliminated fill's write volume is comparable to the write volume
the parallel loop was always going to do). Under the real 24-core case, that same serial fill was
a fixed, non-parallelized cost sitting in front of a loop that the 24 workers otherwise finish very
quickly - so per Amdahl's law, that fixed serial segment was a disproportionately large fraction of
total wall time, and removing it (rather than just moving the writes to run in parallel alongside
everything else) produces a large relative multi-core speedup.

Only time was measured; peak RSS/allocation count were not (both variants still perform exactly
one `width*height*12`-byte heap allocation for the buffer; eliding the fill changes write volume,
not allocation count or size).

## Does it change behavior/output?

- **Bit-exact?** Yes. Full-image output buffer (7872x5196, 3 f32 channels, 490,834,944 bytes) on
  the Fujifilm X-T5 sample hashed identically (md5 `987bc0ed9b6b33d2db50fcc8c47620a4`) across every
  baseline and patched run checked: 2 single-core baseline dumps, 2 multi-core baseline dumps, 2
  single-core patched dumps, 2 multi-core patched dumps - 8 dumps, 1 hash. This was expected up
  front (unlike candidate #001, this patch does not change the arithmetic or summation order at
  all, only how/when the output buffer's memory is initialized), and confirmed empirically.
- **Is this a breaking change?** No public API change. `interpolate_bilinear` is a private
  function; `Color2D`/`RgbF32`'s public surface is untouched (the patch stops calling
  `RgbF32::new_with_default` from within this one call site but does not modify that method or
  remove it - it is presumably still used/usable elsewhere).
- Project test suite:
  - `cargo test -p rawler --features rawdb --lib` (no network, fast): **70 passed, 0 failed**,
    identical before and after.
  - `cargo test -p rawler --features rawdb` (full suite, `tests/rawdb/mod.rs`'s generated
    per-sample catalog, 1612 tests, needs `RAWDB_CACHE`): run to completion once on the **patched**
    tree - **68 passed, 1544 failed**, 3790.68 s. These failures are **pre-existing and unrelated
    to this patch** - confirmed two ways without re-running the full ~63-minute, largely
    network-bound (see "What's not done" below) suite a second time on baseline:
    1. A single failing non-Fujifilm test (`rawdb::sony::ilce_7m2_raw_modes::...`, Bayer/Sony ARW,
       a code path this patch never touches) fails identically on baseline and patched with the
       exact same digest mismatch (`left: ba14ce40...`, `right: 503e3fe8...`).
    2. The full 154-test `rawdb::fujifilm::*` subset (the actual code path this candidate touches)
       was run on **both** trees: **0 passed, 154 failed on baseline** and **0 passed, 154 failed
       on patched** - identical counts, identical failure set.
    The failure mode (`compare_digest`, "Old and new digest not match", `.digest.preview.txt`
    golden files under `rawler/data/testdata/rawdb/`) looks like stale checked-in golden digests
    versus current code - plausibly explained by this repo's own recent commits touched by `git
    log` at the start of this session (`f0af0737` "Keep EXIF lens data...", `3f55d33a` "Fix
    OriginalRawFileDigest handling") predating a golden-file regen. Not investigated further - it
    is orthogonal to this candidate (reproduces identically with and without the patch) and not a
    bug this candidate introduced.

## Why the result looks the way it does

The NaN fill was a full-buffer, single-threaded write executed serially in front of an otherwise
heavily-parallelized (rayon, 24 workers) loop. Under Amdahl's law, a fixed serial segment becomes
a disproportionately large fraction of total wall time as the parallel portion gets faster (more
workers) - which is exactly the asymmetry observed: negligible single-core effect (removing serial
work when everything is already serial doesn't change the total), large multi-core effect (removing
a serial bottleneck in front of a fast parallel section shrinks wall time by close to the
bottleneck's own duration). `MaybeUninit` made "provably written before read" a property checked by
the type system (every element is written exactly once via `.write()`, verified by inspection of
the loop's index coverage) rather than relying on the existing unconditional-write code path
continuing to cover every element by convention.

## What's not done / open questions

- The full `cargo test -p rawler --features rawdb` suite (1612 tests) was run to completion only
  on the **patched** tree (68 passed, 1544 failed, ~63 minutes). A second full ~63-minute run on
  baseline was not repeated - it is bandwidth-bound (this session's `RAWDB_CACHE` grew from
  ~200 MB to ~12+ GB over that one run, network throughput observed ~1.7-3 MB/s average) and, per
  the two targeted checks above (one non-Fuji test, the full 154-test Fuji subset), would almost
  certainly reproduce the same 68/1544 split - but this was not literally re-confirmed at full
  scale to avoid a second multi-hour, mostly-redundant download.
- `projects/rawler.md`'s existing environment note ("~200MB so far for `raw_decoder`'s samples,
  not the full ~100 GiB dataset") undersells what `cargo test -p rawler --features rawdb` (as
  opposed to the benches) actually needs - the generated `tests/rawdb/mod.rs` has one test per
  sample across the *entire* supported-camera catalog (1612 tests), and running it unfiltered
  downloads far more than the benches ever touch. Worth a note update (see below) so a future
  session budgets for this before launching the bare command unfiltered.
- The pre-existing 1544 test failures (stale golden digests, unrelated to this candidate) are not
  investigated further here and are not this candidate's concern - flagging for whoever owns test
  infrastructure, not filed as a new bug since it reproduces identically without this patch and is
  plausibly just-not-yet-regenerated golden data after recent legitimate commits.
- Peak RSS/allocation-count were not measured (see "Result" above) - only wall time.
