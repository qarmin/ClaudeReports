<!--
Candidate 032: cmpxchg16b_atomic_u128_missing_latomic_link
-->

# cmpxchg16b_atomic_u128_missing_latomic_link

Suspect (from the batch prompt): "linking fails with `undefined reference to
__atomic_compare_exchange_16` for any crate instantiating `AtomicU128`/`AtomicI128::compare_exchange`
with `-C target-feature=+cmpxchg16b`, while an identical plain-LLVM build links and runs fine" -
because this backend's atomic-intrinsic lowering (`src/builder.rs`) always emits a libcall to the
sized `__atomic_*_16` builtins regardless of `cmpxchg16b`, and no linker-flag code path here ever
adds `-latomic`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static read of `src/builder.rs` (`compare_exchange`, `atomic_rmw`, `atomic_load`, `atomic_store`) | n/a | Confirmed these unconditionally call `context.get_builtin_function("__atomic_*_{size}")` and place a direct call, for every operand size including 16 bytes, with no size/feature-based branch to an inline `cmpxchg16b` sequence anywhere in this crate. |
| Literal repro attempt: `std::sync::atomic::AtomicU128::compare_exchange` + `-C target-feature=+cmpxchg16b`, `x86_64-unknown-linux-gnu`, both plain host `rustc`/LLVM and this backend | nightly-2026-08-04, debug build, `lto = "off"` | **Falsifies the literal premise for this target**: `AtomicU128`/`AtomicI128` don't exist here at all (`E0432`/`E0635`) on *either* backend - `std`/`core` gate those types on `#[cfg(target_has_atomic_load_store = "128")]`, and `rustc -Z unstable-options --print target-spec-json --target x86_64-unknown-linux-gnu` prints `"max-atomic-width": 64`, a fixed target-spec property independent of any `-C target-feature` flag. So "an identical plain-LLVM build links and runs fine" as literally stated (via `AtomicU128`) is not even a valid comparison on this target - it doesn't compile under LLVM either. |
| Real end-to-end repro via `core::intrinsics::atomic_cxchg`/`atomic_xadd::<u128, ...>` directly (`#![feature(core_intrinsics)]`, `repro/atomic128.rs`) - the exact same compiler intrinsics `AtomicU128`'s methods desugar to on a target where the type *is* available (`aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, both `max-atomic-width: 128`) | `y.sh rustc repro/atomic128.rs -C target-feature=+cmpxchg16b`, release sysroot, this backend vs. plain host `rustc`/LLVM, 2 independent runs each (before/after the fix) | **Reproduces**: this backend fails to link with `undefined reference to \`__atomic_compare_exchange_16'`/`__atomic_fetch_add_16'` - identically whether or not `+cmpxchg16b` is passed (2/2 runs). Plain LLVM fails identically *without* the feature, but links and runs correctly *with* it (inlines native `cmpxchg16b`, never references the libcall symbol) - this is the real point of divergence from LLVM, not "LLVM always links". |
| `-C link-arg=-latomic` as a manual workaround | same repro, this backend | Fixes the link (libatomic.so is present on this system, under `/usr/lib/gcc/x86_64-linux-gnu/15/libatomic.so`) - confirms the suspected mechanism (no code path here ever adds this) and that it's a real, available fix, just not one this backend applies automatically. |
| `git log -S`/`git log --all` on `src/builder.rs` for the atomic call sites | n/a | Found an already-existing, **not yet merged into this branch's history**, upstream fix on `origin/fix/atomic-undefined` (3 commits, most relevant: 72b88cf9 "Implement 16-byte atomic operations for x86-64 with a fallback for other architectures") that adds exactly this: an inline `cmpxchg16b`/native-asm path gated on `has_cx16` (from `-Ctarget-cpu`/`-Ctarget-feature=+cmpxchg16b`), falling back to the existing libcall path otherwise. |
| Cherry-picked 72b88cf9 onto this session's HEAD (4cd7da5f), one trivial import-line conflict in `src/builder.rs`, rebuilt, re-ran the repro and the project's own test suite before/after | `y.sh build --sysroot --release`, `y.sh test --test-libcore --mini-tests --cargo-tests` | Fixes the repro (see "The fix"); no test regressions (see below). |

## Status

**Confirmed**. Reproducible 2/2 runs (fresh `y.sh rustc` invocation, release sysroot, both before
reverting the cherry-picked fix and after re-applying it, in the same worktree).

**Fully reproducible**: **Yes**. `repro/atomic128.rs` is compiled and run through the real,
unmodified backend entry point (`y.sh rustc`, i.e. `rustc -Zcodegen-backend=librustc_codegen_gcc.so`
against a real sysroot built by this same backend) - a real end-to-end compilation and link, not a
hand-built object file or hand-authored GCC IR. This is a single-participant finding (how the
compiler backend lowers and links a given source file), not an interaction between two independent
processes/peers, so CONFIRMATION.md section 11's two-participant bar doesn't apply here in the
first place.

One caveat, stated explicitly per the loop's instructions: the *exact* code named in the original
suspicion (`std::sync::atomic::AtomicU128::compare_exchange`) is not reachable on
`x86_64-unknown-linux-gnu` at all (see table above - `target_has_atomic_load_store = "128"` is
false for this target on stock rustc, regardless of backend or `-C target-feature`). The repro
instead drives the identical codegen path via `core::intrinsics::atomic_cxchg`/`atomic_xadd::<u128,
...>` directly, which is the same intrinsic call `AtomicU128`'s methods would generate on a target
where the type IS available (`aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`). This wasn't tested
on either of those targets this session (no aarch64/macOS cross-toolchain available in this
environment) - see "What's not done" below.

## Class

crash (a build/link-time failure, not a runtime crash, but "the compiled artifact never exists" in
the same practical sense as the project's own candidate 001 classification)

## Root cause

`src/builder.rs`'s `compare_exchange` (backing `atomic_cmpxchg`), `atomic_rmw`, `atomic_load`, and
`atomic_store` all lower every atomic operation, at every operand size, into a direct call to
`context.get_builtin_function(format!("__atomic_*_{size}"))` with no size- or feature-based branch.
For 1/2/4/8-byte operands this is fine on x86-64: GCC/libgccjit recognizes those sized builtins and
lowers them to native lock-prefixed instructions unconditionally (no libcall ever appears in the
object file). For 16-byte operands, that native lowering only exists when the target has
`cmpxchg16b` - and, per the "What was tried" table, this backend applies the corresponding `cx16`
GCC feature the same way regardless of whether `+cmpxchg16b` was requested (verified: the link
failure is byte-for-byte identical with and without the feature) - so the reference to
`__atomic_compare_exchange_16`/`__atomic_fetch_add_16` always remains, unlike LLVM which, once
`+cmpxchg16b` is set, lowers the equivalent LLVM IR directly to inline `cmpxchg16b` and never emits
a call to those symbols in the first place. Since nothing in this codegen backend's own code (no
`link()` override, no native-library injection) ever requests `-latomic`, the reference is left
dangling at link time whenever it's actually generated.

## Introduced by

The unconditional-libcall pattern in `compare_exchange`/`atomic_rmw`/`atomic_load`/`atomic_store`
predates any tracked history in this repository: `git log -S "__atomic_compare_exchange_"
-- src/builder.rs` bottoms out at afae271d "Initial commit" (2020-05-10), the project's very first
commit - there was never a version of this file that handled 16-byte atomics differently. not
determined beyond "present since the initial commit"; not worth a deeper investigation given that
answer.

## The fix

Cherry-picked the relevant upstream commit 72b88cf9 "Implement 16-byte atomic operations for
x86-64 with a fallback for other architectures" (found on the not-yet-merged remote branch
`origin/fix/atomic-undefined`, based on an older point in this project's history) onto this
session's HEAD (4cd7da5f), resolving one trivial single-line import conflict in `src/builder.rs`
(an unrelated later commit had added a different import on the same line - both imports were kept).
This is not an independently-authored fix - see `patch.diff`'s provenance above - but it was built
and verified for real against this session's own tree, not merely inspected.

The fix adds a new `src/intrinsic/atomics.rs` module and threads a `has_cx16` flag (computed from
`-Ctarget-cpu`/`-Ctarget-feature=+cmpxchg16b`, mirroring how LLVM decides whether to inline) through
`CodegenCx`; `atomic_load`/`atomic_store`/`atomic_cmpxchg`/`atomic_rmw` in `src/builder.rs` now try
this inline path first (falling through to the existing unconditional-libcall path when it doesn't
apply, e.g. no `cx16`, non-16-byte size, or non-x86-64 target) - i.e. it's additive, not a rewrite
of the existing fallback.

Before/after (`repro/atomic128.rs`, `y.sh rustc ... -C target-feature=+cmpxchg16b`, release sysroot,
this exact worktree):
- Before: `error: linking with `cc` failed` / `undefined symbol: __atomic_compare_exchange_16` +
  `__atomic_fetch_add_16` (full text in `output.txt`).
- After: compiles and links; running it prints `cmpxchg: old=1 ok=true x=2`, `fetch_add: prev=10
  y=15`, `OK` (full text in `output.txt`).
- Checked the fix isn't overfit to only the compare-exchange call in the original report: also
  exercised `atomic_xadd` (`fetch_add`) at 16 bytes in the same repro - fixed identically.
- Checked the fix doesn't silently paper over the *feature-off* case: recompiling the same repro
  *without* `+cmpxchg16b` against the patched backend still correctly fails to link with the same
  undefined-reference error (matching LLVM's own behavior when the feature isn't enabled) - this is
  expected, not a regression.

- Project test suite: `./y.sh test --test-libcore --mini-tests --cargo-tests`, release sysroot,
  same worktree, full rebuild between runs:
  - Before (patch reverted): 3476 passed; 0 failed; 4 ignored (across `libcore`'s own test binary,
    `--mini-tests`, and `--cargo-tests`' `lang_tests` groups).
  - After (patch applied): 3476 passed; 0 failed; 4 ignored - identical counts, no new failures.

## What's not done / open questions

- Not tested on `aarch64-unknown-linux-gnu` or `x86_64-apple-darwin` (both have `max-atomic-width:
  128`, so `AtomicU128`/`AtomicI128` are actually reachable there through `std`'s own public API,
  which the repro on `x86_64-unknown-linux-gnu` cannot exercise) - no cross-toolchain/macOS SDK
  available in this environment. The upstream fix branch's own second and third commits
  (764d5cf/0bba9a2, "Implement atomic operations for Aarch64 via inline asm...") suggest the
  maintainers were already working through the aarch64 side of this same problem separately; not
  independently verified here.
- `patch.diff` is upstream's own not-yet-merged work (branch `origin/fix/atomic-undefined`, based
  on an older commit than this session's HEAD), rebased here by hand with one trivial conflict
  resolution - it is not a fix authored fresh for this candidate. Whether upstream's own version
  (once actually merged) matches what's cherry-picked here bit-for-bit was not re-checked after the
  fact.
- Did not check whether the same unconditional-libcall gap affects other 16-byte-only atomic
  entry points not exercised by this repro (`atomic_xchg`, `atomic_and`/`or`/`xor`/`nand`,
  `atomic_max`/`min`/`umax`/`umin` at 16 bytes) - the fix's own `atomics.rs` module covers
  `atomic_rmw` generically (not just the two ops in the repro), so these are expected to be fixed
  identically, but weren't individually re-verified.
