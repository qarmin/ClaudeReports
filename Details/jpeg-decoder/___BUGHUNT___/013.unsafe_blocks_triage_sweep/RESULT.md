# unsafe_blocks_triage_sweep

Not a single-suspect candidate: this sweeps and triages every `unsafe` block/fn in `src/` (23
`unsafe` keyword occurrences by raw grep, per the risk-map bullet), maps each to its location and
purpose (bounds elision, SIMD, transmute, FFI), checks whether the safety invariants each relies on
are actually upheld by callers, and specifically checks whether any were introduced/modified by a
`___OPTIMIZATIONS___/` perf-optimize candidate.

## Enumeration (23 raw "unsafe" matches -> 14 real unsafe items + 9 non-code occurrences)

Of the 23 raw matches, 9 are not actual unsafe code: `#![deny(unsafe_code)]` / `#![forbid(...)]` in
`src/lib.rs` (2), `#![allow(unsafe_code)]` / `#[allow(unsafe_code)]` annotations in `src/arch/mod.rs`
(3) and `src/decoder.rs`/`src/idct.rs` (1 each = 2), and the `Option<unsafe fn(...)>` function-pointer
*type* signatures in `src/arch/mod.rs` (2, declaring the type of a pointer, not unsafe code itself).

The remaining 14 are the real unsafe items:

| # | Location | Purpose | Guarded by | Perf-optimize touched? |
|---|---|---|---|---|
| 1 | `src/decoder.rs:1414` (`unsafe { ycbcr(y, cb, cr, output) }`) | Calling an `unsafe fn` pointer returned by `arch::get_color_convert_line_ycbcr()` | Callee (see #6/#10) self-validates via `assert!` before any unchecked access - see below | No |
| 2 | `src/idct.rs:250` (`unsafe { return idct(...) }`) | Calling an `unsafe fn` pointer returned by `arch::get_dequantize_and_idct_block_8x8()` | Callee (see #5/#9) self-validates via `assert!` | No |
| 3 | `src/arch/ssse3.rs:8` `unsafe fn idct8` | SIMD (SSSE3 intrinsics), private column-IDCT kernel, fixed-size `&mut [__m128i; 8]` in/out | Fixed-size array parameter, no pointer arithmetic | No |
| 4 | `src/arch/ssse3.rs:88` `unsafe fn transpose8` | SIMD (SSSE3), private transpose kernel, fixed-size array | Fixed-size array parameter, no pointer arithmetic | No |
| 5 | `src/arch/ssse3.rs:124` `pub unsafe fn dequantize_and_idct_block_8x8` | SIMD, `_mm_loadu_si128`/`_mm_storeu_si128`/`copy_nonoverlapping` on raw pointers derived from slices | Internal `assert!(output.len() > output_linestride.checked_mul(7).unwrap().checked_add(7).unwrap())`; reads are over the fixed `[i16;64]`/`[u16;64]` params (exact size match) | No |
| 6 | `src/arch/ssse3.rs:196` `pub unsafe fn color_convert_line_ycbcr` | SIMD, `_mm_loadu_si128`/`copy_nonoverlapping` on raw pointers | Internal `assert!(output.len()%3==0)` + `assert!(num<=y.len())`/`cb`/`cr`; `num_vecs=(num/8).saturating_sub(1)` deliberately leaves a one-vector border because the 128-bit load reads 16 bytes while only 8 are consumed | No |
| 7 | `src/arch/neon.rs:6` `unsafe fn idct8` | SIMD (NEON), private, fixed-size array | Fixed-size array parameter | No |
| 8 | `src/arch/neon.rs:86` `unsafe fn transpose8` | SIMD (NEON), private, fixed-size array | Fixed-size array parameter | No |
| 9 | `src/arch/neon.rs:113` `pub unsafe fn dequantize_and_idct_block_8x8` | SIMD, `vld1q_s16`/`vst1_u8` on raw pointers | Same `assert!` pattern as #5 | No |
| 10 | `src/arch/neon.rs:171` `pub unsafe fn color_convert_line_ycbcr` | SIMD, `vld1_u8`/`vst3_u8` on raw pointers | Same `assert!` pattern as #6, but only 8-byte loads (`vld1_u8`), so no extra border vector is needed (and none is taken - `num_vecs = num/8` exactly) | No |
| 11-14 | `src/arch/wasm.rs:146,177,209,270` (4x `unsafe { }` blocks inside otherwise-safe `fn`s) | SIMD (wasm32 `simd128`), `v128_load`/`v128_store`/`v128_store64_lane` on raw pointers | Same `assert!` patterns as #5/#6, with inline `// SAFETY:` comments at 175-176 and 268-269 explicitly citing the asserts | No |

**No `unsafe` block anywhere in `src/` is a `transmute`, and none crosses an FFI boundary** - every
one is either (a) calling a SIMD intrinsic on a fixed-size array (`&mut [T; 8]`, no bounds question
at all), (b) calling a SIMD intrinsic that loads/stores through a raw pointer derived
(`.as_ptr()`/`.as_mut_ptr()` + `.wrapping_add(...)`) from a slice, guarded by a preceding `assert!`
in the same function, or (c) invoking one of those `unsafe fn`s through a function pointer obtained
from a small dispatch table (`arch::get_*`) that already runtime-gates on `is_x86_feature_detected!`
(x86/x86_64) or is unconditionally safe to call on `aarch64` (NEON is baseline on that ISA - the
comment at `arch/mod.rs:22` says as much, which matches the ARM architecture reference).

**None of the 14 real unsafe items were introduced or modified by any of the 17 perf-optimize
candidates in `___OPTIMIZATIONS___/`** - confirmed by grepping every `patch.diff` in that directory
for the string "unsafe" (zero matches; see `output.txt`'s enumeration section). Candidate
010.idct_simd_and_scaled_decode (the one perf-optimize candidate that specifically profiled the
SIMD IDCT dispatch) only ever measured the existing `arch::get_dequantize_and_idct_block_8x8()`
call path, never patched inside it - its own `patch.diff` doesn't exist (its `RESULT.md` records a
non-applied finding), consistent with the zero-match grep.

## Invariant check: are the `assert!`-based guards actually sufficient (not just present)?

For each of #5/#6/#9/#10/#11-14, worked the arithmetic by hand against the actual call sites:

- **IDCT (`dequantize_and_idct_block_8x8`, all 3 arches)**: the loop writes
  `[output_linestride*i, output_linestride*i+8)` for `i` in `0..8`; the guard
  `output.len() > output_linestride*7+7` is exactly the necessary-and-sufficient condition (checked
  `.checked_mul`/`.checked_add` so it can't silently wrap on a hostile `output_linestride`, it panics
  instead). Traced the two real callers: `src/worker/rayon.rs` (`Scoped::append_row_locked`) always
  passes a fixed local `[u8; 64]` buffer with `output_linestride = 8` (trivially satisfies the guard
  with 0 slack), and `src/worker/immediate.rs` (`ImmediateWorker::append_row_immediate`) passes a
  suffix of a `Vec<u8>` sized `block_size.width * block_size.height * dct_scale * dct_scale`; working
  through the offset arithmetic for the worst case (the last block of the last MCU row in a batch)
  shows the remaining buffer length is *exactly* `output_linestride*7+8` at minimum (not just
  "enough" - the buffer sizing and the block-offset formula are tight against each other, not
  independently-verified-safe-by-margin), which is the precise boundary the `assert!` requires.
- **Color convert (`color_convert_line_ycbcr`, all 3 arches)**: verified the read/write bounds
  implied by each arch's own vector width against its own `assert!`s (SSSE3's extra `saturating_sub(1)`
  border vector for its 16-byte-load-but-8-used pattern; NEON's tighter 8-byte loads needing no
  border; wasm's `v128_load64_zero`/`v128_store`+`v128_store64_lane` pair) - all three hold with
  either exact-fit or intentional slack, never under-covered.
- The caller in `src/decoder.rs:1406` (`color_convert_line_ycbcr`, the safe wrapper) passes
  potentially-oversized `y`/`cb`/`cr` row buffers (upsampler line-scratch buffers, per candidate
  006's finding about `color_no_convert`) straight through to the arch fn with no pre-check - but
  since the arch fn's own `assert!`s are the actual necessary-and-sufficient condition (not merely a
  sanity check), this is safe regardless of what the caller passes: a caller violating the invariant
  gets a panic (a `crash`-class bug, not memory-unsafety), not UB.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual invariant proof (all 14 unsafe items, arithmetic worked by hand against every real call site in `src/worker/{rayon,immediate}.rs` and `src/decoder.rs`) | N/A (pen-and-paper + code reading) | Every `assert!` guard is the exact necessary-and-sufficient bound for the unchecked access(es) that follow it in the same function; no gap found between what's asserted and what's accessed |
| `miri`, `src/idct.rs`'s existing unit tests, default config (no target-feature forced - exercises the scalar fallback since `is_x86_feature_detected!` returns `false` under default miri) | `nightly-2026-08-21`, `MIRIFLAGS=-Zmiri-disable-isolation`, `cargo miri test --lib idct` | 4/4 tests pass, no UB reported |
| `miri`, same tests, `RUSTFLAGS="-C target-feature=+ssse3"` (confirmed via a throwaway crate that this actually flips `is_x86_feature_detected!("ssse3")` to `true` under miri, so this run genuinely exercises `arch::ssse3::*`'s unsafe blocks, not the scalar fallback) | same miri, ssse3 forced | 4/4 tests pass, no UB reported |
| `miri`, new integration test `tests/miri_simd_boundary.rs` decoding the real `tests/reftest/images/mozilla/jpg-size-{1,2,3,4,5,7,8,9,15,16,17,31,32,33}x{same}.jpg` fixtures (real, pre-existing JPEG files from the project's own test corpus - not hand-authored bytes; `6x6` skipped, it's a mis-extensioned PNG per the project's own `disabled.list`) end-to-end through the public `Decoder` API, ssse3 forced, `--no-default-features` (rayon threading disabled - see note below) | same miri config | All 14 boundary widths (spanning every residue mod 8, including the exact multiples 8/16/32) decode without any miri-reported UB |
| Real `cargo test --lib --tests` on native x86_64 host hardware (confirmed via `/proc/cpuinfo` to have `ssse3`/`sse4_1`/`sse4_2`, so `is_x86_feature_detected!` is `true` for real, no forcing needed), toolchain `+1.87.0` (the pinned `1.61.0` can't build the current `Cargo.lock`'s `wasm-bindgen` dep) | full existing suite: unit tests, `reftest`/`reftest_scaled` (49-image corpus incl. subsampled/grayscale/CMYK/progressive/restart-marker JPEGs, byte-exact vs. PNG reference), `crashtest` (malformed-input fuzz corpus), 4x `rayon-*` multithreaded-decode tests | 22/22 tests pass |
| `miri` cross-interpreting `--target aarch64-unknown-linux-gnu` with `--features nightly_aarch64_neon`, to reach the NEON unsafe items (#7-10) that native x86_64 can't otherwise exercise | `cargo +nightly miri test --target aarch64-unknown-linux-gnu --lib idct --features nightly_aarch64_neon` | Miri itself doesn't support this: `unsupported operation: can't call LLVM intrinsic 'llvm.aarch64.neon.sqrdmulh.v8i16' on architecture 'aarch64'` - a tool limitation (miri's own NEON intrinsic coverage gap), not a code finding |
| Fallback for the NEON path: `qemu-aarch64` (real emulated execution instead of miri interpretation) | checked availability | `qemu-aarch64` is installed, but no aarch64 cross-linker (`aarch64-linux-gnu-gcc`/`-cc`/musl variant) is available in this environment to produce a runnable cross-compiled test binary in the first place - couldn't attempt this fallback this session |
| wasm32 unsafe items (#11-14): compile-time check only, `RUSTFLAGS="-C target-feature=+simd128" cargo build --target wasm32-unknown-unknown` | build only, no execution (no `wasmtime`/`wasm-pack`/browser-driven runner set up in this session; `node` is present but wiring up `wasm-bindgen-test-runner` was out of scope for this candidate's time budget) | Compiles cleanly with `simd128` forced; not executed, so this is compile-time confirmation only - the same manual invariant proof above (see "Invariant check") is what actually covers this arch's correctness reasoning |

Note on `--no-default-features` for the miri boundary-width test: the default `rayon` feature
spawns real OS threads for the worker pool, and miri's interpretation of `crossbeam-epoch`'s
thread-parking path triggered an unrelated miri/threading incompatibility unconnected to any of the
14 unsafe items being triaged here (rayon's own worker-pool internals, not `src/arch/*` or the two
call sites in `decoder.rs`/`idct.rs`) - disabling `rayon` avoids that noise and still exercises every
one of the 14 targeted unsafe items via the single-threaded `ImmediateWorker` path.

## Status

**Not reproducible.** No safety-invariant violation was found across any of the 14 real `unsafe`
items in `src/` after: a hand-worked arithmetic proof that every `assert!` guard is exactly the
necessary-and-sufficient bound for the unchecked access(es) it protects (not merely "probably
enough"); a miri run (with `target-feature=+ssse3` forced so the real SIMD path executes, not the
scalar fallback) across both the existing unit tests and a new boundary-width integration test
spanning every residue mod 8 from width 1 to 33 using real pre-existing JPEG fixtures; and the
project's full existing test suite (49-image reftest corpus, crashtest fuzz corpus, 4 rayon
multithreading tests) passing on real ssse3-capable hardware. The NEON (#7-10) and wasm32 (#11-14)
arches could not be dynamically exercised this session (miri lacks the relevant NEON intrinsic;
no aarch64 cross-linker was available for a qemu fallback; no wasm runtime was wired up) - see "What
was tried" and "What's not done" - but they were covered by the same manual invariant proof, which
does not depend on which arch executes it (the guard arithmetic and the SIMD vector-width math is
architecture-specific but was checked per-arch, not just for SSSE3).

**Fully reproducible**: N/A (not applicable to a Not-reproducible result).

## Class

N/A (not applicable to a Not-reproducible result).

## Root cause

Not applicable - no invariant violation was found. The actual reason every real unsafe item held
up: each SIMD-touching `unsafe fn` (#5/#6/#9/#10/#11-14) validates its own preconditions with an
`assert!` immediately before doing any pointer-based unchecked access, so a caller that violates the
precondition gets a panic (a `crash`-class failure mode, not UB) rather than an actual out-of-bounds
access - and the two indirect-call sites (#1/#2) simply invoke one of those self-validating
functions through a pointer, adding no additional risk. The 6 pure-SIMD-kernel `unsafe fn`s
(#3/#4/#7/#8, `idct8`/`transpose8` in both ssse3.rs and neon.rs) operate purely on fixed-size
`&mut [T; 8]` arrays with no pointer arithmetic at all, so there's no bounds question to check in
the first place.

## Introduced by

N/A - no bug was confirmed.

## The fix

No fix - nothing to fix, per Not-reproducible status.

- Project test suite: 22/22 passing before this candidate's investigation (no source changes were
  made to `src/` - this was a read-only triage/audit; a throwaway integration test was added at
  `tests/miri_simd_boundary.rs` for the investigation itself, not committed to `src/`, and the
  worktree is discarded at the end of this candidate). Not applicable to compare "before/after" since
  no fix was attempted.

## What's not done / open questions

- The NEON path (#7-10) was never dynamically executed this session (only proved correct by hand
  and confirmed to at least compile under `--features nightly_aarch64_neon`) - a future session with
  access to real aarch64 hardware, or an aarch64 cross-linker for a qemu-based run, could close this
  gap. Miri's own lack of `sqrdmulh`/related NEON intrinsic support is a tool limitation worth
  remembering for future aarch64-targeted candidates in this project (see `projects/jpeg-decoder.md`
  update from this session).
- The wasm32 path (#11-14) was only compile-checked (`simd128` forced), never executed under any
  interpreter/runtime - a `wasm-bindgen-test`-based run (the project already depends on
  `wasm-bindgen-test` as a dev-dependency, suggesting there's already a wasm test setup somewhere in
  CI not reproduced locally this session) would close this gap.
- This candidate's boundary-width miri integration test only covers the `color_convert_line_ycbcr`
  path end-to-end (RGB JPEGs); the IDCT `assert!` bound was separately verified by the existing
  `idct::test_dequantize_and_idct_block_8x8*` unit tests (also run under miri+ssse3) plus the hand
  worked arithmetic against both real call sites, but wasn't cross-checked with a grayscale-only or
  4:2:0-chroma-subsampled image under miri specifically (the real hardware `cargo test` run does
  cover grayscale/subsampled images via the full `reftest` corpus, just not under miri's UB
  detection).
- This was a breadth-first triage sweep across all unsafe code, not a deep dig on any single one of
  the 14 items the way a normal single-suspect candidate would be - if a future session wants to
  push harder on one specific item (e.g. a dedicated multi-hour miri fuzz campaign feeding
  the color-convert boundary logic pseudo-random widths, once the NEON/wasm tooling gaps above are
  closed), this triage is the map to start from, not a substitute for that deeper dig.
