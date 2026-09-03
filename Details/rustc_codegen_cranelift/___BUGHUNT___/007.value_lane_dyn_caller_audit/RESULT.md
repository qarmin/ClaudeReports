# value_lane_dyn_caller_audit

`CValue::value_lane_dyn` (`src/value_and_place.rs:256`) turns a runtime lane index into a bare
`imul_imm_u` + `offset_value` + load, with no bounds check and no masking, so every caller has to
mask its own index or read out of bounds. Candidate 001 proved one caller (`llvm.x86.avx2.permd`)
gets this wrong and segfaults. This candidate audits the *other* five call sites - the suspicion
being that hodow only ever fed out-of-range selectors to permd, so a second, equally unmasked
caller could be sitting there unexercised.

Call sites at HEAD 2bc05d45:

| # | site | index it passes | masked? |
|---|---|---|---|
| 1 | `src/intrinsics/llvm_x86.rs:330` `llvm.x86.ssse3.pshuf.b.128` / `llvm.x86.avx2.pshuf.b` low half | `b_lane & 0xf` | yes, `0..=15` into a 16- or 32-lane vector |
| 2 | `src/intrinsics/llvm_x86.rs:342` `llvm.x86.avx2.pshuf.b` high half | `(b_lane & 0xf) + 16` | yes, `16..=31` into a 32-lane vector |
| 3 | `src/intrinsics/llvm_x86.rs:355` `llvm.x86.avx2.permd` | raw `u32` lane | **no** - candidate 001 |
| 4 | `src/intrinsics/simd.rs:350` `sym::simd_extract_dyn` | raw `u32` argument | no, but out-of-bounds is caller UB by contract |
| 5 | `src/intrinsics/llvm_aarch64.rs:427` `llvm.aarch64.neon.tbl1.v8i8` | raw `u8` lane (0..=255) into a 16-lane table | no mask, but the loaded value is discarded by a `select` on `idx >= 16` |
| 6 | `src/intrinsics/llvm_aarch64.rs:441` `llvm.aarch64.neon.tbl1.v16i8` | same | same |

(`CPlace::write_lane_dyn`, `src/value_and_place.rs:757`, is the mirror-image hazard on the store
side and has exactly one caller, `sym::simd_insert_dyn` - included in the testing below.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read every call site against its Intel/ARM/rustc-intrinsic spec | HEAD 2bc05d45 | Only permd (site 3) is genuinely unmasked-and-wrong. Sites 1/2 mask to exactly the reachable lane range. Sites 5/6 do load out of bounds, but a `select(idx >= 16, 0, loaded)` throws the loaded byte away, which matches ARM TBL semantics, so no wrong value can escape. Site 4's contract makes an out-of-range index caller UB. |
| `simd_extract_dyn` / `simd_insert_dyn` contract check | `build/stdlib/library/core/src/intrinsics/simd/mod.rs:36-55` | Both are documented `# Safety: idx must be in-bounds of the vector`. LLVM's `extractelement`/`insertelement` yield poison out of range, so there is no oracle to differ from and no bug to find there. Every in-tree user is guarded (`assert!(index < N)` in `stdarch/crates/core_arch/src/simd.rs:68`, `% 16`/`% 8`/`% 4`/`% 2` in `core_arch/src/s390x/vector.rs:3975-4038`). |
| Differential test, `pshuf.b` boundary values (`checks/pshufb.rs`) | `diffcheck.sh`, opt-level 0; all 256 broadcast selector bytes (so every `0x00..=0xff`, i.e. every `>=0x10` in-lane-overflow and every `>=0x80` zeroing byte) plus 2000 random selector vectors, for both the 128-bit and the 256-bit form | Identical, exit 0. Also identical under `-Ctarget-cpu=native` and under `-O`. |
| Differential test, `simd_extract_dyn` / `simd_insert_dyn` sweep (`checks/extract_dyn.rs`) | `diffcheck.sh`, opt-level 0; 16 `#[repr(simd)]` types - `u8x{2,3,16,32,64}`, `i16x{5,8}`, `u32x{1,4,8}`, `i64x{2,7}`, `f32x4`, `f64x8`, `u128x2`, `i128x3` - every in-bounds index, index laundered through `black_box` so it stays dynamic; insert then re-read all lanes | Identical, exit 0. Also identical under `-Ctarget-cpu=native` and under `-O`. |
| Negative control: run candidate 001's permd bug through the same harness (`checks/permd_control.rs`) | same harness, same build | LLVM `[10, 11, 14, 17, 17, 13, 12, 11]`, cg_clif exit 139 (SIGSEGV). The harness does detect a missing index mask in this exact code, so the "identical" results above are evidence rather than a silent harness failure. |
| Generated sweep (`___BUGHUNT___/tools/lane_dyn_sweep.py`, written for this candidate) | 45 generated programs, seed 1, opt-level 0; cycles `simd_extract_dyn`/`simd_insert_dyn` over random (lane type x lane count) pairs including the non-power-of-two counts 1/3/5/7/12/31, `pshufb` with random selectors, and `permd` with in-range selectors | 45/45 agreed with LLVM. |
| Execute the two aarch64 call sites | tried `rustc --target aarch64-unknown-linux-gnu`, then `dist/cargo-clif build -Zbuild-std=core --target aarch64-unknown-linux-gnu` | Not possible in this environment - four independent blockers, see "What's not done". `qemu-aarch64` is installed, so only the *build* side is missing. |

## Status

**Not reproducible** - no second unmasked `value_lane_dyn` caller was found. Every call site that
can be executed on this host agreed with LLVM bit-for-bit on every input tried:

- `pshuf.b` (both widths): all 256 possible selector byte values in broadcast form + 2000 random
  selector vectors each, three flag configurations (default / `-Ctarget-cpu=native` / `-O`).
- `simd_extract_dyn` + `simd_insert_dyn`: 16 vector types spanning 8/16/32/64/128-bit integer and
  32/64-bit float lanes and lane counts 1,2,3,4,5,7,8,16,32,64, every in-bounds index, three flag
  configurations, plus 15 more randomly generated type/count combinations from the sweep.
- `permd` with in-range selectors: 15 generated programs, ~200-600 random index vectors each.

The permd control reproduced its SIGSEGV on every run of this session (5/5 across
`diffcheck.sh`, the candidate's own `checks/diff_run.sh` and the sweep's positive path), which is
what establishes the harness was actually capable of seeing the bug class it was looking for.

The two aarch64 sites (5 and 6) were **not executed** - see "What's not done". Their code was read
and is value-correct, but that is code reading, not a run, and it is the one gap in this null
result.

## Class

n/a (Not reproducible).

## Root cause

Why the other five sites are safe, so this spot does not get re-suspected:

- **`pshuf.b` (sites 1, 2)** - Intel's `PSHUFB` only ever consults bits `[3:0]` of each selector
  byte (and zeroes the output byte when bit 7 is set), operating within each 128-bit half. cg_clif
  applies exactly that: `band_imm_u(b_lane, 0xf)` for the low half and `+16` on top of the same
  mask for the AVX2 high half. The masked index is therefore always inside the vector it indexes,
  so the raw `value_lane_dyn` load can never leave the object. The `band_imm_u(b_lane, 0x80)` +
  `select` reproduces the zeroing rule.
- **`simd_extract_dyn` / `simd_insert_dyn` (site 4 and the `write_lane_dyn` mirror)** - the
  intrinsics' own documented safety contract is `idx must be in-bounds of the vector`, so an
  out-of-range index is caller UB and there is no defined answer for cg_clif to get wrong. LLVM
  produces poison in that case. All in-tree callers mask or assert first.
- **`tbl1` (sites 5, 6)** - the load *is* performed at an unmasked index (`u8`, so up to 255 bytes
  past the start of a 16-byte table), but the result is immediately discarded by
  `select(icmp_uge(idx, 16), 0, loaded)`, which is exactly AArch64 `TBL`'s "out-of-range index
  yields 0". So unlike permd this cannot produce a wrong value; the residue is a wild read of at
  most 255 bytes past a 16-byte stack slot, which stays inside the already-mapped stack and cannot
  fault. That is why permd is a segfault and this is not: permd's index is a full 32-bit lane, so
  its unmasked offset reaches up to 4 GiB away from the object.

Consequence for the fix shape suggested when candidate 001 was confirmed ("give `value_lane_dyn`
the lane count and mask internally, so no future caller can forget"): **a plain bitmask is not a
valid general fix.** `simd_extract_dyn`/`simd_insert_dyn` accept vectors with non-power-of-two
lane counts (`u8x3`, `i16x5`, `i64x7`, and rustc accepts far more), where `idx & (lane_count - 1)`
is not a wrap into range at all. A generic hardening would have to clamp (`umin(idx, lane_count-1)`)
or select a scratch lane, not mask - and for the two `tbl1` sites even a clamp changes which byte
is loaded (harmlessly, since the value is discarded, but it stops being a no-op refactor). The
mask-internally shape is only correct for permd itself, where `lane_count == 8`.

## Introduced by

n/a - nothing new was found. For reference, `value_lane_dyn`'s unmasked shape dates from
81ea0b29e "Merge commit '8830dccd1d4c74f1f69b0d3bd982a3f1fcde5807' into sync_cg_clif-2023-06-15"
(2023-06-15, bjorn3); only the index widening (4f24d142d, 2025-05-25) and the `imul_imm_u`
spelling (c6c38d3dc, 2026-07-22) changed since.

## The fix

No patch. Nothing reproduced, so there is nothing to fix in this candidate, and a speculative
"harden `value_lane_dyn`" patch is deliberately *not* included: as explained under "Root cause" it
would have to be a clamp rather than a mask, it changes generated code for four call sites that
are currently correct, and choosing between "clamp" and "make the lane count a required argument
and assert at each call site" is a design decision for the maintainers rather than a bug fix.

- Project test suite: `./y.sh test` in the worktree at unmodified HEAD 2bc05d45 - **passed,
  exit 0** (full in-tree items plus the external repos in `download/`; log tail is section 6 of
  `output.txt`). No patch was applied, so there is no separate "after" run; before and after are
  the same unmodified tree.

## What's not done / open questions

- **The two aarch64 `tbl1` call sites were never executed.** Reading them says they are
  value-correct, and this environment cannot produce an aarch64 binary at all - four separate
  blockers, all captured in `output.txt` section 5:
  1. no `rust-std-aarch64-*` installed, so plain `rustc` has no aarch64 `std`/`core` and there is
     no LLVM oracle;
  2. no aarch64 binutils/libc anywhere (`aarch64-linux-gnu-{as,gcc,ld}` absent,
     `/usr/lib/aarch64-linux-gnu` absent, no `ld-linux-aarch64.so`), so nothing can link;
  3. `dist/cargo-clif build -Zbuild-std=core --target aarch64-unknown-linux-gnu` fails in `core`
     with `error: 128bit atomics not yet supported` and in `compiler_builtins` with
     `error: attempt to declare __gttf2 ... returns I64 ... but it was already declared with
     ... returns I32`, so cg_clif cannot even emit an aarch64 object file;
  4. cg_clif assembles `global_asm!`/`asm!` by shelling out to the toolchain's `as`
     (`src/global_asm.rs`, `get_toolchain_binary(sess, "as")`), which on this host is the x86-64
     GNU as - so compiler_builtins' aarch64 asm routines could not be assembled even if 3 were
     fixed.
  `qemu-aarch64` *is* installed, so the missing piece is purely the build side. Standing up an
  aarch64 differential loop (e.g. `rustup target add aarch64-unknown-linux-musl` for the LLVM
  oracle plus self-contained `rust-lld` linking, after fixing blockers 3 and 4) is worth its own
  work item - `src/intrinsics/llvm_aarch64.rs` is 1241 lines and, per the project risk map, has
  never been exercised by any audit. A sibling candidate in the same batch,
  `008.aarch64_neon_tbl1_oob_load`, targets these two sites specifically - read its `RESULT.md`
  before re-doing this part.
- Blocker 3 is two apparently-genuine cg_clif bugs found incidentally, both out of this
  candidate's scope and neither written up here:
  - `src/compiler_builtins.rs:15-32` picks `CMP_RESULT_TY` from `cfg(target_arch)` of the
    **compiler binary** (host) instead of the target being compiled, so cross-compiling any `f128`
    comparison from x86-64 to aarch64 declares `__gttf2`/`__lttf2`/`__eqtf2`/... with an `I64`
    return while compiler_builtins' own definition (`CmpResult = i32` for aarch64) uses `I32` -
    hard error. Same file also has no `i8` case, which compiler_builtins uses for some arches.
    `f128` comparisons work fine on the x86-64 host target (checked: `a>b`, `a<b`, `a>=b`, `a<=b`
    all matched LLVM).
  - `error: 128bit atomics not yet supported` while building `core` for aarch64, where the default
    aarch64 Cranelift ISA presumably lacks the required primitive - the aarch64 analogue of the
    known `has_cmpxchg16b=0` x86 situation.
- `permd` was only swept with **in-range** selectors here, deliberately: out-of-range selectors
  are candidate 001's confirmed bug and would have swamped every sweep run with the same failure.
  Once candidate 001's fix lands, `lane_dyn_sweep.py`'s `gen_permd` should be widened to the full
  `i32` selector range as a regression check.
- Not covered by this audit: intrinsics that select lanes from a *compile-time immediate* rather
  than through `value_lane_dyn` (e.g. `llvm.x86.avx2.vperm2i128` and the `vperm2f128.{ps,pd}.256`
  pair at `src/intrinsics/llvm_x86.rs:363`). Their `imm8` decoding was read and looks right (bits
  `[1:0]`/`[5:4]` select, bits `[3]`/`[7]` zero) but was not differentially tested.
