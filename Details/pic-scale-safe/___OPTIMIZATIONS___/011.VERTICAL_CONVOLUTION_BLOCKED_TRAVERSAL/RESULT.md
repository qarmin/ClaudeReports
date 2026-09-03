# Vertical convolution: wider column-block tier for cache locality (regression)

**Ported from `image` repo's
`___OPTIMIZATIONS___/039.PIC_SCALE_SAFE_VERTICAL_CONVOLUTION_BLOCKED_TRAVERSAL/`.** Originally
measured on 2026-08-25 via a vendored `pic-scale-safe` 0.1.11 copy inside the `image` project.
**Not re-measured in this workspace** - numbers below are copied verbatim. The patch was rebuilt
by hand against this repo's current `HEAD` (`b50e020`) since the original `patch.diff` was a
full-vendored-crate-copy diff (a whole "new file" `fixed_point_vertical.rs`), not a minimal diff;
`fixed_point_vertical.rs`'s `column_handler_fixed_point` cascade is unchanged between 0.1.11 and
current `HEAD`, so the reconstructed diff is exact, not approximate. `git apply --check` passes,
`cargo check --features rayon` compiles clean, and `cargo test --release --features rayon` passes
5/5 (verified this session).

**This is a 100% safe-Rust attempt - no `unsafe`, no `#[target_feature]`.** Read this before
attempting any further vertical-convolution auto-vectorization work: it is a confirmed regression,
not a missed opportunity waiting to be picked up.

## What was tried

Profiling (candidate 015 in `image`'s own `IDEAS.md`, 2026-08-24) showed
`fixed_point_vertical::column_handler_fixed_point` at 62.6% of instructions for a
`DynamicImage::resize(400, 400, Lanczos3)` on a 4032x3024 source, vs. only 15.3% for the
horizontal pass - hypothesized to come from the vertical pass's strided memory-access pattern
(each destination row reads `bounds.size` source rows spaced `src_stride` bytes apart, in small
8/16/32-element column chunks, so each source row is "hopped to" from cold cache state many times
per destination row instead of once). This candidate adds a wider (512-element) column-block tier
to the existing cascading dispatch, reusing the existing generic
`convolve_column_handler_fixed_point_direct_buffer::<T, J, BUFFER_SIZE>` function unmodified with
`BUFFER_SIZE = 512`, tried before the existing 64(aarch64)/32/16/8/1 tiers.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline (registry 0.1.11) | 18.786 ms | 18.698 ms | Lanczos3, 4032x3024->400x300 |
| this candidate (+512-element tier) | 37.083 ms (**+97.4%**, 0/5 faster) | 37.079 ms (**+98.3%**, 0/5 faster) | same |
| baseline | 7.323 ms | n/a | Triangle, same source/dest |
| this candidate | 14.937 ms (**+104.0%**, 0/5 faster) | n/a | Triangle |
| same-binary control | 37.633 -> 37.369 ms (-0.7%, 4/5 "wins") | n/a | noise floor for the Lanczos3 comparison |

## Result

**Decisive regression.** ~2x slower than baseline on both single-core and multi-core, for both
Lanczos3 (+97-98%) and Triangle (+104%), 0/5 paired runs faster in every comparison - far outside
the -0.7% same-binary noise floor. Not proposed for adoption.

## Does it change behavior/output?

- **Bit-exact?** Yes - the 512-element tier calls the exact same per-column accumulation
  function/weights/tap order as the smaller tiers, just with a different column-chunk boundary,
  which cannot change the arithmetic result. Verified in the original session across
  15 output hashes (5 filters x 3 sizes), all identical between baseline and patched.
- **Is this a breaking change?** No - not the winning variant, and bit-exact output means no
  observable behavior change even if it were kept. No `unsafe` anywhere in this patch.
- Project test suite: `cargo test --release --features rayon` - **5 passed, 0 failed** (this
  session, against current `HEAD`). Original measurement additionally quoted `image`'s own full
  suite: 622 passed, 1 pre-existing unrelated failure, identical before/after.

## Why the result looks the way it does

The profiling-based hypothesis (cross-row cache-hopping dominates) undersold a countervailing
cost: the existing small tiers (8/16/32, 64 on aarch64) are not arbitrary sizes -
`BUFFER_SIZE = 8` for the `J = i32` accumulator is exactly one AVX2 register's worth (8 x 32-bit
lanes) - **though this crate's actual generated code never emits an AVX2-width `ymm` op anywhere
in this file; see candidate 012 for the disassembly-confirmed detail that it stays at 128-bit SSE2
`xmm` throughout, without `RUSTFLAGS`/`target_feature`**. For those small sizes, LLVM can keep the
whole accumulator resident in vector registers for the entire tap loop, touching memory only once
per tap for the (small) source read and once at the end to write the narrowed result - no
accumulator spill. Raising `BUFFER_SIZE` to 512 makes that impossible: 512 x i32 (2 KiB) cannot
live in registers, so the accumulator spills to the stack/L1, and every tap's inner loop becomes a
full 512-element read-modify-write against that spilled array instead of a handful of
register-to-register FMAs. For a support width around 40-60 taps (Lanczos3/Triangle downscaling
~10x), that is 40-60x more L1 traffic against a 2 KiB working set, comfortably outweighing the
row-hop savings the larger block was meant to buy.

## What's not done / open questions

- A genuine cache-locality win for this function likely needs to keep the *register-resident*
  accumulator width (8/16/32) while amortizing the row-hop cost some other way - e.g. processing
  several adjacent small sub-blocks per tap-visit with multiple small accumulators kept live
  together, rather than one giant accumulator. **This was tried next - see candidate 012, which
  also regressed** (smaller magnitude, +5-8%, for a more nuanced reason: even the "small" 8-wide
  accumulator is 128-bit/2-lane in this crate's actual codegen, not 256-bit/8-lane as assumed, so
  8 of them still don't fully fit in the available vector registers).
- Smaller intermediate block sizes (64, 128) between the existing 32-tier and the tried 512-tier
  were not measured - the register-spill explanation predicts these would also regress, but this
  wasn't verified empirically.
