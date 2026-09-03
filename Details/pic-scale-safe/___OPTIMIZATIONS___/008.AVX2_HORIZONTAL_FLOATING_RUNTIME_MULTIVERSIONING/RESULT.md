# Extend runtime AVX2 multiversioning to the horizontal and floating-point convolution paths

> **Project policy (2026-08-29): not adoptable.** The repo owner has confirmed unsafe code is
> never in scope in this crate, regardless of measured speedup - `#![forbid(unsafe_code)]` is a
> permanent design commitment, not a default to be relaxed for performance. This candidate is kept
> purely as a historical, fully-measured record of what unsafe SIMD intrinsics would buy. Unsafe/
> SIMD-intrinsics optimization work belongs in the sibling crate `~/Projekty/Obce/pic-scale`
> instead, never here.


**Ported from `image` repo's `___OPTIMIZATIONS___/049.PIC_SCALE_SAFE_AVX2_HORIZONTAL_AND_FLOATING_MULTIVERSIONING/`.**
Originally measured in 2026-08, same vendoring method as candidate 007, built on top of it. **Not
re-measured in this workspace** - numbers below are copied verbatim. Patch rebuilt by hand against
current `HEAD` (`b50e020`); `git apply --check` passes, `cargo check --features rayon` compiles
clean, `cargo test --release --features rayon` passes 5/5. Supersedes/extends candidate 007 (same
`forbid` -> `deny(unsafe_code)` policy tradeoff applies here too - see candidate 007's RESULT.md
for the full discussion, not repeated here).

Candidate 007 multiversioned only the vertical tap loop and explicitly attributed its -27%-vs-005's
-41% gap to the horizontal pass (`fixed_point_horizontal.rs`) and floating-point paths
(`floating_point_vertical.rs`/`floating_point_horizontal.rs`) being left scalar. This candidate
applies the same pattern (shared `#[inline(always)]` impl + `#[target_feature(enable = "avx2")]`
wrapper + `is_x86_feature_detected!` dispatch) to those six functions.

## What was tried

`fixed_point_horizontal.rs`: `convolve_row_handler_fixed_point`/`_4`. `floating_point_horizontal.rs`:
`convolve_row_handler_floating_point`/`_4`. `floating_point_vertical.rs`:
`convolve_column_handler_floating_point_4`. Built on top of candidate 007's vertical fix.

| Variant | single-core p50 | multi-core p50 | scenario |
|---|---|---|---|
| baseline | 19.644 ms | 18.771 ms | Lanczos3, 4032x3024->400x300 (fixed-point path) |
| this candidate | 11.781 ms (**-40.0%**, 5/5) | 11.278 ms (**-39.9%**, 5/5) | same |
| baseline | 7.321 ms | n/a | Triangle (fixed-point path) |
| this candidate | 5.572 ms (**-23.9%**, 5/5) | n/a | same |
| baseline | 194.27 us | n/a | Lanczos3, small image (fixed-point path) |
| this candidate | 122.53 us (**-36.9%**, 5/5) | n/a | same |
| baseline | 122.85 us | 123.23 us | CatmullRom, small image (floating-point path, untouched by 007) |
| this candidate | 74.384 us (**-39.5%**, 5/5) | 74.852 us (**-39.3%**, 5/5) | same |
| baseline | 119.05 us | n/a | Gaussian, small image (floating-point path) |
| this candidate | 70.898 us (**-40.4%**, 5/5) | n/a | same |

## Result

Extending multiversioning to the horizontal and floating-point paths lifts the Lanczos3 win from
candidate 007's -27.0%/-26.9% to **-40.0%/-39.9%** - now within about a point of candidate 005's
-41% whole-crate `RUSTFLAGS` rebuild - and Triangle from -14.5% to **-23.9%**. On the
floating-point path, which candidate 007 left completely untouched, this candidate measures
**-39.5%/-39.3%** on CatmullRom and **-40.4%** on Gaussian - closing essentially all of the gap
candidate 007 attributed to these paths.

## Does it change behavior/output?

- **Bit-exact?** Yes, verified two ways in the original measurement (same 55-hash corpus as
  candidate 007, plus all new `has_avx2` checks forced to `false` to verify the fallback path
  specifically for all six newly-multiversioned functions) - all matched.
- **Is this a breaking change?** Same answer as candidate 007 (no further crate-level lint change
  beyond what 007 already made - this candidate adds three more narrowly-scoped
  `#[allow(unsafe_code)]` sites on top of 007's own).
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. Original measurement additionally quoted `image`'s
  own suite: 622 passed, 1 pre-existing unrelated failure, identical with/without this patch.

## Why the result looks the way it does

Same mechanism as candidate 007, applied to the remaining hot code paths it left scalar. Reaching
within about a point of candidate 005's whole-crate rebuild confirms candidate 007's own diagnosis:
the -27%-vs-41% gap was attributable almost entirely to these two paths being unwidened, not to any
inherent ceiling on what runtime multiversioning can achieve vs. a compile-time flag.

## What's not done / open questions

- No AVX-512 tier - see candidate 010 for extending this to AVX-512.
- The 1-column/1-channel scalar tail functions (both fixed- and floating-point) were left
  un-multiversioned - they only run for a row's last few pixels, where AVX2 setup overhead would
  likely dominate any gain; not measured directly.
- `resize_nearest.rs` is untouched by this candidate (see candidate 001 for that filter's own fix).
- Not independently re-benchmarked against current `HEAD` in this session - only correctness and
  the test suite were re-verified; timing numbers are historical and directional only.
