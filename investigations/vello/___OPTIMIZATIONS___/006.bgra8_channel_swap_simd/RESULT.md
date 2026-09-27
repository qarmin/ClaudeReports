# BGRA8 -> RGBA8 channel swap on image load: scalar loop -> SIMD via existing planar pattern

`ImageSource::from_peniko_image_data` (`sparse_strips/vello_common/src/paint.rs`) converts a
`peniko::ImageData` in `Bgra8` format to the RGBA8 layout `Pixmap` requires by doing
`for pixel in rgba.chunks_exact_mut(4) { pixel.swap(0, 2); }`, marked `// TODO: SIMDify` in the
source. This runs once per loaded/decoded image (cost proportional to pixel count). The same
crate's `pixmap.rs` already has a SIMD per-pixel-component pattern for the structurally similar
premultiply/unpremultiply operations (`load_interleaved_128_u8x64` + `split_u8x64`/`split_u8x32`
+ `combine_u8x16`/`combine_u8x32` + `store_interleaved_128_u8x64`), so this candidate reuses that
exact pattern, recombining the deinterleaved b/g/r/a planes as r,g,b,a instead of doing arithmetic
on them.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`. CPU: x86_64 with AVX2/AVX-512
available (`fearless_simd`'s `Level::try_detect()` picks the AVX2 dispatch path on this machine;
the SIMD helpers used here go through the same `dispatch!` macro as `pixmap.rs`'s own
premultiply/unpremultiply, so AVX2, SSE4.2, and scalar-fallback are all exercised through the
existing dispatch machinery, not something new).

## What was tried

Only one variant: replace the scalar `pixel.swap(0, 2)` loop with a `bgra_to_rgba`/
`bgra_to_rgba_impl` helper mirroring `pixmap.rs`'s `premultiply_rgba8`/`premultiply_rgba8_impl`
structure (`Level::try_detect()` + `dispatch!` + a 64-byte-chunked SIMD body with a scalar tail
for any pixel count not a multiple of 16).

A new Criterion bench group (`pixmap/bgra_to_rgba`, `swap` function) was added to the existing
`sparse_strips/vello_bench` crate (extending its existing `pixmap.rs` bench module rather than
creating a new bench crate, per the "grow a few, don't multiply" harness guidance) - it builds a
1920x1080 `peniko::ImageData` in `Bgra8` format once per iteration and times
`ImageSource::from_peniko_image_data`, i.e. the full conversion (allocation copy + channel
handling + `Pixmap::from_parts`'s own premultiply pass), not just the swap in isolation - matching
how the function is actually used.

**Measurement environment note:** this machine was running 5+ other sibling optimization
candidates concurrently during this session (shared build/bench lock), and `uptime` load average
swung between ~9 and ~24 on a 24-core box over the course of this candidate. Single isolated
readings for this ~3-5ms microbenchmark swung by as much as 60% run-to-run purely from that
contention (one single-core reading hit 5.7ms, a repeat of the *same* patched binary immediately
after hit 3.2ms) - `taskset -c N` pinning alone did not remove this, consistent with
MEASUREMENT.md's own note that pinning stops migration but not scheduling contention on the
pinned core. To get a signal out of this noise, baseline and patched binaries were built into
separate `CARGO_TARGET_DIR`s and run **interleaved** (baseline, patched, baseline, patched, ...)
so both share the same noise conditions at each point in time, and the first (highest-variance)
pair was treated as a warm-up/outlier discard, consistent with MEASUREMENT.md's "first sample
excluded" guidance.

| Variant | single-core p50 (`taskset -c 0`), 3 interleaved pairs after 1 discarded outlier pair | multi-core p50 (unpinned), 2 interleaved pairs | notes |
|---|---|---|---|
| baseline (scalar `pixel.swap(0,2)`) | 3.49 ms / 3.46 ms / 3.50 ms | 3.51 ms / 3.50 ms | Criterion point estimate (median of the estimated distribution), `--warm-up-time 1 --measurement-time 1.5` |
| SIMD (`bgra_to_rgba_impl`) | 3.01 ms / 3.00 ms / 3.18 ms | 3.11 ms / 3.04 ms | same conditions, interleaved immediately after each baseline reading above |

Discarded outlier pair (both readings elevated together, consistent with a system-wide
contention spike rather than a code difference): baseline 4.74 ms, patched 3.38 ms.

## Result

**Time**, single-core: baseline medians {3.49, 3.46, 3.50} ms vs. patched medians {3.01, 3.00,
3.18} ms across 3 interleaved pairs -> approximately **9-14% lower** time for
`from_peniko_image_data` on a 1920x1080 BGRA8 image (per-pair deltas: -13.8%, -13.3%, -9.1%).
Multi-core (unpinned): 2 interleaved pairs, -11.4% and -13.1%. All 5 pairs point the same
direction (patched faster) despite the environment's high run-to-run noise; no pair showed
patched slower than its paired baseline once the discarded outlier pair (which showed both
readings elevated by a similar amount, i.e. an environment spike, not a signal) is excluded.
Because the timed closure includes the pre-existing `to_vec()` copy and `Pixmap::from_parts`'s
own SIMD premultiply pass (both unchanged by this patch), the true speedup of the swap portion in
isolation is larger than this ~10-14% headline number - this measures the effect on the whole
`from_peniko_image_data` call, which is also the number a caller would actually experience.

No memory/allocation-count claim is made - this patch doesn't add or remove any allocation
compared to baseline (same `to_vec()`, same `Pixmap::from_parts`), only changes how the existing
buffer's bytes are permuted in place.

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified with a dedicated unit test (see below) comparing the SIMD path's
  output against the original scalar `pixel.swap(0, 2)` loop across pixel counts 0, 1, 15, 16,
  17, 31, 32, 33, 100, and 257 (covering zero pixels, one pixel, and sizes below/exactly
  at/straddling/above the 64-byte/16-pixel SIMD chunk boundary the tail-handling code has to get
  right) with pseudo-random input bytes (including random alpha, to also exercise
  `Pixmap::from_parts`'s premultiplication on top of the swapped channels) - all cases matched
  exactly.
- **Is this a breaking change?** No - internal implementation detail of `from_peniko_image_data`,
  no public API change, no observable output change for any input.
- Project test suite:
  - `cargo test --release -p vello_common`: baseline 159 passed / 0 failed / 1 ignored ->
    patched 160 passed / 0 failed / 1 ignored (the +1 is the temporary correctness-check test
    added for this investigation, described above and in `patch.diff`'s absence from the
    permanent suite - it was **not** included in `patch.diff`, per this skill's own rule against
    adding permanent tests to the project; it existed only in the working tree during this
    session and was discarded on revert).
  - `cargo test --release -p vello_cpu`: 81 passed / 0 failed, identical before and after. Both
    runs also hit the same **pre-existing, unrelated** doctest failure
    (`error[E0432]: unresolved import 'glifo::atlas::GlyphCacheStats'` in a `vello_cpu` doc
    comment) on a completely unmodified checkout, confirming it predates this patch.
  - `cargo test --release -p vello_sparse_tests` (the crate with the most direct visual/snapshot
    coverage of the rendering pipeline): **not usable in this environment** - this checkout's
    reference PNG snapshots are Git LFS pointer files (`git-lfs` is not installed here; the
    worktree setup printed "This repository is configured for Git LFS but 'git-lfs' was not
    found on your path"), so essentially every snapshot-comparison test fails identically on a
    completely unmodified baseline checkout with
    `Unsupported(UnsupportedError { format: Unknown, kind: Format(Unknown) })` (confirmed via a
    direct run of one such test, `opacity::opacity_on_layer_hybrid`, on unmodified baseline) -
    3538 failed / 89 passed / 1230 ignored, identical failure mode regardless of this patch. This
    is an environment limitation (missing `git-lfs`), not something this patch caused or could
    fix; no BGRA8-specific test exists in that suite regardless (grepped for `Bgra8` /
    `from_peniko_image_data`, no hits outside `vello_common`'s own test module).

## Why the result looks the way it does

The scalar loop being replaced is already about as cheap an operation as exists per pixel (one
2-byte swap), so this is a small, low-risk win rather than a large one: reusing the
already-proven planar SIMD load/store pattern from `pixmap.rs` avoids the per-pixel branch/index
overhead of `chunks_exact_mut(4).swap(0, 2)` by processing 16 pixels (64 bytes) per iteration
through vectorized shuffle/unpack instructions. Initial single-shot readings (not part of the
table above) showed the *opposite* direction (patched appearing slower) purely from measurement
noise in the shared, contended CI-like environment this session ran in - only after switching to
interleaved same-instant baseline/patched pairs did a consistent, reproducible direction emerge.
This is a concrete illustration of MEASUREMENT.md's warning that `taskset` alone does not
eliminate scheduling noise; a reader re-running this on a quiet, dedicated machine should expect
a cleaner and very likely larger (multi-run noise added scatter in both directions here) delta
than the ~10-14% reported above.

## What's not done / open questions

- Not benchmarked on non-x86_64 (NEON/scalar-fallback) targets - the same `dispatch!` machinery
  the existing premultiply/unpremultiply code uses is reused unmodified, so risk is low, but no
  ARM measurement was taken here.
- The environment's git-lfs gap means the project's visual snapshot suite could not be used as an
  additional correctness signal beyond the dedicated unit test written for this investigation;
  a reader with `git-lfs` available could re-run `cargo test -p vello_sparse_tests` for extra
  confidence, though no test in that suite currently targets the BGRA8 path specifically.
- A cleaner, quieter-machine re-measurement (not sharing the box with 5 other concurrent
  candidates) would tighten the confidence interval on the ~10-14% figure; the direction (patched
  faster) was consistent across every non-outlier pair measured here but the exact magnitude has
  real uncertainty given the noise described above.
