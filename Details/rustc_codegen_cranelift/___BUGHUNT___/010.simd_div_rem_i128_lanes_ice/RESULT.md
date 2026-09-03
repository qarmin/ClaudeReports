# 010. `simd_div`/`simd_rem` on 128-bit SIMD lanes ICE cg_clif through the `src/base.rs` `bug!()` funnel

The candidate was the funnel itself: `src/base.rs:222` turns **every**
`Err(ModuleError::Compilation(..))` from `module.define_function` into `bug!(..)`, so any Cranelift
lowering gap is reported as "the compiler unexpectedly panicked. This is a bug" (with a backtrace
and an `rustc-ice-*.txt` dump) rather than as a diagnostic. Candidates #002 and #006 were two
instances; the question was whether more gaps hide behind it.

One more does: `core::intrinsics::simd::simd_div` / `simd_rem` on a `#[repr(simd)]` vector with
`i128`/`u128` lanes. cg_clif lowers them lane-by-lane straight to Cranelift's
`sdiv`/`udiv`/`srem`/`urem`, and no Cranelift backend has an `I128` lowering for those four
instructions, so the backend ICEs on source that LLVM compiles and runs correctly.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-run the hodow x86 intrinsic suite (`tools/hodow_audit/`) against cg_clif on the **baseline** `x86-64` ISA preset, with `measurement_patches.diff` applied so the build survives the first gap, and grep the build log for `Error while defining` | 19154-test hodow build, `TARGET_CPU_FLAG=""`, ~5 min | 9 funnel hits, all already-known: 1x `atomic_cas.i128` (candidate #006) and 8x `Unexpected SSA-value type: {f32x8,f64x4,i64x4,f32x16,f64x8,i64x8}` from the `_mm*_stream_*` family (candidate #002). **No new gap on the x86 intrinsic surface.** |
| Static reading of Cranelift 0.135's own `Unsupported` sites plus `isa/x64/lower.isle`, to bound what can reach the funnel on x86_64-linux at all | static | Only three sources are reachable here: a missing ISLE rule for an (instruction, type) pair; `Unexpected SSA-value type` for a vector wider than 128 bits (`isa/x64/inst/mod.rs:1428`); and "Too many return values to fit in registers" (`isa/x64/abi.rs:346`, unreachable on x86_64 SysV, which never puts more than 2 slots in return registers). `CodeTooLarge`/`RegisterMappingError`/`Regalloc` also funnel into the same `bug!()` but were not reachable in any construction tried. |
| Boundary-value probes of the plausible remaining gaps, one file each via `tools/diffcheck.sh` | opt-level 0: 64-bit portable-SIMD (`Simd<i32,2>`) across a `#[inline(never)]` boundary; `__m256` by value across an `extern "C"` and a `#[target_feature(enable="avx")]` `#[inline(never)]` boundary; `f128` SIMD lanes; a 3-lane `#[repr(simd)]` type | All negative for cg_clif: the 64-bit-vector and `__m256`-by-value cases compile and match LLVM; `f128` lanes and 3-lane `simd_add` are rejected by **rustc itself** (`optimized_mir` / `broken MIR ... MCP#838`) on both backends, so they cannot be attributed to cg_clif. |
| **Generated sweep** over the whole generic-SIMD type matrix - new script `tools/simd_type_sweep.py` | 4396 single-file programs = 66 `core::intrinsics::simd::*` intrinsics x 13 lane types (`i8..i128`, `u8..u128`, `f16`, `f32`, `f64`) x lane counts {1,2,3,4,8,16,32}, compiled with `dist/rustc-clif --emit=obj -Copt-level=0`, 24-way parallel, ~4 min | **28 FUNNEL hits (all `src/base.rs:222`)**: `simd_div` and `simd_rem` x `i128`/`u128` x all 7 lane counts, message `Unsupported("should be implemented in ISLE: inst = `sdiv.i128` / `udiv.i128` / `srem.i128` / `urem.i128`")`. Also 7 non-funnel cg_clif ICEs (`simd_cast` `f16`->`f16`, panics at `src/codegen_f16_f128.rs:197`) - a separate bug, left in the new-suspects list. 3794 OK, 112 rejected by rustc, remaining "ICE" hits are rustc frontend ICEs from the generator's own `simd_gather`/`simd_insert_dyn` cases (they fail identically under LLVM). |
| Minimise + differential-test the hit | `tools/diffcheck.sh`, single file, `#[repr(simd)] struct I128x2([i128;2])`, `simd_div`/`simd_rem` behind `#[inline(never)]`, opt-level 0 **and** `-O` | Reproduced both ways: LLVM exits 0 and prints the correct quotients/remainders; cg_clif exits 101 with the ICE. Also reproduces at `-O` (MIR GVN does not const-fold this one). |
| Standalone `repro/` end-to-end via `dist/cargo-clif` | Real cargo crate, opt-level 0 in both profiles, 12 vector shapes (`i128`/`u128` x 1,2,4,8,16,32 lanes) over boundary values, real binaries run and diffed | 3/3 ICE unpatched; 3/3 byte-identical to LLVM patched |

## Status

**Confirmed** - deterministic.

The ICE reproduces 3/3 full `repro/` cycles (fresh `CARGO_TARGET_DIR` each time) through the real
`dist/cargo-clif`, plus 28/28 compile-only sweep cases, plus the minimised single file under both
`diffcheck.sh` opt-level 0 and `-O`. The patched build produces LLVM-identical output 3/3 cycles,
and 72/72 across the (op x lane type x lane count) matrix re-run 3 times.

Machine: Ryzen 9 7900 (Zen 4), rustc nightly-2026-08-24, cg_clif @ 2bc05d45 (Cranelift 0.135),
default target-cpu (Cranelift's baseline `x86-64` preset). The gap is ISA-independent - `sdiv.i128`
has no lowering on any Cranelift backend at any feature level, so `-Ctarget-cpu=native` ICEs too.

**Fully reproducible**: **Yes**. `repro/` is a real cargo crate compiled by the real
`dist/cargo-clif` wrapper; the ICE in `output.txt` is that compiler actually panicking, and the
post-fix lines are the real generated binary actually running and printing values that match plain
rustc/LLVM's on the same source. There is no second participant to fake - the input is Rust source
handed to the real compiler entry point.

## Class

crash (backend ICE on valid Rust that LLVM compiles and runs correctly)

## Root cause

Two independent things, one on top of the other.

1. **The gap.** `src/num.rs`'s `codegen_int_binop` starts with
   `if let Some(res) = crate::codegen_i128::maybe_codegen(fx, bin_op, in_lhs, in_rhs) { return res; }`,
   and `src/codegen_i128.rs` turns `BinOp::Div`/`BinOp::Rem` on a 128-bit type into a
   `__udivti3`/`__divti3`/`__umodti3`/`__modti3` compiler-builtins call - because Cranelift has no
   `I128` lowering for `sdiv`/`udiv`/`srem`/`urem` on any backend. `src/intrinsics/simd.rs`'s
   `simd_add | ... | simd_div | simd_rem | ...` arm never consults that path: inside
   `simd_pair_for_each_lane` it matches only on `lane_ty.kind()` being `ty::Uint(_)`/`ty::Int(_)`
   and emits `fx.bcx.ins().sdiv(x_lane, y_lane)` (and `udiv`/`srem`/`urem`) for **any** integer
   lane width, `u128`/`i128` included. Every other operation in that arm (`iadd`, `imul`, `ishl`,
   `band`, ...) does have `I128` rules in Cranelift's x64 backend, which is why only division and
   remainder fall over.
2. **The funnel.** With no ISLE rule, `machinst/lower.rs:794` returns
   `Err(CodegenError::Unsupported("should be implemented in ISLE: inst = `sdiv.i128` ..."))`, and
   `src/base.rs`'s catch-all `Err(err) => bug!("Error while defining {name}: {err:?} ...")` turns
   that into an ICE: "error: the compiler unexpectedly panicked. This is a bug", a full backtrace,
   ~150 lines of post-optimization Cranelift IR and a `rustc-ice-*.txt` dump asking the user to
   file a rustc bug report. Nothing in the output says which Rust construct is unsupported.

The trigger needs `#![feature(repr_simd, core_intrinsics)]` - `core::simd` cannot reach it, because
portable-simd's `SimdElement` is implemented only for `i8..i64`/`u8..u64`/`isize`/`usize`/`f32`/`f64`
(`library/portable-simd/crates/core_simd/src/masks.rs`), not for 128-bit types. So this is the same
reachability class as candidate #003 (`simd_funnel_shl`): valid nightly Rust that LLVM compiles and
runs correctly, reached through the documented generic-SIMD intrinsics rather than through a stable
API.

## Introduced by

7fdd058c "Emulate some simd intrinsics" (2019-07-27) - the commit that first implemented
`simd_div`/`simd_rem` in cg_clif, as `simd_binop!(fx, intrinsic, udiv|sdiv(x, y) -> ret)` in the
then-monolithic `src/intrinsics.rs`. The per-lane `udiv`/`sdiv`/`urem`/`srem` emission has survived
every refactor since (`c1d699d3` removing the macros, `d89f307e` moving the code into
`src/intrinsics/simd.rs`, `ff5d860e` adding the `f16` wrapper) and is the same code today at
`src/intrinsics/simd.rs:433-440`. Found with `git log -S "simd_div"` and `git log -L 439,439`; no
bisect was needed.

The `bug!()` funnel it surfaces through took its current shape in c8620a3c "Nicer error when
implementation limits are exceeded" (2023-06-20), which replaced a bare
`module.define_function(..).unwrap()` with the `match` that special-cases
`CodegenError::ImplLimitExceeded` and leaves `Err(err) => panic!("Error while defining {name}: ..")`
for everything else; that arm later became `bug!(..)` and grew the Cranelift-IR dump. So the funnel
was never a regression - it is the pre-existing `unwrap()` behaviour, kept deliberately for
everything the author had not yet classified.

## The fix

`patch.diff`, +53/-13 across three files, in two independent halves.

1. **The gap** (`src/intrinsics/simd.rs`, `src/codegen_i128.rs`). The `__udivti3`/`__divti3`/
   `__umodti3`/`__modti3` libcall emission is factored out of `codegen_i128::maybe_codegen` into a
   new `codegen_i128::codegen_div_rem(fx, is_div, is_signed, lhs, rhs)` that works on raw
   `Value`s, so the scalar path and the per-lane SIMD path share exactly one implementation. Four
   new arms in `simd.rs`'s `simd_div`/`simd_rem` match `ty::Uint(UintTy::U128)` / `ty::Int(IntTy::I128)`
   ahead of the generic `ty::Uint(_)`/`ty::Int(_)` arms and call it. The scalar path's behaviour is
   unchanged (same libcall names, same argument order).
2. **The funnel** (`src/base.rs`). A new
   `Err(ModuleError::Compilation(CodegenError::Unsupported(msg)))` arm, placed before the
   catch-all, reports the gap with `dcx.fatal` - the same shape the neighbouring
   `CodegenError::ImplLimitExceeded` arm already used - instead of `bug!()`. The catch-all
   `bug!()` stays for the variants that really are internal errors (`Verifier` already had its
   own arm; `CodeTooLarge`, `RegisterMappingError`, `Regalloc`). This is a **diagnostics** change,
   not a correctness one: a program that hits a remaining gap still fails to compile, it just
   fails with

   ```
   error: the Cranelift codegen backend cannot compile <symbol>: <what is unsupported>
          note: this operation is not yet supported by the Cranelift backend for this target; please report it at https://github.com/rust-lang/rustc_codegen_cranelift/issues
   ```

   instead of a panic, a backtrace and an ICE dump file.

Verified against more than the one triggering input:

| case | before | after |
|---|---|---|
| `repro/` via `dist/cargo-clif`, 12 vector shapes (`i128`/`u128` x 1,2,4,8,16,32 lanes) over `MIN`/`MAX`/`0`/`+-1`/`1<<64`/`1<<100`/`1<<126`/`1<<127` boundary values | ICE (exit 101) | byte-identical to LLVM (see `output.txt`) |
| Sweep matrix re-run with `diffcheck.sh` (run-time output diffed, not just compiled): `simd_div`/`simd_rem` x `i128`/`u128` x lane counts 1,2,4,8,16,32, x3 repeats | ICE | **72/72 identical to LLVM** |
| Same at `-O` instead of opt-level 0 | ICE | identical to LLVM |
| Same with `-Ctarget-cpu=native` | ICE | identical to LLVM |
| Scalar `i128`/`u128` `/` and `%`, 6 numerators x 5 divisors incl. `MIN+1`, `MAX`, `1<<100` (regression check on the refactored `codegen_i128`) | identical to LLVM | still identical to LLVM |
| `_mm256_stream_pd` (candidate #002's gap, deliberately not fixed here) | ICE + backtrace + ICE dump file | one-line `error:` + `note:`, exit 1 |
| Non-power-of-two lane counts (3 lanes) | ICE | compiles, but **excluded from the verification**: the LLVM-built binary is itself nondeterministic run to run on the harness's byte-hash of a 48-byte `#[repr(simd)]` value (`0`, `12874219923451329666`, `0`, `17549655065593273789`, `0` on five consecutive runs), i.e. the harness reads padding. Not a backend difference. |

- Project test suite (`./y.sh test` in the isolated worktree, in-tree items + the `download/`
  repos): **before the patch: exit 0. After the patch: exit 0.** Both runs emit exactly 61
  `test result: ok.` lines, 0 `test result: FAILED`, and the multiset of pass counts is identical
  between the two runs (largest: 2798, 1400, 1222 x5, 1200, 1020, 924 x5).

## What's not done / open questions

- The funnel half only *renames* the failure for the gaps that remain. Candidate #002's
  `Unexpected SSA-value type: f64x4` and candidate #006's `atomic_cas.i128` still do not compile
  without those candidates' own patches; they just no longer claim to be a compiler bug.
- The diagnostic has no `Span`. `compile_fn` only has the symbol name, so the error names a mangled
  symbol and points at no source location. Threading a span down to `define_function` (or looking
  one up from the `FuncId`) would be a much better diagnostic and was not attempted.
- The new `dcx.fatal` aborts the whole compilation, like the `ImplLimitExceeded` arm next to it. An
  alternative (used by `src/intrinsics/llvm_x86.rs` for unsupported `llvm.x86.*`) is to warn and
  replace the body with a run-time trap, which lets the rest of the crate build; that is a
  maintainer policy call, not a mechanical one, so it was not done.
- Non-x86_64 targets were not exercised (no cross toolchain this session). The claim that
  `sdiv.i128` has no lowering anywhere is from reading Cranelift 0.135's `isa/*/lower.isle`, not
  from a run - but the fix routes 128-bit lanes to a libcall on every target, so it cannot regress
  a backend that does have the lowering, only forgo it.
- `simd_cast`/`simd_as` from `f16` to `f16` (and `f128` to `f128`) is a **second, distinct** cg_clif
  ICE the sweep turned up - `clif_int_or_float_cast`'s guard is `matches!(from_ty, F16 | F128) ||
  (matches!(to_ty, F16 | F128) && from_ty != to_ty)`, so an identity `f16` cast is routed into
  `codegen_f16_f128::codegen_cast`, whose `(F16, F16)` case is `unreachable!`. That is a panic in
  cg_clif's own code, not the `base.rs` funnel, so it was left for its own candidate.
- The sweep only covers the *generic* `core::intrinsics::simd::*` surface. The `llvm.x86.*` surface
  was covered by the hodow re-run, but `llvm.aarch64.*`, inline asm and the ABI/`PassMode` surface
  were only spot-probed.
