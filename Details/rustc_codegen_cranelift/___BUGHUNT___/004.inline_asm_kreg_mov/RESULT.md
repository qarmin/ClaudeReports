# 004. inline_asm: AVX-512 mask registers spilled/reloaded with `mov` instead of `kmov`

cg_clif implements `asm!` by emitting a hand-written `global_asm!` wrapper function and passing
operands through a stack buffer. `src/inline_asm.rs`'s `save_register`/`restore_register` build
that text per register class; only `reg`/`reg_abcd` and `xmm/ymm/zmm_reg` are special-cased, and
everything else falls into a generic `mov [rbx+0x..], <reg>` / `mov <reg>, [rbx+0x..]` arm. AVX-512
mask registers (`kreg`, i.e. `k1`..`k7`) land there, and `mov` cannot address a mask register - the
assembler rejects the wrapper and the whole crate fails to build. Lead came from the 2026-09-02
hodow audit (`IDEAS.md` row 5); the risk map in `projects/rustc_codegen_cranelift.md` already
flagged `src/inline_asm.rs`'s generic branch.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential test vs LLVM, `tools/diffcheck.sh`, real-world trigger | `_mm_mask_load_sd` (stdarch implements it with `asm!` on `in("k1")`), opt-level 0, main tree's unpatched `dist/rustc-clif` | Reproduced. LLVM prints `[4.0, 0.0]`; cg_clif: `error: invalid operand for instruction` on `mov k1, [rbx+0x18]`, then `error: Failed to assemble ...` |
| Boundary-value construction around the suspect operation: hand-written `asm!` per operand kind | `in("k1")`, `out("k7")`, `inout("k3")`, two kregs at once, 16-bit (avx512f) and 64-bit (avx512bw) widths, `k0`/`kreg0` | Reproduced for every kind that rustc allows an operand in - `mov k1, [rbx+0x0]` on the input side, `mov [rbx+0x0], k2` on the output side, both in the same wrapper for `inout`. `k0` (class `kreg0`) turned out to have an empty `supported_types()`, so it can only ever be a clobber and is not reachable through this path. |
| Generated sweep (`tools/kreg_sweep.rs`: 44 `asm!` calls over 8 x 16-bit and 6 x 64-bit mask values x 4 operand shapes, plus the 6 stdarch intrinsics), run through `diffcheck.sh` on both sides | opt-level 0, 3 repeat runs per side | Unpatched: 16 `invalid operand for instruction` diagnostics over 13 distinct bad instructions (`mov k1, [rbx+0x0]` ... `mov [rbx+0x0], k7`), crate does not build, 3/3 runs. Patched: output byte-identical to LLVM, 3/3 runs. |
| Read `rustc_target/src/asm/{x86,aarch64,mod}.rs` to bound the bug class | nightly-2026-08-24 sources | `mmx_reg`/`x87_reg`/`tmm_reg` (x86) and `preg` (aarch64 SVE) also fall into the generic arm, but all have an empty `supported_types()` **and** are in the `"C"` clobber ABI, so neither the operand nor the clobber path reaches them. `kreg` is the only class that is actually reachable and broken. |
| Project test suite | `./y.sh test` in the worktree, before and after the patch | Both clean - see "The fix". |

## Status

**Confirmed**. Deterministic: it is a compile-time assembler rejection, not a timing- or
layout-dependent runtime symptom. 3/3 repeat runs of the 45-line sweep through
`tools/diffcheck.sh` against the unpatched `dist/rustc-clif` (exit 1 = disagrees with LLVM) and
3/3 against the patched worktree (exit 0 = identical). One full `just test` cycle in this folder,
from a pristine clone of 2bc05d45, reproduced it and then showed it gone after `patch.diff`
(captured in `output.txt`).

**Fully reproducible**: **Yes**. `repro/` is a normal crate built by the real `dist/cargo-clif`
out of a real `./y.sh build` of the cloned backend; the reproduction *is* that real build failing,
and after the patch the same real binary runs and prints the same values as the LLVM-built one.
Nothing is hand-asserted or stubbed. The build failure reproduces on any x86-64 host (the
assembler, not the CPU, rejects the instruction); the value checks in `repro/` additionally need
an AVX-512 CPU and print `SKIP` without one.

## Class

crash (hard compile error - the backend refuses valid Rust that LLVM accepts; it kills the whole
crate's build, not just the affected function)

## Root cause

`src/inline_asm.rs` generates the `asm!` wrapper as assembly *text*. Around the user's asm it has
to move each operand between the caller-provided stack buffer (`rbx`) and the operand's assigned
register, which `save_register`/`restore_register` do by writing a literal instruction chosen from
the register's class:

```rust
match reg {
    InlineAsmReg::X86(reg) if matches!(reg.reg_class(),
        xmm_reg | ymm_reg | zmm_reg) => { /* movups / vmovups */ }
    _ => {
        write!(generated_asm, "    mov [rbx+0x{:x}], ", offset.bytes()).unwrap();
        reg.emit(generated_asm, InlineAsmArch::X86_64, None).unwrap();
    }
}
```

`kreg` is not in the guard, so a `k1`..`k7` operand gets `mov [rbx+0x18], k1` (output side) or
`mov k1, [rbx+0x18]` (input side). x86 has no `mov` form that takes a mask register - the AVX-512
mask move is a separate opcode, `kmov{b,w,d,q}` - so the assembler reports `invalid operand for
instruction` for each such line and cg_clif turns that into `error: Failed to assemble ...`. The
error is per-`asm!`-wrapper but aborts the compilation, so one such `asm!` anywhere in the
dependency graph fails the whole crate.

Real-world reach: `core::arch`'s `_mm_mask_load_ss`, `_mm_mask_load_sd`, `_mm_maskz_load_ss`,
`_mm_maskz_load_sd`, `_mm_mask_store_ss` and `_mm_mask_store_sd` are implemented in stdarch with
`asm!` on `in("k1")` (there is no LLVM intrinsic for them), so merely calling one of these six is
enough. That is how the hodow audit hit it (6 monomorphizations).

Choosing the right `kmov` width is the non-obvious part. `rustc_target`'s `kreg` class declares
`avx512f: I8, I16; avx512bw: I32, I64`, i.e. an operand wider than 16 bits is only accepted when
`avx512bw` is in `tcx.asm_target_features(def_id)`. `kmovq` itself also requires `avx512bw`, so
emitting it unconditionally would put an instruction the function is not guaranteed to be allowed
to execute into an avx512f-only function; `kmovw` unconditionally would truncate a 32/64-bit mask
to 16 bits (a silent wrong answer). The two conditions coincide exactly, so the correct choice is
`kmovq` iff `avx512bw` is in the same feature set rustc used to type-check the operand.

## Introduced by

726e329f "Add basic inline asm support for x86_64" (2020-07-10) introduced the unconditional
`mov [rbp+0x..], <reg>` / `mov <reg>, [rbp+0x..]` pair that is still the generic arm today; at
that point only explicit `reg`-class registers were supported (`expect_reg` did
`unimplemented!()` for any register *class*), so no `kreg` could reach it yet. It first became
reachable with 799e0679 "Merge commit '97e5045...' into sync_cg_clif-2021-12-20", which brought in
`allocatable_registers`-based allocation for arbitrary register classes. The same omission for
`xmm/ymm/zmm_reg` was noticed and fixed in 9a925322 "Merge commit '493d427...' into
sync_cg_clif-2026-03-25" (the arm this patch extends), but `kreg` was not covered there.

## The fix

`patch.diff` (+38/-16, all in `src/inline_asm.rs`):

- Turns `save_register`/`restore_register` from associated functions into `&self` methods (their
  `arch` parameter was always `self.arch` anyway) so they can read
  `self.tcx.asm_target_features(self.enclosing_def_id)`.
- Adds a `kreg`/`kreg0` arm to both, emitting `kmov{q,w}` instead of `mov`.
- Picks the width with a new `kreg_mov()` helper: `kmovq` when `avx512bw` is in the enclosing
  function's asm target features, `kmovw` otherwise - which mirrors exactly the rule rustc uses to
  decide whether a `kreg` operand may be wider than 16 bits, so the emitted instruction is both
  wide enough for any accepted operand and never wider than the function's features allow.
  `asm_target_features` already includes implied features, verified: a function annotated only
  `#[target_feature(enable = "avx512vbmi")]` gets `kmovq`.

Checked against more than the one triggering input (`CONFIRMATION.md` section 7): the sweep in
`tools/kreg_sweep.rs` and `repro/src/main.rs` cover the input operand, the output operand, `inout`, two live
kregs in one `asm!` (two independent slots at different offsets), the registers `k1`/`k2`/`k3`/
`k4`/`k5`/`k6`/`k7`, mask values `0`, `1`, `0x00ff`, `0xff00`, `0x5a5a`, `0xffff`, `1<<63`,
`u64::MAX`, `0x0123456789abcdef`, both the avx512f (16-bit) and avx512bw (64-bit) widths, an
implied-feature path (`avx512vbmi`), and all six stdarch masked-scalar intrinsics. Every one now
matches LLVM byte for byte; `objdump` of the patched output confirms `kmovw` in the avx512f-only
wrappers and `kmovq` in the avx512bw ones.

- Project test suite: `./y.sh test` in the worktree, full run (all of `config.txt`: the
  `no_sysroot`/`base_sysroot` items plus `testsuite.extended_sysroot` = rand, regex, graviola,
  portable-simd) - **before the patch: passed**, exit 0; **after the patch: passed**, exit 0.
  Every `test result:` line in both runs is `ok` with `0 failed`; no new failures. (`./y.sh test`
  aborts on the first failure, so exit 0 on the whole run is itself the no-regression statement.)
  The suite has no `asm!`-with-`kreg` coverage of its own, which is why the bug survived in it.

## What's not done / open questions

- No test was added to the project's own suite. `example/` would be the natural home, but any such
  test only compiles usefully on a target where rustc accepts `kreg` operands, so it needs an
  `#[cfg(target_arch = "x86_64")]` + `#[target_feature]` shape a maintainer should sign off on.
- `kreg0` (`k0`) is included in the new guard for completeness but is currently unreachable:
  `rustc_target` gives it an empty `supported_types()`, so it can only be a clobber, and all of
  `k0`..`k7` are in the `"C"` clobber ABI, which `allocate_stack_slots` filters out of the
  clobber-save path. If rustc ever allows a `k0` operand the arm is already correct.
- Same generic-`mov` arm, deliberately left alone: `mmx_reg`, `x87_reg`, `tmm_reg` on x86-64 and
  `preg` on aarch64 would all be emitted with an instruction that cannot address them, but all
  four have an empty `supported_types()` and are in their target's clobber ABI, so neither the
  operand nor the clobber path can reach them today. They are latent, not currently triggerable.
- The wider design point is untouched: cg_clif chooses these instructions from the register class
  alone, with no knowledge of the operand's actual type, so the slot is always the class's widest
  type and the mov width is a per-class guess. That is fine for every class that exists now but is
  the shape of the mistake that produced this bug, and a future class with feature-dependent
  widths would hit it again.
- Only the `x86_64` arm of `save_register`/`restore_register` was audited for this class of
  mismatch; the AArch64 and RiscV64 arms were only skimmed.
