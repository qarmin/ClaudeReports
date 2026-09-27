# AVX2/SSE u16 unpremultiply: cvtps_epi32 integer-indefinite overflow collapses to 0 instead of clamping to max_colors

Backlog item from `IDEAS.md` (sourced from `007.avx2-sse-u16-madd-epi16-signedness-sibling-audit/RESULT.md`):
`avx_unpremultiply_alpha_rgba_u16_row_avx`/`DisassociateAlphaDefault` (`src/avx2/alpha_u16.rs`/
`src/sse/alpha_u16.rs`) compute `color * (max_colors / alpha)` in f32 and cast with
`_mm256_cvtps_epi32`/`_mm_cvtps_epi32`; when that product exceeds `i32::MAX` for a small-but-nonzero
alpha, the cast produces the x86 "integer indefinite" value (`0x80000000`), which
`_mm256_packus_epi32`/`_mm_packus_epi32`'s signed-to-unsigned saturation then collapses to `0`
instead of clamping high to `max_colors`. Candidate 007 confirmed this deterministically at the
direct-function level (both backends, full u16 domain) but, despite an extensive automated search
(9+ filters, extreme ratios, 2D convolution, adversarial constructions, thousands of seeds), could
not find a public `Scaler`-API input that reaches this state - this candidate's job was to either
find a genuine trigger or apply a defensive fix regardless.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Symbolic/interval reasoning over the overflow condition | Manual derivation: `color * (max_colors/alpha) > i32::MAX` with `color,alpha` u16 (0..=65535) | Only `bit_depth == 16` (`max_colors = 65535`) can ever overflow (`max_colors <= 4095` at bit_depth 10/12 caps the product at ~1.05M, far below `i32::MAX`); and for `bit_depth == 16` the *only* satisfiable case is `alpha == 1` with `color >= 32769` (`alpha == 2` would need `color > 65537`, impossible since `color <= 65535`). This narrowed the search to one exact integer target instead of a broad region. |
| Real end-to-end diagnostic probe: temporary instrumentation added to `alpha_handle_u16::unpremultiply_alpha_rgba_u16` (the crate's own dispatch point, reached by every backend) printing the actual `color * (max_colors/alpha)` product whenever `0 < alpha <= 20`, driven through the real public `Scaler` API across a broad sweep (12 resampling filters incl. Ginseng/Lanczos3/6/EwaLanczos4Sharpest/Robidoux family, 8 width ratios from near-1:1 to 4096:1, 2000 random seeds per filter/ratio, plus a targeted adversarial "opaque-bright block next to opaque-black block next to transparent zone" construction swept over block widths and dst widths) | `cargo build --release --example bughunt_probe` (temporary, reverted after use), debug print gated behind `BUGHUNT_PROBE_OVERFLOW` env var | The plain random sweep alone found several near-misses climbing toward the threshold (e.g. `Ginseng`, width 2048->2047, seed 806: `alpha=1, color=25394`, 77% of the way to the `color>=32769` threshold; `Ginseng`, width 10000->9999, seed 116: `alpha=2, color=54604`, 83% of the way to its own threshold) and then **actual overflow hits**: `Ginseng` filter, width 10000 -> 9999 (near-1:1 downscale), seeds 765/815, e.g. `color=39662, alpha=1, product=2599249170 > i32::MAX (2147483647)` - confirming the state candidate 007 couldn't reach *is* produced by the real convolution pipeline, just needed the right filter (`Ginseng`'s negative-lobe ringing) at a large near-1:1 downscale ratio rather than the filters/ratios candidate 007 tried. |
| Real end-to-end reproduction via the public API: three release builds (`repro/simd` avx-enabled default features, `repro/sse` sse-only, `repro/scalar` default-features=false+threading) of identical source, differing only in which Cargo features are compiled into the `pic-scale` dependency, all calling `Scaler::plan_rgba_resampling16(.., premultiply_alpha: true, bit_depth: 16)` with `ResamplingFunction::Ginseng`, 10000->9999 downscale, deterministic full-range pseudorandom RGBA (seeds 765 and 815) | release build, `repro/compare.sh` | **Reproduced, 2/2 seeds, both backends.** Seed 765: AVX2 and SSE both output `0` at index 14588 (pixel 3647, channel R) where the scalar reference outputs `65535` and the pixel's (unaffected-by-unpremultiply) alpha channel is `1` - the exact overflow signature, not the separate zero-alpha bug. Seed 815: 3 consecutive channel hits (indices 25160-25162, R/G/B of one pixel) in both backends, same signature. Bit-for-bit identical across 3 reruns of every binary (deterministic). |

## Status

**Confirmed.**

Reproduction rate: 2/2 seeds tried (765, 815), both AVX2 and SSE backends, each deterministic and
repeat-stable across 3 reruns of the same release build. Root-caused down to the individual
mismatching pixel/channel (confirmed via the pixel's alpha channel, which unpremultiply leaves
untouched, so the same value is directly visible in both the buggy and reference output) showing
`alpha == 1` in every case - the one integer value the symbolic analysis above identified as
capable of triggering the overflow at `bit_depth == 16`. This is the real end-to-end public API
entry point (`Scaler::plan_rgba_resampling16` -> `Resampling::resample`), run against three builds
of the identical source differing only in which CPU-dispatch Cargo features are compiled into the
`pic-scale` dependency - not a hand-constructed intermediate state.

**Fully reproducible**: **Yes.** `repro/compare.sh` builds all three binaries from a pristine clone
at `base_commit` and diffs their real output for 2 seeds, checking specifically for the
overflow-signature mismatch (SIMD output `0` where scalar outputs `65535` at a nonzero-alpha pixel,
which distinguishes it from the separate, still-unpatched-in-this-tree zero-alpha bug from candidate
007); `just test` runs the whole clone -> before -> apply -> after cycle unattended and was actually
executed (not just read) during this session - see `output.txt`.

## Class

logic

## Root cause

Both `avx_unpremultiply_alpha_rgba_u16_row_avx` (`src/avx2/alpha_u16.rs`) and
`DisassociateAlphaDefault::disassociate_chunk`/`sse_unpremultiply_row_u16`
(`src/sse/alpha_u16.rs`) compute, per color channel, `color as f32 * (max_colors as f32 / alpha as
f32)` and cast the result to `i32` with `_mm256_cvtps_epi32`/`_mm_cvtps_epi32` before packing down
to `u16` with `_mm256_packus_epi32`/`_mm_packus_epi32`. At `bit_depth == 16` (`max_colors =
65535`), a pixel whose premultiplied color and alpha the resize convolution has decorrelated (via
negative-lobe filter ringing - the same general mechanism candidate 007's zero-alpha bug exploits,
just landing on `alpha == 1` instead of `alpha == 0`) so that `color >= 32769` while `alpha == 1`
produces a product exceeding `2^31 - 1` (`i32::MAX`). `CVTPS2DQ`/`VCVTPS2DQ` on an out-of-i32-range
input does not wrap or saturate to `i32::MAX` - it produces the x86 "integer indefinite" value
`0x80000000` (`i32::MIN` when read as signed). `_mm256_packus_epi32`/`_mm_packus_epi32` then
performs a **signed** 32-bit to **unsigned** 16-bit saturating pack: a very negative signed input
saturates to `0`, not `65535`. The subsequent `_mm256_min_epu16`/`_mm_min_epu16` clamp against
`max_colors` (added, per the code, specifically to clamp results from *above*) cannot rescue this,
since `min(0, 65535) == 0` - the value has already collapsed to the wrong end of the range. The
scalar reference (`unpremultiply_alpha_rgba_row` in `src/alpha_handle_u16.rs`) never hits this
because it computes the product as `f32` throughout and clamps with `.min(max_colors as f32)`
*before* any integer cast - f32 has ample range (~3.4e38) to represent the unclamped product
exactly (well within its 24-bit mantissa's exact-integer range for these magnitudes), so the clamp
correctly saturates to `65535` instead of wrapping through a signed-integer collapse. All three
loops in both backends' unpremultiply functions (AVX2: 32-wide main loop, 16-wide remainder,
scalar-buffer tail; SSE: `disassociate_chunk`, called from both the aligned-chunk loop and the
padded-buffer remainder path) share the identical unclamped-cast pattern.

This is a distinct root cause from candidate 007's zero-alpha blend bug (a masking/blend logic
error at `alpha == 0` that doesn't touch the arithmetic at all) - this one is purely a
float-to-integer range/overflow bug at `alpha == 1`, affecting the arithmetic in both backends
identically (unlike candidate 007's bug, which was AVX2-only; SSE was already correct there).

## The fix

In both backends, clamp the float product to `max_colors` (as an `f32`) immediately before the
`cvtps_epi32`/`cvtps_epi32` cast, instead of only after packing down to `u16`:

- `src/avx2/alpha_u16.rs`: in all three loops of `avx_unpremultiply_alpha_rgba_u16_row_avx`, insert
  `lo0 = _mm256_min_ps(lo0, v_scale_colors);` (and the equivalent for `hi0`/`lo1`/`hi1`/`lo`/`hi` in
  the other two loops) right before the corresponding `_mm256_cvtps_epi32` call, reusing the
  already-computed `v_scale_colors = _mm256_set1_ps(max_colors as f32)`.
- `src/sse/alpha_u16.rs`: `sse_unpremultiply_row_u16` now takes an extra `v_max_colors: __m128`
  parameter (the same value `disassociate_chunk` already computes) and clamps
  `_mm_mul_ps(_mm_cvtepi32_ps(lo), a_lo_f)`/`..hi..` against it with `_mm_min_ps` before the
  `_mm_cvtps_epi32` calls; its 3 call sites (R/G/B) were updated to pass `v_max_colors` through.

This keeps every existing branch (the zero-alpha handling, the post-pack `min_epu16` clamp for the
normal in-range case, the exact-division bit_depth-10/12 paths which never reach this function)
completely unchanged - it only prevents the specific float value that would overflow the cast from
ever reaching it.

Verified: `repro/compare.sh` against a pristine clone shows **REPRODUCED, 2/2 seeds** (AVX2 and SSE
both hit the exact `0`-vs-`65535` overflow signature at a nonzero-alpha pixel) before the patch, and
**did not reproduce, 0/2 seeds** after - see `output.txt`. Also verified with a broader, unrelated
diagnostic sweep: the same real-API probe harness used to originally find this trigger (12 filters
x 8 ratios x 2000 seeds + the adversarial block sweep) showed the probe's own `BUGHUNT_PROBE_HIT`
detector (which checks the *pre-dispatch* premultiplied buffer, independent of which backend
runs - so unaffected by this fix, still fires the same both before and after) fires identically in
both trees, while the actual AVX2/SSE dispatch output only diverged from the scalar reference at the
overflow-signature pixels *before* the patch (all `alpha == 0` after the patch, i.e. only the
separate, already-known, still-unpatched-in-this-tree candidate-007 bug remains).

- Project test suite: **66 passed, 0 failed** before the patch; **66 passed, 0 failed** after
  (`cargo test --release`, default features, this host). No test regressed; no test newly failed;
  no new tests were added by this candidate's patch (unlike candidate 007, which added its own
  differential unit tests).

## What's not done / open questions

- The pre-existing, separate candidate-007 zero-alpha bug is still unpatched in the base tree (per
  this skill's standing rule of always reverting between candidates) and remains visible in
  `repro/compare.sh`'s raw diff output alongside this candidate's now-fixed overflow signature -
  `compare.sh` specifically filters for the overflow signature (nonzero alpha) to isolate this
  candidate's bug from that one; a maintainer applying both candidates' patches together would clear
  both classes of mismatch.
- Only `bit_depth == 16` can trigger this (see the symbolic analysis above) - `bit_depth` 10/12 use
  a completely different, exact-integer-division code path (`_mm256_div_by_1023_epi32`/
  `_mm256_div_by_4095_epi32` etc.) that never reaches `cvtps_epi32` at all, so this candidate did
  not need to (and did not) touch that code.
- Only x86_64 AVX2/SSE4.1 were checked; `src/neon/` unpremultiply was not audited for an analogous
  `vcvtq_s32_f32` overflow-collapse pattern - no ARM hardware available on this host (already
  tracked as its own lower-priority `IDEAS.md` row from candidate 006/007's sessions).
- The premultiply direction (`Avx2PremultiplyExecutorAnyBit`/`Sse41PremultiplyExecutorAny` in the
  same two files) has its own `cvtps_epi32` casts but multiplies `color * alpha * (1/max_colors)`,
  whose maximum possible value (`65535 * 65535 / 65535 = 65535`) can never approach `i32::MAX` -
  not in scope for this bug class and not touched by this patch.
- The random-sweep trigger (`Ginseng` filter, large near-1:1 downscale ratio) was found by breadth
  (2000 seeds x 8 ratios x 12 filters) rather than by directly solving for the adversarial
  block-pattern construction described in candidate 007's own follow-up suggestion; that
  construction was also implemented and swept in this session's probe harness but did not itself
  turn up a hit before the random sweep already had, so it wasn't pursued further once a real
  end-to-end repro existed via the simpler random-sweep path.
