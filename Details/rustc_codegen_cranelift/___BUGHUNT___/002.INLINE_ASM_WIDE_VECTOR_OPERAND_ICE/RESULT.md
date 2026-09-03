# 002.INLINE_ASM_WIDE_VECTOR_OPERAND_ICE

cg_clif hands Cranelift a >128-bit SSA vector value (`f64x4`, `i64x8`, `f32x16`, ...) for any
inline-asm `in`/`inout`/`out` operand that is a 256- or 512-bit vector, and Cranelift has no such
value type, so `module.define_function` fails and the compiler ICEs. Backlog row 3 in `IDEAS.md`
pointed at the `_mm*_stream_*` family; the actual defect turned out to be in `src/inline_asm.rs`,
not in the `nontemporal_store` intrinsic, and it covers plain hand-written `asm!` too.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential test vs LLVM, single file (`tools/diffcheck.sh`) | `_mm256_stream_pd` only, opt-level 0, `-Ctarget-cpu=native`, ~4s | Reproduced immediately: `Error while defining ..._mm256_stream_pd: Compilation(Unsupported("Unexpected SSA-value type: f64x4"))`, exit 101 vs LLVM's exit 0 |
| Read the backend to find the real path | `src/intrinsics/mod.rs` `nontemporal_store`, then `src/inline_asm.rs` | `nontemporal_store` is *not* involved - stdarch implements `_mm*_stream_*` with `asm!(... in(ymm_reg) a ...)` (stdarch #1541). The ICE comes from `CValue::load_scalar` on an asm operand |
| Generated sweep, one case per file (`tools/wide_asm_sweep.sh`, written for this candidate) | 15 cases: 8 stdarch `_mm*_stream_*`, 5 hand-written `asm!` with ymm/zmm operands, 2 controls; opt-level 0, `-Ctarget-cpu=native`; also re-run on the default `x86-64` ISA preset | 13/15 ICE with 6 distinct offending SSA types (`f64x4`, `f32x8`, `i64x4`, `f64x8`, `f32x16`, `i64x8`). The 2 controls (a scalar `f64` in `ymm_reg`, a 128-bit `inout(xmm_reg)`) pass. Needed one case per file because cg_clif aborts the whole crate on the first failing CGU |
| Standalone `repro/` end-to-end via `dist/cargo-clif` | Real cargo crate, opt-level 0, real binary run and diffed against the LLVM-built binary; 3 repeats each side | 3/3 ICE unpatched, 3/3 identical-to-LLVM patched |
| Full `just test` cycle from a pristine `git clone` | clone @ 2bc05d45 -> `./y.sh build` -> repro -> `git apply patch.diff` -> `./y.sh build` -> repro | before: cg_clif cannot compile the crate; after: all 15 cases byte-identical to LLVM |

## Status

**Confirmed**

Deterministic compile-time failure: 15/15 runs (3 full `repro/run.sh` cycles with a fresh
`target-clif` each time, plus the 15-case sweep run twice, plus one full `just test` from a fresh
clone). Machine: Ryzen 9 7900 (Zen 4, AVX-512), rustc nightly-2026-08-24, cg_clif @ 2bc05d45,
opt-level 0. Reproduces both with `-Ctarget-cpu=native` and on Cranelift's default `x86-64` preset,
so it is not ISA-preset dependent.

**Fully reproducible**: **Yes**. `repro/` is a real cargo crate built by the real
`dist/cargo-clif`; the failure is the real backend refusing real valid Rust, and after the fix the
real generated binary is run and its output compared line-by-line with the LLVM-built binary's.

## Class

crash

## Root cause

`src/inline_asm.rs` moves every inline-asm operand between the Cranelift function and the asm
wrapper's stack slot through a **single Cranelift SSA value**:

- inputs: `CInlineAsmOperand::In { value: Value }` / `InOut { in_value: Value }`, filled in
  `codegen_inline_asm_terminator` with `crate::base::codegen_operand(fx, value).load_scalar(fx)`;
- outputs: `call_inline_asm` loads the slot back with
  `asm_clif_type(fx, lane_type).unwrap().by(lane_count).unwrap()`.

For a 256/512-bit vector operand (`in(ymm_reg)`, `out(zmm_reg)`, ...) `CValue::load_scalar`
(`src/value_and_place.rs:135`) computes the clif type as `scalar_to_clif_type(element).by(count)`.
`Type::by` happily produces `f64x4`/`i64x8`/`f32x16` - Cranelift's *type system* defines vector
types up to 512 bits - but no Cranelift backend supports a vector wider than 128 bits, so the
resulting `load`/`store` is rejected during lowering with
`Unsupported("Unexpected SSA-value type: f64x4")`, surfacing as an ICE at `src/base.rs:222`
(`Error while defining <fn>`).

Everything else about wide vectors in cg_clif already goes lane-by-lane or through memory, and the
generated asm wrapper itself is already correct for these register classes: `save_register`/
`restore_register` emit `vmovups` for `xmm_reg`/`ymm_reg`/`zmm_reg`. The Cranelift-side copy into
and out of the wrapper's stack slot is the only part that assumed the value fits in an SSA value.

This is reachable from **stable, safe-to-call** `core::arch` intrinsics: stdarch implements
`_mm256_stream_{pd,ps,si256}`, `_mm256_stream_load_si256`, `_mm512_stream_{pd,ps,si512}` and
`_mm512_stream_load_si512` with `asm!` and a `ymm_reg`/`zmm_reg` operand ("see #1541, we should use
inline asm to be sure, because LangRef isn't clear enough"), so merely *calling* one of them makes
the whole crate fail to compile under cg_clif. Hand-written `asm!` with a `__m256i`/`__m512i`
operand fails the same way.

## Introduced by

726e329f "Add basic inline asm support for x86_64" (2020-07-10) - the very first inline-asm
implementation already did
`inputs.push((reg, new_slot(reg.reg_class()), crate::base::trans_operand(fx, value).load_scalar(fx)))`,
and the operand's value has been a single Cranelift `Value` ever since. `git log -S load_scalar --
src/inline_asm.rs` bottoms out there.

Two later commits made it actually reachable rather than changing the defect:
9a925322 "Merge commit '493d427c7d3423f085f05179e36a35b4943b1379' into sync_cg_clif-2026-03-25"
taught `save_register`/`restore_register` about `ymm_reg`/`zmm_reg` (before that the wrapper itself
was also wrong), and the stdarch-side switch of `_mm*_stream_*` from `nontemporal_store` to `asm!`
(rust-lang/stdarch#1541, not in this repo's history) is what put the broken path behind stable
intrinsics.

## The fix

`patch.diff` (+151/-99; the semantic part is +70/-18 in `src/inline_asm.rs`, the rest is one
mechanical `.into()` per existing `value:`/`in_value:` call site in `src/intrinsics/llvm_x86.rs`
and `src/intrinsics/llvm_aarch64.rs`).

- New `AsmValue { Val(Value), ByRef(Pointer, Size) }` replaces the bare `Value` in
  `CInlineAsmOperand::In`/`InOut`, with `impl From<Value>` so all 81 existing internal call sites
  (which only ever pass <=128-bit values) just gain `.into()`.
- New `asm_input_value` picks `ByRef` when the operand's layout is larger than
  `MAX_ASM_SSA_VALUE_SIZE` (16 bytes, the widest Cranelift SSA type), using `CValue::force_stack`
  to get a pointer - for a wide vector the value is already in a stack slot, so this is free.
- `call_inline_asm` copies a `ByRef` input into the wrapper's stack slot with
  `emit_small_memory_copy` (alignment claimed as 1/1: the wrapper's slot is only 16-byte aligned
  and nothing is known about the operand's own storage), and returns a >16-byte output through
  `place.write_cvalue(fx, CValue::by_ref(slot, place.layout()))` instead of a `load` of an
  impossible type.

Verified against more than the one triggering input: the sweep covers all 6 distinct wide SSA types
that were failing, `in`/`out`/`inout` directions, integer and float lane types, 4/8/16 lanes,
stdarch intrinsics and raw `asm!`, on both `-Ctarget-cpu=native` and the default `x86-64` Cranelift
preset. All 13 previously-ICEing cases now produce byte-identical output to LLVM, and the two
controls (scalar `f64` in `ymm_reg`; 128-bit `inout(xmm_reg)`) still pass, i.e. the <=128-bit path
is unchanged.

- Project test suite (`./y.sh test` in the worktree, in-tree items + the external repos in
  `download/`): **before** the patch 61/61 `test result: ok` blocks, 23390 tests passed, 0 failed,
  exit 0. **After** the patch: identical - 61/61 ok blocks, 23390 passed, 0 failed, exit 0. Same
  set of test items in both runs, no new failures.

## What's not done / open questions

- Only x86-64 was exercised. The same `AsmValue::ByRef` path is arch-independent and
  `src/intrinsics/llvm_aarch64.rs` was updated mechanically, but no aarch64 machine was available
  to run anything. If rustc ever allows an operand wider than 128 bits in an aarch64 register class
  (SVE `z`/`p` registers), `save_register`'s `str q0` would also need widening - that part is
  untouched here.
- Clobber save/restore for wide registers was not separately tested. `save_register`/
  `restore_register` already use `vmovups` for `ymm_reg`/`zmm_reg`, but a `clobber_abi("C")` asm
  block that actually clobbers `ymm`/`zmm` state and then relies on it being restored was not
  written.
- Correct non-temporal-store code still cannot run under cg_clif for an unrelated reason:
  `_mm_sfence()` lowers to `llvm.x86.sse.sfence`, which is unimplemented and aborts at run time
  (`llvm.x86.sse.sfence is not yet supported`). The stdarch docs require an `_mm_sfence()` after
  every `_mm*_stream_*`, so this fix unblocks compilation but a realistic caller still aborts. The
  repro deliberately omits the fence (x86 guarantees same-thread self-consistency for the reads it
  does) so that this separate gap does not mask the one being fixed.
- The fix routes wide operands through memory unconditionally. That is what the wrapper's own
  `vmovups` already implies, so no extra copy is introduced relative to what the asm wrapper needs,
  but it does mean cg_clif will never keep a 256/512-bit asm operand in a register across the call
  - that requires Cranelift support for wider vector types, which does not exist.
- Whether the 81-line mechanical `.into()` churn is acceptable to the maintainers, or whether they
  would prefer changing the field to `CValue<'tcx>` (which would need a layout at each of those
  call sites) is a decision for them.
