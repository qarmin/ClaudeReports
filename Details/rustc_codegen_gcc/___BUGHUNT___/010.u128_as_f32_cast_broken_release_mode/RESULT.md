# u128_as_f32_cast_broken_release_mode

Issue #705 ("Cast from u128 to f32 is broken in release mode") reports that the upstream rustc
test `tests/ui/numbers-arithmetic/u128-as-f32.rs` (now permanently marked `//@ ignore-backends:
gcc` upstream) started failing in release mode after a libgccjit rebase, and that printing the
`f32` value first makes the test pass - a classic reordering/optimizer-sensitive symptom. The
candidate hypothesis was a missing barrier/volatile issue in `int_to_float_cast`/
`gcc_int_to_float_cast` (`src/int.rs`) for the manually-lowered 128-bit-to-float conversion.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential boundary-value construction: ported the exact literals/expected results from upstream's `u128-as-f32.rs` into a standalone `black_box`-guarded program, compiled with plain `rustc`/LLVM (host toolchain, `rustc 1.99.0-nightly`) as ground truth, and with this backend both with (`-O`) and without (`-C opt-level=0`) optimizations | `CHANNEL=release ./y.sh rustc`, release build of the backend; single run each | rustc/LLVM: passes. This backend without optimization: passes. This backend with `-O` (also reproduces at `-C opt-level=1` and `2`, not just `3`): panics on `assert_eq!(black_box::<u128>(0xffffff80000000000000000000000000u128) as f32, f32::INFINITY)` with `left: inf, right: inf` - i.e. the exact symptom quoted in the issue's own comments. |
| Bisection of the original 13-assertion test down to a minimal 2-line repro (`repro/u128_as_f32.rs`), rebuilding and rerunning after each reduction step | Same release build; ~10 intermediate variants tried (see below) | The bug requires (a) `-O`/opt-level >= 1, (b) two consecutive `u128 as f32` casts of `black_box`-guarded values reached through an `assert_eq!` macro whose cast expression is written inline as the macro's own argument (not first bound to a `let`, which does not reproduce - see next row) |
| Static + dynamic disassembly (`--emit=asm`) of the minimal repro, and a follow-up hand-written variant that keeps the boolean `b == f32::INFINITY` in a local variable and prints it before re-asserting | Same release build | The generated code for the *first* `assert_eq!` does a real `vucomiss`+conditional-jump comparison against `f32::MAX`'s bit pattern (correct). For the *second*, GCC emits a real call to `__floatuntisf` (confirming this environment's libgccjit lacks native 128-bit-int support, `is_native_int_type` is false here) whose result is stored (for the panic message only) but is never actually compared - the branch that would do the comparison is gone; `main` falls straight into the `assert_failed` call unconditionally. A follow-up variant printing `b == f32::INFINITY` as `eq` shows `eq=false` (bits printed alongside: `bits=7f800000`, the exact canonical `+inf` bit pattern, equal to `f32::INFINITY.to_bits()`), yet a *second*, lexically-identical `assert_eq!(b, f32::INFINITY)` immediately below that same `println!` does **not** panic in that variant - i.e. the very same logical comparison evaluates differently depending only on the surrounding code shape, not on the actual (verified-correct) runtime value. |

## Status

**Confirmed.** Reproduces 3/3 runs (`release`-built backend, `-O`), deterministically (not
flaky/seed-dependent) - see `output.txt` for one full captured run and `repro/run.sh`'s own
three-way (rustc/LLVM reference, backend `-O0`, backend `-O`) comparison.

**Fully reproducible**: Yes - `repro/u128_as_f32.rs` is compiled and run through the real,
unmodified `rustc` + `rustc_codegen_gcc` pipeline (`./y.sh rustc`), the same real entry point an
end user hits; nothing about the trigger is faked or hand-simulated.

## Class

logic (miscompile - wrong result from a correctly-typed, fully-defined program; the process does
not crash, it produces or reports an incorrect equality result)

## Root cause

This is a genuine optimizer-level miscompilation, not a bug in this backend's own MIR-to-GCC-IR
translation logic for the cast itself. On this environment's libgccjit build,
`is_native_int_type` is false for u128 (no native GCC `__int128` support), so
`int_to_float_cast` (`src/int.rs:944-976`) takes its extern-call path, emitting a real call to
`__floatuntisf`. That call executes at runtime and computes the mathematically correct result
(verified directly: the resulting bit pattern is `0x7f800000`, canonical `+infinity`, identical to
`f32::INFINITY`'s own bits). The bug is that GCC's optimizer, at `-O`/opt-level >= 1, statically
decides the subsequent equality comparison against `f32::INFINITY` is always false and removes the
runtime comparison + branch entirely for the *second* of two consecutive `black_box(..) as f32`
casts reached through an inline `assert_eq!` argument - even though the same comparison, computed
from the same (correct) runtime value, evaluates true when structured slightly differently in
source (see the "print `eq` then assert" experiment above). This means GCC is propagating a wrong
compile-time-folded outcome for a comparison whose operand is nominally opaque (guarded by
`std::hint::black_box`, itself implemented in `src/intrinsic/mod.rs`'s `sym::black_box` arm via an
empty volatile inline-asm with an address input operand plus a blanket `"memory"` clobber). Since
the same source-level equality check is sometimes folded and sometimes not, purely as a function of
surrounding code shape rather than of the (always-correct) runtime value, the fault sits in GCC's
own constant-propagation/range-folding of a call reachable through this backend's `black_box`
barrier - matching the issue's own history of this test only breaking after a specific upstream
GCC/libgccjit rebase (`b8f4bbf1b995d9511bad6652e8d774620b2bd833` -> `3e3878e7183f1d1e1ccf5aba99da7fbb36c012c8`
in the `rust-lang/gcc` fork, per the issue body), not this repository's own commit history.

## Introduced by

Not determined - the regression lives in the vendored `rust-lang/gcc` fork's own GCC snapshot
(the specific two commits bracketing it are already named in issue #705's own body:
`b8f4bbf1b995d9511bad6652e8d774620b2bd833` (last known-good) and
`3e3878e7183f1d1e1ccf5aba99da7fbb36c012c8` (first known-bad)), not in `rustc_codegen_gcc`'s own
git history. `libgccjit.version` in this repository is bumped by many "Update GCC version"/
"Merge commit ... into subtree-update_cg_gcc_*" commits that reference git-subtree merge hashes,
not the raw `rust-lang/gcc` commit hashes named in the issue, so a direct `git log -S`/`git blame`
search in this repo does not resolve which of those bump commits first pulled in the regression;
pinning it down would require checking out and rebuilding each intermediate `rust-lang/gcc`
snapshot in that range and rerunning `repro/run.sh` against each (a real `git bisect`-style search,
but across a different repository's history than this one, and expensive per step since each
requires a GCC rebuild) - not attempted this session.

## The fix

No verified fix. One bounded fix was attempted and rejected:

Strengthened `black_box`'s inline-asm barrier in `src/intrinsic/mod.rs` (`sym::black_box` arm) by
also declaring `result`'s own memory as an explicit output operand (via
`RValue::dereference`+`add_output_operand` with constraint `"+m"`/`"=m"`, in addition to the
existing address-as-register input + blanket `"memory"` clobber), on the theory that this would
give GCC's alias analysis a concrete memory location to invalidate rather than relying solely on
the blanket clobber. This compiled and linked the backend itself successfully, but **crashed
libgccjit with SIGSEGV** while compiling the repro program itself (reproduced twice, with two
different constraint variants: a separate `"=m"` output + `"r"` address input, and a single
`"+m"` read-write memory operand) - i.e. the fix attempt is unsound in this environment, not just
ineffective, and was reverted (`git checkout -- src/intrinsic/mod.rs`) before this candidate's
folder was written. No other bounded fix was attempted this session, since the root cause is
GCC-internal optimizer behavior outside this repository's own code.

- Project test suite: N/A - no patch survived verification to test against. A quick sanity check
  (`./y.sh test --mini-tests`, ~15s) passes on the unmodified tree (confirms the build/toolchain
  itself is healthy in this worktree; not a before/after comparison for a fix, since none exists).

## What's not done / open questions

- Whether this also affects `u128 as f64`/`i128 as f32`/`i128 as f64` (the extern-call path also
  covers `__floatuntidf`/`__floattisf`/`__floattidf`) was not checked - the minimal repro only
  exercises `u128 as f32`, matching the issue title.
- Whether the same class of "GCC folds a comparison against a value that passed through
  `black_box`" miscompile can be triggered without a 128-bit conversion at all (i.e. whether this
  is really 128-bit-specific or a more general `black_box`/constant-folding interaction that just
  happens to be easiest to trigger here) was not explored further after one earlier experiment
  with plain `u64` (using intentionally-*incorrect* expected values, an authoring mistake on this
  session's part, not a real finding) turned out to be a false lead once corrected.
- This is very likely only fixable upstream (in the vendored GCC/libgccjit fork), consistent with
  the issue remaining open with no code-level fix attached as of this session; the two `add_output_operand`
  crash variants above are themselves worth a closer look in a future session (a
  `"m"`/`"+m"`-constrained output/read-write memory operand on an extended-asm statement appears
  to be a genuinely unstable/untested path in this libgccjit build, independent of whether it would
  have fixed this candidate's own bug).
