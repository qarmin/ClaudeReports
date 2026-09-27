# AVX2 u8 unpremultiply: packus_epi32/mulhi_epu16 output misinterpreted as signed by packus_epi16

Backlog row from `IDEAS.md` (sourced from candidates `003`/`004`): audit `src/sse/alpha_*.rs`,
`src/avx2/alpha_*.rs`, `src/neon/alpha_*.rs` RGBA-only premultiply/unpremultiply kernels for a
lane-count/remainder-handling mistake inside the 4-channel vectorized loop itself, continuing the
bug class candidate `004` found in `src/avx512/check_alpha.rs`. The `as_chunks::<N>().0`
remainder-cascade shape itself was already exhaustively audited by candidate `005` (0 bugs found,
code reading + differential testing across all 8 x86_64 alpha files). This candidate re-examined
the same 8 files by direct random-content differential fuzzing (not just candidate 005's
sequential-width sweep) against their own scalar reference, which surfaced a different, real bug
in `src/avx2/alpha_u8.rs`'s u8 unpremultiply (not a remainder-handling bug - see "Root cause").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of the `as_chunks`/buffer-fallback remainder cascade in all 8 `src/{avx2,sse}/alpha_*.rs` files, independent of candidate 005's own read | reading only | Confirmed the same conclusion as candidate 005: every cascade correctly covers 100% of the row (cascade-to-smaller-chunk, buffer-retry-through-same-kernel, or scalar fallback); no unfixed `check_alpha.rs`-style dropped-remainder bug found |
| In-crate randomized differential fuzz test (`#[cfg(test)] mod ___tmp_alpha_fuzz_test`, temporary, reverted): 20000 random-length (0-90 px)/random-content trials per backend/dtype (u8, u16 bit_depth=10, f32) comparing AVX2/SSE premultiply+unpremultiply against the crate's own scalar reference | debug build, `cargo test --lib`, this host (avx2+sse4.1 detected) | u16(10)/f32: 0 mismatches (exact/tolerance). u8: AVX2 **unpremultiply** (never covered by candidate 005, which only added an AVX2 **premultiply** remainder test for u8) failed immediately: `[124,236,0,41,138,**42**,32,24,...]` vs scalar `[124,236,0,41,138,**43**,32,24,...]` - a real divergence, present in this pipeline's own `PreferQuality` and (default) `PreferSpeed` `WorkloadStrategy` variants alike |
| Isolated full-AVX2-chunk-only probe (`pixels=8`, exact multiple of the 32-byte main-loop chunk, no tail/remainder path reachable at all) vs. tail-only probe (`pixels=3`), 5000 random trials each, to determine whether the mismatch was remainder-related | debug build, `cargo test --lib -- --nocapture` | `full_chunk_mismatches=1233/5000` (24.7%), `tail_mismatches=543/5000` - **not** a remainder-handling bug (fires identically in the pure-main-loop case with zero tail bytes); one full-chunk example: `avx=[...,0,13,...]` vs `scalar=[...,255,13,...]` at alpha=1 - a catastrophic (not ±1) divergence, isolated by construction from the tail-cascade code path entirely |
| Root-cause via exact hand-picked byte content (`premult=200, alpha=1`, repeated x8 = one full AVX2 32-byte chunk) plus a source-level diagnostic probe (temporary `eprintln!`, reverted) printing the raw pre-pack `i32`/`u16` intermediate value | debug + release builds | `Avx2DisassociateAlpha::disassociate_chunk` (used by `WorkloadStrategy::PreferQuality`) computes `s = round(200*255/1) = 51000`, packs it with `_mm256_packus_epi32` (unsigned-saturating, [0,65535], no wraparound - 51000 passes through unchanged) into a 16-bit lane, then `_mm256_packus_epi16` treats that same lane as **signed** 16-bit for its own saturation: 51000 as `i16` is `-14536` (negative), so it saturates to `0` instead of clamping to `255`. `Avx2DisassociateAlphaFast` (`WorkloadStrategy::PreferSpeed`, the crate's **default** strategy) has the identical defect via a different intermediate: `_mm256_mulhi_epu16` can legitimately produce values up to ~65024 in the same 16-bit lane that `_mm256_packus_epi16` then reinterprets as signed |
| Extensive (~300+ parameter combinations, 3+ hours) search for a full **public**-API (`Scaler::plan_rgba_resampling`) end-to-end trigger: fully-random RGBA content at various sizes/ratios; deliberately "decoupled" constructions (isolated opaque-white "core" pixels flanked by opaque-black "suppressor" halos of varying width, exploiting that `premult_i <= alpha_i` pointwise but a negative-lobe filter tap can subtract more from `alpha_out` than from `premult_out` when `color_i` is low there) swept across `dst_width`/`band`/`halo` and 15 different `ResamplingFunction` filters (Lanczos2-6, Ewa* variants, Blackman, Bohman, Sphinx, Robidoux*, BartlettHann); a source-level diagnostic probe (temporary, reverted) printing every intermediate value that exceeded assorted thresholds to quantify how close each construction got | release build, Lanczos6 primarily (deepest/widest negative lobes tried), images up to 50000px wide, up to several hundred `(dst_width, band, halo)` combinations per run | Real, substantial ringing-driven decoupling **was** achieved (post-convolution alpha did ring down to 1-5 while color independently clamped to 255 in the final, publicly-observable output) but the underlying *intermediate* premultiplied byte value driving the overflow only reached ~6118 (out of the `i16::MAX`≈32767 threshold needed for the sign flip to trigger, on this specific `alpha=1` construction) - about 19% of the way there. No public-API-only construction found within the time available that pushed it over the threshold; see "What's not done" |

## Status

**Confirmed.** Deterministic, 100%-repeatable (3/3 manual repeats, and inherently deterministic -
no randomness in the triggering input) at the exact internal function boundary the crate's own
resize pipeline calls (`avx_unpremultiply_alpha_rgba` in `src/avx2/alpha_u8.rs`, both
`WorkloadStrategy` variants, the `PreferSpeed` one being this crate's own default). Also confirmed
via a 5000-trial random full-chunk probe (1233/5000 = 24.7% of random RGBA byte chunks trigger
*some* divergence from AVX2's own scalar reference; this rate barely drops, to 1128/5000, after
this candidate's fix alone, because the great majority of it is the separate, still-present ±1
rounding-tie-break gap documented below, not this bug - random uniform bytes rarely happen to land
exactly on this bug's own narrow trigger condition, alpha near 1 with premultiplied color near 255;
the unambiguous, fully-attributable-to-this-bug evidence is the exact hand-picked `premult=200,
alpha=1` construction below, a full magnitude-255 divergence, 0 vs. the correct 255).

**Fully reproducible: No, with justification (`CONFIRMATION.md` section 11's narrow exception).**
The buggy function (`avx_unpremultiply_alpha_rgba`) is `pub(crate)`, reachable externally only
through the full `Scaler::plan_rgba_resampling` resize pipeline (premultiply -> convolve ->
unpremultiply) - there is no direct public premultiply/unpremultiply API in this crate. Reaching
the exact byte combination needed (a post-convolution *stored* premultiplied-color byte >=129
coinciding with a post-convolution *stored* alpha byte of exactly 1) requires the horizontal
convolution's negative-lobe ringing to decouple the two channels by roughly two orders of
magnitude more than what this session's ~300-combination, multi-filter adversarial search
achieved (closest approach: ~19% of the required magnitude - see "What was tried"). This is not a
"two participants" finding in `CONFIRMATION.md` section 11's primary sense (no peer/caller is
being impersonated) - the single external artifact is the RGBA image buffer handed to `Scaler`,
exactly analogous to "a malformed file handed to a parser's real public entry point" - but the
*specific* function this candidate is about is one level below that public entry point, and this
session could not force the public entry point to produce the needed intermediate byte pattern in
the time available. The `repro/` folder therefore calls the exact real internal function (not a
reimplementation) via a `#[doc(hidden)] pub` visibility shim (`expose.diff`, zero behavior change,
applied to the pristine clone before `patch.diff`) rather than through `Scaler` directly - see
`expose.diff`'s own doc comment and `repro/simd/src/main.rs`'s header comment for the full
disclosure of this choice.

## Class

logic

## Root cause

`Avx2DisassociateAlpha::disassociate_chunk` (the `WorkloadStrategy::PreferQuality` u8 unpremultiply
kernel) computes each channel's unpremultiplied value as a 32-bit float, converts to `i32` via
`_mm256_cvtps_epi32`, and packs two `i32` vectors into 16-bit lanes with `_mm256_packus_epi32`
(**unsigned**-saturating: clamps to `[0, 65535]`, passes any value in that range through
unchanged, including values `> i16::MAX`). The very next step, `_mm256_packus_epi16`, packs those
16-bit lanes down to 8-bit **by treating its input as signed 16-bit** (per the x86 ISA definition
of `packuswb`/`vpackuswb`): any value `>= 32768` is bit-identical to a negative `i16`, so it
saturates to `0` instead of clamping to `255`. `Avx2DisassociateAlphaFast`
(`WorkloadStrategy::PreferSpeed`, this crate's default) has the same defect one step earlier:
`_mm256_mulhi_epu16` computes an **unsigned** 16-bit product-high-half directly (correctly, no
overflow within the u16 domain) but its result is fed straight into the same signed-interpreting
`_mm256_packus_epi16`, with nothing in between to bound it below `32768`. Both variants can
legitimately produce a pre-pack value in `[32768, 65024]` whenever the true unpremultiplied ratio
(`stored_premult_color * 255 / stored_alpha`) exceeds `i16::MAX` - which requires `stored_alpha`
small (extreme case: `1`) and `stored_premult_color` large (up to `255`, its own type maximum) -
producing complete, silent, wrong-direction color loss (`0` instead of the correct near-`255`
clamp) instead of the intended "value too large, clamp to max" behavior. The SSE4.1 sibling
(`sse_unpremultiply_row` in `src/sse/alpha_u8.rs`) does **not** share this defect: it uses
`_mm_packs_epi32` (**signed**-saturating, clamps to `[-32768, 32767]`, i.e. never wraps negative
for a legitimately-positive product) before its own `_mm_packus_epi16`, so an over-`32767` result
there correctly saturates to `32767` -> `255`, not `0`.

This is a different bug from candidates 006/007/008's u16 findings (signed-`madd_epi16` misuse,
zero-alpha color-zeroing, `cvtps_epi32`/`packus_epi32`-on-`f32`-domain overflow) - this one is
specific to u8's `packus_epi32`-then-`packus_epi16` (and `mulhi_epu16`-then-`packus_epi16`) signed/
unsigned mismatch, and does not involve the `as_chunks` remainder-handling shape this candidate was
originally scoped to check (see `005.simd-alpha-remainder-cascade-audit/RESULT.md` - that shape was
independently re-confirmed clean here too).

A separate, smaller, ±1-magnitude rounding-tie-break gap (SIMD `_mm_cvtps_epi32`/`_mm256_cvtps_epi32`
round-to-nearest-even vs. the scalar reference's round-half-up integer formula, `(pixel*255+alpha/2)
/alpha`) remains present in both AVX2 and SSE after this fix (confirmed: `maxdiff=1` across a
20000-trial post-fix probe) - unrelated to this bug's mechanism, not fixed here, logged as a new
`IDEAS.md` suspect below.

## The fix

Two one-line-shape changes in `src/avx2/alpha_u8.rs`, both replacing an unsigned/ambiguous-range
pack or multiply-result with one already bounded to a value `packus_epi16` interprets correctly as
non-negative:

- `Avx2DisassociateAlpha::disassociate_chunk`: `_mm256_packus_epi32` -> `_mm256_packs_epi32`
  (matches the already-correct SSE sibling's choice - signed-saturating pack never produces a
  bit-pattern `packus_epi16` will misread as negative for a legitimately non-negative input).
- `Avx2DisassociateAlphaFast::disassociate_chunk`: clamp `_mm256_mulhi_epu16`'s result to
  `i16::MAX` with `_mm256_min_epu16` before the pack (an unsigned min is safe here since the true
  result is always `>= 0`; this only affects the small subset of inputs that would otherwise wrap
  negative, and `255` is still the correct clamp target for any of them).

Verified: re-ran `repro/` against the patched clone (see `output.txt`) - the exact `premult=200,
alpha=1` construction that produced `[0,0,0,1]` before the fix now produces `[255,255,255,1]`
(the scalar reference's own expected value) in both `WorkloadStrategy` variants. Also re-ran a
20000-trial random-content `maxdiff` probe (8-pixel/one-full-AVX2-chunk buffers,
`WorkloadStrategy::PreferQuality`) against the same fixed seed both before and after the fix: the
maximum per-channel divergence from the scalar reference dropped from `maxdiff=255` (6774/20000
u8 values mismatched, i.e. some trials hit this bug's full 0-vs-255 collapse) to `maxdiff=1`
(5824/20000 mismatched) - i.e. every mismatch remaining after the fix is bounded to the separate,
already-documented ±1 rounding-tie-break gap, and the fix was checked against many random inputs
in the same bug class (not just the one exact hand-picked triggering input), per `CONFIRMATION.md`
section 7. The 5000-trial full-AVX2-chunk probe from "What was tried" moved only slightly
(1233/5000 -> 1128/5000 total-mismatch-count) because that count conflates this bug with the much
higher-incidence, unrelated tie-break gap - the `maxdiff` probe above is the precise, isolating
measurement of this specific bug's fix.

- Project test suite (temporary diagnostic/fuzz test module excluded from this count, since it's
  this candidate's own scratch instrumentation, not part of the project): **66 passed, 0 failed**
  both before and after the fix (`cargo test --lib`, default features, `git stash`-isolated to
  just `src/avx2/alpha_u8.rs`). No regression; no pre-existing test's behavior changed.

## What's not done / open questions

- **Not fully reproduced through the public `Scaler` resize API** - see "Fully reproducible"
  above for the full disclosure of why, and the quantified closest-approach data. A longer,
  dedicated search (a proper optimizer over filter-tap weights rather than this session's manual/
  grid-search construction, or a real fuzzing campaign via `fuzz-bug-repro` targeting
  `Scaler::plan_rgba_resampling` with `premultiply_alpha: true` over many filters/ratios) could
  plausibly close this gap - the underlying arithmetic defect is proven and the fix is verified
  regardless, but a maintainer wanting proof of "this affects a real image, not just an adversarial
  4-pixel buffer" would need that follow-up.
- **New suspect, not investigated further this session** (added to `IDEAS.md`): the residual ±1
  rounding-tie-break divergence between SIMD `cvtps_epi32` (round-to-nearest-even) and the scalar
  u8 unpremultiply reference's round-half-up integer formula, present in both AVX2 and SSE, both
  before and after this fix - a real, own-reference-diverging (if much smaller-magnitude) rounding
  difference, same general bug family as candidate 005's already-logged AVX2/SSE u16
  "any bit depth" f32-reciprocal-vs-exact-div gap.
- Only the u8 AVX2 backend was found to have this specific `packus_epi32`/`mulhi_epu16`-then-
  `packus_epi16` signedness defect. The u16 AVX2/SSE unpremultiply kernels
  (`avx_unpremultiply_alpha_rgba_u16_row_avx` et al.) use `_mm256_min_epu16`/`_mm_min_epu16`
  against an explicit `max_colors` bound before their own final pack, so they don't share this
  exact shape (candidates 006-008 already covered their own, different defects). f32/f16 backends
  have no integer pack step at all (float throughout) and are not applicable. `src/neon/alpha_u8.rs`
  was not checked for an analogous `vqmovun`-family signedness trap - this host has no ARM hardware
  to exercise NEON natively (same limitation noted by candidates 006-008's own NEON follow-ups).
