# 012. per-function `#[target_feature]`: an `extern "C"` fn passing a >128-bit SIMD vector by value ICEs cg_clif

Candidate 006's "what's not done" section noted that cg_clif never consults per-function
`#[target_feature(enable = ...)]` and that anything LLVM only compiles because of it should be
expected to fail here. This candidate went looking for such a program. The one it found is an
`extern "C"` (or `extern "sysv64"`) function that takes or returns a 256-/512-bit SIMD vector **by
value**: rustc's `abi_required_features` check only accepts that when `avx`/`avx512f` is enabled,
and on cg_clif a per-function `#[target_feature]` is the *only* channel that can enable it - after
which the backend ICEs at `src/base.rs:222` with
`Compilation(Unsupported("Unexpected SSA-value type: f32x8"))`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the backend for every place per-function target features could matter | `src/lib.rs` `target_config`/`build_isa`, `src/inline_asm.rs`, `git grep target_feature` | `codegen_fn_attrs(..).target_features` is read in exactly one file, `src/inline_asm.rs` (register allocation via `tcx.asm_target_features`, and the aarch64 `.arch_extension` lines). Everything else compiles against a single `TargetIsa` built from `-C target-cpu` alone, and `target_config` hardcodes the globally-enabled feature set to `fxsr/sse/sse2/x87` on x86_64. So the attribute reaches the backend only indirectly - through the `FnAbi` rustc computes for a function whose features it considers enabled. |
| Differential test vs LLVM, single file (`tools/diffcheck.sh`) | opt-level 0, hand-written `#[target_feature(enable = "avx")]` programs | First attempt (attribute on the callee only) failed *identically* on both sides: rustc rejects the call site with "this function call uses SIMD vector type `__m256` which ... requires the `avx` target feature, which is not enabled in the caller". Putting the attribute on the caller too made LLVM compile and print `[1.5, 2.5, ...]` while cg_clif ICEd. **Reproduced.** |
| Boundary-value construction around the suspect operation | opt-level 0: Rust ABI vs `extern "C"` vs `extern "sysv64"`; `__m128`/`__m128i` (128-bit) vs `__m256`/`__m256i`/`__m256d` vs `__m512`/`__m512i`/`__m512d`; argument-only, return-only, both; bare vector vs `#[repr(C)]` struct wrapper; direct call vs `extern "C"` fn pointer; vector mixed with `i32`/`f64` arguments | The trigger is exactly "the ABI asks for a vector wider than 128 bits in a register". 128-bit `extern "C"` works. Rust-ABI 256-bit works (rustc passes anything over 16 bytes indirectly there). Everything 256-bit-or-wider through an explicit `extern` ABI ICEs, in argument position, return position, or both, and whether the vector is bare (`PassMode::Direct`) or inside a `#[repr(C)]` struct (`PassMode::Cast`). |
| Alternative channels for the feature, to check the `#[target_feature]` link | `-C target-feature=+avx`, `-C target-cpu=native`, `-C target-cpu=x86-64-v3`, with the attribute removed | All three are a *hard error* under cg_clif ("...requires the `avx` target feature, which is not enabled") while LLVM compiles and runs, because rustc's ABI check reads `sess.internal_target_features`, which `target_config` hardcodes. This confirms per-function `#[target_feature]` is the only way to reach the ICE on this backend. |
| Generated sweep, one case per file (`tools/tf_abi_sweep.sh`, written for this candidate) | 18 cases: 6 vector types x {bare, `#[repr(C)]` struct} + 3 other shapes (`extern "sysv64"`, fn pointer, mixed scalar/vector args) + 3 controls; opt-level 0; run against both the unpatched main tree and the patched worktree | Unpatched: **15 ICE**, 3 controls match LLVM. Six distinct offending SSA types (`f32x8`, `f64x4`, `i64x4`, `f32x16`, `f64x8`, `i64x8`) plus `i8x32`/`i8x64` from the `PassMode::Cast` path. Patched: 0 ICE, 15 clean `error:` diagnostics, 3 controls still match LLVM. One case per file is required because cg_clif aborts the whole crate on the first CGU it cannot define. |
| Optimization level / ISA matrix | opt-level 0, `-O`, `-Ctarget-cpu=native`, all with the attribute present | ICEs in all three. Unlike candidate 006's case this is not ISA-flag dependent - Cranelift has no >128-bit vector type on any ISA. |
| A 2048-bit `#[repr(simd)] struct V([f32; 64])` through the same path | opt-level 0 | Ruled out as a cg_clif case: **rustc itself** ICEs first (`broken MIR ... rustc_mir_transform/src/validate.rs:82`), identically under both compilers. Not counted here. |
| Standalone `repro/` end-to-end via `dist/cargo-clif` | Real cargo crate, two binaries, opt-level 0, real binaries run and diffed against the LLVM-built ones; 4 runs unpatched, 2 patched | Unpatched: 4/4 the `control` binary matches LLVM byte-for-byte and the `wide` binary ICEs. Patched: `control` still matches, `wide` is a plain `error:` with no ICE. |
| Project test suite | `./y.sh test` in the isolated worktree, before and after the patch | Both exit 0 - see "The fix". |
| Full `just test` cycle from a pristine clone | `just repo_url=https://github.com/rust-lang/rustc_codegen_cranelift.git ...` (this sandbox has no GitHub SSH key, so the documented https fallback was used): clone @ 2bc05d45 -> `./y.sh build` -> `reproduce-before` -> `git apply patch.diff` -> `./y.sh build` -> `reproduce-after` | before: `wide: ICE`, `RESULT: BUG PRESENT` (exit 1); after: `wide: CLEAN ERROR`, `RESULT: no ICE` (exit 0), with `control` matching LLVM in both halves |

## Status

**Confirmed** - deterministic compile-time failure, not timing- or layout-dependent.

- `repro/run.sh` through the real `dist/cargo-clif`: **4/4 runs** ICE on the `wide` binary against
  the unpatched `dist/` of cg_clif @ 2bc05d45 (with `rm -rf repro/target-clif` before each run so
  nothing is served from cargo's cache), and 4/4 runs the `control` binary matched LLVM.
- `tools/tf_abi_sweep.sh`: **15/15 cases ICE, twice** against the unpatched backend; 0/15 ICE
  against the patched worktree, twice.
- Machine: Ryzen 9 7900 (Zen 4, AVX-512), rustc nightly-2026-08-24, cg_clif @ 2bc05d45
  (Cranelift 0.135), opt-level 0. Also reproduced at `-O` and with `-Ctarget-cpu=native`.

**Fully reproducible**: **Yes**. `repro/` is a real cargo crate compiled by the real
`dist/cargo-clif` wrapper; the ICE in `output.txt` is that compiler actually panicking, and the
`control` lines are real generated binaries actually running and printing values identical to plain
rustc/LLVM's on the same source. There is no second participant to fake - the input is Rust source
handed to the real compiler entry point.

## Class

crash (backend ICE on valid Rust that LLVM compiles and runs correctly)

## Root cause

Three things line up:

1. **cg_clif never sees per-function target features, but rustc's ABI check does.** cg_clif
   compiles every function against one `TargetIsa` built in `src/lib.rs`'s `build_isa` from
   `-C target-cpu` alone, and its `target_config` reports a hardcoded
   `fxsr/sse/sse2/x87` as the globally-enabled feature set on x86_64 (a pre-existing FIXME).
   `codegen_fn_attrs(..).target_features` is consulted nowhere except `src/inline_asm.rs`.
   Meanwhile rustc computes `FnAbi` and runs its `abi_required_features` check against
   *the function's own* features plus the global set. So an `extern "C" fn(__m256) -> __m256` is
   accepted only when the definition and the caller both carry
   `#[target_feature(enable = "avx")]` - and that is precisely the one channel cg_clif cannot
   influence, which is why `-C target-feature=+avx` and `-C target-cpu=native` hard-error here
   while the attribute silently succeeds and hands the backend a request it cannot satisfy.

2. **`src/abi/pass_mode.rs` builds the wide vector parameter unconditionally.** For a bare vector
   rustc emits `PassMode::Direct` with `BackendRepr::SimdVector`, and
   `get_abi_param`/`get_abi_return` do
   `AbiParam::new(crate::intrinsics::clif_vector_type(tcx, self.layout))` - which is
   `scalar_to_clif_type(element).by(count).unwrap()`, i.e. `F32X8`/`F64X4`/`I64X4`/`F32X16`/
   `F64X8`/`I64X8`. For a vector inside a `#[repr(C)]` struct rustc emits `PassMode::Cast` with a
   `Reg { kind: Vector, size: 32|64 }`, and `reg_to_abi_param` does
   `types::I8.by(size).unwrap()`, i.e. `I8X32`/`I8X64`. Neither path has any width check.

3. **Those Cranelift types exist but no backend can hold them.** `Type::by()` happily produces
   `F32X8`, so nothing fails at signature-construction time. The failure surfaces at the very end,
   in `Module::define_function`, as
   `Err(ModuleError::Compilation(CodegenError::Unsupported("Unexpected SSA-value type: f32x8")))`,
   and `src/base.rs:222` turns *any* such `Err` into `bug!(...)` - "the compiler unexpectedly
   panicked. This is a bug" - rather than a diagnostic. (Same final step as candidate 006's ICE.)

The 128-bit `extern "C"` cases work because `F32X4`/`I8X16` are real x64 SSA types, and the
Rust-ABI 256-bit case works because rustc passes anything over 16 bytes indirectly under
`extern "Rust"`, so no wide Cranelift value is ever needed.

## Introduced by

fc595f1a "[WIP] Use FnAbi everywhere instead of our own abi calculations" (2021-01-26) - it deleted
the fallback that had made this case work. 67028cee "Use PassMode::ByVal for Abi::Vector"
(2020-03-28) introduced by-value vector passing but only when `clif_vector_type` returned `Some`,
falling back to `PassMode::ByRef` otherwise; 6170fc61 "Split abi adjustments out" (2021-01-25)
kept that as `if clif_vector_type(tcx, arg_abi.layout).is_none() { arg_abi.make_indirect() }`;
fc595f1a removed the whole adjustment block, after which the wide-vector `AbiParam` was built
unconditionally. eed1f754 "Don't store vector types in ssa variables" (2023-03-25) then dropped
`clif_vector_type`'s `Option` and its `vector_ty.bits() == 128` guard, changing the symptom from an
`Option::unwrap` panic to today's `Unexpected SSA-value type` ICE. The `PassMode::Cast` half of the
class arrived with `RegKind::Vector` support (e8f48e4b "[WIP] Implement PassMode::Cast",
2021-01-29) and never had a width check at all.

Found with `git log -S` on `clif_vector_type`, `RegKind::Vector`, `vector_ty.bits() == 128` and
`if let Some(vector_ty) = crate::intrinsics::clif_vector_type`; no bisect was needed. Note that
rustc's `abi_required_features` hard error is much newer than any of these commits - before it
existed the same ICE was reachable with no `#[target_feature]` attribute at all, which is why the
introducing commits predate the target-feature framing.

## The fix

`patch.diff` (+57/-17, all in `src/abi/pass_mode.rs`, `cargo fmt --check` clean) - **no ICE, actionable diagnostic instead**;
it does not make the program compile. Two guards plus one helper:

- `vector_to_abi_param(tcx, layout)` replaces the two bare
  `AbiParam::new(clif_vector_type(tcx, self.layout))` sites in `get_abi_param`/`get_abi_return`,
  and rejects `layout.size.bits() > 128` before ever building the type.
- `reg_to_abi_param`'s `RegKind::Vector` arm rejects `size * 8 > 128` the same way, covering the
  `PassMode::Cast` path; `tcx` is threaded through `cast_target_to_abi_params` for it.
- Both funnel into `reject_unsupported_vector_abi`, which calls `tcx.dcx().fatal` with the same
  diagnostic shape `conv_to_call_conv` already uses for unsupported call conventions
  ("call conv {c:?} is not yet implemented"):

      error: passing SIMD vector type `std::arch::x86_64::__m256` by value is not yet supported by
      rustc_codegen_cranelift: the chosen ABI requires it to be passed in a 256-bit vector
      register, but the widest vector type Cranelift has is 128 bits. Pass it behind a pointer or
      reference, or use the default Rust ABI (which passes it indirectly) instead of an explicit
      `extern` ABI.

Verified, per `CONFIRMATION.md` section 7, against more than the one triggering input:

| case | before | after |
|---|---|---|
| `repro/` `wide` via `dist/cargo-clif` (`extern "C"` `__m256` arg + return) | ICE | clean `error:`, no ICE |
| `repro/` `control` via `dist/cargo-clif` (`extern "C"` `__m128`/`__m128i`, Rust-ABI `__m256`) | matches LLVM | still matches LLVM byte-for-byte |
| `___BUGHUNT___/tools/tf_abi_sweep.sh`: `__m256`/`__m256i`/`__m256d`/`__m512`/`__m512i`/`__m512d`, bare (`PassMode::Direct`) | 6 ICE (`f32x8`, `i64x4`, `f64x4`, `f32x16`, `i64x8`, `f64x8`) | 6 clean errors |
| same 6 wrapped in a `#[repr(C)]` struct (`PassMode::Cast`) | 6 ICE (`i8x32` x3, `i8x64` x3) | 6 clean errors |
| `extern "sysv64"`, `extern "C"` fn pointer, vector mixed with `i32`/`f64` args | 3 ICE | 3 clean errors |
| 3 controls (`extern "C"` `__m128`, `__m128i`, Rust-ABI `__m256`) | match LLVM | still match LLVM |
| `-O` and `-Ctarget-cpu=native` on the original trigger | ICE | clean error |

- Project test suite (`./y.sh test` in the isolated worktree - in-tree items plus the `download/`
  repos): **before the patch: exit 0. After the patch: exit 0.** Both runs emit 61
  `test result: ok.` lines with an identical multiset of pass counts and zero `FAILED`. The suite
  aborts on the first failure and neither run aborted. No new failures. (Run three times in total:
  once before the patch and twice after - the second post-patch run was after `cargo fmt`
  reflowed two of the new call sites, so the recorded `patch.diff` is exactly what was tested.)

## What's not done / open questions

- **This patch removes the crash, not the capability gap.** The program LLVM compiles still does
  not compile under cg_clif. Making it work needs one of two maintainer decisions: (a) Cranelift
  gaining 256-/512-bit vector types, which is the only way to actually implement the SysV vector
  ABI (`__m256` goes in `ymm0`), or (b) restoring the pre-fc595f1a `make_indirect()` fallback,
  which would make pure-Rust programs work but is knowingly ABI-incompatible with C - that is
  exactly what the surviving "FIXME implement Vector Abi in a cg_llvm compatible way" comment on
  the same match arms is about. (b) was deliberately not done here: silently using a
  non-C-compatible ABI for an `extern "C"` function trades a compile-time crash for a silent
  miscompilation of real FFI.
- **The complementary failure was confirmed but not fixed**: with the attribute removed,
  `-C target-feature=+avx`, `-C target-cpu=native` and `-C target-cpu=x86-64-v3` all make cg_clif
  hard-error at rustc's ABI check on a program LLVM compiles and runs, because `target_config`
  hardcodes `internal_target_features`. That is the existing `IDEAS.md` `target_config`/`build_isa`
  row, not this candidate.
- **No wrong-answer case was found in this bug class** - every failure is compile-time. Whether
  cg_clif's `extern "C"` ABI for 128-bit vectors and for the `PassMode::Cast` paths actually
  matches the C ABI byte-for-byte was not checked against a real C compiler; the controls only
  prove cg_clif agrees with rustc/LLVM-built Rust, which is the same ABI computation on both sides.
- **Non-x86_64 was not exercised** (no cross toolchain this session). The new check should be inert
  on aarch64, where the widest `Reg { kind: Vector, .. }` rustc emits today is 16 bytes, but that
  is from reading `rustc_target`'s callconv code, not from a run. SVE `z`-register ABIs would hit
  the new diagnostic rather than the old ICE.
- `reject_unsupported_vector_abi` uses `tcx.dcx().fatal` with no span, because
  `get_abi_param`/`get_abi_return` are called from signature construction and have no span to hand.
  A `span_fatal` at the function's own span would be a nicer diagnostic and needs plumbing that was
  out of scope here.
