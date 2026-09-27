# 018. vcvtps2ph_256_unimplemented

`core::arch::x86_64::_mm256_cvtps_ph` (stable public API, F16C) lowers to the LLVM intrinsic
`llvm.x86.vcvtps2ph.256`. `src/intrinsics/llvm_x86.rs` has an arm for `llvm.x86.vcvtps2ph.128`
(added by 4748c537) but none for `.256`, so the 256-bit form falls into that file's `_ =>`
catch-all: a compile-time warning plus `codegen_panic_nounwind`. The crate builds "successfully"
and the real binary aborts the first time the intrinsic is executed, while the 128-bit sibling
right next to it in the same `f16c.rs` module computes the correct answer. Suspected from
candidate 009's read of the same file.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/intrinsics/llvm_x86.rs` (`grep -n 'vcvtps2ph\|f16'`) | n/a | Exactly one hit: the `.128` arm at llvm_x86.rs:1320. No `.256` arm, so it can only reach the `_ =>` arm at llvm_x86.rs:1349. |
| Differential test vs LLVM, `___BUGHUNT___/tools/diffcheck.sh` | single file, opt-level 0, default target-cpu | Reproduced on the first try: LLVM exit 0 / cg_clif exit 134, `llvm.x86.vcvtps2ph.256 is not yet supported`. |
| Family sweep around the suspect: all four F16C intrinsics (`_mm_cvtps_ph`, `_mm256_cvtps_ph`, `_mm_cvtph_ps`, `_mm256_cvtph_ps`) in one program, plus a control with only the 256-bit ps->ph call removed | opt-level 0, diffcheck.sh | `_mm256_cvtps_ph` is the *only* one missing. The control (other three) is byte-identical to LLVM under unpatched cg_clif, and the build log emits exactly one `unsupported x86 llvm intrinsic` warning. |
| Boundary-value construction: 8 hand-built input vectors x all 5 legal rounding-control immediates (stdarch static-asserts `IMM8 <= 4`) | opt-level 0, diffcheck.sh, and again at `-Ctarget-cpu=native` and at `OPT=1` | Unpatched: aborts. Patched: identical to LLVM in all three configurations. Inputs cover exact powers of two, exact f16 ties, the 65520 overflow-to-infinity threshold, f16 subnormals, +/-0, +/-inf, quiet and signalling NaN bit patterns, `f32::MAX/MIN/MIN_POSITIVE`, and the `0x477f_efff`/`0x477f_f000` pair that straddles the rounding boundary. |
| Standalone `repro/` crate through the real `dist/cargo-clif` (86 lines of raw f16 bit patterns, 4 sections incl. repeated use in one function, input-register-preservation and a ps->ph->ps round trip) | debug, `[profile.*] opt-level = 0` | 5/5 runs abort unpatched (exit 134); 5/5 runs byte-identical to LLVM patched (same sha256 as LLVM's stdout). |
| `objdump -d` on the patched and unpatched binaries | opt-level 0 | Patched: 3 real `vcvtps2ph $imm,%xmm0,%xmm0` per immediate (1 from the `.128` control + 2 from the new arm's two halves); unpatched: 1 per immediate. The new lowering emits real instructions and threads the immediate through, it is not a silent no-op. |

Not used, deliberately: `miri` (never reaches codegen) and ASAN/UBSAN (they instrument the
compiler, not the code it emits) - neither can see a backend codegen gap.

## Status

**Confirmed**

- Unpatched cg_clif: 5/5 runs of the same `repro/` binary abort with `llvm.x86.vcvtps2ph.256 is
  not yet supported`, exit 134 (SIGABRT). Deterministic - the trap is emitted unconditionally at
  compile time, so it fires on the first execution of the intrinsic.
- The compile-time warning (`unsupported x86 llvm intrinsic llvm.x86.vcvtps2ph.256; replacing
  with trap`) appears on every build, but the crate still builds successfully, so the failure only
  shows up at run time.
- LLVM (the oracle), same source: exit 0, prints all 86 lines.

**Fully reproducible**: **Yes** - `repro/` is a normal Cargo crate built by the real
`dist/cargo-clif` and the resulting real binary is executed; the only thing that changes between
the two runs is which cg_clif built it.

## Class

crash

## Root cause

`src/intrinsics/llvm_x86.rs::codegen_x86_llvm_intrinsic_call` dispatches on the intrinsic name
string. There is an arm for the 128-bit half-float conversion:

    "llvm.x86.vcvtps2ph.128" => { ... vcvtps2ph xmm0, xmm0, {imm8} via codegen_inline_asm_inner ... }

and nothing for `llvm.x86.vcvtps2ph.256`, so it reaches the terminal `_ =>` arm at
src/intrinsics/llvm_x86.rs:1349:

    fx.tcx.dcx().warn(format!("unsupported x86 llvm intrinsic {}; replacing with trap", intrinsic));
    ...
    crate::base::codegen_panic_nounwind(fx, &msg, span);

i.e. the call site is replaced by an unconditional non-unwinding panic.

Why the 256-bit form was skipped when the 128-bit one was added is visible in the shape of the
existing arm: it does `a.load_scalar(fx)` and hands the resulting single Cranelift `Value` to an
inline-asm `xmm0` operand. That cannot work for a `__m256` input - Cranelift has no >128-bit
vector type, so `clif_type_from_ty` returns `None` for `f32x8`, the `CValue` is always `ByRef`,
and `load_scalar` is unreachable for it. This is the same >128-bit-SSA-vector wall that
candidate 002 hit from the other direction. The instruction itself is not the problem: `VCVTPS2PH`
with a ymm source writes only the low 128 bits of an xmm destination, and each 128-bit lane of the
source is converted independently, so the operation splits cleanly in half.

## Introduced by

4748c537 Add `llvm.x86.vcvtps2ph.128` (#1613)

That commit is what created the asymmetry this candidate is about. Before it, both widths hit the
same `_ =>` warn + trap arm, so `_mm_cvtps_ph` and `_mm256_cvtps_ph` failed identically; after it,
one of the two works and the other aborts. The underlying `_ =>` warn +
`codegen_panic_nounwind` fallback that the 256-bit form lands in dates to 1a268911 "Replace llvm
intrinsics with runtime trap" (2019-07-28); `llvm.x86.vcvtps2ph.256` itself has never had an
implementation (`git log -S vcvtps2ph` returns only 4748c537).

## The fix

`patch.diff` (+54/-0, `src/intrinsics/llvm_x86.rs`) adds a `"llvm.x86.vcvtps2ph.256"` arm right
after the `.128` one. It reads the two 128-bit halves of the `__m256` input as `u128` lanes
(`value_typed_lane(fx, fx.tcx.types.u128, half)` - the same trick the existing
`llvm.x86.avx.vperm2f128.*` arms already use to split a 256-bit vector), runs the *same*
`vcvtps2ph xmm0, xmm0, {imm8}` inline asm the `.128` arm uses on each half into its own stack
slot, and stores the low 4 f16 lanes (64 bits) of each half result into lanes 0 and 1 of the
`__m128i` return place.

Reusing the same instruction, rather than reimplementing f32 -> f16 with Cranelift's own
`fdemote`, is deliberate: `imm8` is a rounding-control field (0 = nearest-even, 1 = down,
2 = up, 3 = truncate, 4 = use MXCSR) that Cranelift has no way to express, and hardware
`vcvtps2ph` also has its own denormal/NaN-payload behaviour. Delegating to the instruction makes
the 256-bit arm bit-exact with the 128-bit one by construction.

Two things make the split correct here:

- `VCVTPS2PH ymm -> xmm` converts each 128-bit source lane independently and writes the 8 results
  packed into the low 128 bits of the destination, so lane `i` of the 256-bit result only ever
  depends on lane `i` of the source. No cross-lane behaviour to preserve.
- SIMD locals in cg_clif are never SSA (`clif_type_from_ty` in src/common.rs returns `None` for
  any SIMD/ADT type), so both `ret` and the input `CValue` are guaranteed to be memory places and
  the `value_typed_lane`/`place_typed_lane` helpers - which `unreachable!()` on a `Var` - are safe
  to use on them.

Checked against more than the one triggering input (all via the real `dist/cargo-clif`/
`dist/rustc-clif` binary, opt-level 0):

| input | unpatched cg_clif | patched cg_clif |
|---|---|---|
| `_mm256_cvtps_ph::<0>` (nearest-even), 8 boundary-value vectors | abort | matches LLVM |
| `_mm256_cvtps_ph::<1>` (round down), same 8 vectors | abort | matches LLVM |
| `_mm256_cvtps_ph::<2>` (round up), same 8 vectors | abort | matches LLVM |
| `_mm256_cvtps_ph::<3>` (truncate), same 8 vectors | abort | matches LLVM |
| `_mm256_cvtps_ph::<4>` (use MXCSR), same 8 vectors | abort | matches LLVM |
| the same 8 vectors through `_mm_cvtps_ph` as two halves (control, already implemented) | matches LLVM | matches LLVM, and equals the 256-bit result lane for lane |
| `_mm_cvtph_ps` / `_mm256_cvtph_ps` (the other direction, both widths) | matches LLVM | matches LLVM |
| 3x `_mm256_cvtps_ph::<4>` on the same input inside one function | abort | matches LLVM (the two stack slots are not aliased across uses) |
| input vector read back after the call | abort | unchanged, matches LLVM (the lowering does not clobber its input) |
| `_mm256_cvtph_ps(_mm256_cvtps_ph::<0>(a))` round trip | abort | matches LLVM |
| the whole sweep at `-Ctarget-cpu=native` | abort | matches LLVM |
| the whole sweep at `-O` (`OPT=1`; MIR GVN const-folds part of it, so this is a weaker check - kept only as a non-regression) | abort | matches LLVM |

- Project test suite (`./y.sh test` in the worktree, full run including the `download/` external
  repos): **before the patch: pass, exit 0**; **after the patch: pass, exit 0**. Both runs
  executed the identical list of 35 items - the two logs' `[AOT]`/`[BUILD]`/`[COPY]`/`[JIT]`/
  `[PATCH]`/`[TEST]` lines were diffed and are identical - including `[TEST] sysroot` and the
  external repos. No new failures. (`./y.sh test` aborts on the first failure, so exit 0 means
  every item ran.) `cargo fmt -- --check` is also clean with the patch applied.

## What's not done / open questions

- The two-halves lowering emits two `vcvtps2ph` instructions plus two stack-slot round trips
  where LLVM emits one instruction. Correct but slower; a single `vcvtps2ph xmm0, ymm0, imm8`
  would need a 256-bit inline-asm operand, which is exactly what candidate 002 found ICEs
  (`ymm`-class operands go through the same wide-SSA-vector path Cranelift cannot represent).
  If that is ever fixed, this arm should be collapsed into one instruction.
- The MXCSR rounding mode (`imm8 == 4`) is tested only with MXCSR at its default value. Setting a
  non-default rounding mode from Rust needs `_MM_SET_ROUNDING_MODE`, which itself goes through
  `llvm.x86.sse.stmxcsr`/`ldmxcsr` - not checked here, and possibly another gap in the same file.
- Only the `ps -> ph` direction at 256 bits was missing. The AVX-512 relatives
  (`llvm.x86.avx512.mask.vcvtps2ph.512` and friends) are still unimplemented and untouched -
  they need a 512-bit split (four halves) plus writemask/`sae` handling, a separate suspect.
- The candidate `justfile`'s `clone`/`build-clif` recipes could not be executed in this sandbox
  (`git ls-remote git@github.com:...` fails with "Host key verification failed" - no outbound
  ssh). The equivalent cycle was validated for real: `git clone --no-local` of the local
  checkout, `git checkout 2bc05d45`, `git apply patch.diff` -> applies cleanly, 54/0 numstat; and
  `just llvm`, `just clif`, `just diff`, `just family`, `just disasm` were all run for real with
  `CLIF_ROOT` pointed at both an unpatched and a patched checkout, producing the output in
  output.txt.
