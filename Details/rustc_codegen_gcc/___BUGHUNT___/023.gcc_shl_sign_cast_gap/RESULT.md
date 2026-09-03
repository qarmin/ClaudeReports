# gcc_shl_sign_cast_gap

`gcc_shl` (`src/int.rs:679-705`) is the sibling of the confirmed-buggy `gcc_lshr`
(candidate 018): when `a`/`b` are both native int types with different declared GCC
signs, it round-trips `a` (or `b`) through a cast keyed off the *other* operand's sign before doing
the native `a << b`, guarded by the same kind of FIXME comment ("remove the casts when libgccjit
can shift an unsigned number by an unsigned number"). This looked suspicious because it's the same
file, the same general "GCC-declared-sign-vs-Rust-semantics" bug family that produced confirmed
bugs elsewhere in this project (004, 006, 018), and the backlog note argues (but doesn't prove)
that left-shift's low-order result bits don't depend on operand sign the way right-shift's fill bit
does, so the mismatch that broke `lshr`/`ashr` might not actually matter here.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static reading of `gcc_shl`'s dispatch plus the caller-side `rustc_codegen_ssa::base::build_shift_expr_rhs` (found in the installed `rust-src` component, `compiler/rustc_codegen_ssa/src/base.rs:333`) that constructs the shift amount before `Builder::shl` is ever called | - | `build_shift_expr_rhs` always normalizes the shift-amount operand's *width* to match the shifted value's width (`bx.trunc`/`bx.zext` to `lhs_llty`, i.e. to `a`'s own GCC type, whenever the original widths differ) before masking it into `[0, width)`; only when the two operands already have the *same* width does it leave the shift-amount's original (possibly opposite-sign) type untouched. So `gcc_shl`'s two mixed-sign branches can only ever be reached with `a_type`/`b_type` of *equal size*, differing only in declared sign - confirmed live via temporary instrumentation (see next row) rather than assumed from reading alone. |
| Manual boundary-value construction + real end-to-end compile through the actual backend (`./y.sh rustc`, release/`-O`, real `libgccjit.so`) - `repro/shl_boundary.rs`: `u32<<i32` (top-two-bits-set), `i32<<u32` (shift into the sign bit), `i64<<u64` (all-ones/`-1i64`), `u64<<i64` (`u64::MAX`, shift by 63), `u8<<i8` (`0xFF`, shift by 7), `i16<<u16` (`i16::MIN`, shift by 1) - every check compared against Rust's own `wrapping_shl` on the same input | Release build (`CHANNEL=release ./y.sh build --release` once, then `./y.sh rustc shl_boundary.rs -O`); 5 repeated runs of the compiled binary | All 6 checks passed, 5/5 identical runs, exit 0, "all shl checks passed" printed each time. See `output.txt`'s "Step 1-3" section. |
| Root-cause instrumentation: temporary `eprintln!` added to both mixed-sign branches in `gcc_shl` (reverted before finishing), same release rebuild+run | Release build; single run | Confirmed both branches are genuinely exercised by the repro, always with `a_size == b_size` (`4/4`, `8/8`, `1/1`, `2/2` bytes) as predicted from reading `build_shift_expr_rhs` - never a width mismatch. See `output.txt`'s "Step 4-5" section. |
| Mutation-testing sanity check: deliberately injected a real, wrong bit into the `unsigned_a_signed_b` branch's result (`result ^ 1` before the final cast) to confirm the repro harness actually detects a genuine deviation rather than passing vacuously, same release rebuild+run, then reverted (`git checkout -- src/int.rs`) and rebuilt+re-ran to confirm the tree was back to a clean pass | Release build; single run each way | The injected mutation was caught immediately: `assertion left == right failed: u32 << i32 mismatch: got 0x9 expected 0x8`, exit 101. After reverting, the unmodified code passed cleanly again (exit 0). See `output.txt`'s "Step 6-7" section. |

## Status

**Not reproducible.** Covered: both mixed-sign branches of `gcc_shl`'s native/native dispatch
(`a` unsigned/`b` signed and the mirror case), at boundary values chosen to make a sign-dependent
shift bug maximally visible if one existed (top bits set, all-ones, `MIN`, shift by `width-1`),
across every width the backend treats as "native" (8/16/32/64-bit - `i128`/`u128` are never
`is_native_int_type` in this backend regardless of host `libgccjit` capability, so they never reach
these branches at all, see "What's not done" below), through the real backend at release/`-O`
(the optimization level most likely to expose a UB-based misoptimization if GCC's tree/RTL passes
ever exploited signed-left-shift-overflow being undefined behavior in C). 5/5 repeat runs identical
on the unmodified code; the harness's own sensitivity was independently verified by injecting a
one-bit mutation into the exact branch under test and confirming it was caught immediately, then
confirming a clean revert restored the passing behavior.

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not a bug, for a reason specific to left-shift that does not apply to `gcc_lshr`/`gcc_ashr`
(candidate 018): both of `gcc_shl`'s mixed-sign branches only ever fire when `a_type` and `b_type`
have **the same bit width**, differing only in declared GCC sign (confirmed live via
instrumentation, not just inferred - see above). A cast between two GCC integer types of equal
width is a pure bit-pattern-preserving reinterpretation (2's-complement wraparound, the same
guarantee `SignType`'s round-trip casts rely on elsewhere in this file) - it changes how the value
is *interpreted*, never which bits are set. Left-shift's own result bits, unlike right-shift's, do
not depend on that interpretation at all: bit `i` of `a << b` is bit `(i - b)` of `a` (or `0` if
`i < b`), for every `i`, regardless of whether `a`'s declared type happens to be signed or
unsigned - there is no analogue of right-shift's "which bit fills in from the vacated end" question
that made `gcc_lshr`/`gcc_ashr`'s shared, sign-blind implementation genuinely wrong. So:

- `a_type` unsigned, `b_type` signed, same width: `a` is bit-reinterpreted to `b_type` (no value
  bits change), shifted, then reinterpreted back to `a_type` (no value bits change) - the round
  trip is a no-op on the actual bit pattern, so the computed result is identical to not casting at
  all.
- `a_type` signed, `b_type` unsigned, same width: only `b` (the already-masked, small,
  non-negative shift amount) is reinterpreted to `a_type` - again a no-op on `b`'s actual value
  since a masked shift amount in `[0, width)` is representable identically in either sign's
  encoding, and `a` itself is never touched.

Both branches therefore compute the exact same bit pattern GCC's plain, uncast `a << b` would, for
every input tried (including inputs specifically chosen to set the sign bit and to shift into or
past it) - the FIXME's own casts appear to exist only to satisfy libgccjit's requirement that a
shift's two operands share a declared type (confirmed separately: forcing a genuine *width*
mismatch between the cast source and target, by mistake, made libgccjit abort compilation outright
rather than silently compute a wrong value - see `output.txt`, an earlier iteration of the
mutation-testing step that was discarded once it turned into a compile-time ICE instead of the
intended silent-wrong-value test), not to correct an actual sign-dependent shift semantics gap the
way the equivalent-looking casts in `division_operation`/`gcc_lshr`/`gcc_ashr` do.

This does not clear the identical-looking cast pattern in `gcc_lshr`/`gcc_ashr` (candidate 018,
already Confirmed) - that bug is real precisely because right-shift's result *does* depend on the
interpreted sign (logical vs. arithmetic fill), a distinction that has no counterpart in left-shift.

## Introduced by

N/A - not reproducible, no buggy commit to identify.

## The fix

No fix - nothing to fix. No `patch.diff` in this folder.

- Project test suite: N/A, no fix was applied. The two transient modifications used during the
  investigation (branch-coverage `eprintln!`s, then the one-bit mutation) were both applied to the
  main tree only temporarily and reverted (`git checkout -- src/int.rs`) before this candidate's
  investigation ended, with a full clean rebuild + repro re-run afterward confirming the tree
  matches upstream and the repro passes.

## What's not done / open questions

- Only the `a_native && b_native` dispatch path (the one with the sign-keyed casts under
  investigation) was checked. `gcc_shl`'s other branches (`a_type.is_vector() && b_type.is_vector()`,
  `a_native && !b_native`, and the array-based non-native 128-bit decomposition) were read but not
  independently boundary-tested this session - they don't contain the sign-keyed cast pattern this
  candidate was scoped to, and the non-native 128-bit shift-decomposition logic was already
  independently checked by candidate 014 (`int_rs_width_cast_logic`, "checked by hand against the
  standard double-width shift algorithm").
- This backend's own `is_native_int_type` (`src/context.rs:374`) never classifies `i128`/`u128` as
  native regardless of the host `libgccjit`'s actual 128-bit support, so `gcc_shl`'s mixed-sign
  branches can never be reached for 128-bit shifts at all on any target - not a gap this candidate
  needed to cover, since the sign-cast pattern under investigation simply doesn't apply there.
- Did not sweep for whether some other, non-shift call site relies on `gcc_shl`'s specific choice
  of which operand gets cast (`a` vs `b`) in a way that would matter even though the shift's own
  result doesn't change - `gcc_shl` is only called from `Builder::shl`/`Builder::unchecked_shl` per
  a repo-wide grep, so this is unlikely to matter but wasn't exhaustively traced.
