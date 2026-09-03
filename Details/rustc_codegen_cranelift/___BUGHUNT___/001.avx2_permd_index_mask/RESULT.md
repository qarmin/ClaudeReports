# 001. llvm.x86.avx2.permd: dynamic lane index not masked to 3 bits

`src/intrinsics/llvm_x86.rs`'s `"llvm.x86.avx2.permd"` arm (the lowering of
`_mm256_permutevar8x32_epi32` / `_mm256_permutexvar_epi32`) feeds the raw 32-bit index lane
straight into `CValue::value_lane_dyn`, which does an unchecked `imul_imm_u` + `offset_value`.
Intel's VPERMD only reads the low 3 bits of each index lane, so every index value is legal
hardware-wise; the neighbouring `pshuf.b` arm in the same file does mask (`band_imm_u(.., 0xf)`),
which is what made the missing mask here stand out. Lead came from the 2026-09-02 hodow audit
(`IDEAS.md` row 1).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the suspect arm and of `value_lane_dyn` | `src/intrinsics/llvm_x86.rs:348-362`, `src/value_and_place.rs:256` | `value_lane_dyn` has no masking or bounds handling at all; `pshuf.b` masks, `permd` does not. Confirms the shape of the suspicion. |
| Differential test vs LLVM, `tools/diffcheck.sh` | single file `tmp/c001/t1.rs`, opt-level 0, default `x86-64` ISA, 5 index bases (0, 8, 16, 0x80000000, 0xfffffff8) + one single-out-of-range lane | **DIFFERENT.** LLVM exit 0 with the correct answer for every base; cg_clif printed stack garbage for base 8, the literal index values for base 16, then SIGSEGV (exit 139). |
| Same, `RUSTFLAGS_EXTRA=-Ctarget-cpu=native` | opt-level 0 | Same divergence, so it is not a preset/ISA artifact. |
| Same, `OPT=1` | opt-level 3 | Still diverges (`#[inline(never)]` + runtime-loaded index defeats MIR GVN const-folding). |
| Boundary/random sweep, `tools/diffcheck.sh` | `tmp/c001/t2.rs`: 20 index values x 8 lane positions + 500 random `u32` index vectors, both `_mm256_permutevar8x32_epi32` and `_mm256_permutexvar_epi32` | Unpatched: 702 diverging output lines then SIGSEGV. Patched: `RESULT: identical`, exit 0. |
| Standalone `repro/` under real `dist/cargo-clif` vs plain `cargo` | 323 self-checking cases, debug build (opt-level 0), nightly-2026-08-24 | LLVM: `323 cases checked, 0 wrong`, exit 0. cg_clif @ 2bc05d45: 73 `WRONG` lines then SIGSEGV, exit 139. Full output in `output.txt`. |

## Status

**Confirmed**

Reproduces 5/5 runs (`repro/` under `dist/cargo-clif`, debug build, opt-level 0, default `x86-64`
Cranelift preset), every run exiting 139 (SIGSEGV) after the same 73 `WRONG` lines. The *set of
failing cases* is deterministic; the specific wrong values are not, because they are whatever bytes
happen to sit past the vector's 32-byte stack slot (e.g. lane 0 of the `all+8` case printed
`1239850816` in one run and `2916060688` in another - different stack addresses, same bug). It also
reproduces on `-Ctarget-cpu=native` and at `-O`.

The patched build gives output byte-identical to LLVM's (verified by diffing the two blocks of
`output.txt`), 3/3 runs, exit 0.

**Fully reproducible**: **Yes**. `repro/` is a normal cargo binary compiled by the real
`dist/cargo-clif` and executed; the wrong values and the segfault come out of that real generated
binary. LLVM's answer on the same source is the oracle and is captured in the same `output.txt`.
There is no second participant to stand in for - the input is ordinary Rust source.

## Class

logic (miscompilation) - with an out-of-bounds read in the generated code as its mechanism, so it
also manifests as a crash.

## Root cause

VPERMD's documented semantics are

```
FOR j := 0 to 7
    id  := idx[j*32+2 : j*32]          // low 3 bits only
    dst[j*32+31 : j*32] := a[id*32+31 : id*32]
```

i.e. the index lane is taken modulo 8; no index value is invalid. cg_clif instead did:

```rust
let index = idx.value_typed_lane(fx, fx.tcx.types.u32, j).load_scalar(fx);
let index = fx.bcx.ins().uextend(fx.pointer_type, index);
let value = a.value_lane_dyn(fx, index).load_scalar(fx);
```

`value_lane_dyn` (`src/value_and_place.rs:256`) turns a dynamic lane index into
`ptr + index * lane_size` with no mask and no bounds check - it is purely a pointer computation on
the `ByRef` stack slot holding `a`. So for an index lane of 8 the generated code loads 4 bytes from
32 bytes past the start of `a`'s slot: adjacent stack contents. For an index of, say,
`0xffff_ffff` the offset is 16 GiB and the load faults, which is the SIGSEGV. The wrong-value and
crash cases are the same defect at different magnitudes.

Two consequences worth separating: any `_mm256_permutevar8x32_epi32` / `_mm256_permutexvar_epi32`
call whose index lanes are not already in 0..=7 gets the wrong answer, and the same call is an
out-of-bounds read of the caller's stack frame (an information leak / fault in code that is
perfectly safe on real hardware and under LLVM).

## Introduced by

4a4535a5 "add `llvm.x86.avx2.permd` intrinsic" (2024-05-11, Folkert) - the arm has never masked
the index; found with `git log -L 348,362:src/intrinsics/llvm_x86.rs`. It reached this repo's own
history via the sync merge ed7d97e4 "Merge commit '3270432f...' into sync_cg_clif-2024-05-13"
(2024-05-13), which is what `git log -S 'llvm.x86.avx2.permd'` alone reports.

## The fix

One instruction, mirroring what the `llvm.x86.ssse3.pshuf.b.128` arm two cases above already does:

```rust
let index = idx.value_typed_lane(fx, fx.tcx.types.u32, j).load_scalar(fx);
// vpermd only looks at the low 3 bits of each index lane.
let index = fx.bcx.ins().band_imm_u(index, 0x7);
let index = fx.bcx.ins().uextend(fx.pointer_type, index);
```

`patch.diff`, +2/-0 lines in `src/intrinsics/llvm_x86.rs`.

Verified beyond the single triggering input (CONFIRMATION.md section 7): the sweep in
`tmp/c001/t2.rs` covers 20 boundary index values at each of the 8 lane positions plus 500 random
`u32` index vectors, for both `_mm256_permutevar8x32_epi32` and the AVX512VL alias
`_mm256_permutexvar_epi32` (stdarch lowers both to `llvm.x86.avx2.permd`), on the default `x86-64`
preset and on `-Ctarget-cpu=native`. Unpatched: 702 diverging lines then SIGSEGV. Patched:
`RESULT: identical` on every configuration. `repro/`'s own 323 cases go from 73 wrong + SIGSEGV to
`323 cases checked, 0 wrong`, byte-identical to LLVM.

- Project test suite: `./y.sh test` in the worktree, **exit 0 before, exit 0 after**. 61 `test
  result: ok` blocks both times with identical per-block pass counts (313, 1020, 474, 2798, 594,
  ... , 1200), so no new failures and none silently skipped. (`./y.sh test` aborts on the first
  failure, so the baseline was run with `src/` reverted before the patch was re-applied, not
  assumed.)

## What's not done / open questions

- The other `value_lane_dyn` call sites were read but not differentially tested. `pshuf.b` (x86)
  masks correctly. `llvm.aarch64.neon.tbl1.v8i8` / `.v16i8`
  (`src/intrinsics/llvm_aarch64.rs:427,441`) look like the same class in a milder form: they do
  the `value_lane_dyn` load *unconditionally* with an index of 0..=15 and only afterwards `select`
  the zero for out-of-range indices, so for `tbl1.v8i8` (an 8-byte table) indices 8..=15 perform a
  real out-of-bounds load whose value is then discarded - a potential fault rather than a wrong
  value. Untested; no aarch64 host here.
- `simd_extract_dyn` (`src/intrinsics/simd.rs:350`) also passes an unmasked index, but that
  intrinsic's contract makes an out-of-range index UB, so it is not obviously a bug.
- A cleaner alternative to per-call-site masking would be for `value_lane_dyn` to take the lane
  count and mask/clamp itself, so a future intrinsic cannot forget. That is a maintainer call, so
  the patch here stays minimal and local.
- `llvm.x86.avx2.permps` (`_mm256_permutevar8x32_ps`) is not implemented at all in this backend, so
  the float twin of this bug cannot be reached yet; whoever adds it will need the same mask.
