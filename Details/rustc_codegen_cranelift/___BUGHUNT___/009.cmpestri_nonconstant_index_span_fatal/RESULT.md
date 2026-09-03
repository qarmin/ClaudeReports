# 009.cmpestri_nonconstant_index_span_fatal

`src/intrinsics/llvm_x86.rs` hard-errors with `span_fatal` when the immediate operand of
`llvm.x86.sse42.pcmpestri128` / `pcmpestrm128` / `pclmulqdq` / `aesni.aeskeygenassist` /
`sha1rnds4` / `vcvtps2ph.128` is not recognised as a compile-time constant (lines 771, 824, 871,
908, 1066, 1331), and `src/intrinsics/simd.rs` does the same for `simd_insert` / `simd_extract`
(lines 274, 324). The recognition is done by a heuristic whole-body MIR scan
(`src/constant.rs:498 mir_operand_get_const_val`), so the suspicion was that a debug build (no MIR
inlining, no const-prop) of a crate that wraps these behind a const-generic wrapper could leave the
immediate as a plain operand and get a hard error on valid Rust.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Read `mir_operand_get_const_val` and enumerate every path that returns `None`, then check what MIR stdarch actually produces for each of the 8 call sites | `src/constant.rs:498-609`, `src/intrinsics/llvm_x86.rs`, `src/intrinsics/simd.rs`, stdarch sources under `build/stdlib/library/stdarch/` | 5 distinct `None` paths found (place with projection; second assignment to the same local in the `Cast` arm; widening cast `Ordering::Greater`; any `InlineAsm`/`TailCall` terminator or `CopyNonOverlapping` statement *anywhere* in the body; any other `Rvalue`). None of them is producible from stdarch's actual bodies, which are one `Cast` statement plus the call. `_mm_cvtps_ph` and `_mm256_cvtps_ph` pass the const generic with no cast at all, so their operand is already `Operand::Constant`. |
| 2. Generated call-shape sweep, differential against LLVM (`___BUGHUNT___/tools/constarg_sweep/`, also copied into `probe/`) | 17 shapes (literal / const-generic wrapper / `#[inline]` wrapper / two branches same imm / two branches different imm / loop / closure / 3-arm match / recursion / two monomorphisations in one body / `asm!` in the same body) x 8 flag sets (`-Copt-level=0,1,2,3,s,z`, `-Copt-level=0 -Zinline-mir=yes`, `-Copt-level=3 -Zmir-enable-passes=-GVN`), covering `_mm_cmpestri`, `_mm_cmpestrm`, `_mm_cvtps_ph`, `_mm_sha1rnds4_epu32`, `_mm_clmulepi64_si128`, `_mm_aeskeygenassist_si128`, `_mm_insert_epi8`/`_mm_extract_epi8` = 136 cg_clif compiles + 136 rustc compiles | 134/136 cells compile. The only 2 `NOTCONST` cells are `-Copt-level=0 -Zinline-mir=yes` and `-Copt-level=3 -Zmir-enable-passes=-GVN`, i.e. nightly MIR-pass-schedule overrides. Every default and every stable `-C` flag set is clean. |
| 3. Boundary construction of the exact MIR shape the scan fails on, plus a stable-flag matrix, plus a check of whether the `simd_insert`/`simd_extract` sites are reachable at all | `probe/asm_poison.rs` (an `asm!` and `_mm_cmpestri::<0>` in one `#[target_feature(enable="sse4.2")]` fn); flags `-Cdebug-assertions=on`, `-Zub-checks=yes`, `-Ccodegen-units=1`, `-Cdebuginfo=2`, `-Cinstrument-coverage`, `-Clto=fat`, `-Cpanic=unwind`; `probe/simd_extract_cast_index.rs` calling `core::intrinsics::simd::simd_extract(v, IDX as u32)` | The mechanism is real but only with the `-Z` overrides above (see output.txt section 2 for the failing MIR). `-Cinstrument-coverage`/`-Clto=fat`/`-Cpanic=unwind` are rejected by cg_clif for unrelated reasons; the rest compile fine. The `simd_insert`/`simd_extract` sites are dead code: rustc's own frontend rejects a non-`const`-item index (`error: 2nd argument of simd_extract is required to be a const item`) before codegen, identically under LLVM and cg_clif. |
| 4. Corpus cross-check (no new run needed) | Existing `___BUGHUNT___/tools/hodow_audit/` logs from the 2026-09-02 run: 19154 self-checking x86 intrinsic tests built with cg_clif at opt-level 0, 38 of them covering `cmpestri`/`cmpestrm`/`sha1rnds4`/`cvtps_ph`/`clmulepi64`/`aeskeygenassist` | `zgrep -c "is not a constant"` = 0 across all preserved logs; the build completed and the tests ran, so none of the 8 `span_fatal`s fired on that corpus either. |

## Status

**Not reproducible**

The candidate's specific hypothesis - "a debug build (`-Copt-level=0`) of a crate that wraps these
intrinsics behind a const-generic wrapper still leaves the immediate as an operand and hard-errors"
- is wrong, and demonstrably so:

- At `-Copt-level=0`/`1` there is no MIR inlining, so the `llvm.x86.*` call always sits inside
  stdarch's own two-statement body (`_9 = const IMM8 as i8; call pcmpestri128(..., move _9)`),
  which the `Rvalue::Cast` arm of `mir_operand_get_const_val` resolves without difficulty. A
  const-generic wrapper in user code makes no difference: `IMM8` is substituted by
  monomorphisation before codegen ever sees the body.
- At `-Copt-level>=1` (and `s`/`z`) `mir_opt_level` becomes 2, which enables **both** `Inline`
  **and** `GVN`. Inlining is the only thing that can move the call into an arbitrary caller body,
  and whenever it happens GVN has already folded `const IMM8 as i8` into a literal, so the operand
  arrives as `Operand::Constant` and the whole heuristic scan is bypassed. The two regimes never
  overlap in a way that exposes the scan.
- The `simd_insert`/`simd_extract` variants of the same `span_fatal` are unreachable outright,
  because rustc type-checks the index as a required `const` item first.

Coverage, so a future session does not retry this: 136 cg_clif compiles over 17 call shapes x 8
flag sets (`probe/run.sh`, matrix in output.txt), the additional stable-flag matrix in approach 3,
and the 19154-test hodow corpus. `-Copt-level=0` (the candidate's own premise) was covered for all
17 shapes.

**Fully reproducible**: n/a (nothing reproduced under default flags).

## Class

n/a - not reproducible.

## Root cause

Why the suspect is safe today, and exactly how thin the margin is:

`mir_operand_get_const_val` does not evaluate the operand - it walks **every statement and
terminator of the entire enclosing MIR body**, looking for a single defining assignment of the
operand's local, and bails out to `None` on anything it does not understand. Two of those bail-outs
are whole-body poisons rather than definition-related: `TerminatorKind::InlineAsm { .. } => return
None` and `NonDivergingIntrinsic::CopyNonOverlapping(..) => return None` fire regardless of which
local they touch, and they fire even *after* the correct constant has already been found (the loop
keeps scanning and `return None` discards `computed_scalar_int`). So "does this immediate have a
constant value" is answered as "does anything, anywhere in this function, look unfamiliar".

That is only harmless because of the pass-schedule coincidence above. Forcing the two regimes to
overlap makes it fire immediately: `-Copt-level=0 -Zinline-mir=yes` (inlining on, GVN off) on a
function containing one `asm!` and one `_mm_cmpestri::<0>` gives

    error: Index argument for `_mm_cmpestri` is not a constant

on a program LLVM compiles and runs (`0 42`) - see output.txt section 2. The dumped MIR there shows
the immediate as `_9 = const 0_i32 as i8`, perfectly resolvable; the scan fails purely because
`bb0` holds an unrelated `asm!` terminator.

Two further latent weaknesses in the same function, neither reachable today, both recorded as new
suspects:

- The `Rvalue::Use` arm does `computed_scalar_int = mir_operand_get_const_val(fx, operand)` with
  **no** `computed_scalar_int.is_some()` guard, unlike the `Rvalue::Cast` arm right above it, which
  returns `None` on a second assignment. A body with two `Use` assignments of *different* constants
  to the same local would therefore silently take whichever comes last in MIR statement order - a
  wrong immediate baked into the emitted `pcmpestri`/`pclmulqdq`/`vcvtps2ph`, i.e. a silent
  miscompilation rather than an error. Same shape also lets a later `None` erase an earlier good
  value.
- A widening cast (`Ordering::Greater => return None`) is unhandled. No stdarch intrinsic currently
  widens its immediate (all const generics are `i32` and all casts are narrowing or same-size), so
  it is unreachable, but a future stdarch signature change would turn it into a hard error.

## Introduced by

not determined, because nothing reproduces - there is no buggy commit to point at. For context, the
oldest `span_fatal` of this family is 0416f122 "Error on non-constant simd_insert/simd_extract
index" (2020-10-16), the heuristic scan around it is the `FIXME(rust-lang/rust#85105)` workaround at
`src/constant.rs:505` for `IMM8 as u32` casts, and the `TerminatorKind::InlineAsm { .. } => return
None` poison came in with d6b03451 (sync_cg_clif-2021-05-27).

## The fix

No patch. There is nothing reproducible to fix at default flags, and hardening the scan (making the
`InlineAsm`/`CopyNonOverlapping` bail-outs local to the scanned place, and giving the `Rvalue::Use`
arm the same double-assignment guard as the `Rvalue::Cast` arm) would be a robustness change with no
failing test to validate it against - exactly the "patch that papers over nothing" this workspace's
ground rules say not to force.

- Project test suite: `./y.sh test` in
  `tmp/wt/009.cmpestri_nonconstant_index_span_fatal` (unpatched baseline), exit 0 - all `[AOT]`/
  `[JIT]` items plus the external repos (`rust-random/rand`, `regex`, `graviola`, `portable-simd`,
  `sysroot_tests`) passed. No "after" run: no patch was applied.

## What's not done / open questions

- `src/intrinsics/llvm_aarch64.rs:467` and `:485` use the same `mir_operand_get_const_val` for an
  `imm32` argument (via `.unwrap_or_else(|| span_bug!(...))`, so a `None` there is an ICE, not a
  diagnostic). Not exercised - this is an x86 host and the aarch64 intrinsic file has never been
  audited (see the risk map).
- The `Rvalue::Use` double-assignment gap described above is a *miscompilation* class, not an error
  class, and was not chased to a reproduction. It needs a MIR body with two `Use` assignments of
  different constants to one local reaching a const-arg intrinsic; no default-flag construction for
  it was found in this session.
- `llvm.x86.vcvtps2ph.256` (`_mm256_cvtps_ph`) has no handler at all and falls into the warn+trap
  arm, while `.128` is implemented. Noticed while reading `src/intrinsics/llvm_x86.rs:1319`; it is
  an instance of the already-known unimplemented-intrinsic class, not a new class.
- Whether cg_clif should reject `-Zinline-mir=yes` at `-Copt-level=0` (or, better, not depend on the
  pass schedule at all) is a maintainer decision, not something this candidate settles.
