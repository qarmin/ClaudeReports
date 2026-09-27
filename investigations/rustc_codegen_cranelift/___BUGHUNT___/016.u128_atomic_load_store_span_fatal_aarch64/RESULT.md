# 016. `atomic_load`/`atomic_store` on `u128` is a `span_fatal`, so cg_clif cannot codegen `core` for aarch64 / s390x / x86_64-apple-darwin

Lead from candidates #007 and #008: `src/intrinsics/mod.rs` rejects 128-bit `atomic_load` /
`atomic_store` with `dcx().span_fatal(.., "128bit atomics not yet supported")` for every crate
except `compiler_builtins`. Since `core` itself instantiates those intrinsics on any target where
`target_has_atomic = "128"` holds, the suspicion was that cg_clif cannot build a sysroot for
aarch64 at all out of tree, and that in-tree this is only hidden by
`patches/0027-stdlib-128bit-atomic-operations.patch` deleting `AtomicU128`/`AtomicI128` from the
sysroot's `core`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/intrinsics/mod.rs` atomic arms + `patches/0027-...` + Cranelift 0.135 `isa/*/lower.isle` | static reading | The `span_fatal` is unconditional outside `compiler_builtins`. Cranelift 0.135 has `I128` atomic lowering rules **only** in `isa/x64/lower.isle` (4 rules: `atomic_load`, `atomic_store`, `atomic_cas`, `atomic_rmw`), each gated on `has_cmpxchg16b`; `isa/aarch64`, `isa/s390x`, `isa/riscv64` have none. Patch 0027 only removes the `atomic_int!` invocations guarded by `target_has_atomic_load_store = "128"`, so it is a no-op for the in-tree x86_64 sysroot and load-bearing only for the 128-bit-atomic targets. |
| `rustc --print cfg --target <triple>` inventory of which targets set `target_has_atomic="128"` | 7 triples | aarch64-unknown-linux-gnu, aarch64-apple-darwin, aarch64-unknown-none, s390x-unknown-linux-gnu, x86_64-apple-darwin: **yes**. x86_64-unknown-linux-gnu, i686-unknown-linux-gnu, riscv64gc-unknown-linux-gnu: no. So the affected set is "every aarch64 target, s390x, and x86_64 macOS", and plain x86_64 Linux (the only configuration cg_clif's own CI exercises) is exactly the one that cannot see the bug. |
| Differential `-Zbuild-std=core` cross build, plain rustc vs `dist/cargo-clif`, via a new script `___BUGHUNT___/tools/buildstd_core_check.sh` | `#![no_std]` lib crate, opt-level 0, 4 targets | **Reproduced.** LLVM: 4/4 OK. cg_clif: aarch64-unknown-linux-gnu, s390x-unknown-linux-gnu and x86_64-apple-darwin all fail with `error: 128bit atomics not yet supported` at `library/core/src/sync/atomic.rs:4000:23`; x86_64-unknown-linux-gnu (the control, no `target_has_atomic="128"`) succeeds on both sides. |
| Host-side single-file differential (`diffcheck.sh`) so the bug also has a *runnable* form | `core::intrinsics::atomic_{load,store}` on `u128`/`i128`, 8 u128 + 6 i128 boundary values, opt-level 0, `-Ctarget-cpu=native` | **Reproduced.** LLVM compiles and prints all 14 lines; cg_clif refuses the program with the same hard error even though the ISA it was handed (`native` = Zen 4, `has_cmpxchg16b=1`) does have a lowering for it. |
| Boundary-value construction around the operation | 0, 1, `u128::MAX`, `u128::MAX-1`, `1<<64`, `(1<<64)-1`, `1<<127`, a mixed nibble pattern; `i128::MIN`, `i128::MAX`, -1, 0, `-(1<<64)`, `1<<100`; orderings SeqCst/Relaxed/Acquire | Irrelevant to *triggering* (it is a type-based hard error, so every value fails identically) but used to verify the fix: all 14 values x 3 orderings must match LLVM afterwards, which they do. |
| `git log -S` / `git show` archaeology | `-S "128bit atomics not yet supported"` over all refs | 3 hits; the arm was introduced by ef4512b7 (below). No bisect needed. |
| Wider bug class after the fix: a real `AtomicU128` user | `core::sync::atomic::AtomicU128::{load,store}` + `fetch_add` + `compare_and_swap`, `-Zbuild-std=core --target s390x-unknown-linux-gnu` | load/store now compile (3 warnings); `fetch_add`/`compare_and_swap` hit a **different** pre-existing bug - `atomic_rmw.i128`/`atomic_cas.i128` ICE the backend (`should be implemented in ISLE`), which is candidate 006, not this one. Confirms the patch's scope is exactly load/store. |

## Status

**Confirmed** - deterministic. `just before` against the unpatched `dist/` reproduces identically
**3/3 runs**: 3 targets hard-erroring with `128bit atomics not yet supported`, no aarch64 `libcore`
rlib produced, and the host program refused. `just after` against the patched worktree is
identically clean **3/3 runs**. Both are release builds of cg_clif @ 2bc05d45 with rustc
nightly-2026-08-24, test programs at opt-level 0.

**Fully reproducible**: **Yes**. Every line in `output.txt` is the real `dist/cargo-clif` /
`dist/rustc-clif` wrapper actually running: the "before" half is the real backend actually
hard-erroring while codegening the real `library/core`, and the "after" half is the real generated
x86_64 binary actually running and printing 14 values that match plain rustc/LLVM's byte for byte,
plus the real aarch64 `libcore-*.rlib` (4749168 bytes) that cg_clif now produces. There is no
second participant to fake - the input is Rust source handed to the real compiler entry point.

## Class

crash (backend hard-errors on valid Rust that LLVM compiles; it makes three whole target families
uncompilable rather than just making one operation fail)

## Root cause

`src/intrinsics/mod.rs`, the `sym::atomic_load` and `sym::atomic_store` arms (lines 874-928 at
2bc05d45), both contain:

    ty::Uint(UintTy::U128) | ty::Int(IntTy::I128) => {
        // FIXME implement 128bit atomics
        if fx.tcx.is_compiler_builtins(LOCAL_CRATE) {
            // special case for compiler-builtins to avoid having to patch it
            crate::base::codegen_panic_nounwind(fx, "128bit atomics not yet supported", ..);
            return Ok(());
        } else {
            fx.tcx.dcx().span_fatal(source_info.span, "128bit atomics not yet supported");
        }
    }

Three things combine:

1. The rejection is a `span_fatal` (a hard, unrecoverable error) for every crate that is not
   `compiler_builtins`, and it is unconditional - it does not look at whether the target ISA can
   actually lower the operation.
2. `core` is such a crate, and it *does* instantiate `core::sync::atomic::atomic_load::<u128>` /
   `atomic_store::<u128>` whenever `target_has_atomic_load_store = "128"` is set, because
   `AtomicU128`/`AtomicI128` exist on those targets and their `#[inline]` methods are codegened
   into `core`'s own CGUs. That cfg is set for all aarch64 targets, s390x and x86_64-apple-darwin.
   So `core` cannot be codegened for those targets at all - not the operation, the *whole crate*,
   and therefore the whole target.
3. `rustc`'s `TargetConfig` (what `src/lib.rs`'s `target_config` returns) has no knob for the
   maximum atomic width, so cg_clif cannot tell rustc "this backend has no 128-bit atomics" and
   have `core` stop defining `AtomicU128`. The only existing workaround is
   `patches/0027-stdlib-128bit-atomic-operations.patch`, which physically deletes `AtomicU128` and
   `AtomicI128` from the sysroot source - and that patch only applies to cg_clif's *own* in-tree
   sysroot build. Any out-of-tree `-Zbuild-std` (or a `./y.sh build --target aarch64-*` sysroot,
   or any attempt to cross-compile with cg_clif) gets the unpatched `core` and the hard error.

Because `rustc --print cfg --target x86_64-unknown-linux-gnu` does *not* list
`target_has_atomic="128"`, plain x86_64 Linux - the one configuration cg_clif's own `./y.sh test`
runs in - is precisely the configuration that cannot observe this.

## Introduced by

ef4512b7 "Codegen 128bit atomic loads and stores for compiler builtins as trap" (bjorn3,
2022-03-19). That commit added both branches of the arm above in one go; its own message reads
"128bit atomics are unstable and only enabled on AArch64 and x86_64 macOS. Cranelift doesn't
support 128bit atomics yet", i.e. the non-x86_64 targets were known to be the affected ones and the
`compiler_builtins` escape hatch was added only for `compiler_builtins`. `core` was handled the
other way, by 537db9a1 "Disable 128bit atomic operations on macOS" (2021-02-18), the commit that
added `patches/0027-Disable-128bit-atomic-operations.patch`, which predates it. It reached this checkout
via the sync commit d1904e58 ("Merge commit '370c397ec...' into sync_cg_clif-2022-03-20"); the
`sess.span_fatal` -> `dcx().span_fatal` rename happened later in d9025ceb. Found with
`git log --all -S "128bit atomics not yet supported"`; no bisect needed.

## The fix

`patch.diff` (+43/-30, all in `src/intrinsics/mod.rs`), two helpers plus the two call sites:

1. `isa_supports_128bit_atomics(fx)` asks the *actual* ISA being compiled for
   (`fx.module.isa()`): x86_64 **and** `has_cmpxchg16b` set. This mirrors Cranelift 0.135's real
   capability exactly - those are the only conditions under which any `I128` atomic lowering rule
   exists.
2. `codegen_unsupported_128bit_atomic(..)` replaces the `span_fatal` with the shape the backend
   already uses for unsupported `llvm.*` intrinsics (`src/intrinsics/llvm_x86.rs:1352`): a
   `span_warn` naming the intrinsic, plus `codegen_panic_nounwind` so the operation aborts at run
   time if it is ever reached. The `compiler_builtins` special case collapses into this (it just
   skips the warning, as before).

Consequence: `core` codegens for aarch64/s390x/x86_64-apple-darwin, a program that never touches a
128-bit atomic is unaffected, one that does aborts with `128bit atomics not yet supported` instead
of taking the whole crate graph down at compile time, and on an ISA that *can* do it
(`-Ctarget-cpu=native`, or any `-Ctarget-cpu` implying cmpxchg16b) cg_clif now emits the real
instruction sequence instead of erroring.

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7:

| case | before | after |
|---|---|---|
| `-Zbuild-std=core --target aarch64-unknown-linux-gnu` (`just aarch64-core`) | `error: 128bit atomics not yet supported`, no `libcore` rlib | `libcore-*.rlib` (4749168 bytes) produced by cg_clif |
| `-Zbuild-std=core --target s390x-unknown-linux-gnu` | same hard error | builds clean, exit 0 |
| `-Zbuild-std=core --target x86_64-apple-darwin` | same hard error | builds clean, exit 0 |
| `-Zbuild-std=core --target x86_64-unknown-linux-gnu` (control, no 128-bit atomics) | already OK | still OK - no regression |
| host program, `-Ctarget-cpu=native`, `atomic_load`/`atomic_store` on u128 x 8 boundary values x 3 orderings and i128 x 6 boundary values | hard error, no binary | compiles, runs, output byte-identical to plain rustc/LLVM |
| host program, default `x86-64` ISA (no cmpxchg16b) | hard error | 5 warnings naming each call site + the binary aborts with `128bit atomics not yet supported` at the first one (exit 134) |
| real `core::sync::atomic::AtomicU128::{load,store}` user, `--target s390x-unknown-linux-gnu` (`just atomic-u128`) | `core` itself doesn't build, so unreachable | builds, 3 warnings |
| real `AtomicU128::fetch_add` / `compare_and_swap` user, same target | unreachable | reaches and exposes the *separate* `atomic_rmw.i128` ICE (candidate 006) - unchanged by this patch, as intended |

- Project test suite (`./y.sh test` in the isolated worktree, in-tree items + the `download/`
  repos): **before the patch: exit 0. After the patch: exit 0.** Both runs emit exactly 61
  `test result: ok.` lines with an *identical* multiset of pass counts (`diff` of the sorted
  "N passed" lines is empty) and 0 `test result: FAILED`. The suite aborts on the first failure and
  neither run aborted.

## What's not done / open questions

- `aarch64-unknown-linux-gnu` and `aarch64-apple-darwin` still cannot be built end to end after
  this patch, because of a **second, unrelated cg_clif bug** this candidate uncovered:
  `src/compiler_builtins.rs:15-32` picks `CMP_RESULT_TY` with `#[cfg(target_arch = ...)]`, i.e.
  from the **host**, not from the target being compiled for. Cross-compiling x86_64 -> aarch64
  therefore declares `__gttf2`/`__lttf2`/... returning `I64` while `compiler_builtins` defines them
  returning `I32` (correct for aarch64), and the build dies with `error: attempt to declare
  `__gttf2` with signature ... returns: I64 ... but it was already declared with ... I32`. This is
  also a latent *silent* ABI bug for any f128 comparison in a cross-compiled crate. Not fixed here
  (it is a different root cause in a different file); s390x and x86_64-apple-darwin do build fully,
  which is what demonstrates this candidate's fix.
- The 128-bit atomic **rmw/cas** arms are untouched: they have no 128-bit guard at all and ICE
  (`should be implemented in ISLE: inst = atomic_rmw.i128 ...`). That is candidate 006. A combined
  fix would route all fourteen atomic arms through `codegen_unsupported_128bit_atomic`; this patch
  deliberately does not, to keep the two findings separable.
- Because `-Zbuild-std` compiles sysroot crates with `--cap-lints allow`, which turns off
  `can_emit_warnings` for the whole session, the new `span_warn` is **not** printed while `core`
  itself is being built - `core`'s own `AtomicU128` instantiations become traps silently. A user
  crate does see the warning (section 4 of `output.txt`). Making the diagnostic survive
  `--cap-lints` (or emitting it as a lint that can be denied) is a maintainer call.
- `patches/0027-stdlib-128bit-atomic-operations.patch` was left in place. With this patch it is no
  longer needed to make a sysroot build succeed, but removing it would change `AtomicU128` from
  "does not exist" to "exists and traps" for in-tree sysroots, which is a behaviour decision.
- Nothing was *executed* on aarch64 or s390x: there is no aarch64/s390x linker or libc in this
  sandbox (only `qemu-aarch64`), so the cross-target evidence is compile-only (`core` codegens, the
  rlib exists). The run-time evidence is the x86_64 `-Ctarget-cpu=native` path, where Cranelift's
  own lowering is exercised for real.
- `aarch64-unknown-none` also sets `target_has_atomic="128"` but was not tested here; it has its own
  separate blocker (candidate 014).
