# atomic_load_ignored_ty_plain_loads

`Builder::atomic_load` (`src/builder.rs:1085-1101`) never uses its `_ty: Type<'gcc>` parameter
(`// FIXME(antoyo): use ty.`) and always returns whatever fixed-signedness type GCC's own
`__atomic_load_{size}` builtin declares, regardless of the caller's real Rust-level signedness.
Candidate `004.atomic_fetch_max_min_sign_bug` (Confirmed) root-caused this exact mechanism for
`atomic_extremum`'s internal CAS-loop comparison, but explicitly left open whether the same
`atomic_load` return value, consumed through the *plain* `sym::atomic_load` intrinsic path
(`rustc_codegen_ssa/src/mir/intrinsic.rs`'s `OperandValue::Immediate(bx.atomic_load(...))`, with no
corrective cast), has a similar latent sign bug for ordinary `AtomicI*`/`AtomicU*::load()` calls
later widened (`as i64`) or compared. This candidate targets that unaudited path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading: traced how the `OperandValue::Immediate` produced by `atomic_load` is actually consumed downstream - `rustc_codegen_ssa/src/mir/rvalue.rs`'s `IntToInt` cast (`bx.intcast(imm, to_backend_ty, is_signed)`) and `codegen_scalar_binop`'s `bx.icmp(bin_op_to_icmp_predicate(op, is_signed), ...)`, both of which pass an externally-derived `is_signed`/`IntPredicate` computed from the *real* Rust-level layout, not from the operand's own GCC-declared type. Then read this backend's own `intcast` (`src/builder.rs:1383-1399`) and `gcc_icmp` (`src/int.rs:449` onward, native-int branch at line ~620) implementations. | - | Both `intcast` and `gcc_icmp` read the operand's *actual* declared type only to decide whether a same-width reinterpret-cast to the *correct* sign (`to_signed`/`to_unsigned`, driven by the externally-supplied `is_signed`/predicate) is needed before the real numeric operation - they never trust the operand's own declared sign for the final semantics. Since a same-width signed<->unsigned reinterpret-cast in GCC preserves the bit pattern, this launders away whatever "wrong" fixed signedness `__atomic_load_N` declared, as long as the *bits* loaded are correct (which they are - only the C-level type tag on the return value is wrong, not the loaded bytes). This is structurally different from `atomic_extremum`'s bug (candidate 004): that code built a raw `new_comparison(ComparisonOp::LessThan, ...)` directly off `previous_value.get_type()` with no such laundering step. |
| Differential repro (`test.rs`): `AtomicI{8,16,32,64}::load()` of a negative value (`-1`, and the type's `MIN`) then widened via `as i32`/`as i64` and compared against literal `0`; `AtomicU{8,16,32,64}::load()` of a top-bit-set value (`200u8`, `255u8`, `60000u16`, `u32::MAX`, `u64::MAX`) then widened, against plain `rustc`/LLVM as ground truth. Compiled via `CHANNEL=release ./y.sh rustc test.rs -o ...` (real end-to-end backend entry point), both default (`-Copt-level=0`) and `-Copt-level=3`. | Real binaries run 3x each (opt-level=0) plus once at opt-level=3, plus once compiled with plain host `rustc` for reference - see `output.txt`. | All 20 checks (widen + negative-compare, per width/signedness) matched plain `rustc`/LLVM exactly, in every run, at every optimization level. `TOTAL_FAILURES=0` throughout. |
| Extended differential repro (`test2.rs`, `test3.rs`): signed and unsigned division on a loaded value with no intervening cast at all (`loaded / 3`), `AtomicIsize`/`AtomicUsize` (pointer-width) load + compare + widen to `i64`/`u128`, direct comparison between two separately-loaded `AtomicU8` values (`lx > ly`, no literal operand), exhaustive `match`/range-pattern dispatch on a loaded negative `i8` and a loaded high-bit-set `u8`, and `.min()`/`.max()` on a loaded value. | Same build/run method as above, against plain `rustc`/LLVM. | All checks in both files matched plain `rustc`/LLVM exactly (`TOTAL_FAILURES=0` in both). No divergence found in any consumption pattern tried (cast, comparison-to-constant, atomic-vs-atomic comparison, division, match/range dispatch, min/max). |

## Status

**Not reproducible.** Covered: `AtomicI8/I16/I32/I64` loads of negative values and
`AtomicU8/U16/U32/U64` loads of top-bit-set values, each widened (`as i32`/`as i64`/`as u128`) and
compared against a literal, at both `-Copt-level=0` and `-Copt-level=3`; `AtomicIsize`/`AtomicUsize`
(pointer-width); direct atomic-vs-atomic comparison with no literal operand; signed/unsigned
division directly on the loaded value with no intervening cast; `match`/exclusive-range-pattern
dispatch on a loaded negative/high-bit-set value; `.min()`/`.max()`. None diverged from plain
`rustc`/LLVM in any of 3 repeat runs (opt-level=0) or the single opt-level=3/reference runs.

## Class

N/A (not reproduced)

## Root cause

The suspect turned out to be safe: although `atomic_load` does return a value whose GCC-declared
type has the "wrong" fixed signedness (confirmed by candidate 004 for the same builtin), every
sign-sensitive consumer actually reachable from the plain `sym::atomic_load` intrinsic path
(`intcast` for `as`-casts, `gcc_icmp` for comparisons, and by extension anything built on top of
those, like division and range-pattern lowering) is itself parameterized by an externally-supplied,
*independently correct* signedness (`is_signed: bool` / `IntPredicate::{Int,U,S}...`) derived from
the real Rust-level layout at the call site in `rustc_codegen_ssa`, not from the operand's own
declared GCC type. Both functions use the operand's actual type only to perform a same-width,
bit-preserving reinterpret-cast to the *correct* sign before doing the real operation - so
`atomic_load`'s wrong type tag is silently corrected on every path that was tried. This is exactly
the mechanism candidate 004 identified as *missing* in `atomic_extremum`'s internal CAS-loop
comparison (a raw `new_comparison` call with no equivalent externally-supplied sign parameter) -
that absence is what made 004's bug real, and its presence in `intcast`/`gcc_icmp` is what appears
to make this candidate's suspected bug not manifest for the plain-load path. The underlying FIXME
in `atomic_load` is still real (the function still ignores `_ty`), but the specific failure mode
this candidate predicted (a widening/sign-sensitive consumer of the plain load's immediate silently
using the wrong extension) was not found to occur for any consumer tried.

## Introduced by

N/A - not reproduced, no bug to date.

## The fix

No fix attempted - nothing reproduced to fix. `atomic_load`'s `_ty` parameter is still unused
(cosmetic/latent FIXME only, per the "Root cause" analysis above), but no observable divergence
from correct behavior was found through it on any path tried.

- Project test suite: N/A - no fix was made, so no before/after comparison applies. (The project's
  existing test suite was not additionally re-run for this Not-reproducible candidate since nothing
  in the working tree was changed.)

## What's not done / open questions

- Not every possible consumer of `atomic_load`'s return value was enumerated - only casts (`as`),
  comparisons (to a literal and to another atomic), division, match/range-pattern dispatch, and
  min/max were tried. A consumer that uses the operand's *raw* GCC-declared type directly (the way
  `atomic_extremum`'s internal comparison did, per candidate 004) without going through
  `intcast`/`gcc_icmp`-style laundering could still exhibit this bug class if one exists somewhere
  else in the codegen paths reachable from a plain load - none was found by code reading for this
  session, but the audit was not exhaustive over every `Rvalue`/intrinsic lowering path.
  `atomic_extremum` itself was already fixed in candidate 004 and is excluded here (out of scope,
  not "plain" `sym::atomic_load`).
  - 128-bit atomics (`AtomicI128`/`AtomicU128`, currently unstable) were not tested - `gcc_icmp`'s
  non-native-int branch (`src/int.rs:456` onward) has a structurally different sign-handling path
  (`dyncast_array` on the declared type) that wasn't exercised by any repro here and could behave
  differently if `atomic_load`'s declared return type for 16-byte atomics doesn't dyncast the way
  that branch expects.
  - The underlying `atomic_load` FIXME itself was not fixed (no patch), since there was nothing
    concretely broken to fix in this session's testing - a future session tightening the load's
    return type to the true caller-supplied `ty` would still be a reasonable cleanup (removing a
    latent trap for any future consumer that doesn't launder sign the way `intcast`/`gcc_icmp`
    currently do), just not one this candidate found a live bug to justify.
