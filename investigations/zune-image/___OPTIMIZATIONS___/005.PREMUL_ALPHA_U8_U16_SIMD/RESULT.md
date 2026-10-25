# premultiply_u8/premultiply_u16 AVX2 dispatch

`crates/zune-imageprocs/src/premul_alpha.rs`'s `premultiply_u8`/`premultiply_u16` (the hot
multiply-add-shift loops used by `PremultiplyAlpha::execute_impl` for the two most common bit
depths) were plain scalar `iter_mut().zip().for_each()` loops. `rustc --version`:
`rustc 1.98.1 (48a229cea 2026-09-01)`, `cargo 1.98.1 (797e8a9bc 2026-08-05)` for all builds below.

## What was tried

Two variants, in order:

1. **`std::simd` (portable-simd feature, nightly)**, modeled directly on this same module's
   existing `unpremultiply_f32`/`std_simd.rs` pattern - widening `u8`/`u16` lanes to `u16`/`u32`,
   doing the multiply-add-shift, narrowing back. Built and benched, then **discarded**: `objdump`
   showed the hand-written `Simd<u8, 16>` kernel compiles to the exact same SSE2 instruction
   sequence (`punpcklbw`/`pmullw`/`paddw`/`psrlw`/`packuswb`) that LLVM's auto-vectorizer already
   produces from the *plain scalar* loop at this crate's default (generic x86-64, SSE2-only)
   compile target - confirmed by disassembling both binaries' `premultiply_u8` symbol side by
   side. Paired timing was flat (no measurable difference), consistent with identical codegen.
   This variant's code was not kept; the table below covers only the variant that was kept.
2. **AVX2 runtime dispatch via `simd_fn!`** (this crate's own existing AVX2/NEON dispatch macro,
   `crates/zune-imageprocs/src/macro_utils.rs`, already used by `sobel.rs`/`bilateral_filter.rs`/
   `rotate.rs`) - wraps the *same* scalar loop body in a `#[target_feature(enable = "avx2")]`
   function, runtime-selected via `is_x86_feature_detected!("avx2")` with the plain scalar
   fallback kept for non-AVX2/non-x86 targets. No nightly toolchain, no Cargo feature flag needed
   - `avx2` is already part of this crate's default feature set. `objdump` confirms this compiles
   to 256-bit AVX2 code (`vpmovzxbw`/`vpmullw`/`vpaddw`/`vpsrlw`/`vpackuswb`/`vpermq`, `ymm`
   registers) instead of the SSE2 code above. **This is the variant kept** (`patch.diff`).

Bench: a standalone `examples/bench_premul_alpha.rs` driver (copied into the cloned tree by
`justfile`, not part of `patch.diff` - no existing project bench/example reached these free
functions directly; the workspace's own criterion bench, `benchmarks/benches/imageprocs.rs`,
requires libvips as a system dependency not available in this environment, and even when
buildable only measures the full `PremultiplyAlpha` operation end-to-end, not this inner loop in
isolation). Simulates one RGB(A) frame: 3 color channels of 2,073,600 elements each (1920x1080),
each premultiplied in place against the same alpha channel, hashing the result every iteration.

| Variant | single-core p50 (`taskset -c2`, paired, 9 runs) | multi-core p50 (unpinned, paired, 9 runs) | notes |
|---|---|---|---|
| baseline (scalar) - u8 | 0.337 ms | 0.332 ms | auto-vectorizes to SSE2, see above |
| patched (AVX2 dispatch) - u8 | 0.191 ms (**-43.3%**, 9/9 pairs) | 0.188 ms (**-43.4%**, 9/9 pairs) | |
| baseline (scalar) - u16 | 0.621 ms | 0.607 ms | auto-vectorizes to SSE2 |
| patched (AVX2 dispatch) - u16 | 0.456 ms (**-26.6%**, 9/9 pairs) | 0.458 ms (**-24.5%**, 9/9 pairs) | |

All four comparisons via `tools/paired.sh` (`RUNS=9`, alternating A/B), all four reported
`verdict: consistent` (9/9 pairs the same direction).

## Result

**Time.** A real, paired-confirmed win for both bit depths, consistent between single-core
(`taskset -c2`) and multi-core (unpinned) measurement: **u8 ~43% faster, u16 ~25-27% faster**,
measured on the isolated `premultiply_u8`/`premultiply_u16` functions (3-channel, 1920x1080-sized
buffers). This is a micro-benchmark of the functions in isolation, not a full `PremultiplyAlpha`
end-to-end decode - the relative contribution of this loop to a full image-processing pipeline
(which also does colorspace conversion, table setup, etc.) was not separately measured. No
memory/allocation-count claim made (no allocation pattern changed - same in-place mutation, same
buffers).

## Does it change behavior/output?

- **Bit-exact?** yes.
  - `bench_premul_alpha`'s own fnv1a64 hash of the premultiplied output is identical baseline vs
    patched for both u8 (`2aafcd25be044494`) and u16 (`b61d0f18dfe72e43`), every iteration.
  - Added two unit tests directly in `premul_alpha.rs` (`tests::premultiply_u8_matches_reference_exhaustive`,
    `tests::premultiply_u16_matches_reference`) comparing the dispatched implementation against a
    plain scalar reference formula: u8 is checked **exhaustively** over the full 256x256
    `(color, alpha)` domain (both a length that's a multiple of the AVX2 lane width, 32, and a
    trimmed 12,345-element length to exercise the scalar remainder path); u16 is checked against
    2,003 random `(color, alpha)` pairs (also not a multiple of the AVX2 lane width, 16) via
    `nanorand`. Both pass.
- **Is this a breaking change?** no - same public API (`premultiply_u8`/`premultiply_u16`
  signatures unchanged), same output values.
- Project test suite:
  - `cargo test --release -p zune-imageprocs` (the only crate whose source changed): lib tests
    **77 passed / 0 failed -> 79 passed / 0 failed** (the 2 new tests above; no other test
    changed status), doc-tests **27 passed / 0 failed** both before and after.
  - A full `cargo test --release --workspace` was attempted; `zune-inflate`'s own lib test suite
    fails (29 passed / 30 failed, e.g. `tests::tests::test_all_byte_values`,
    `streaming_decoder::streaming_bitstream::tests::test_resume_across_chunk_boundary`) - this
    crate was not touched by this patch, and the same failures reproduce identically on a clean
    checkout of the unmodified `base_commit` with no `zune-imageprocs` changes applied (verified
    separately), so this is a pre-existing issue in the repo at this commit, unrelated to this
    candidate. Not investigated further here - flagged for the batch-level `___BUGHUNT___/` note
    per this session's own instructions rather than fixed in this folder.

## Why the result looks the way it does

The scalar loop is embarrassingly parallel (no loop-carried dependency, branch-free), so LLVM's
auto-vectorizer already turns it into SSE2 SIMD at this crate's default compile target - that's
why the first variant tried (a hand-written `std::simd` kernel at the same, default vector width)
measured no improvement: it's regenerating code LLVM already produces. The real gap wasn't
"scalar vs SIMD", it was "16-byte SIMD vs 32-byte SIMD" - the crate already ships a
`#[target_feature(enable = "avx2")]` runtime-dispatch macro (`simd_fn!`) for exactly this
situation elsewhere (`sobel.rs`, `bilateral_filter.rs`, `rotate.rs`), but `premul_alpha.rs` wasn't
using it. Wrapping the same scalar source in that macro lets LLVM's auto-vectorizer pick 256-bit
registers instead, doubling the elements processed per instruction - consistent with the ~43%
u8 win (close to, but short of, a full 2x, likely bounded partly by memory bandwidth: `mean` p95
figures show more scheduler noise on the u16 run, which processes more bytes per element). u16's
smaller ~26% win versus u8's ~43% is consistent with this too - the u16 kernel needs a second
widening step (`u16` -> `u32`, since `65535 * 65535` doesn't fit in `u16`), so its arithmetic has
more instructions per AVX2 vector relative to the memory traffic it does, and also handles half as
many elements per 256-bit register (16 x u16 vs 32 x u8), leaving less of the win headroom that
widening the SIMD width alone buys.

## What's not done / open questions

- `unpremultiply_u8`/`unpremultiply_u16` (the division-avoidance / `fastdiv_u32` reciprocal-table
  path, same module) were **not** touched in this pass - vectorizing those is a substantially
  different problem (the reciprocal magic number is a `u128` gathered per-element from a
  256-entry/65,536-entry table, not a fixed-shift constant), out of scope for this candidate's
  time budget. Left as an idea for a future candidate.
- No NEON (aarch64) path was added - `simd_fn!`'s own NEON branch only fires if the macro body is
  rewritten with NEON intrinsics; as used here (a plain scalar body handed to the macro) NEON
  falls through to the identical scalar fallback used by any other non-AVX2 target. A maintainer
  wanting an aarch64 win here would need a separate NEON-specific kernel, following the same
  widen/multiply/narrow shape.
- Only the isolated function-level benchmark was measured, not a full end-to-end `PremultiplyAlpha`
  or image-decode pipeline - the relative share of total decode/process time this loop accounts
  for in a realistic workload was not profiled in this pass (contrast with candidate 003's HEIC
  color-convert candidate, which did measure end-to-end Amdahl's-law-bounded impact).
