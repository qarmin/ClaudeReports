# lowp: fast path for a fully-opaque solid fill composited under a per-pixel AA mask

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`, x86_64, SSE4.2 and
AVX2 SIMD levels (this machine has no AVX-512/NEON).

Part of the profiled "fine-rasterizer per-pixel alpha compositing" cluster
(`vello_cpu::fine::lowp` + `rasterize_region`/`paint_fill`), which totaled ~37% of samples at
scale=8 in the assigned profiling data. Within that cluster, `sparse_strips/vello_cpu/src/fine/lowp/mod.rs`'s
`alpha_fill::alpha_composite_solid` (used by `fill_solid` whenever a solid-color fill has a
per-pixel AA-coverage mask - i.e. every anti-aliased edge pixel of any opaque solid-colored path:
text, vector shapes, etc.) always computed `effective_alpha = src_alpha * mask / 255` even when
`src_alpha == 255` (fully opaque color), in which case `effective_alpha` is just `mask` - the
multiply is redundant work done on every masked pixel of every opaque solid fill.

No existing criterion bench reached this code path at all: `fine_solid` (`vello_bench::fine::fill`)
only ever called `paint_fill` with `alphas: None`. Added `opaque_long_masked` /
`transparent_long_masked` to that existing bench module (extending the harness, not creating a
new one - see `sparse_strips/vello_bench/src/fine/fill.rs`), which build a `Tile::HEIGHT`-bytes-
per-pixel-column coverage mask with varied values (`(i * 37) % 256`) and pass it as `alphas` to
`paint_fill`, so the `alpha_fill::alpha_composite_solid` branch is what's actually measured
end-to-end through the real `Fine`/`FineKernel` API (not a synthetic microbenchmark of the inner
function in isolation).

## What was tried

Hoisted the `src_alpha == 255` check out of `alpha_fill::alpha_composite_solid`'s per-32-byte-chunk
loop (it's loop-invariant - `src` is the same solid color for the whole call) and added a
specialized `alpha_composite_inner_opaque` that uses `mask` directly as `effective_alpha`,
skipping the `normalized_mul_u8x32(src_a, mask_v)` (widen -> multiply -> approx-div-255 -> narrow)
sequence entirely for that case. The non-opaque branch is untouched (still calls the original
`alpha_composite_inner`).

Single-core (`taskset -c 3`), paired alternating A/B runs (baseline binary vs. patched binary,
same process each side, `criterion --warm-up-time 0.5 --measurement-time 1.2`), 6 pairs per SIMD
level, `fine/fill/opaque_long_masked` (256px span, opaque `ROYAL_BLUE`, varied mask):

| Variant | u8_sse4_2 (median of 6) | u8_avx2 (median of 6) | notes |
|---|---|---|---|
| baseline | ~462 ns | ~309 ns | unpatched `alpha_composite_solid` |
| patched | ~340 ns | ~209 ns | opaque fast path added |

B (patched) won 6/6 pairs on both SIMD levels (`paired.sh`-style alternation - see MEASUREMENT.md
section 2b). Approximate deltas: **sse4_2 ~-26% to -29%**, **avx2 ~-30% to -35%**.

`fine/fill/transparent_long_masked` (non-opaque color, same mask, unaffected branch) - 5 paired
runs each level, wins mixed both directions within noise (avx2: baseline 301-326ns vs. patched
306-323ns; sse4_2: baseline 461-479ns vs. patched 467-478ns) - no measurable change, as expected
since this path is untouched.

Independently, `cargo bench`'s own criterion-history comparison (single run each side, not
paired, just corroborating) reported: `opaque_long_masked_u8_sse4_2: -26.7%`,
`opaque_long_masked_u8_avx2: -28.0%` (both `p = 0.00 < 0.05`, "Performance has improved").

Multi-core: not separately measured - this is a per-pixel-chunk CPU micro-optimization inside a
single-threaded rasterization worker; `vello_cpu`'s multithreading parallelizes across strip rows
by spawning this same per-row work on multiple threads, so the per-row cost reduction applies
identically whether run on one core or scaled across many - a multi-core run would show the same
percentage change modulo threading overhead, not a different mechanism.

## Result

Real, consistent win for the specific code path it targets: a fully-opaque solid-color fill
composited under a per-pixel AA-coverage mask (the common case for anti-aliased opaque shape/text
edges) is **~26-35% faster** (single-core, paired-comparison, 6/6 and 6/6 wins) in
`alpha_fill::alpha_composite_solid`/`fill_solid`. The non-opaque branch, sharing the same function,
is unaffected (within noise). This is a narrow, hot micro-optimization, not a whole-scene number -
its actual end-to-end impact on a real render depends on what fraction of a scene's solid-fill
pixel work falls on partially-covered (mask-present) pixels vs. fully-covered interior pixels
(which already use the separate, unaffected `copy_solid`/no-mask fast path).

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified computationally for all 256 possible mask byte values: this crate's
  `div_255` approximation is `(x + 255) >> 8` (see `vello_common::util::Div255Ext`), and
  `(255 * mask + 255) >> 8 == mask` for every `mask` in `0..=255` (checked exhaustively with a
  short Python script, zero mismatches) - so `normalized_mul_u8x32(255, mask)` and `mask` are
  identical for every input, meaning the patched fast path produces bit-for-bit the same output as
  the original code whenever `src_alpha == 255`. This is a real algebraic simplification, not an
  approximation change.
- **Is this a breaking change?** No - internal-only, no public API change.
- Project test suite:
  - `cargo test --release -p vello_cpu` (unit tests, `--lib`): **81 passed, 0 failed** both before
    and after the patch. A separate `--doc` failure (`E0432: unresolved import
    glifo::atlas::GlyphCacheStats` in `vello_cpu/src/text_debug.rs`) is pre-existing and unrelated
    to this patch or this code path (a `debug_assertions`-gated re-export mismatch in the `glifo`
    dependency) - present identically on the unpatched tree.
  - `cargo test --release -p vello_sparse_tests` (the project's own pixel-snapshot test suite,
    which would be the most direct correctness check for this exact change - anti-aliased solid
    fills): **N/A in this sandbox** - this repository stores its reference snapshot PNGs via
    git-lfs, and `git-lfs` is not installed here, so every snapshot file is an LFS pointer stub
    (plain ASCII text, confirmed with `file`) rather than a real image; essentially the entire
    suite (3538 of 3627 non-ignored tests, including renderer paths this patch never touches, e.g.
    `hybrid` variants) fails identically regardless of this patch - this is a pre-existing
    environment limitation, not a regression signal one way or the other. The bit-exactness
    argument above is the actual correctness evidence for this candidate.

## Why the result looks the way it does

`normalized_mul_u8x32` is itself a widen (u8->u16) -> multiply -> `div_255` (add 255, shift right
8) -> (implicit narrow back to u8 by the caller) sequence - real, non-trivial per-chunk SIMD work.
When `src_alpha` is a compile-time-invariant-per-call constant equal to 255, that whole sequence
collapses algebraically to the identity function on `mask`, so skipping it removes real work
without approximating anything. The check itself is hoisted to once per call (not per chunk),
so the saving is the full multiply/div_255 sequence per 32-byte chunk, for every masked pixel of
every opaque solid fill - exactly the shape of win the ~20%+ profiled share of `alpha_composite`-
family functions at scale=8 would predict for a scene with a meaningful fraction of anti-aliased
opaque edges.

## What's not done / open questions

- Only the solid-color masked path (`alpha_fill::alpha_composite_solid`) was touched, matching
  "one fix per candidate". The sibling `alpha_fill::alpha_composite` (masked buffer-source
  compositing, e.g. gradients/images under a mask) has the same `src_a` widen/multiply per pixel,
  but there `src_a` varies per pixel (`next_src.splat_4th()`), so the same "hoist a scalar check"
  approach doesn't directly apply - it would need a per-chunk "is this chunk's source fully
  opaque" check instead, which is a different, unverified idea worth a separate candidate.
- The `highp` (f32) kernel (`sparse_strips/vello_cpu/src/fine/highp/mod.rs`) has an analogous
  `alpha_composite_solid` and wasn't investigated - float multiply-by-1.0 is already a no-op at
  the hardware level in most cases, so the same win is not expected there, but this wasn't
  measured.
- `vello_sparse_tests` (pixel-snapshot correctness) could not be exercised in this sandbox due to
  missing `git-lfs`; re-running it in an environment with `git-lfs` available (before/after this
  patch) would be a stronger correctness signal than the bit-exactness argument alone, even though
  that argument is already exhaustive over the relevant input space (256 mask values).
