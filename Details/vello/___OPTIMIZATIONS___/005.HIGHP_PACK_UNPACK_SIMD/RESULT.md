# highp (`F32Kernel`) `pack`/`unpack`: SIMDify the per-pixel u8<->f32 tile conversion

`sparse_strips/vello_cpu/src/fine/highp/mod.rs`'s `F32Kernel::pack`/`unpack` (the functions that
copy the f32 blend scratch buffer to/from the u8 output pixmap row, run for every tile drawn via
the 'highp' f32 fine-rasterization path) were explicitly flagged `// TODO: SIMDify` and took an
unused `_simd: S` parameter - the conversion was done one scalar float/byte at a time in a
`for y in 0..height { for dx in 0..width { ... } }` loop, even though the same crate already has
a working SIMD `u8<->f32` conversion (`NumericVec::from_f32`/`from_u8`, used elsewhere in this
same file and in `fine/common/image.rs`). Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`,
`cargo 1.98.0`.

## What was tried

The scratch buffer's layout for a fixed tile column `dx` is 16 contiguous `f32`s (4 rows x RGBA),
matching `f32x16`/`u8x16` exactly - but the destination pixmap is real row-major image data (4
separate row slices via `Region::areas()`), so the natural SIMD op is one `f32x16`/`u8x16`
conversion **per column** (`dx`), scattered into up to 4 row buffers 4 bytes at a time - the
inverse loop nesting from the original scalar code (which iterated `y` outer, `dx` inner, reading
one strided 4-float group from `scratch` per pixel).

`pack`: for each `dx`, load 16 contiguous floats, convert with `u8x16::from_f32` (mul-add by 255
+ round, same helper `rounded_blurred_rect.rs` already uses), then copy each row's 4-byte group
into `region.areas()[y]`. `unpack`: for each `dx`, gather 4 bytes from each of up to 4 row slices
into a 16-byte array, convert with `f32x16::from_u8` (which multiplies by `1.0/255.0`, exactly
what the original code's own `// TODO: ... multiply by 1.0/255.0 instead` comment asked for
instead of dividing), and store 16 contiguous floats back into `scratch`.

**First attempt regressed 10-13x** (not reported as a separate row - caught before finishing this
candidate): calling `f32x16`/`u8x16` SIMD methods directly, without wrapping the loop body in
`simd.vectorize(|| { ... })`, produces standalone (non-`target_feature`-annotated) codegen for
that call site - `fearless_simd`'s AVX2 SIMD ops are `#[inline(always)]` but not individually
`#[target_feature]`-gated, so they only compile to real vector instructions when the call site
they get inlined into is itself wrapped in `vectorize` (which applies
`#[target_feature(enable="avx2,...")]` to an inner function). Every other method in this same
file (`copy_solid`, `apply_mask`, `apply_tint`, ...) already follows this pattern; `pack`/`unpack`
were the only two methods that didn't, most likely because the original code never called any
`Self::NumericVec`/`f32x16`/`u8x16` methods at all (100% scalar). Wrapping the loop body in
`simd.vectorize(#[inline(always)] || { ... })` fixed it - see `patch.diff` (the version below).

Benchmark: extended the project's own `sparse_strips/vello_bench` `fine_pack` criterion group
(which already benches `pack_block`/`unpack_block` for the u8 kernel) with two new bench
functions, `pack_block_f32`/`unpack_block_f32`, exercising `Fine::<S, F32Kernel>`. The group's
existing macro (`#[vello_bench]`, `vello_dev_macros::bench`) already generates f32-kernel
variants for every annotated bench function, but they are deliberately commented out project-wide
to keep the default bench run short - rather than re-enabling those for every `#[vello_bench]`
function in the crate, `pack_block_f32`/`unpack_block_f32` are two small, additional, hand-written
functions in `vello_bench/src/fine/pack.rs` that bench exactly `F32Kernel::pack`/`unpack` across
every available SIMD level (`avx2`/`sse4_2` on this x86_64 host). See `harness.diff` - this is a
harness-only addition (no library behavior change), applied for *both* the baseline and patched
runs below so the only difference between the two measurements is the library patch itself.

Single-core numbers are `taskset -c 3`, run under `flock` serialized against sibling candidates'
own builds/benchmarks in the same batch session; baseline was measured twice back-to-back with no
code change to establish the noise floor (both binaries confirmed distinct via `md5sum` before
every comparison, per MEASUREMENT.md section 2).

| Variant (avx2) | single-core p50 | unpinned p50 | notes |
|---|---|---|---|
| `pack_block_f32`, baseline run 1 | 1.86 µs | - | |
| `pack_block_f32`, baseline run 2 (noise-floor check) | 1.86 µs | 1.69 µs | |
| `pack_block_f32`, patched run 1 | 490 ns | - | |
| `pack_block_f32`, patched run 2 (repeat) | 485 ns | 466 ns | |
| `unpack_block_f32`, baseline run 1 | 713 ns | - | |
| `unpack_block_f32`, baseline run 2 (noise-floor check) | 688 ns | 697 ns | |
| `unpack_block_f32`, patched run 1 | 450 ns | - | |
| `unpack_block_f32`, patched run 2 (repeat) | 496 ns | 371 ns | |

| Variant (sse4_2, single-core, `taskset -c 3`) | baseline | patched | notes |
|---|---|---|---|
| `pack_block_f32` | 1.84 µs | 721 ns | |
| `unpack_block_f32` | 714 ns | 613 ns | |

Independently re-confirmed end-to-end from a **fresh scratch `git clone`** of the pinned
`base_commit` (validating `justfile`'s own `clone` -> apply `harness.diff` -> bench-baseline ->
apply `patch.diff` -> bench-patched sequence for real, unpinned - i.e. this row is realistic
wall-clock, not single-core):

| Variant (fresh clone, unpinned) | baseline | patched | criterion-reported change |
|---|---|---|---|
| `pack_block_f32_sse4_2` | 1.668 µs | 1.175 µs | -41.6% (p=0.00) |
| `pack_block_f32_avx2` | 1.666 µs | 626 ns | -64.3% (p=0.00) |
| `unpack_block_f32_sse4_2` | 701 ns | 501 ns | -29.3% (p=0.00) |
| `unpack_block_f32_avx2` | 704 ns | 584 ns | -19.3% (p=0.00) |

## Result

Time only (no allocation/RSS claim - this candidate doesn't change buffer sizes or allocation
counts, only the conversion loop itself).

`pack_block_f32_avx2`: ~1.86 µs -> ~0.485-0.49 µs, a consistent **~3.8x speedup** (pinned,
repeated). `unpack_block_f32_avx2`: ~0.69-0.71 µs -> ~0.45-0.5 µs, a **~1.5x speedup** (smaller,
because `unpack`'s gather-then-convert pattern does more scalar byte-shuffling per SIMD op than
`pack`'s scatter). `sse4_2` shows the same direction with a smaller margin (`pack` ~2.5x,
`unpack` ~1.15-1.2x) since SSE4.2 has narrower registers and less efficient byte
shuffle/interleave instructions than AVX2 for the `f32_to_u8`/`u8_to_f32` conversion helpers this
patch reuses. The independent fresh-clone unpinned re-run (different binaries, different run,
still large effect sizes and `p = 0.00` for every comparison) confirms the same direction and a
similar order of magnitude, ruling out a single-machine-state fluke.

## Does it change behavior/output?

- **Bit-exact?** Mostly yes, with two characterized differences found by a dedicated correctness
  check (`sparse_strips/vello_cpu/examples/pack_unpack_check.rs`, not shipped - a throwaway
  example built for this candidate, comparing the new implementation against the exact original
  scalar formulas across randomized inputs at several `(width, height)` shapes including partial
  tiles):
  1. **`pack`, values within the legitimate premultiplied-color range `[0.0, 1.0]`**: bit-exact
     match against `(src * 255.0 + 0.5) as u8`, every trial (0, 1, and uniform-random-in-`[0,1)`
     categories, ~120 trials x several shapes, zero mismatches).
  2. **`pack`, deliberately out-of-gamut inputs (`[-0.2, 1.3]`, an adversarial test category, not
     a realistic input)**: does **not** match. The original scalar `as u8` cast saturates
     out-of-range floats to 0/255; the new path's hardware truncating float->int conversion
     (`f32_to_u8`'s `to_int::<u32x16>()`) does not, and can produce an arbitrary byte for
     out-of-range input instead of a clean clamp. This file's own compositing math already clamps
     the one `Compose` mode that can otherwise exceed `[0, 1]` (`Plus`, via an explicit
     `.min(1.0).max(0.0)` in `compose.rs`) before it would ever reach `pack`, so this should not
     be reachable through normal rendering - but there is no explicit clamp inside `pack` itself
     either before or after this patch, so this is a real (if believed-unreachable) difference in
     how the two implementations handle an out-of-range input, not just a theoretical curiosity.
  3. **`unpack`, all inputs**: differs from the original by up to `5.96e-8` (`2^-24`, one ULP at
     that scale) - this is `f32x16::from_u8`'s `* (1.0 / 255.0)` (multiply by a precomputed
     reciprocal) vs. the original's `/ 255.0` (a true divide), which is exactly the optimization
     the removed `// TODO: ... multiply by 1.0/255.0 instead` comment already called for. Utterly
     negligible for 8-bit color data (results still round-trip to the same `u8` on the next
     `pack`).
- **Is this a breaking change?** No - no public API changed (`F32Kernel::pack`/`unpack` are
  `#[doc(hidden)]`/crate-internal), no documented behavior or file format changed. Both
  characterized differences above are the same category of change already accepted elsewhere in
  this codebase for the exact same conversion helpers (`u8x16::from_f32`/`f32x16::from_u8`),
  which are already used for real pixel output in `rounded_blurred_rect.rs` and `image.rs`.
- **Project test suite** (`cargo test --release -p vello_cpu --features f32_pipeline --lib`):
  - Before: 81 passed, 0 failed.
  - After: 81 passed, 0 failed. **Identical, no new failures.** (No existing unit test in this
    crate directly exercises `F32Kernel::pack`/`unpack` - the dedicated example above is this
    candidate's own correctness net.)
  - `vello_cpu`'s doc-tests fail to even compile in this environment, both before and after: a
    pre-existing, unrelated issue (`sparse_strips/vello_cpu/src/text_debug.rs` imports
    `glifo::atlas::GlyphCacheStats`, gated behind `#[cfg(all(debug_assertions, feature = "std"))]`
    and therefore missing in a `--release` doctest build).
  - `vello_sparse_tests` (the crate with the repo's own snapshot tests that actually exercise
    `RenderMode::OptimizeQuality`, i.e. the highp path this candidate touches) could not be used
    for correctness verification in this environment: this sandbox has no `git-lfs` binary, so
    the repo's reference PNGs check out as plain LFS pointer text files, not real images.
    Confirmed this is pre-existing and unrelated to the patch by running the full suite twice,
    once on pristine `HEAD` and once with the patch applied: **identical result both times - 89
    passed, 3538 failed, 1230 ignored** (`cargo test --release -p vello_sparse_tests`). The
    failure count is unchanged bit-for-bit by this patch; a maintainer with `git-lfs` installed
    should re-run this suite (particularly any `OptimizeQuality`/highp-tagged case) to get a real
    snapshot-based confirmation.

## Why the result looks the way it does

The scalar loop did 4 individual `f32 -> u8` casts per pixel with no vectorization at all - one
`mulps`/`cvttps2dq`-equivalent per lane, plus per-element bounds-checked slice indexing, versus
one wide SIMD load + convert + store per tile column (16 elements) in the patched version.
`unpack`'s smaller win (vs. `pack`'s ~3.8x) is because its per-column work still includes a
4-way, 4-byte-at-a-time gather from up to 4 separate row slices before the single wide conversion
- that gather is not itself vectorized (loop-carried dependency on 4 different `&[u8]` slices with
runtime-variable `height`), so it remains a real fraction of `unpack`'s total cost while `pack`'s
equivalent scatter is comparatively cheaper (writing is generally cheaper than the mixed
load-then-shuffle pattern gathering requires). The bug hit while writing this patch (10-13x
regression from omitting `simd.vectorize`) is itself informative: every other method in this file
wraps its SIMD body in `vectorize`, and `pack`/`unpack` are conspicuous, matching exceptions -
consistent with them being the only two methods in the file that were never touched by whatever
pass added SIMD to everything else here.

## What's not done / open questions

- The out-of-gamut `pack` behavior difference (bullet 2 above) is believed unreachable through
  normal rendering (given `Plus`'s own explicit clamp and every other `Compose`/blend formula's
  bounded algebra), but this was not exhaustively proven for every blend mode / filter
  combination in this file (`blend.rs`'s non-separable Mix formulas, `compose.rs`'s non-`Plus`
  modes) - a maintainer who wants a hard guarantee here could add an explicit `.clamp(0.0, 1.0)`
  in `pack` itself before the `u8x16::from_f32` conversion (at some extra cost) rather than relying
  on the invariant holding transitively through every other code path that feeds `scratch`.
- `vello_sparse_tests`'s real snapshot suite (the actual correctness authority for the highp path)
  could not be run meaningfully here due to missing `git-lfs` - see above. This is the single most
  important remaining verification step before trusting this patch beyond what this candidate's
  own scratch example checked.
- Only `sse4_2`/`avx2` were measured (this machine is x86_64); `neon` and the portable fallback
  use the same `NumericVec`/`f32x16`/`u8x16` machinery already validated elsewhere in this crate,
  but were not separately benchmarked here.
