# Building this crate with -C target-feature=+avx2 auto-vectorizes the vertical convolution tap loop (measurement only)

**Ported from `image` repo's `___OPTIMIZATIONS___/044.PIC_SCALE_SAFE_AVX2_TARGET_FEATURE_REBUILD/`.**
Originally measured in 2026-08 by rebuilding the completely unmodified registry `pic-scale-safe`
0.1.11 crate (no vendoring, no source changes anywhere) with different `RUSTFLAGS`. **Not
re-measured in this workspace** - numbers below are copied verbatim; this session made no source
change to verify against, since none was made in the original either. **No `patch.diff` in this
folder, correctly** - per this skill's own rule, a pure build-configuration measurement with no
source change gets no patch.

This crate's vertical-convolution tap loop (`fixed_point_vertical.rs`) never uses `std::arch`
intrinsics or `#[target_feature]` - it relies entirely on the default auto-vectorizer, which only
emits instructions legal for whatever target features the crate is compiled with. Since this crate
(like most published crates) is built against the generic x86-64 baseline for portability, the
tap loop only ever gets 128-bit SSE2 codegen today. This candidate checked what happens if the
*same, unmodified* source is rebuilt with `RUSTFLAGS="-C target-feature=+avx2"`.

## What was tried

Two builds of the unmodified registry crate, in separate `CARGO_TARGET_DIR`s: default `RUSTFLAGS`
(baseline) vs. `-C target-feature=+avx2`. Disassembly (`objdump`) of
`column_handler_fixed_point::<u8, i32>` confirmed the baseline emits zero `ymm`/`zmm` instructions
(118 `xmm`-operand instructions only) while the AVX2 build emits 62 `ymm`-operand instructions with
no source change at all.

| Variant | single-core p50 | multi-core p50 | scenario |
|---|---|---|---|
| baseline | 18.766 ms | 18.703 ms | Lanczos3, 4032x3024->400x300 |
| `+avx2` | 10.978 ms (**-41.5%**, 5/5) | 11.048 ms (**-40.9%**, 5/5) | same |
| baseline | 7.325 ms | n/a | Triangle, same source/dest |
| `+avx2` | 5.502 ms (**-24.9%**, 5/5) | n/a | same |
| baseline | 199.66 us | n/a | Lanczos3, small image (400x300->32x24) |
| `+avx2` | 119.59 us (**-40.1%**, 5/5) | n/a | same |

## Result

An AVX2-width rebuild of the completely unmodified crate is **-41.5%/-40.9%** (single/multi-core)
on a large Lanczos3 resize, **-24.9%** on Triangle, and **-40.1%** on a small-image Lanczos3 case -
all far outside the measured same-binary noise floor (+0.7%). This is a whole-crate effect (every
auto-vectorizable loop gets the wider width), not specific to any one function.

## Does it change behavior/output?

- **Bit-exact?** Yes, confirmed directly (40 hashes across 5 filters, matched exactly between
  builds).
- **Is this a breaking change?** N/A - no source change was made to this crate. There is correctly
  no `patch.diff`: forcing `+avx2` as a blanket build default would illegal-instruction-crash on
  any non-AVX2 x86_64 host, which is not an acceptable default for a general-purpose crate that
  makes no such assumption anywhere else.
- Project test suite: the *original* measurement quoted `image`'s own suite (622 passed, 1
  pre-existing unrelated failure, identical with/without the flag). Not re-run against this crate's
  own suite in this session since no source or build-flag change was made here to verify.

## Why the result looks the way it does

`-C target-feature=+avx2` lets LLVM's auto-vectorizer widen the same safe-Rust tap loop from
128-bit SSE2 (4x `i32` lanes) to 256-bit AVX2 (8x `i32` lanes) with zero source changes. Doubling
the lane width roughly halves the loop iterations needed for the same column count, consistent
with the ~41% wall-clock improvement (not exactly 2x, since non-vectorized setup/dispatch overhead
doesn't share in the speedup).

## What's not done / open questions

- **No source-level change is proposed here** - see candidates 007/008 in this workspace for the
  safe alternative (runtime `is_x86_feature_detected!("avx2")` + `#[target_feature(enable =
  "avx2")]` multiversioning), which gets most of this same win without requiring a non-default
  build flag, at the cost of the crate's own `#![forbid(unsafe_code)]` guarantee.
- Whether downstream consumers who already build with `RUSTFLAGS="-C target-cpu=native"` are
  already getting this speedup today with zero code changes was not verified beyond this
  measurement.
- No AVX-512 rebuild was tried in this specific candidate - see candidate 006 for that.
