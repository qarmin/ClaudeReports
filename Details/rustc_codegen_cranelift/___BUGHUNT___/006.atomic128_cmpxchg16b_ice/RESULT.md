# 006. 128bit atomics: `atomic_cas.i128` / `atomic_rmw.i128` ICE cg_clif instead of being rejected

Lead from the 2026-09-02 hodow audit: the only difference between the `-Ctarget-cpu=native` run and
the default `x86-64` baseline-ISA run was one extra failing test,
`test_x86_64_cmpxchg16b_cmpxchg16b`. Cranelift's baseline `x86-64` preset has `has_cmpxchg16b=0`,
so the suspicion was that 128-bit CAS silently traps at run time there. It does something else:
the backend ICEs at compile time.

`src/intrinsics/mod.rs` guards `atomic_load`/`atomic_store` against 128-bit types
(`"128bit atomics not yet supported"`), but the `atomic_cxchg`/`atomic_cxchgweak` arm and all
twelve read-modify-write arms have no such guard, so they hand an `I128` value straight to
Cranelift's `atomic_cas`/`atomic_rmw`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `patches/0027-stdlib-128bit-atomic-operations.patch` + `src/intrinsics/mod.rs` atomic arms | static reading | `AtomicU128`/`AtomicI128` are deleted from the sysroot's `core`, so they can't be the trigger; but `atomic_cxchg` and the rmw arms have no 128-bit guard at all, unlike `atomic_load`/`atomic_store`. Identified `core::arch::x86_64::cmpxchg16b` (stable since 1.67) as the reachable stable entry point. |
| `diffcheck.sh` differential test vs LLVM, single-file `cmpxchg16b` program | opt-level 0, default target-cpu, no extra flags | Both sides fail: LLVM emits an `__atomic_compare_exchange_16` libcall that doesn't link; cg_clif ICEs. Not a clean signal yet. |
| Same, with the flag/opt-level matrix that makes LLVM succeed | `-Ctarget-feature=+cmpxchg16b` @ opt-level 0; `-O` with no flags; `-Ctarget-cpu=x86-64 -Ctarget-feature=+cmpxchg16b`; `-Ctarget-cpu=native`; `-Ctarget-cpu=x86-64-v3` | **Reproduced.** In the first three configurations LLVM compiles and prints the correct values while cg_clif ICEs (`Error while defining ...: Compilation(Unsupported("should be implemented in ISLE: inst = 'v173 = atomic_cas.i128 v1, v2, v3'"))`). `-Ctarget-cpu=native` and `-Ctarget-cpu=x86-64-v3` work on both sides, which pins the trigger to the ISA flag rather than to the intrinsic. |
| Boundary-value / bug-class sweep over the whole 128-bit atomic surface | single file, `core::intrinsics` `atomic_{load,store,cxchg,cxchgweak,xchg,xadd,xsub,and,or,xor,nand,umax,umin,max,min}` on `u128`/`i128`, values `0`, `u128::MAX`, `1<<64`, mixed pattern, opt-level 0 | Unpatched cg_clif rejects the whole file (`atomic_load` hits its pre-existing hard error first); with the `atomic_load`/`atomic_store` calls removed the rmw arms ICE the same way (`atomic_rmw` reproduced standalone in `repro/`'s sibling test). Patched cg_clif matches LLVM byte-for-byte on all 15 operations x 4 values. |
| Read Cranelift 0.135 `src/isa/*/lower.isle` | static reading | Only the x64 backend has any `I128` atomic lowering, and all four rules (`atomic_load`, `atomic_store`, `atomic_cas`, `atomic_rmw`) are gated on `(if-let true (has_cmpxchg16b))`. aarch64/riscv64/s390x have none, so the fix's "unsupported unless x86_64+cmpxchg16b" predicate is exact. |

## Status

**Confirmed** - deterministic. The ICE reproduces 3/3 runs on the unpatched `dist/rustc-clif`
(release build of cg_clif @ 2bc05d45, rustc nightly-2026-08-24) and once more end-to-end through
`dist/cargo-clif` in `repro/`; the patched build produces LLVM-identical output 3/3 runs.

**Fully reproducible**: **Yes**. `repro/` is a real cargo crate compiled by the real
`dist/cargo-clif` wrapper; the ICE in `output.txt` is that compiler actually panicking, and the
post-fix lines are the real generated binary actually running and printing values that match plain
rustc/LLVM's on the same source. There is no second participant to fake here - the input is Rust
source handed to the real compiler entry point.

## Class

crash (backend ICE on valid Rust that LLVM compiles and runs correctly)

## Root cause

Three things line up:

1. `src/lib.rs`'s `build_isa` only ever passes `-C target-cpu` to Cranelift. `-C target-feature` is
   dropped entirely (a pre-existing FIXME, also visible in `target_config`, which reports only
   `fxsr/sse/sse2/x87`). So `-Ctarget-feature=+cmpxchg16b` leaves Cranelift's `has_cmpxchg16b` at
   the `x86-64` preset's default of `0`.
2. `src/intrinsics/mod.rs` guards only `sym::atomic_load` and `sym::atomic_store` against
   `u128`/`i128`. `sym::atomic_cxchg | sym::atomic_cxchgweak` and the twelve rmw arms
   (`atomic_xchg`/`xadd`/`xsub`/`and`/`or`/`xor`/`nand`/`max`/`min`/`umax`/`umin`) accept any
   `ty::Uint(_) | ty::Int(_) | ty::RawPtr(..)` and emit `atomic_cas`/`atomic_rmw` with whatever
   `fx.clif_type` returns - `I128` for a 128-bit type.
3. Cranelift's only `I128` atomic lowering rules are in the x64 backend and every one of them is
   gated on `has_cmpxchg16b`, so with the flag off `ISLE` finds no rule and `define_function`
   returns `Err(ModuleError::Compilation(CodegenError::Unsupported(..)))`. `src/base.rs:222` turns
   any such `Err` into `bug!(...)`, i.e. an ICE ("the compiler unexpectedly panicked. This is a
   bug"), rather than a diagnostic.

The reachable stable trigger is `core::arch::x86_64::cmpxchg16b`, which is `#[stable(since =
"1.67.0")]` and lowers to `core::intrinsics::atomic_cxchg::<u128>`. `AtomicU128`/`AtomicI128` are
deleted from cg_clif's sysroot by `patches/0027-stdlib-128bit-atomic-operations.patch`, so they are
*not* the path in; `std::arch` is. It ICEs in three configurations that LLVM handles fine:

- opt-level 0 with `-Ctarget-feature=+cmpxchg16b` (the documented way to ask for the instruction),
- opt-level 0 with `-Ctarget-cpu=x86-64 -Ctarget-feature=+cmpxchg16b`,
- `-O` with **no flags at all** - LLVM inlines `cmpxchg16b`'s body into its own
  `#[target_feature(enable = "cmpxchg16b")]` function and emits the real instruction, so a plain
  `cargo build --release` of any crate calling `std::arch::x86_64::cmpxchg16b` compiles under LLVM
  and ICEs under cg_clif.

At opt-level 0 with no flags LLVM also fails (an `__atomic_compare_exchange_16` libcall that has no
definition to link against), so that one configuration is not a cg_clif-only failure - but it is
still an ICE rather than an error.

## Introduced by

f2f54520 "Use real atomic instructions instead of a global lock" (2021-02-09) - the commit that
replaced cg_clif's global-lock atomic shim with real `atomic_cas`/`atomic_rmw` calls and introduced
all thirteen unguarded arms. At that time Cranelift had no 128-bit atomics whatsoever, and
`patches/0027` (2021-02-18) removed `AtomicU128`/`AtomicI128` from the sysroot nine days later, so
the 128-bit path was assumed unreachable. The `atomic_load`/`atomic_store` guards were added
separately (ef4512b7 "Codegen 128bit atomic loads and stores for compiler builtins as trap",
2022-03-19, and the 2022-03-20 sync commit d1904e58) and were never extended to cxchg/rmw. A
`wip_128bit_atomics` branch exists (ca67d23e "[WIP] 128bit atomics", 2023-02-01) but never landed.

Found with `git log -S "atomic_cas"` / `git log -S "128bit atomics not yet supported"`; no bisect
was needed.

## The fix

`patch.diff` (+99/-33 across `src/intrinsics/mod.rs` and `src/lib.rs`), two parts:

1. **No more ICE.** A new `reject_unsupported_128bit_atomic()` helper checks the *actual* ISA
   (`fx.module.isa().isa_flags()` for `has_cmpxchg16b`) and, when 128-bit atomics have no lowering,
   emits the same diagnostic shape `atomic_load`/`atomic_store` already used - `span_fatal`
   normally, `codegen_panic_nounwind` inside `compiler_builtins`. It is now called from all
   fourteen atomic arms (load, store, cxchg, cxchgweak and the twelve rmw ops). Because the check
   is on the real ISA flag rather than unconditional, the `atomic_load`/`atomic_store` arms also
   stop rejecting 128-bit types on targets where Cranelift *can* lower them, which is a behaviour
   improvement rather than just error-message polish.
2. **A working escape hatch.** `build_isa` now forwards `+cmpxchg16b` / `-cmpxchg16b` from
   `-C target-feature` to Cranelift's `has_cmpxchg16b` on x86_64. This is deliberately narrow: it
   is the one x86 ISA flag Cranelift consults *only* to decide whether 128-bit atomics have a
   lowering at all (verified by grepping Cranelift 0.135 - the flag is read by exactly four
   `lower.isle` rules and the `Atomic128RmwSeq`/`Atomic128XchgSeq` emitters), so honouring it
   cannot change the codegen of anything else. The broader FIXME (forward all of
   `-C target-feature`) is left alone.

Verified, per `CONFIRMATION.md` section 7, against more than the one triggering input:

| case | before | after |
|---|---|---|
| `repro/` via `dist/cargo-clif`, `-Ctarget-feature=+cmpxchg16b`, 6 boundary values x {success, failure} CAS | ICE | matches LLVM exactly (see `output.txt`) |
| `-Ctarget-cpu=x86-64 -Ctarget-feature=+cmpxchg16b` | ICE | matches LLVM |
| `-O`, no flags | ICE | clean error: "128bit atomics require the cmpxchg16b target feature; try `-Ctarget-feature=+cmpxchg16b` or a `-Ctarget-cpu` that implies it" |
| opt-level 0, no flags | ICE | same clean error (LLVM fails to link here too) |
| `-Ctarget-cpu=native` (already worked) | correct | still correct - no regression |
| `-Ctarget-cpu=x86-64-v3` (already worked) | correct | still correct - no regression |
| `just family`: `atomic_{load,store,cxchg,cxchgweak,xchg,xadd,xsub,and,or,xor,nand,umax,umin,max,min}` on `u128`/`i128`, values 0 / `u128::MAX` / `1<<64` / mixed | rejected or ICE | all 15 ops x 4 values byte-identical to LLVM |
| `atomic_xadd::<u128>` alone, `+cmpxchg16b` | ICE (`atomic_rmw.i128`) | matches LLVM |

- Project test suite (`./y.sh test` in the isolated worktree, in-tree items + the `download/`
  repos): **before the patch: exit 0. After the patch: exit 0.** Both runs emit exactly 61
  `test result: ok.` lines with an identical multiset of pass counts (largest: 2798, 1400, 1222 x5,
  1200, 1020, 924 x5) and 0 failed everywhere. The suite aborts on the first failure and neither
  run aborted.

## What's not done / open questions

- The `-O`-with-no-flags case still differs from LLVM: cg_clif errors where LLVM succeeds. Closing
  that needs cg_clif to honour per-function `#[target_feature(enable = ...)]` (LLVM only manages it
  there because inlining puts the CAS inside `cmpxchg16b`'s own `#[target_feature]` function), which
  is a much larger change and a maintainer decision. The patch turns it into an actionable
  diagnostic, not a working compile.
- Only `cmpxchg16b` is forwarded from `-C target-feature`. Every other feature is still silently
  dropped (`src/lib.rs` `target_config` FIXME); a general mapping would change the codegen of
  already-supported operations and needs its own audit. This is the same root cause as the
  `cfg(target_feature)` workaround the hodow audit needs.
- The 128-bit atomics still go through `MemFlagsData::trusted()`, i.e. cg_clif asserts the pointer
  is aligned. `cmpxchg16b` faults on a non-16-byte-aligned address, and `std::arch::x86_64::cmpxchg16b`
  documents that as the caller's contract, so this matches LLVM - but it was not tested with a
  misaligned pointer.
- `patches/0027-stdlib-128bit-atomic-operations.patch` still deletes `AtomicU128`/`AtomicI128` from
  the sysroot. With part 1 of this patch in place that removal is no longer needed for correctness
  on `-Ctarget-cpu=native`/`x86-64-v3`, but un-patching it would make `AtomicU128` compile-error on
  the default target-cpu instead of not existing - a maintainer call, not attempted here.
- Non-x86_64 targets were not exercised at all (no cross toolchain in this session). The reasoning
  that they have no `I128` atomic lowering is from reading Cranelift 0.135's `lower.isle` files, not
  from a run.
