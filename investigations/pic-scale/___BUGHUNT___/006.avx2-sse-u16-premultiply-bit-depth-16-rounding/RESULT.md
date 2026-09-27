# AVX2/SSE u16 premultiply "any bit depth" executor rounds incorrectly for bit_depth == 16

Backlog item from `IDEAS.md` (sourced from `005.simd-alpha-remainder-cascade-audit/RESULT.md`):
`Avx2PremultiplyExecutorAnyBit` (`src/avx2/alpha_u16.rs`) and `Sse41PremultiplyExecutorAny`
(`src/sse/alpha_u16.rs`) - the "any bit depth" premultiply path used whenever `bit_depth` isn't 10
or 12, including the very common `bit_depth == 16` case - round via an f32 reciprocal-multiply
approximation instead of the crate's own exact fixed-point `div_by_65535` reference, and candidate
005 had already isolated a 144/5000 (2.88%) ±1 LSB divergence at the raw-function level (no
remainder involved) but not root-caused or fixed it as its own candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Full-pipeline differential via the crate's PUBLIC API: two release builds of the same input program (`Scaler::plan_rgba_resampling16`, `premultiply_alpha=true`, `bit_depth=16`) differing only in which `pic-scale` Cargo features are enabled (`avx,sse,neon,rdm,threading` default vs. `default-features=false, features=["threading"]`, forcing the scalar-only fallback) at 1:1 identity size, 16-wide RGBA, random full-range and then low-alpha inputs | debug asserts on (release, `-O` but no `#[no_std]`/panic=abort changes), 30-80 random seeds | 0/mismatches at 1:1 identity size for both random-full-range and low-alpha inputs - the round-trip premultiply→(lossless 1:1 Bilinear/identity convolve)→unpremultiply masks the ±1 LSB intermediate divergence almost completely, since the final unpremultiply's own rounding absorbs it. This ruled out a naive full-pipeline identity-resize repro without proving the bug absent - see next row. |
| Same public-API differential, but with a real 512→256 Bilinear **downscale** (so the horizontal convolve genuinely blends 2 source pixels' premultiplied values instead of passing one through unchanged) | release, seeds 1..200, full 0..=65535 random color+alpha range | **Reproduced**: 19/50 seeds (~38%) showed 1-3 of 1024 output `u16` values differing by exactly the AVX2/SSE-enabled build vs. the scalar-only build; deterministic and repeat-stable (3/3 identical reruns per build per seed). This is the real, end-to-end, public-API-only repro used for `repro/`. |
| Root-cause via code reading of `Avx2PremultiplyExecutorDefault<BIT_DEPTH>`/`Sse41PremultiplyExecutorDefault<BIT_DEPTH>` (the *already-correct*, exact-division executor used today only for `BIT_DEPTH` 10/12) to check whether simply re-routing `bit_depth == 16` to it (as the backlog note suggested) is actually safe | manual code reading + an initial "obvious fix" attempt, rebuilt and re-ran the same 200-seed differential | The obvious fix (just adding a `bit_depth == 16` branch dispatching to `Avx2PremultiplyExecutorDefault::<16>`/`Sse41PremultiplyExecutorDefault::<16>` unchanged) is **not** safe and made things far worse: 76510/102400 (~75%) of compared values were wrong, many clamped/saturated to 65535. Root cause: `Avx2PremultiplyExecutorDefault`/`Sse41PremultiplyExecutorDefault`'s `premultiply_chunk` multiplies via `_mm256_madd_epi16`/`_mm_madd_epi16`, which treats its 16-bit lanes as **signed**. That's harmless for `BIT_DEPTH` 10/12 (max value 4095, always positive as `i16`), but for `BIT_DEPTH` 16 any color/alpha value >= 32768 has its sign bit set when reinterpreted as `i16`, corrupting the product; the resulting (sometimes negative) 32-bit sum is then fed to `_mm256_div_by_65535_epi32`/`_mm_div_by_65535_epi32`, whose `_mm256_srli_epi32`/`_mm_srli_epi32` is a **logical** (not arithmetic) shift, turning a negative sum into a huge unsigned value that saturates to 65535 on `packus_epi32`. This is a distinct, pre-existing latent bug in the "Default" executor that the backlog note's suggested fix would have silently activated for a bit depth it was never exercised at. Captured in `output.txt` for anyone tempted to re-apply the naive fix. |

## Status

**Confirmed.**

Reproduction rate: 19/50 random seeds (~38%) at the chosen input distribution (full 0..=65535
color/alpha range, 512→256 Bilinear downscale, `bit_depth=16`), each mismatch deterministic and
reproduced identically across repeat runs of the same build (3/3). This is the real end-to-end
public API entry point (`Scaler::plan_rgba_resampling16` -> `Resampling::resample`), run against
two builds of the identical source differing only in which CPU-dispatch Cargo features are
compiled into the `pic-scale` dependency - not a hand-constructed intermediate state.

**Fully reproducible**: **Yes.** `repro/compare.sh` builds both binaries from a pristine clone at
`base_commit` and diffs their real output for 50 seeds; `just test` runs the whole clone -> before
-> apply -> after cycle unattended and was actually executed (not just read) during this session.

## Class

logic

## Root cause

`Avx2PremultiplyExecutorAnyBit`/`Sse41PremultiplyExecutorAny` (used for every `bit_depth` other
than 10/12, i.e. every 16-bit-per-channel image, the single most common case) compute
`u32 * u32` products up to ~4.29e9 and round-trip them through a single `f32` reciprocal multiply
(24-bit mantissa, exact only up to ~1.68e7), instead of using the crate's own exact fixed-point
`div_by_65535` that the scalar reference (`alpha_handle_u16.rs::premultiply_alpha_rgba_row`) uses
for `bit_depth == 16`. This produces a genuine, reproducible ±1-LSB-out-of-65535 divergence from
the crate's own reference output, which a real downscaling resize (where the convolve stage
combines two divergent premultiplied source pixels rather than passing one straight through)
surfaces in the final public-API output on a non-trivial fraction of pixels.

`_mm256_div_by_65535_epi32`/`_mm_div_by_65535_epi32` (an exact integer-division helper) already
exists and is already used correctly by `Avx2PremultiplyExecutorDefault<10/12>`/
`Sse41PremultiplyExecutorDefault<10/12>` - but naively re-routing `bit_depth == 16` to those
existing structs (as the backlog note's phrasing suggested might just work) is unsafe: their
`premultiply_chunk` multiplies via `_mm256_madd_epi16`/`_mm_madd_epi16`, which interprets its
16-bit lanes as **signed**. `BIT_DEPTH` 10/12 never exceeds 4095 so this is harmless there, but
`BIT_DEPTH` 16 values up to 65535 wrap negative above 32767, corrupting the product and, via the
division helper's logical right-shift, producing saturated/garbage output for roughly 3 out of 4
pixels once any component reaches the upper half of the 16-bit range - a second, previously-latent
bug in code that had simply never been exercised at the full 16-bit range before.

## The fix

Two changes, both in `patch.diff`:

1. In `Avx2PremultiplyExecutorDefault<BIT_DEPTH>::premultiply_chunk` (`src/avx2/alpha_u16.rs`) and
   `Sse41PremultiplyExecutorDefault<BIT_DEPTH>::premultiply_chunk` (`src/sse/alpha_u16.rs`): use
   `_mm256_mullo_epi32`/`_mm_mullo_epi32` (unsigned-safe 32-bit low-product) instead of
   `_mm256_madd_epi16`/`_mm_madd_epi16` specifically when `BIT_DEPTH == 16`, keeping the original
   `madd_epi16`/`madd_epi16` path unchanged for `BIT_DEPTH` 10/12 (zero behavior change there -
   `mullo_epi32` on the same already-zero-extended 32-bit lanes would in fact give an identical
   result for 10/12 too, but the fix is scoped to only touch the depth that was actually broken,
   per `CONFIRMATION.md`'s "checked against more than just the one triggering input" bar without
   also churning already-correct, already-tested code paths).
2. In `avx_premultiply_alpha_rgba_u16_row` (`src/avx2/alpha_u16.rs`) and
   `premultiply_alpha_sse_rgba_u16_row_impl` (`src/sse/alpha_u16.rs`): add a `bit_depth == 16`
   branch that dispatches to `Avx2PremultiplyExecutorDefault::<16>`/
   `Sse41PremultiplyExecutorDefault::<16>` (now safe per change 1) instead of falling through to
   `Avx2PremultiplyExecutorAnyBit`/`Sse41PremultiplyExecutorAny`.

Verified: after the fix, `repro/compare.sh` against the same 50 seeds (and a wider 200-seed sweep
run live during this session, not just the 50 shipped in `repro/`) shows **0** mismatches between
the SIMD-enabled and scalar-only builds - see `output.txt`.

- Project test suite: **75 passed, 0 failed**, both before and after the patch
  (`cargo test --lib --features nightly_f16`, default features + `nightly_f16`, this host). No
  test regressed; no test newly failed.

## What's not done / open questions

- `unpremultiply_alpha_rgba_u16`/the SSE equivalent (the *reverse* direction) were not audited for
  the same class of bug in this candidate - only the premultiply direction was in scope
  (`avx_unpremultiply_alpha_rgba_u16_row_avx` visibly already uses a per-pixel float reciprocal
  divide for every bit depth including 16, matching the scalar reference's own approximate path
  for unpremultiply, so it may not have an *exact*-reference counterpart to diverge from at all -
  not independently verified this session).
- Only x86_64 (AVX2/SSE4.1) backends were in scope, matching the original backlog item; `src/neon/`
  was not checked for an analogous "any bit depth" approximate path or a matching
  `madd`-vs-`mullo` signedness issue - this host has no ARM hardware to exercise it natively.
- The fix is scoped to `BIT_DEPTH == 16` only, per the reasoning in "The fix" above; it does not
  touch (and this session did not need to touch) the `BIT_DEPTH` 10/12 code paths, which were
  already correct.
- gray-alpha (LA) u16 premultiply was not in scope for this candidate (see candidate 003: no LA
  SIMD kernels exist in this codebase at all).
