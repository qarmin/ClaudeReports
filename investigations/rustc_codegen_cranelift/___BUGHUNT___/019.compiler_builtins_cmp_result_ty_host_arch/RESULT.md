# 019. `CMP_RESULT_TY` for the f128 comparison libcalls is picked from the HOST's `cfg(target_arch)`

`src/compiler_builtins.rs:15-32` chooses the Cranelift return type of the
`__eqtf2`/`__netf2`/`__lttf2`/`__letf2`/`__gttf2`/`__getf2` soft-float comparison libcalls with
`#[cfg(target_arch = ...)]` / `#[cfg(target_pointer_width = ...)]` - i.e. from the machine cg_clif
itself was compiled for, not from the target it is compiling for. Lead recorded by candidates #007,
#014 and #016: #014 and #016 both ended with aarch64 `compiler_builtins` still refusing to build
after their own fixes, with `error: attempt to declare __gttf2 ... returns: I64 ... but it was
already declared ... I32`. The same `cfg` cascade also has no `i8` case, which compiler-builtins
uses on AVR, and no wasm case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/compiler_builtins.rs` against compiler-builtins' own `CmpResult` (`library/compiler-builtins/compiler-builtins/src/float/cmp.rs` in rust-src, version 0.1.160) | static reading | The upstream definition is `i32` for `target_arch = aarch64 \| arm64ec \| target_family = wasm`, `i8` for `avr`, `isize` otherwise. cg_clif's copy covers only aarch64/arm64ec + pointer width, and evaluates all of it against the **host**. So every non-aarch64 host compiling for aarch64 (and vice versa) disagrees with the libcall it is calling. |
| Which mismatching targets are actually reachable? `rustc --print cfg` + Cranelift's ISA list | 331-target list, Cranelift 0.135 (x64/aarch64/s390x/riscv64 only) | Only the aarch64 family is reachable: 32-bit and 16-bit pointer targets have no Cranelift backend at all, and neither do wasm or AVR. From an x86_64 host every aarch64 target is affected; from an aarch64 host every non-aarch64 target would be, symmetrically. |
| Compile the **real** `compiler_builtins` crate (rust-src) for `aarch64-unknown-linux-gnu`, plain rustc vs `dist/rustc-clif` | `--crate-type lib`, opt-level 0, `panic=abort`, `core` prebuilt by rustc for the same target, `--cfg f16_enabled --cfg f128_enabled --cfg feature="compiler-builtins"/"arch"/"mem"/"unmangled-names"` | **Reproduced on the first attempt.** rustc: 4802730-byte rlib. cg_clif: `error: attempt to declare __letf2 with signature ... returns: [I64] ..., but it was already declared with signature ... returns: [I32]`, then `error: aborting due to 1 previous error`. This is why aarch64 `compiler_builtins` still does not build after #014's and #016's patches. |
| Boundary-value construction: a runnable end-to-end differential. Real aarch64 `no_std` program doing 81 f128 comparisons (9x9 matrix of 0.0, -0.0, 1.0, -1.0, 3.5, 1e30, -1e30, +inf, -inf; all six operators per pair, `black_box`ed, printed via raw `write` syscall), linked static against the LLVM-built real `compiler_builtins`, executed under `qemu-aarch64` | opt-level 0, `panic=abort`, `--emit=link` through `rust-lld -flavor gnu -static -e _start`; `repro/run.sh` | **Reproduced. Silent wrong answer, no diagnostic at all.** 35 of the 81 comparison lines differ: wherever `a < b` holds, LLVM prints mask 14 (`ne\|lt\|le`) and cg_clif prints 50 (`ne\|gt\|ge`) - i.e. every strict `<`/`<=` becomes `>`/`>=`. Identical output on 5/5 and later 3/3 repeat runs of each binary. |
| Disassembly of the actual call boundary (`llvm-objdump` on the two aarch64 binaries) | - | Root cause pinned exactly (see below): compiler-builtins' `to_default_cmp_result` ends in `ldr w0, [sp, #0xc]`, so a `-1` result leaves `x0 = 0x00000000_FFFFFFFF`; cg_clif's caller declared the return as `I64` and compares the whole 64-bit register against 0, so `-1` reads as `+4294967295`. Deterministic, not uninitialised-garbage-dependent. |
| Generated sweep over the whole bug class: `repro/sweep_targets.sh` builds `core` + the real `compiler_builtins` for 7 targets with both compilers | 4 aarch64 targets (gnu/musl/apple-darwin/android) + 3 pointer-sized controls (s390x, x86_64, riscv64gc) | Before: 4/4 aarch64 targets FAIL under cg_clif (`__letf2`/`__gttf2`), 3/3 controls OK. After the fix: 7/7 OK. LLVM: 7/7 OK in both runs. |
| Control that the host path is untouched: `___BUGHUNT___/tools/diffcheck.sh` on an 11x11 f128 comparison matrix (adds NaN and `MIN_POSITIVE`) for `x86_64-unknown-linux-gnu` | opt-level 0 | Byte-identical to LLVM both before and after the patch (121 lines each). The host target is exactly the configuration that cannot see this bug, which is why `./y.sh test` never caught it. |
| `git log --all -S "CMP_RESULT_TY"` / `-S "CmpResult"` | whole history | 2 hits, pinned immediately (see "Introduced by"); no bisect needed. |

## Status

**Confirmed** - deterministic.

`repro/run.sh` against the pristine `dist/` (cg_clif @ 2bc05d45, `./y.sh build`, rustc
nightly-2026-08-24) reproduces both halves every time: cg_clif refuses the real `compiler_builtins`
crate for aarch64, and the aarch64 binary it produces for a user crate prints 35 wrong comparison
masks out of 81. The run-time half was checked 5/5 runs during investigation and 3/3 runs in the
captured `output.txt`, byte-identical each time (`md5` of the run-time output: LLVM
`671697c99028`, cg_clif `4e4b1fe93d7f`). The compile half was reproduced on 2/2 full `run.sh`
invocations and 2/2 sweeps. After the patch, all three cg_clif binaries print output identical to
LLVM's on 3/3 runs.

**Fully reproducible**: **Yes**. Every result comes from the real `dist/rustc-clif` wrapper
compiling real valid Rust (the *actual* `compiler_builtins` crate shipped in `rust-src`, and a
plain `#![no_std]` binary using ordinary `f128` `==`/`<`/`<=`/`>`/`>=` operators), and from the
resulting real aarch64 ELF actually executing under `qemu-aarch64` and printing values that are
then diffed against plain rustc/LLVM's. The callee side is not faked either: it is the real
`compiler_builtins` crate, built for the same target, both in the "prebuilt sysroot" shape
(LLVM-built, PART 2 - what a normal cross build gets) and, after the fix, in the all-cg_clif shape
(PART 3). Nothing is hand-asserted.

Note on toolchain plumbing inside the repro: there is no rustup `rust-std` for the cross target, so
`core` and `compiler_builtins` are compiled from the `rust-src` component, and the final link is
done by the `rust-lld` that ships with the toolchain (`-static -e _start`, no libc) because there is
no aarch64 C toolchain in this environment. That is ordinary bare-metal cross plumbing; the crate
under test and the code cg_clif has to compile are real. The repro also needs a **non-aarch64
host** - the bug is host-vs-target confusion, so on an aarch64 host the aarch64 target happens to be
correct (and every other target becomes wrong instead).

## Class

logic (silently wrong run-time answer: every `<`/`<=` on `f128` evaluates as `>`/`>=` in
cross-compiled code) plus, in the same root cause, a hard error that makes `compiler_builtins`
uncompilable for the whole aarch64 target family

## Root cause

`src/compiler_builtins.rs` (lines 13-32 at 2bc05d45):

    // Needs to stay in sync with compiler-builtins

    // Aarch64 uses `int` rather than a pointer-sized value.
    #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
    pub(crate) const CMP_RESULT_TY: Type = types::I32;
    ...
    #[cfg(not(any(target_arch = "aarch64", target_arch = "arm64ec")))]
    #[cfg(target_pointer_width = "64")]
    pub(crate) const CMP_RESULT_TY: Type = types::I64;

`CMP_RESULT_TY` is a `const`, so its value is fixed when **cg_clif itself** is compiled, and the
`cfg`s that select it describe cg_clif's own host. `src/codegen_f16_f128.rs`'s `fcmp` then uses that
one constant for every target it ever compiles for:

    let res = fx.lib_call(name, vec![F128, F128], vec![AbiParam::new(CMP_RESULT_TY)], &[lhs, rhs])[0];
    let zero = fx.bcx.ins().iconst(CMP_RESULT_TY, 0);
    fx.bcx.ins().icmp(int_cc, res, zero)

compiler-builtins' own `CmpResult` is a *target* property
(`library/compiler-builtins/compiler-builtins/src/float/cmp.rs`): `i32` on
aarch64/arm64ec/wasm, `i8` on AVR, pointer-sized `isize` elsewhere. So on an x86_64 host targeting
aarch64, cg_clif emits `call __lttf2 -> i64` while the function it is calling returns `i32`.

Two distinct consequences, both observed:

1. **Hard error when the crate that defines the libcall is the one being compiled.** While
   codegening `compiler_builtins` itself, cg_clif declares the *Rust* definition of `__letf2` from
   its real signature (`-> i32`) and, for `compiler_builtins`' own internal `f128` comparisons,
   declares the *libcall* `__letf2` with `-> i64`. Cranelift's module rejects the second
   declaration: `error: attempt to declare __letf2 with signature ... returns: [I64] ..., but it
   was already declared with ... returns: [I32]`. `compiler_builtins` therefore cannot be codegened
   for any aarch64 target - which is precisely the leftover blocker candidates #014 and #016 hit.

2. **Silent miscompilation everywhere else.** In a user crate the definition lives in another
   rlib, so nothing checks the signature. On AArch64 the AAPCS callee returns an `int` in `w0` and
   leaves the top half of `x0` unspecified; compiler-builtins' `to_default_cmp_result` compiles to
   `ldr w0, [sp, #0xc]`, which *zero*-extends. A `-1` ("less than") result therefore arrives as
   `x0 = 0x00000000_FFFFFFFF`, and cg_clif's `icmp slt x0, 0` - operating on the full I64 it
   declared - sees `+4294967295`. Every strict less-than becomes false and every greater-than
   becomes true. This is deterministic, not a garbage-register race.

`x86_64-unknown-linux-gnu`, the only configuration `./y.sh test` and cg_clif's CI exercise, is
exactly the configuration where host and target agree, which is why nothing in-tree notices.

The mismatch is also symmetric: an aarch64 host would pick `I32` and get it wrong for every
pointer-sized target (there, the extra 32 bits *are* meaningful, so `__lttf2`'s -1 would be
truncated rather than widened). That direction was not tested - no aarch64 host available here.

## Introduced by

992c2c6b "Use the correct return type for compiler-builtins float compares" (bjorn3, 2025-10-01),
which reached this checkout via the sync merge c90a9d83 "Merge commit 'a0b865dc8782...' into
sync_cg_clif-2025-11-08". Before that commit `fcmp` hard-coded `types::I32` with a FIXME pointing
at rust-lang/compiler-builtins#919; that was correct for aarch64 and wrong for x86_64. 992c2c6b
fixed the x86_64 case by introducing the `CMP_RESULT_TY` `cfg` cascade - and in doing so made the
aarch64 cross-compilation case wrong for the first time. Found with `git log --all -S
"CMP_RESULT_TY"` (2 hits); no bisect needed.

## The fix

`patch.diff` (2 files, +35/-11):

1. `src/compiler_builtins.rs`: replace the host-`cfg`'d `const CMP_RESULT_TY` with
   `pub(crate) fn cmp_result_ty(sess: &Session) -> Type`, which reads `sess.target.arch` and
   `sess.target.pointer_width` - i.e. the target actually being compiled for - and reproduces
   compiler-builtins' table variant for variant: `I32` for `AArch64`/`Arm64EC`/`Wasm32`/`Wasm64`,
   `I8` for `Avr`, otherwise pointer-sized (`I16`/`I32`/`I64`, with a `dcx().fatal` instead of a
   silent wrong answer for any other width). The `#[cfg(feature = "jit")] type CmpResult` used by
   the `builtin_functions!` extern declarations keeps its host `cfg`s and gains a comment saying
   why: the JIT can only ever run host code, so there host *is* target.
2. `src/codegen_f16_f128.rs`: `fcmp`'s F128 arm calls `cmp_result_ty(fx.tcx.sess)` once and uses it
   for both the `AbiParam` and the `iconst` zero it compares against.

Verified against more than the one triggering input (CONFIRMATION.md section 7):

| case | before | after |
|---|---|---|
| real `compiler_builtins` crate, `--target aarch64-unknown-linux-gnu` (`repro/run.sh` PART 1) | `error: attempt to declare __letf2 ... I64 ... already declared ... I32`, no rlib | 3724182-byte rlib produced by cg_clif |
| same, `aarch64-unknown-linux-musl` / `aarch64-apple-darwin` / `aarch64-linux-android` (`repro/sweep_targets.sh`) | all 3 FAIL (`__letf2` / `__gttf2`) | all 3 OK |
| controls `s390x-unknown-linux-gnu` / `x86_64-unknown-linux-gnu` / `riscv64gc-unknown-linux-gnu` (pointer-sized `CmpResult`) | already OK | still OK - no regression |
| 81 f128 comparisons (9x9 boundary matrix, all 6 operators) in a real aarch64 binary run under qemu, callee = LLVM-built `compiler_builtins` (PART 2) | 35/81 masks wrong (`a<b` reported as `a>b`) | 81/81 byte-identical to LLVM, 3/3 runs |
| the same aarch64 binary, callee = the `compiler_builtins` **cg_clif itself** built (PART 3) | unreachable (PART 1 fails) | 81/81 byte-identical to LLVM, 3/3 runs |
| host `x86_64` 11x11 f128 comparison matrix incl. NaN and `MIN_POSITIVE`, `diffcheck.sh` | already identical to LLVM | still identical - the host path is unchanged |

- Project test suite: `./y.sh test` was run in the isolated worktree both **before** and **after**
  the patch (in-tree items from `config.txt` plus the external repos seeded in `download/`).
  Before: **exit 0**, 61 `test result: ok.` lines, 0 `test result: FAILED`. After: **exit 0**, 61
  `test result: ok.` lines, 0 `test result: FAILED`, and the sorted multiset of per-item pass
  counts is identical to the before run (`diff` of the sorted "N passed" lines is empty). The suite
  aborts on the first failing item and neither run aborted. No new failures.

## What's not done / open questions

- The `Wasm32`/`Wasm64`/`Avr` and `I16` arms of the new `cmp_result_ty` are **unreachable today**:
  Cranelift 0.135 has backends only for x64, aarch64, s390x and riscv64, so cg_clif never sees a
  wasm, AVR or 16-bit target. They are there because the file's own contract is "needs to stay in
  sync with compiler-builtins" and the old code silently got two of them wrong; nothing in this
  session exercised them.
- The reverse direction (aarch64 host -> x86_64/s390x/riscv64 target, where the old code would pick
  `I32` for a target whose `CmpResult` is 64-bit) is fixed by the same change but was **not
  tested**: there is no aarch64 host in this environment. Only the x86_64-host -> aarch64-target
  direction was reproduced and verified.
- `f16` comparisons are not affected: `fcmp`'s `F16` arm promotes to `f32` and uses a native
  Cranelift `fcmp`, so no `__*hf2` comparison libcall is emitted. Only the `F128` arm calls a
  comparison libcall at all. Not separately verified beyond reading the code.
- Nothing was executed on a real aarch64 machine - the run-time evidence is `qemu-aarch64`
  user-mode emulation. The bug is a calling-convention/register-width disagreement that qemu models
  faithfully (and the disassembly confirms it independently), but it is emulation, not hardware.
- End-to-end `cargo -Zbuild-std=core --target aarch64-*` still does not work with cg_clif after
  this patch alone: `core` itself needs candidate #016's fix (`128bit atomics not yet supported`
  is a `span_fatal`), and the bare-metal `aarch64-unknown-none` family additionally needs candidate
  #014's fix. This candidate deliberately isolates its own bug by building `core` with plain rustc
  and handing it to cg_clif, so it needs neither patch. Whether the three together make the target
  usable was not checked.
- `src/compiler_builtins.rs` is the only place found where a **host** `cfg` decides something about
  the **target**'s ABI, but the file was the only one audited for that pattern this session; a
  sweep of `#[cfg(target_` across `src/` would be a cheap follow-up.
