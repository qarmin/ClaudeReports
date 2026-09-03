# SIMD alpha premultiply/unpremultiply remainder-handling audit (as_chunks::<N>().0 shape)

Backlog row from `IDEAS.md` (sourced from `004.avx512-alpha-check-remainder/RESULT.md`): candidate
004 confirmed the `.as_chunks::<N>().0` idiom (which silently drops the tail in `.1`) as a real,
single-level, remainder-dropping bug in `src/avx512/check_alpha.rs`. A grep for
`as_chunks::<[0-9]*>()\.0` found 12 more occurrences of the same call shape across
`src/avx2/alpha_{f16,u16,f32,u8}.rs` and `src/sse/alpha_{u16,f32,u8,f16}.rs` (the SIMD alpha
premultiply/unpremultiply kernels, as opposed to `check_alpha.rs`'s alpha-*detection* kernels).
None of these 8 files had been individually checked for whether the same single-level,
remainder-dropping shape exists.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code reading of every `.as_chunks::<N>().0`/`.as_chunks_mut::<N>().0` call site in all 8 flagged files, tracing what happens to the matching `.1` (the dropped-in-004's-bug remainder) | reading only | Every site is structurally different from `check_alpha.rs`'s bug: `.1` is always captured into a `rem`/`src_rem` variable and then either (a) fed into a second, smaller `as_chunks::<M>()` cascade down to a final scalar/near-scalar tier (`avx2/alpha_u16.rs`: 32->16; `avx2/alpha_f32.rs`: 32->8->4, and RGBA-f32 buffers are always a multiple of 4 elements so the final tier's own `.1` is always empty by construction), or (b) copied into a fixed-size zero-padded stack buffer, processed through the *same* one-vector-wide kernel used for full chunks, then copied back out only for `..rem.len()` (`avx2/alpha_u8.rs`, `avx2/alpha_f16.rs` falls back to the scalar `premultiply_pixel_f16_row`/`unpremultiply_pixel_f16_row`, and all 4 `src/sse/alpha_*.rs` files use the same buffer-or-scalar-fallback pattern). In every case 100% of the row's bytes/elements are visited by some tier; none of the 8 files has an unreachable/no-op remainder branch the way `check_alpha.rs` did. |
| Differential/property testing: added temporary `#[cfg(test)]` modules (see `verification_tests.diff`) to all 8 files, comparing each SIMD premultiply/unpremultiply function against its scalar reference (`alpha_handle_{u8,u16,f32,f16}.rs`) across pixel widths 1..200 (1..260 for the 64/32-wide f16 chunk sizes), for every bit depth (`u16`: 10/12/16) - specifically targeting every width whose byte/element count is *not* an exact multiple of the SIMD chunk size, i.e. every width that exercises the `.1` remainder path | debug build, `cargo test --lib --features nightly_f16`, this host (avx2+sse4.1 detected) | u8 backends (`avx2`/`sse`, exact-equality vs. scalar): all widths 1..260 matched bit-for-bit, 0 mismatches - confirms no remainder byte is ever left unprocessed. u16/f32/f16 backends: exact-equality initially found small mismatches (see below), which a second, targeted check proved were unrelated to remainder handling (see next row) - after switching to a tolerance-based comparison sized to that known, unrelated precision gap, all widths 1..260 (and all `u16` bit depths) passed for every remaining backend/dtype, i.e. no width produced a large, "this data was never written" style deviation anywhere. |
| Root-cause and isolate the initial exact-equality mismatches (u16 bit_depth 16, f32, f16 backends) to confirm they are *not* the remainder-drop bug being investigated | targeted probe: fixed `width=16` for `avx2::alpha_u16` (16*4=64 elements, an *exact* multiple of the AVX2 u16 executor's 16-lane main-loop chunk, so `as_chunks::<16>().1` is empty and the buffer-fallback branch is provably never entered), 5000 independent random seeds, exact-equality vs. scalar reference | 144/5000 (2.88%) still mismatched by ±1 LSB **with zero remainder bytes involved at all** - proves the earlier mismatches are a pre-existing, main-loop-and-remainder-alike precision artifact: `Avx2PremultiplyExecutorAnyBit`/`Sse41PremultiplyExecutorAny` (used whenever `bit_depth` isn't 10 or 12, including the very common `bit_depth == 16` case) compute `u32 * u32` products up to ~4.29e9 and round-trip them through `f32` (24-bit mantissa, exact only up to ~1.68e7) with a single shared reciprocal scale, while the scalar reference for `bit_depth == 16` uses an exact fixed-point integer division (`div_by_65535`) - a genuine, reproducible ±1-LSB-out-of-65535 divergence between the SIMD "any bit depth" path and its own scalar reference, unrelated to `as_chunks`/remainder handling and present throughout the main loop, not concentrated at the tail. Filed as a new `IDEAS.md` suspect (not this candidate's bug - see "What's not done" below). A similar (much smaller, ~1e-3-relative) direct-division-vs-reciprocal-multiply drift explains the f32/f16 mismatches. |

## Status

**Not reproducible.** No occurrence of the assigned `.as_chunks::<N>().0`-drops-the-remainder shape
was found in any of the 8 flagged files - all 12 call sites correctly hand the `.1` remainder to a
smaller-chunk cascade, a padded-buffer retry through the same kernel, or a scalar fallback that
covers it completely. This was checked twice: once by reading the code, once empirically via
tolerance-bounded differential testing across every remainder-triggering width for every
backend/dtype (u8: exact match, 0/260 widths mismatched; u16/f32/f16: within a tolerance sized to a
separate, confirmed-unrelated precision gap, 0/200-260 widths mismatched per backend).

## Class

N/A (not reproduced)

## Root cause

N/A - the suspected bug class (silent data loss from a single-level `as_chunks::<N>().0` iteration
with no cascade, as in candidate 004's `check_alpha.rs` finding) does not occur at any of these 12
call sites. Each site's author wrote the same defensive pattern candidate 004 found *missing* in
`check_alpha.rs`: capture `.1` into a variable, then explicitly consume all of it via one of the
three complete-coverage strategies described above (cascade to a smaller `as_chunks`, buffer +
retry through the same kernel, or scalar fallback). `check_alpha.rs`'s bug was specifically that it
never captured `.1` into anything at all (`for chunk in row.as_chunks::<64>().0.iter()` with no
`let _ = row.as_chunks::<64>().1` anywhere in the function) - a strictly narrower, and evidently not
copy-pasted, mistake that these 8 files don't share.

## The fix

Not applicable - no bug found, no patch to write.

- Project test suite: 83 passed, 0 failed (`cargo test --lib --features nightly_f16`, default
  features + `nightly_f16`) both before adding this candidate's temporary tests (75 pre-existing
  tests colected without the 8 new test functions) and after (83 = 75 + 8 new tests, all passing).
  No pre-existing test regressed; no source file was changed (see `verification_tests.diff` for the
  test-only diff, reverted from the working tree before this candidate folder was written).

## What's not done / open questions

- **New suspect, not investigated further this session** (added to `IDEAS.md`): AVX2/SSE u16
  premultiply (`Avx2PremultiplyExecutorAnyBit` in `src/avx2/alpha_u16.rs`,
  `Sse41PremultiplyExecutorAny` in `src/sse/alpha_u16.rs`) routes `bit_depth == 16` - the single
  most common 16-bit-per-channel case - through the approximate float-reciprocal "any bit depth"
  path instead of an exact-integer-division path, even though both files already define and use an
  exact `_mm256_div_by_65535_epi32`/`_mm_div_by_65535_epi32` helper for the *other* two supported
  bit depths (10, 12) via `Avx2PremultiplyExecutorDefault<16>`/`Sse41PremultiplyExecutorDefault<16>`
  - those generic structs would work correctly for `BIT_DEPTH = 16` (the exact-division helper
  already has a `BIT_DEPTH != 10 && != 12` fallthrough case built for exactly this), but the
  dispatch functions (`avx_premultiply_alpha_rgba_u16_row`, `premultiply_alpha_sse_rgba_u16_row_impl`)
  never route `bit_depth == 16` there. Net effect: ~2.88% of pixels (measured, 5000-trial sample)
  premultiplied via AVX2/SSE at 16-bit depth round to a different u16 value (by 1) than the crate's
  own scalar reference for the identical input - not a memory-safety issue and not the remainder
  bug asked about here, but a real, own-reference-diverging rounding difference worth its own
  candidate.
- Not investigated: whether the smaller, apparently-intentional direct-division-vs-reciprocal-
  multiply choice in the tail tiers of `avx2/alpha_f32.rs`'s unpremultiply (8-wide/4-wide loops use
  `_mm256_div_ps`/`_mm_div_ps` directly, while the 32-wide main loop and the scalar reference both
  compute a shared reciprocal then multiply) is worth normalizing for consistency - the resulting
  drift is far smaller (~1e-3 relative) and was only characterized enough here to rule it out as
  unrelated to remainder handling, not root-caused as its own issue.
- Only x86_64 (AVX2/SSE4.1) backends were covered, matching the backlog row's scope; `src/neon/`'s
  equivalent alpha kernels (if they use the same `as_chunks` pattern) were not checked - this
  host has no ARM hardware to exercise them natively.
- gray-alpha (LA, 2-channel) SIMD kernels don't exist in this codebase at all (per candidate 003),
  so there was nothing to check there beyond the RGBA kernels already covered.
