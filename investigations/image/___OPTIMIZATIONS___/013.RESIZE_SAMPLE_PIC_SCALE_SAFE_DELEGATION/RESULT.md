# imageops::resize (free function) - prototype delegation to pic-scale-safe

`src/imageops/sample.rs`'s standalone `imageops::resize` free function (generic over any
`I: GenericImageView`) has always run its own `Filter{kernel,support}`-based
`vertical_sample`/`horizontal_sample` algorithm, entirely separate from `DynamicImage::resize`'s
pic-scale-safe-backed implementation in `src/imageops/resize.rs` (candidate 002's target). This
candidate (`IDEAS.md`'s long-standing "should the free function delegate to pic-scale-safe
instead" entry, noted while working candidates 002/005) prototypes an actual delegation: a fast
path inside `resize()` that, for the single most common concrete case - a fully packed, fully
contiguous `Rgb<u8>` buffer (an `RgbImage`, or any `GenericImageView` with that exact layout) -
hands the resize off to `pic_scale_safe::resize_rgb8` instead of running the crate's own filter
kernel loop, detected at runtime via `TypeId` (since `resize`'s pixel type is a generic parameter,
not something `DynamicImage`-style concrete-enum dispatch can reach). Every other pixel
type/channel-count/layout (16-bit, f32, grayscale, alpha, non-contiguous views such as a
`SubImage`) falls back to the original algorithm unchanged - this is a scoped prototype to measure
whether delegating is worth the architectural cost at all, not a complete replacement of every
case `resize` currently supports.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: the crate's own pre-existing
`benches/imageops.rs::bench_resize` ("resize 400x300 <Filter>" and "large/resize 2000x2000
<Filter>" cases, 1920x1080 `RgbImage` source - already the exact pixel type/layout this patch
targets), reused as-is.

All numbers are from alternating A/B runs of two prebuilt `--release` criterion binaries (baseline
vs. patched), `--warm-up-time 1 --measurement-time 2` (`--measurement-time 3` for the large case),
seconds apart on the same machine (`uptime` load ~4-5 on 24 cores throughout, within this
workspace's established noise tolerance).

**`resize 400x300 Triangle`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 6.9655 ms | 6.9925 ms | 7.0113 ms | 6.7963 ms | 7.8309 ms |
| patched | 1.6791 ms | 1.6786 ms | 1.6651 ms | 1.6801 ms | 1.6629 ms |

Patched faster in 5/5 pairs. Median baseline 6.9925 ms, median patched 1.6791 ms (~4.2x faster).

**`resize 400x300 Lanczos3`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 17.960 ms | 17.748 ms | 18.116 ms | 18.146 ms | 18.056 ms |
| patched | 4.5543 ms | 4.8443 ms | 4.5891 ms | 4.6292 ms | 5.6988 ms |

Patched faster in 5/5 pairs. Median baseline 18.056 ms, median patched 4.6292 ms (~3.9x faster).

**`resize 400x300 Triangle`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 6.8715 ms | 6.8552 ms | 6.8646 ms | 6.8483 ms | 6.8534 ms |
| patched | 1.6748 ms | 1.6739 ms | 1.6709 ms | 1.6694 ms | 1.6681 ms |

Patched faster in 5/5 pairs. Median baseline 6.8552 ms, median patched 1.6709 ms (~4.1x faster).

**`resize 400x300 Lanczos3`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 17.934 ms | 17.813 ms | 17.790 ms | 17.738 ms | 18.062 ms |
| patched | 4.6039 ms | 4.5888 ms | 4.6020 ms | 4.5942 ms | 4.5806 ms |

Patched faster in 5/5 pairs. Median baseline 17.813 ms, median patched 4.5942 ms (~3.9x faster).

Single-core and multi-core numbers are near-identical for both variants: `pic_scale_safe::resize_rgb8`
is not actually parallelized here, because this dependency version's `rayon` feature is not
forwarded from `image`'s own `Cargo.toml` yet (candidate 002's finding/TODO comment at
`Cargo.toml` line 103 - "add pic-scale-safe rayon in the next breaking release" - applies equally
here). All of this candidate's win is from the algorithm/implementation change itself (SIMD in
pic-scale-safe, presumably, plus removing the generic per-tap `Filter` closure-call/trait-dispatch
overhead), not from parallelism.

**`large/resize 2000x2000 Triangle`, single-core (`taskset -c 3`, `--measurement-time 3`):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 52.729 ms | 53.296 ms | 53.987 ms |
| patched | 12.962 ms | 17.556 ms (noisy sample, low/high [14.858/20.434]) | 13.317 ms |

Patched faster in 3/3 pairs; median baseline 53.296 ms, median patched ~13.3 ms (excluding the one
noisy sample, ~4x faster; even including it, still ~3x faster) - same order of speedup as the
400x300 case, confirming the win isn't an artifact of one specific output size.

## Result

A large, consistent win: roughly 3.9-4.2x faster wall-clock for `imageops::resize` on a plain
`RgbImage` (the fast path's target case), across both a cheap filter (`Triangle`) and an expensive
one (`Lanczos3`), at both a modest downscale (1920x1080 -> 400x300) and a much larger resize
(-> 2000x2000) - 18/18 paired runs in the patched direction. This is a substantially larger win
than candidate 005's ~23-27% (which sped up the same free function's old algorithm in place,
without changing which algorithm runs) - consistent with `IDEAS.md`'s original note that this was
"a bigger potential win than any further micro-optimization of the current implementation."

## Does it change behavior/output?

- **Bit-exact? No**, for every case where the fast path actually fires (a plain, fully contiguous
  `RgbImage`). Characterized via a temporary standalone example (`examples/_resize_diff_tmp.rs`,
  written and run this session, removed before capturing `patch.diff`) that resizes a
  deterministic 1920x1080 `RgbImage` (same generator as the bench) 1920x1080 -> 400x300 with all
  five `FilterType` variants, dumping the raw output buffer from a baseline build and a patched
  build for byte-level comparison:

  | Filter | bytes differing | max abs diff (0-255 scale) | dominant delta |
  |---|---|---|---|
  | Nearest | 99.75% (359100/360000) | 254 | +1 (81%), plus a smaller fraction of larger jumps (2, 14, 15, up to -242) from picking an adjacent source pixel |
  | Triangle | 96.59% (347735/360000) | 2 | +1 (86.5%), +2 (9.8%) |
  | CatmullRom | 92.41% (332693/360000) | 3 | +1 (88%), 0 (7.6%), +2 (3.6%) |
  | Lanczos3 | 89.51% (322225/360000) | 5 | +1 (82%), 0 (10.5%), +2 (4.8%) |
  | Gaussian | 94.46% (340045/360000) | 123 | +1 (64%), 0 (5.5%), -1 (3.6%), +2/+3 (~5.6% combined); ~14.9% of bytes differ by more than 10 |

  Two distinct causes, disentangled by the delta histograms above:
  1. **The dominant `+1` delta across every filter, including `Nearest`,** is the already-known,
     already-reported accumulator bug in the baseline's `vertical_sample`
     (`___OTHER___/001.RESIZE_VERTICAL_SAMPLE_OFF_BY_ONE/`, found by candidate 005, present on
     `main` as of this same base commit): the old algorithm's normalized-weight accumulator is
     seeded with `1.0` instead of `0.0`, adding a flat `+1` (before clamping) to every channel of
     every pixel that passes through it - which is every pixel, since `resize` always calls
     `vertical_sample` before `horizontal_sample`. `pic_scale_safe::resize_rgb8` does not have this
     bug, so the patched output is the *more correct* one for this specific delta, not an
     unexplained regression.
  2. **The remaining spread (0, +2, +3, and for `Nearest`/`Gaussian` specifically, much larger
     jumps)** is a genuine algorithm/implementation difference beyond the known bug: different
     rounding of which source pixel(s) contribute (visible in `Nearest`'s occasional 14/-242-sized
     jumps, where the test pattern's `(x*13+y) % 256` value changes sharply between adjacent source
     columns) and, for `Gaussian` specifically, a much wider spread (max diff 123, ~15% of bytes
     differing by more than 10) consistent with the old algorithm's `gaussian_kernel` (sigma 0.5,
     support 3.0, defined locally in `sample.rs`) and pic-scale-safe's own `Gaussian`
     `ResamplingFunction` not being parameterized the same way - i.e. these are two different valid
     Gaussian-family filters, not a rounding-precision difference.
  - **A `SubImage` crop (packed pixels, padded rows) and an upscale case both produced byte-identical
    output** between baseline and patched (0/16200 and 0/1620000 bytes differing) in the same
    check - confirming the fast path's stricter-than-candidate-005 "fully contiguous, not just
    packed-per-pixel" guard correctly declines and falls back to the original algorithm for
    non-`RgbImage`-shaped inputs, rather than silently misinterpreting a strided buffer.
- **Is this a breaking change?** Yes, in two ways if actually shipped (this is a prototype, not
  proposed for merge as-is):
  1. Numeric output changes for every `RgbImage` resize (see table above) - a semver-relevant
     behavior change for any downstream code that depends on `imageops::resize`'s exact pixel
     values (golden-image tests, hashes, etc.), separate from whether the new values are "more
     correct."
  2. `resize`'s public signature gained a `where I::Pixel: 'static` bound (needed for the
     `TypeId`-based dispatch inside the new fast path) - this is almost certainly satisfied by
     every real `Pixel` implementation already (all are plain, lifetime-free structs), but it is
     still a signature change to a public generic function, not merely an internal one.
- **Project test suite:** 331 passed, 0 failed, both before and after (same count, no new
  failures) - `cargo test --release --lib`. The existing suite does not pin `imageops::resize`'s
  exact numeric output for `RgbImage` inputs closely enough to catch the delta characterized above
  (consistent with candidate 005's own note that this suite didn't catch the pre-existing +1 bias
  either).

## Why the result looks the way it does

Two effects compound: (1) `pic_scale_safe::resize_rgb8` is presumably SIMD-accelerated internally
(candidate 002's profiling/README notes point at this dependency using explicit SIMD paths), which
this crate's own generic `Filter{kernel: Box<dyn Fn(f32) -> f32>, support}` loop has no equivalent
of - every filter tap here is a dynamic closure call, not a compile-time-specialized SIMD kernel;
(2) the old algorithm always makes two full passes (`vertical_sample` then `horizontal_sample`)
through an intermediate `Rgba32FImage` (4 `f32` channels per pixel regardless of the source's real
channel count, see the file's own `MAX_CHANNEL` constant/comments), while pic-scale-safe's
`resize_rgb8` works natively in 3-channel `u8` end to end with no such intermediate widening. Both
of these are structural properties of the two implementations, not something a micro-optimization
of the existing filter loop (like candidates 005/009/012) could close - matching `IDEAS.md`'s
original framing of this as an architectural-level opportunity distinct from the file's other
tried optimizations.

## What's not done / open questions

- **Scope**: only `Rgb<u8>` (a plain, fully contiguous `RgbImage`) is delegated. Extending this to
  `Rgba<u8>` (needs pic-scale-safe's alpha-premultiplication handling, mirrored from
  `src/imageops/resize.rs`'s `premultiply_alpha`/`has_constant_alpha`/`unpremultiply_alpha`, none
  of which this prototype touches), `Luma<u8>`/`LumaA<u8>`, and the 16-bit/f32 variants
  `DynamicImage::resize` already supports would each need their own `TypeId` arm and their own
  correctness characterization - not attempted here; this candidate exists to answer "is
  delegating worth it at all", which the ~4x number above answers for the one case tried.
- **The `TypeId`-based runtime dispatch pattern itself is new to this codebase** (no prior
  candidate or existing source uses `TypeId` for this kind of generic specialization) - if this
  direction were pursued for real, it's worth deciding whether this pattern (vs. e.g. a sealed
  trait with a provided default returning `None`, letting concrete pixel types opt in explicitly
  and statically rather than via a runtime `TypeId` check + `unsafe` pointer-cast pair) is the
  preferred shape for the crate; this prototype picked the `TypeId` route because it required no
  changes to the `Pixel`/`GenericImageView` trait definitions themselves.
- **The numeric-difference table above was produced against `pic_scale_safe::resize_rgb8` without
  the `rayon` feature forwarded** (see "What was tried" - not enabled in this dependency version
  yet); if a future session forwards it (per candidate 002/the `Cargo.toml` TODO), the numeric
  output should be re-characterized, since parallel chunked reduction can change floating-point
  summation order for filters that accumulate more than one tap (everything except `Nearest`).
- Did not attempt `Rgba<u8>`/alpha-aware delegation, nor 16-bit/f32/grayscale variants (see
  "Scope" above) - all remain on the original algorithm, unaffected by this patch.
- Did not profile with `perf`/disassembly to break down how much of the ~4x is SIMD vs. the
  removed `f32`-intermediate/dynamic-closure overhead specifically - `perf` remains unavailable in
  this environment (see `README.md`); both are named as plausible contributors, not measured
  separately.
