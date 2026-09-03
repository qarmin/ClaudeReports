# Extend the AVX-512 runtime dispatch tier to the horizontal and floating-point convolution paths

> **Project policy (2026-08-29): not adoptable.** The repo owner has confirmed unsafe code is
> never in scope in this crate, regardless of measured speedup - `#![forbid(unsafe_code)]` is a
> permanent design commitment, not a default to be relaxed for performance. This candidate is kept
> purely as a historical, fully-measured record of what unsafe SIMD intrinsics would buy. Unsafe/
> SIMD-intrinsics optimization work belongs in the sibling crate `~/Projekty/Obce/pic-scale`
> instead, never here.


**Ported from `image` repo's `___OPTIMIZATIONS___/058.PIC_SCALE_SAFE_AVX512_HORIZONTAL_AND_FLOATING_MULTIVERSIONING/`.**
Originally measured in 2026-08, built on top of candidate 009 (053 in `image`'s own numbering).
**Not re-measured in this workspace** - numbers below are copied verbatim. Patch rebuilt by hand
against current `HEAD` (`b50e020`) on top of candidate 009; `git apply --check` passes, `cargo
check --features rayon` compiles clean, `cargo test --release --features rayon` passes 5/5. Same
`forbid` -> `deny(unsafe_code)` tradeoff as candidates 007/008/009 applies (inherited, not
repeated).

Candidate 009 added an AVX-512 tier only to the vertical dispatcher. This candidate applies the
same pattern to the five horizontal/floating-point functions candidate 008 already multiversioned
for AVX2: `convolve_row_handler_fixed_point`/`_4` (`fixed_point_horizontal.rs`),
`convolve_row_handler_floating_point`/`_4` (`floating_point_horizontal.rs`), and
`convolve_column_handler_floating_point_4` (`floating_point_vertical.rs`).

## What was tried

Added a third `#[target_feature(enable = "avx512f,avx512bw,avx512vl")] unsafe fn ..._avx512`
wrapper to each of the five functions, alongside their existing AVX2 wrapper from candidate 008,
using the same `has_avx512 -> has_avx2 -> scalar` cascade shape as candidate 009.

| Filter | single-core | multi-core | notes |
|---|---|---|---|
| Lanczos3 (fixed-point, large image) | **-1.4%** paired (5/5) / **-4.5%** pinned criterion (p<0.05) | +0.2% (inside noise, 4/5) | cross-method spread noted, not reconciled |
| Lanczos3 (small image) | +0.4% (inside noise, 3/5) | n/a | |
| Triangle (fixed-point) | **-3.4%** (7/10, needed 10 runs - unusually wide noise floor on this scenario) | n/a | |
| CatmullRom (floating-point, new ground) | **-1.5%** (5/5) | **-1.7%** (5/5) | |
| Gaussian (floating-point, new ground) | **-3.6%** (5/5) | n/a | |

## Result

**Real but modest and scenario-dependent** - the floating-point path (CatmullRom/Gaussian, the
first place this AVX-512 tier reaches genuinely new ground) shows a slightly larger relative win
(-1.5% to -3.6%) than the fixed-point path (-1.4% to -3.4%, arguably 0% on two of four Lanczos3
measurements), since candidate 009 already captured most of the available fixed-point-vertical win
before this candidate started. Disassembly directly confirmed that two of the five newly-tiered
functions (`convolve_row_handler_floating_point`/`_4`) never get wider-than-`xmm` codegen from LLVM
at either the AVX2 or AVX-512 tier - adding an AVX-512 tier to them is harmless (bit-exact) but
contributes nothing measurable; this is a real, disassembly-confirmed finding, not a measurement
gap.

## Does it change behavior/output?

- **Bit-exact?** Yes, verified three ways in the original measurement (same technique as candidate
  009: real execution, AVX-512-forced-off fallback, and both-forced-off scalar fallback, all
  matched across 55 hashes).
- **Is this a breaking change?** Same answer as candidates 007/008/009 - no additional crate-level
  lint change; this candidate adds five more narrowly-scoped `#[allow(unsafe_code)]` sites.
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. Original measurement additionally quoted `image`'s
  own suite: 622 passed, 1 pre-existing unrelated failure, identical with/without this patch.

## Why the result looks the way it does

Same mechanism as every AVX-512 candidate in this workspace. The smaller, uneven result vs.
candidate 009's own vertical-only step has two confirmed causes: (1) two of the five functions
structurally don't auto-vectorize past 128 bits regardless of target feature (disassembly-confirmed
dead weight, not a bug), and (2) the floating-point-vertical function this candidate touches was
genuinely scalar-at-AVX-512 before (candidate 009's own scope was fixed-point-vertical only), so it
has more room to gain than the fixed-point path, which candidate 009 already mostly captured.

## What's not done / open questions

- **The two disassembly-confirmed dead-weight functions could be reverted to AVX2-only** with no
  expected measured cost - not done here, since correctness is unaffected either way and this
  candidate's scope was "apply the pattern uniformly," not "prune afterward." A future candidate
  could investigate *why* LLVM won't vectorize these two loops (a data-dependent trip count, or a
  trait-based accumulation pattern that doesn't lower to SIMD).
- `resize_nearest.rs` is untouched (see candidate 001).
- AVX-512 downclocking was not directly instrumented.
- Not independently re-benchmarked against current `HEAD` in this session - only correctness and
  the test suite were re-verified; timing numbers are historical and directional only.
- This is the most AVX-512-complete candidate in this workspace's 007-010 line (vertical +
  horizontal + floating-point, all three tiers) - a reader wanting the fullest safe SIMD coverage
  would likely start here (on top of candidate 008, since this candidate builds on it), though all
  four remain separately documented per this skill's "one fix per candidate" convention.
