# Alpha premultiply/unpremultiply per-bit-depth code: luma-alpha copy-paste-from-RGBA check

Backlog entry (`IDEAS.md`, added 2026-08-25, source: sibling-project `pic-scale-safe` cross-
reference): that codebase's `src/alpha.rs` had a copy-paste-from-RGBA bug in its luma-alpha
(2-channel) premultiply/unpremultiply variant for one bit-depth branch. This candidate checked
whether `pic-scale`'s equivalent per-bit-depth alpha code (`src/alpha_handle_u8.rs`,
`alpha_handle_u16.rs`, `alpha_handle_f16.rs`, `alpha_handle_f32.rs`, plus SIMD variants under
`src/sse/`, `src/avx2/`, `src/neon/`, `src/avx512/`, `src/wasm32/`) has the same bug class.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read every gray-alpha (2-channel, "LA") code path in `src/alpha_handle_u8.rs`, `alpha_handle_u16.rs`, `alpha_handle_f32.rs`, `alpha_handle_f16.rs` and every dispatcher that wires them up in `src/image_store.rs`'s `AssociateAlpha`/`UnassociateAlpha` impls | source at current HEAD (`7eec7cf`) | Found the crate's LA handling is architecturally different from `pic-scale-safe`'s (a single `alpha.rs` with per-bit-depth branches sharing helper code between RGBA and LA): here every LA premultiply/unpremultiply path (`u8`, `u16`, `f32`) is its own small, independent function operating on `as_chunks::<2>()` with no shared/copied logic from the 4-channel `as_chunks::<4>()` RGBA functions - no evidence of a copy-paste origin by inspection. `f16` has **no** LA premultiply/unpremultiply code at all (no `GrayAlphaF16` image-store type exists; `ImageStore<f16, 2>` is `CbCrF16`, a chroma pair unrelated to alpha). |
| Grepped `src/sse/`, `src/avx2/`, `src/neon/`, `src/avx512/`, `src/wasm32/` for any gray-alpha/2-channel alpha premultiply function (`grep -rn "pub(crate) fn" src/{sse,avx2,neon,avx512,wasm32}/alpha_*.rs`, `find src/avx512 -iname '*alpha*'`) | source at current HEAD | **None of the SIMD/WASM backends implement a gray-alpha (2-channel) alpha premultiply/unpremultiply function at all** - every backend only has an RGBA (4-channel) variant (`*_premultiply_alpha_rgba*`/`*_unpremultiply_alpha_rgba*`). `src/avx512` doesn't even have RGBA premultiply/unpremultiply, only `avx512_has_non_constant_cap_alpha_rgba8` (an alpha-check, not premultiply). Every dispatcher in `image_store.rs` for `ImageStore<T, 2>` (`u8`/`u16`/`f32`) calls the scalar `*_impl` function directly with **no** SIMD fn-pointer swap - unlike the RGBA dispatchers, which do swap `_dispatcher` to a SIMD function when the relevant CPU feature is detected. This rules out the specific bug shape from the sibling project (a SIMD LA path that shares/copies indexing from a 4-channel SIMD RGBA kernel): there is no SIMD LA code here to have copied wrongly from RGBA in the first place. |
| Differential test: temporarily added `#[cfg(test)] mod bughunt_diff_tests` to `alpha_handle_u8.rs`/`alpha_handle_u16.rs`/`alpha_handle_f32.rs` (see `verification_tests.diff`), constructing an RGBA pixel with R=G=B=luma and the same alpha, and asserting the LA premultiply/unpremultiply row functions produce byte-identical results to the RGBA row functions on the corresponding channels | debug build, `cargo test --lib bughunt_diff_tests` and `cargo test --lib bughunt_diff_tests --no-default-features`, reverted after capture | u8: exhaustive, all 65536 (luma, alpha) pairs for both premultiply and unpremultiply - 0 mismatches. u16: densely sampled (~252x252 grid per bit depth) across all three real bit depths (10/12/16-bit) for both directions - 0 mismatches. f32: 201x201 grid over `[0.0, 1.0]` for both directions - 0 mismatches. All 6 new tests passed under both `--no-default-features` (forces the scalar-only fn pointers) and default features (SIMD feature-detected at runtime on this AVX2/AVX-512/SSE4.1-capable host, though as established above this makes no difference for LA since no SIMD LA path exists to dispatch to). |

## Status

**Not reproducible.** No copy-paste-from-RGBA bug (or any other premultiply/unpremultiply
discrepancy) was found in `pic-scale`'s luma-alpha (2-channel) code for any bit depth/type that
supports it (u8, u16 at 10/12/16-bit, f32). Coverage: every LA code path was read; every SIMD
backend directory was checked and confirmed to have no LA-specific alpha code at all; and a
differential test against the RGBA reference path (exhaustive for u8, densely sampled for u16/f32)
found zero mismatches in either direction.

## Class

N/A (Not reproducible).

## Root cause

Not applicable to the "no bug found" result. For *why the backlog entry's suspicion doesn't hold
here*: `pic-scale-safe`'s bug arose because its LA and RGBA premultiply/unpremultiply variants
shared a single `alpha.rs` file with parallel per-bit-depth branches, and one LA branch was
apparently copy-edited from the corresponding RGBA branch with a mistake left in. `pic-scale`'s
scalar LA functions (`premultiply_alpha_gray_alpha_row_impl`/`unpremultiply_alpha_gray_alpha_row_impl`
in u8; `premultiply_alpha_gray_alpha_row`/`unpremultiply_alpha_gray_alpha_row` in u16;
`premultiply_gray_alpha_f32_row`/`unpremultiply_gray_alpha_f32_row` in f32) are each written as
their own small, self-contained function operating on 2-element chunks - not derived by editing a
copy of the 4-element RGBA version - so there was no copy-paste step for a channel-index mistake to
survive. More significantly, this crate's LA support never got a SIMD implementation in any
backend (SSE/AVX2/AVX-512/NEON/WASM) at all, so the specific mechanism the sibling bug depended on
(a hand-written SIMD kernel written for one channel count reused/adapted for another without fully
updating the indexing) doesn't have anywhere to occur here - LA always runs through the plain
scalar path regardless of detected CPU features.

## The fix

No fix - no bug confirmed. `verification_tests.diff` in this folder is **not** a source fix; it is
the temporary `#[cfg(test)]` differential coverage added and run this session (then reverted from
the real working tree per the workflow's "always leave the tree clean" rule), kept here as durable
evidence of what was checked and how.

- Project test suite: 66/66 passed before (baseline HEAD), 72/72 passed after applying the
  temporary differential tests (66 pre-existing + 6 new) - no new failures, no source change was
  made. See `output.txt` for the actual captured runs.

## What's not done / open questions

- `f16` has no gray-alpha (LA) image type/support at all in this crate (confirmed via
  `image_store.rs`/`AssociateAlpha`/`UnassociateAlpha` impls and grep for `GrayAlphaF16`) - there
  is nothing to check for this bug class there, not a gap in this candidate's coverage.
- This candidate did not check the RGBA-only SIMD alpha kernels themselves (`src/sse/alpha_*.rs`,
  `src/avx2/alpha_*.rs`, `src/neon/alpha_*.rs`) for internal bugs unrelated to the LA copy-paste
  question (e.g. a lane-count/remainder-handling bug within the RGBA vectorized loop itself) - that
  is a distinct suspect, already noted separately in `IDEAS.md`/`projects/pic-scale.md` under
  "Remainder/edge handling in SIMD resize kernels".
- Noticed while investigating (not part of this candidate's scope, added to `IDEAS.md`): every
  gray-alpha (LA) premultiply/unpremultiply dispatcher (`u8`/`u16`/`f32`) always uses the scalar
  fallback with no SIMD acceleration, unlike every RGBA dispatcher in the same files, which does
  feature-detect and swap in a SIMD implementation. This is a performance gap, not a correctness
  bug (the scalar path was verified correct above) - worth a `perf-optimize` look if LA-heavy
  workloads (e.g. grayscale-with-alpha PNGs) matter for this crate's users.
