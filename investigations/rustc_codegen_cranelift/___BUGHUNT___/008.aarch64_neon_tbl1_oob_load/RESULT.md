# 008 - aarch64_neon_tbl1_oob_load

**Status: Not reproducible** (as a wrong answer or a crash). The premise of the candidate is
factually wrong, and the part of it that *is* true is not observable.

Suspect as filed: `llvm.aarch64.neon.tbl1.v8i8` / `.v16i8`
(`src/intrinsics/llvm_aarch64.rs:418-445`) do the `value_lane_dyn` load unconditionally and only
`select` a zero afterwards, so "with the 8-byte table an index of 8..=15 performs a real
out-of-bounds load". Source: candidate #001 (`llvm.x86.avx2.permd` index masking).

## What the code actually does

```rust
"llvm.aarch64.neon.tbl1.v8i8" => {
    intrinsic_args!(fx, args => (t, idx); intrinsic);
    let zero = fx.bcx.ins().iconst(types::I8, 0);
    for i in 0..8 {
        let idx_lane = idx.value_lane(fx, i).load_scalar(fx);
        let is_zero = fx.bcx.ins().icmp_imm_u(IntCC::UnsignedGreaterThanOrEqual, idx_lane, 16);
        let t_idx = fx.bcx.ins().uextend(fx.pointer_type, idx_lane);   // <- unmasked, 0..=255
        let t_lane = t.value_lane_dyn(fx, t_idx).load_scalar(fx);      // <- load happens anyway
        let res = fx.bcx.ins().select(is_zero, zero, t_lane);
        ret.place_lane(fx, i).to_ptr().store(fx, res, MemFlagsData::trusted());
    }
}
```

Two separate claims have to be told apart:

1. **"the v8i8 variant has an 8-byte table, so 8..=15 is out of bounds" - false.** LLVM's
   `int_aarch64_neon_tbl1` is `Intrinsic<[llvm_anyvector_ty], [llvm_v16i8_ty, LLVMMatchType<0>]>`:
   only the *index/result* vector is polymorphic, the table is always `<16 x i8>`. stdarch agrees -
   the sole caller of the `.v8i8` name declares
   `fn _vqtbl1(a: int8x16_t, b: uint8x8_t) -> int8x8_t`
   (`core_arch/src/aarch64/neon/generated.rs:18521-18529`, reached through `vqtbl1_s8` /
   `vqtbl1_u8` / `vqtbl1_p8`). The `t` operand cg_clif sees is therefore a 16-byte object in both
   arms, indices 0..=15 are in bounds, and `>= 16 -> 0` is exactly the AArch64 `TBL Vd, {Vn.16B},
   Vm` semantics. The `.v8i8` name refers to the 8-lane index/result, not to a half-size table.
   The LLVM oracle run in `output.txt` confirms this empirically: for the `vqtbl1_u8` index vector
   `[8,9,10,11,12,13,14,15]` LLVM prints `24 25 26 27 28 29 30 31`, i.e. `table[8..=15]`, not
   zeroes - so the table LLVM indexes is 16 bytes wide even in the `.v8i8` arm.
   (`vtbl1_s8`, the arm32 8-byte-table intrinsic, links to `llvm.arm.neon.vtbl1` on 32-bit ARM and
   on aarch64 is lowered by stdarch itself, not by this code.)

2. **"the load is performed unconditionally, before the select can save it" - true, but for
   indices >= 16, not 8..=15, and it is not observable.** The disassembly in `output.txt` shows
   `ldrb w4, [x0, x4]` with `x4` = the raw index byte, i.e. the emitted code really does read
   `table_ptr + idx` for `idx` up to 255 - up to 239 bytes past the 16-byte table - and only then
   does `cmp w3, #0x10` + `csel` discard the loaded byte. The result is still correct, and the
   address is always inside the caller's stack frame (at opt-level 0 the `t` operand is a MIR
   local, so `value_lane_dyn` gets a pointer to that local's stack slot, and the stack grows
   downwards, so `slot + 0..255` is memory that is already mapped). It cannot fault there, and it
   cannot change the printed answer.

## What was tried (all commands run for real; captured output in `output.txt`)

The host is x86_64 and there is no aarch64 rust-std, no aarch64 C toolchain and no aarch64
hardware here, so a differential harness had to be built first. It is now a reusable tool:
`___BUGHUNT___/tools/aarch64_diffcheck.sh` - it compiles `core` from rust-src once per backend,
builds a `#![no_std] #![no_main]` static PIE with rust-lld, runs it under `qemu-aarch64`, and
diffs the two stdouts. Prerequisite for the cg_clif side, discovered on the way:
**unpatched cg_clif cannot compile `core` for aarch64 at all** (see "new suspects" below); the
worktree therefore carried `repro/128bit_atomics_measurement.diff` for every run below - that
diff touches only the two 128-bit-atomic `span_fatal`s, nothing in the tbl1 path.

Worktree: `tmp/wt/008.aarch64_neon_tbl1_oob_load` at 2bc05d45 (+ the measurement diff).

1. **Boundary-value construction** (`repro/tbl1.rs`, 8 hand-picked index vectors per variant:
   0..7, 8..15, the 15/16/17 boundary, 16..23, 64/100/127/128/200/254/255, all-255, and mixed
   in-range/out-of-range):

       WORKDIR=... CLIF_ROOT=$WT bash ___BUGHUNT___/tools/aarch64_diffcheck.sh repro/tbl1.rs
       -> llvm exit=0, clif exit=0, IDENTICAL (16 lines)

2. **Exhaustive sweep** (`repro/tbl1_sweep.rs`): every index byte value 0..=255, placed in every
   lane position (8 positions for `vqtbl1_u8`, 16 for `vqtbl1q_u8`), plus an all-lanes-equal
   vector for each value - 6400 result lines:

       WORKDIR=... CLIF_ROOT=$WT bash ___BUGHUNT___/tools/aarch64_diffcheck.sh repro/tbl1_sweep.rs
       -> llvm exit=0, clif exit=0, IDENTICAL (6400 lines)

   Index vectors are read through `read_volatile` and the programs are built at `-Copt-level=0`,
   so nothing is const-folded away on either side.

3. **Machine-code inspection** (`just disasm`): dumped the aarch64 code cg_clif emits for
   `core::arch::aarch64::vqtbl1` and confirmed the unconditional `ldrb wN, [x0, xN]` with an
   unmasked index. This is the only positive finding, and it is a latent defect in the emitted
   code rather than a bug in the program's behaviour.

Not attempted, and why: engineering a segfault out of claim 2 would need the `t` operand to be a
`ByRef` into a global whose last 16 bytes sit exactly at the end of the last mapped page of a
PT_LOAD segment. On a normal Linux binary the read-only segment is followed immediately by the
executable and writable segments at contiguous page-aligned addresses, so `rodata_end + 0..255`
stays mapped; a repro that only crashes under a hand-crafted linker script would not be a real
reproduction.

## Hardening (kept as `hardening.diff`, deliberately NOT `patch.diff`)

`hardening.diff` inserts `band_imm(idx_lane, 0x0f)` before the `uextend`/`value_lane_dyn` in both
arms (+4/-2 lines), so the byte load is always in bounds. Verified in the worktree:

- rebuilt with `./y.sh build`;
- the emitted code now has `and w9, w7, #0xf` in front of `ldrb w9, [x0, x9]`;
- `repro/tbl1.rs` still IDENTICAL (16 lines), `repro/tbl1_sweep.rs` still IDENTICAL (6400 lines);
- `./y.sh test --skip-test testsuite.extended_sysroot`: **exit 0 before the change, exit 0 after**
  (594 passed / 0 failed both times). That suite is x86_64-only and does not execute this code
  path, so it is a "did not break anything else" check, not coverage.

It is filed as hardening rather than as `patch.diff` because there is no observable defect for it
to fix: it changes no output, and the load it removes could not fault. Whoever touches
`value_lane_dyn`'s callers next should still apply it - the same shape *was* a real
miscompilation in candidate #001 (`llvm.x86.avx2.permd`), and it is only accidental stack layout
that keeps it harmless here.

## Origin of the code (for the record, not an "IntroducedBy" for a bug)

`git blame src/intrinsics/llvm_aarch64.rs`: the `.v16i8` arm is 5d85a244 (2023-10-09), the
`.v8i8` arm 0061bb7f (2024-01-26, "Merge commit '3e50cf65...' into sync_cg_clif-2024-01-26";
rust-side commit 24361a1b "Fix portable-simd tests", 2024-01-21). The `>= 16` comparison was
reworded to `icmp_imm_u` in d28bb795 (2026-07-09). The unmasked index has been there since the
code was written.

## New suspects noticed while doing this

- **`atomic_load`/`atomic_store` for u128 are a `span_fatal`, which makes cg_clif unable to
  compile `core` for aarch64 at all** (`src/intrinsics/mod.rs:874-928`). `rustc-clif --target
  aarch64-unknown-linux-gnu` on `library/core/src/lib.rs` dies with "128bit atomics not yet
  supported" at `core/src/sync/atomic.rs:4000`, because aarch64 has `target_has_atomic = "128"`
  and `atomic_load::<u128>` gets monomorphised. In-tree this is papered over by
  `patches/0027-stdlib-128bit-atomic-operations.patch` (applied only to the sysroot build), so any
  out-of-tree `-Zbuild-std` for aarch64 hits a hard error. Related to candidate 006.
- **`aarch64-unknown-none` (and presumably every `os = "none"` target) ICEs in cg_clif**:
  `src/driver/aot.rs:52` `unwrap()` on `Backend(binary format is unknown)` from `make_module`.
  Reproduced with `rustc-clif --target aarch64-unknown-none --crate-type lib <core/src/lib.rs>`.
  A missing-binary-format mapping, unwrapped instead of reported.
- `src/intrinsics/llvm_aarch64.rs` implements `tbl1` only; `llvm.aarch64.neon.tbl2/3/4` and the
  whole `tbx*` family fall into the `_ =>` warn + `codegen_panic_nounwind` arm, so
  `vqtbl2_u8`/`vqtbx1_u8`/... compile with a warning and abort at run time. Now cheap to survey
  with `tools/aarch64_diffcheck.sh`, and the other ~1200 lines of that file have never been
  differentially tested at all.

## Files here

- `output.txt` - real captured output of all three steps above.
- `repro/tbl1.rs`, `repro/tbl1_sweep.rs` - the two aarch64 no_std programs.
- `repro/aarch64_diffcheck.sh` - copy of `___BUGHUNT___/tools/aarch64_diffcheck.sh`, so the folder
  stays self-contained (that workspace dir is not part of the upstream repo a `just clone` fetches).
- `repro/128bit_atomics_measurement.diff` - prerequisite to build `core` for aarch64 with cg_clif.
- `hardening.diff` - the index-masking change described above (not a fix for an observable bug).
- `justfile` - `just test` clones 2bc05d45, builds cg_clif, runs both programs LLVM-vs-clif, then
  dumps the disassembly. There is no `patch.diff`, so there is no apply/reproduce-after cycle.
