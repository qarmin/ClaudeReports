# neon_arch_dynamic_verification

`src/arch/neon.rs`'s unsafe NEON IDCT/color-convert kernels (`idct8`, `transpose8`,
`dequantize_and_idct_block_8x8`, `color_convert_line_ycbcr`) were only proven safe by manual
invariant proof in candidate 013.unsafe_blocks_triage_sweep - miri cannot interpret the aarch64
NEON intrinsics this crate uses (`vqrdmulhq_n_s16` lowers to `sqrdmulh`, which hits "unsupported
operation: can't call LLVM intrinsic" under miri even when cross-targeting
`aarch64-unknown-linux-gnu`), and that session had no aarch64 cross-linker or qemu setup available
to actually execute this code path for real. This candidate closes that specific gap: getting a
real, dynamic execution of the NEON kernels (not miri interpretation, not manual proof), on an
environment that this time has both a working cross-build path and `qemu-aarch64`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-check aarch64 cross-linker/qemu availability (the exact blocker candidate 013 hit) | `which qemu-aarch64 aarch64-linux-gnu-gcc`, `apt-get install -s gcc-aarch64-linux-gnu` (no sudo available), `rustup target add aarch64-unknown-linux-gnu` | `qemu-aarch64` is present. No aarch64 cross-gcc/linker package is installable (no root, and the package isn't in this environment's apt sources) - same blocker as 013 for the `-gnu` target specifically. |
| Self-contained cross build via `aarch64-unknown-linux-musl` + rustc's own `rust-lld` (bundled musl CRT objects/`libc.a` ship with the `rust-std` component - no external linker needed at all) | `rustup target add aarch64-unknown-linux-musl && rustup component add llvm-tools-preview`; `RUSTFLAGS="-C link-self-contained=yes -C linker-flavor=ld.lld -C linker=rust-lld" cargo +nightly build --release --target aarch64-unknown-linux-musl` | Produces a real statically-linked aarch64 ELF (`file`: "ELF 64-bit LSB executable, ARM aarch64 ... statically linked") - this is the missing piece from candidate 013, and works with zero extra system packages. |
| Static verification the NEON code is actually compiled in and reachable (not dead-code-eliminated) | `nm --demangle=rust -S` + `llvm-objdump -d` (from `rustup component add llvm-tools-preview`, itself cross-arch-capable) on the produced aarch64 binary | `jpeg_decoder::arch::neon::dequantize_and_idct_block_8x8` has its own retained symbol containing real `sqrdmulh`/`sqadd`/`sqsub` instructions (matching `vqrdmulhq_n_s16`/`vqaddq_s16`/`vqsubq_s16` in `idct8`); `color_convert_line_ycbcr`'s NEON body (inlined into `decoder::color_convert_line_ycbcr`) contains real `ushll`/`sqrdmulh`/`sqshrun`/`st3` (matching `vshll_n_u8`/`vqrdmulhq_n_s16`/`vqshrun_n_s16`/`vst3_u8`) - see `output.txt`. |
| Real dynamic execution under `qemu-aarch64` (genuine aarch64 NEON instruction emulation, not source-level interpretation) of a standalone harness decoding 36 real fixtures (33 from `tests/reftest/images/` - including all boundary widths mod 8 from candidate 013's own miri test, both grayscale/RGB/CMYK/progressive/restart-marker/16-bit-quant-table images - plus 3 `Decoder::scale()`-scaled variants of `rgb.jpg` at 250x167/125x84/63x42, which exercise a different `output_linestride`/block-offset arithmetic path through the same `assert!`-guarded kernels), comparing every decoded pixel against the project's own PNG references with the exact tolerance rule `tests/reftest/mod.rs`'s `reftest_decoder` uses | `cargo +nightly build --release --target aarch64-unknown-linux-musl` (rayon feature enabled - default features, so the multithreaded `worker/rayon.rs` call path into the NEON kernels is exercised too, not just the single-threaded `ImmediateWorker` path), then `qemu-aarch64 <binary>`, run 3 times | 36/36 fixtures decoded byte-correctly on all 3 runs, deterministic (identical output/allocation count every run) |
| Real dynamic memory-safety instrumentation standing in for ASan/miri (neither is available for aarch64 in this environment - no aarch64 ASan runtime ships with any installed toolchain, and miri cannot interpret the intrinsics at all): a custom `#[global_allocator]` that serves every heap allocation from its own `mmap`+`mprotect` region flanked by two `PROT_NONE` guard pages, placing the user's data flush against the trailing guard page so a single-byte heap overflow write (the exact failure mode an unsound `assert!` bound in `dequantize_and_idct_block_8x8`/`color_convert_line_ycbcr` would produce) becomes an immediate `SIGSEGV` under `qemu-aarch64`'s real page-fault handling | Same 3 runs as above, ~14,100 heap allocations served by the guard-page allocator each run | Zero `SIGSEGV`s across all 3 runs - no out-of-bounds heap write was ever caught |
| Native x86_64 sanity check of the harness itself (same guard-page allocator, same fixtures, scalar/SSSE3 code path instead of NEON) | `cargo +stable build --release` (no cross target), run natively | 36/36 fixtures OK, guard-page allocator active with 0 SIGSEGVs - confirms the harness and comparison logic themselves are correct (not silently vacuous) before trusting the aarch64 result |
| Project's existing test suite, native x86_64, unrelated baseline health check (no source changes were made this candidate) | `cargo +stable test --release` in the checked-out worktree, toolchain `+stable` (the pinned `1.61.0` can't build the current `Cargo.lock`'s `crc32fast`/`wasm-bindgen` deps) | 23/23 tests pass (5 lib unit tests, 12 `reftest`, 4 rayon multithreading tests, 2 doc-tests) |

## Status

**Not reproducible.** No memory-safety violation or incorrect-output bug was found in
`src/arch/neon.rs`'s NEON kernels after real aarch64 execution (not miri, not manual proof):
cross-compiled statically for `aarch64-unknown-linux-musl` using rustc's own self-contained
`rust-lld` (no external cross-gcc needed - this is the environment gap candidate 013 explicitly
could not close), confirmed via `nm`/`llvm-objdump` that the actual NEON intrinsics are compiled in
and reachable (not eliminated by inlining/dead-code elimination), then run under `qemu-aarch64`
(genuine aarch64 NEON instruction emulation) 3 times with a guard-page heap allocator active. All
36 fixtures (spanning every residue mod 8 in width/height, grayscale/RGB/CMYK/progressive/
restart-marker/16-bit-quantization-table images, plus 3 `Decoder::scale()`-scaled variants
exercising a different linestride/offset arithmetic path) decoded byte-correctly against the
project's own PNG references on all 3 runs, and the guard-page allocator (~14,100 allocations
served per run) never caught a single out-of-bounds heap write. This corroborates candidate 013's
manual invariant proof with real dynamic evidence for the one arch that session could not execute.

**Fully reproducible**: N/A (not applicable to a Not-reproducible result - "did not reproduce" was
itself reproduced identically on 3/3 runs, which is the relevant repeatability claim here).

## Class

N/A (not applicable to a Not-reproducible result).

## Root cause

Not applicable - no bug was found. The reason the manual proof holds up dynamically: exactly as
candidate 013 found by hand, `dequantize_and_idct_block_8x8` and `color_convert_line_ycbcr` both
validate their own preconditions with an `assert!` immediately before any pointer-based unchecked
access (`vld1q_s16`/`vst1_u8`/`vst3_u8`), and the two real callers (`src/worker/rayon.rs` and
`src/worker/immediate.rs`) size their buffers to satisfy that `assert!` exactly (tight fit in the
worst case, not merely "enough with margin"). Real aarch64 execution under `qemu-aarch64` with a
guard-page allocator is exactly the kind of dynamic check that would have caught a gap between
"the assert looks sufficient on paper" and "the actual generated NEON code writes past what the
assert allows" (a compiler codegen bug, a miscounted vector width, an off-by-one in the loop bound)
- none was found.

## Introduced by

N/A - no bug was confirmed.

## The fix

No fix - nothing to fix, per Not-reproducible status. No source changes were made to `src/` this
candidate; the worktree used for investigation was discarded unmodified.

- Project test suite: N/A for a before/after fix comparison (no fix was attempted). For reference,
  the existing suite was run once as a baseline health check: 23/23 passing, native x86_64,
  `+stable` toolchain (unrelated to the aarch64/NEON investigation, since the project's existing
  test suite has no aarch64/qemu-based test of its own to run).

## What's not done / open questions

- **wasm32 (`src/arch/wasm.rs`) still has the same gap candidate 013 left open**: compile-checked
  only (`simd128` forced), never dynamically executed under any interpreter/runtime. This
  candidate did not attempt to close that gap (out of scope - the candidate was specifically about
  NEON) - a future session with `wasm-bindgen-test`/`wasmtime`/a browser-driven runner wired up
  could close it the same way this session closed the NEON one.
- The guard-page allocator catches heap-buffer overflows/underflows via `SIGSEGV` but does not
  catch every conceivable memory-safety issue a full ASan/miri run would (e.g. a use-after-free
  where the freed page hasn't been reused yet and no guard page is crossed, or a data race - not
  relevant here since none of the 4 NEON functions touch shared mutable state). It is a
  purpose-built stand-in for the two tools that are genuinely unavailable for aarch64 in this
  environment (no aarch64 ASan runtime ships with any installed Rust toolchain; miri cannot
  interpret these intrinsics at all), not a full replacement for either.
- This candidate's fixture set (36 real images) is broad but not exhaustive - it reuses candidate
  013's own boundary-width selection (every residue mod 8, widths 1-33) plus grayscale/CMYK/
  progressive/restart/16-bit-quant-table/scaled-decode coverage, but does not include a dedicated
  fuzz campaign against the NEON path specifically. Given the guard-page allocator technique proven
  here now works end-to-end for aarch64, a longer fuzz-style campaign (many more/larger images,
  more `scale()` target sizes) would be straightforward to run with the same harness if a future
  session wants deeper coverage - this session's 36-fixture, 3-repeat-run pass is not exhaustive
  proof, just real dynamic evidence where previously there was none.
- The self-contained musl cross-build + `qemu-aarch64` technique demonstrated here (no external
  aarch64 cross-gcc, no root/sudo required) is itself worth remembering for any future
  aarch64-targeted candidate in this project - recorded in `projects/jpeg-decoder.md`.
