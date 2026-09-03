# gcc_icmp_native_sign_dedup_check

Backlog dedup check against candidate 018 (`gcc_lshr_ashr_missing_sign_cast`): `gcc_icmp`'s native
comparison path (`src/int.rs:608-644`, the final `else` branch, reached whenever neither operand
is a non-native ("big") integer array type nor a pair of ordinary data pointers) was flagged as a
possible comparison point since it allegedly *already* implements the defensive pattern `gcc_lshr`/
`gcc_ashr` were missing - deriving the correct sign from the requested `IntPredicate` and casting
both operands to that sign before comparing, rather than trusting whatever sign the operands'
*declared* GCC type happens to carry going in (which is exactly what made `gcc_lshr`/`gcc_ashr`
vulnerable in candidate 018: GCC's native `>>`/comparison operators pick behavior based on the
operand's declared C-level signed/unsigned type, unlike LLVM where sign lives only in the
instruction/predicate, not the type). This candidate re-reads the code and empirically checks
whether that claim holds, and whether a live variant of the same sign-mismatch bug family survives
anywhere in `gcc_icmp`'s native path (the vector-comparison skip, and the function-pointer special
case, both looked like they could plausibly diverge from the "always re-derive sign" pattern on
first read).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: compared `gcc_icmp`'s native `else` branch (`src/int.rs:608-644`) against `gcc_lshr`'s `src/int.rs:71-99` (candidate 018) and `to_signed`/`to_unsigned` (`src/common.rs:394-455`) | N/A (static reading only) | Confirmed the scalar match arm explicitly re-derives the target sign from `op` (`IntPredicate::IntUGT\|...` -> `a_type.to_unsigned(cx)`, `IntSGT\|...` -> `a_type.to_signed(cx)`) and casts **both** `lhs` and `rhs` to that type before calling `new_comparison`, regardless of whatever sign `a_type` (the operands' *declared* type going in) happens to carry - structurally different from `gcc_lshr`'s bare `a >> b`, which has no such re-derivation step at all. Also noted two spots that looked like they could still diverge from this pattern: (1) `if !a_type.is_vector() { ... }` skips the sign-cast entirely for vector operands; (2) the function-pointer special case (`dyncast_function_ptr_type`) reassigns `lhs`/`rhs` but not the `a_type` variable used afterwards to pick `to_unsigned`/`to_signed`, so a stale (pre-cast) `a_type` reaches those calls. |
| Manual boundary-value construction, scalar path: real end-to-end compile of a standalone Rust source (`repro/scalar_sign_dedup.rs`) through the real backend (`./y.sh rustc`, release build, real `libgccjit.so`) - `transmute::<*const (), usize>`/`transmute::<*const (), isize>` (the exact candidate-018 sign-corruption trick) followed by `<`/`>`/`<=`/`>=`/`==` against boundary values, plus ordinary (non-transmuted) `i64`/`u32`/`u64` boundary comparisons and raw-pointer ordering comparisons | Release build; 3 repeat runs | All assertions passed, 3/3 identical runs (exit 0, "all scalar/pointer checks passed"). The pointer-to-integer transmute's declared-sign corruption (verified for real in candidate 018 via instrumentation - `ptrtoint` bridges through a signed `isize` register type) does **not** propagate into a wrong comparison result here, because `gcc_icmp` re-casts to the correct sign regardless of the incoming declared sign. |
| Manual boundary-value construction, SIMD-vector path (escalation): `repro/simd_sign_dedup.rs`, `#![feature(portable_simd)]`, `transmute::<i8x16, u8x16>`/`transmute::<u8x16, i8x16>` of an all-`0xFF` vector, then `simd_gt`/`simd_lt` (which lower to `rustc_codegen_ssa::base::compare_simd_types` -> `Builder::icmp` -> `gcc_icmp`'s *same* native `else` branch, confirmed by reading `compare_simd_types` in the pinned nightly's own `rustc_codegen_ssa/src/base.rs`) plus non-transmuted vector sanity checks | Release build; 3 repeat runs | All assertions passed, 3/3 identical runs (exit 0, "all simd checks passed"). Root cause for why the `is_vector()` skip is not exploitable in practice: a SIMD vector's GCC element type is derived directly from the *destination* layout's `Scalar::primitive()` sign (`scalar_gcc_type_at`, `src/type_of.rs:263`) at the point the vector value is bitcast/transmuted into being - unlike the scalar ptr-to-int case, there is no intermediate untyped/wrongly-typed register stage the value passes through first, so a vector's declared sign at the point it reaches `gcc_icmp` is always already correct for its current Rust type. |
| Manual boundary-value construction, function-pointer path (escalation): `repro/fnptr_cmp.rs` - `<`/`>`/`<=`/`>=`/`==` on two distinct `fn()` values (Rust warns this is unspecified/address-based but still requires no crash and a self-consistent, irreflexive result) | Release build; 3 repeat runs | No crash, no panic, self-consistent result (`fp1 < fp1` false, `fp1 <= fp1` true, not both `<` and `>` true), 3/3 identical runs. Reading `to_signed`/`to_unsigned`'s fallback arm (`src/common.rs:429`/`456`, `else { *self }`) explains why the stale-`a_type` observation above is harmless: for a non-integer type (a function-pointer type matches none of the `is_i8`/`is_u8`/etc. checks), both helpers are a no-op identity cast, so comparing raw function-pointer bit patterns proceeds unchanged either way. |

## Status

**Not reproducible.** All three escalating checks (scalar/pointer, SIMD-vector, function-pointer)
passed 3/3 runs each, release build, real end-to-end compiles through the actual backend
(`./y.sh rustc`). No sign-mismatch miscomparison was found anywhere in `gcc_icmp`'s native path.

## Class

N/A (no bug confirmed).

## Root cause

Not applicable in the sense of "why it's broken" - instead, this documents why the suspect turned
out to be safe. `gcc_icmp`'s native scalar/pointer comparison arm is structurally immune to the bug
class that hit `gcc_lshr`/`gcc_ashr` (candidate 018): it never trusts an operand's already-declared
GCC-level sign to decide comparison behavior. Instead it always re-derives the *target* sign purely
from the semantic `IntPredicate` being requested (`IntUGT`/`IntULT`/... -> unsigned,
`IntSGT`/`IntSLT`/... -> signed) and casts both operands to that freshly-chosen sign right before
comparing - so even if an operand's declared type carries the "wrong" sign going in (as happens for
real after a pointer-to-integer transmute, per candidate 018's confirmed root cause), the cast that
`gcc_icmp` performs unconditionally overwrites it with the correct one before the comparison
happens. `gcc_lshr`/`gcc_ashr` had no equivalent step at all - they fed the operand straight into
GCC's native `>>`, which is precisely what made the declared-sign corruption externally observable
there. The two spots that looked like potential gaps on first read turn out not to be reachable
with a wrongly-signed operand in practice: the vector-comparison sign-cast skip is safe because a
SIMD vector's GCC element type is always freshly derived from its current Rust type's sign at every
bitcast/transmute boundary (`scalar_gcc_type_at`, unlike the scalar ptrtoint path which bridges
through an intermediately-mistyped register); the function-pointer special case's stale `a_type`
read is harmless because `to_signed`/`to_unsigned` no-op (identity) on any non-integer type,
function pointers included.

## Introduced by

N/A - no bug found, nothing to attribute.

## The fix

N/A - not reproducible, no fix needed. `gcc_icmp`'s native path already implements the same
defensive "re-derive sign from the semantic operation, don't trust the declared type" pattern that
candidate 018's patch added to `gcc_lshr`/`gcc_ashr`; no change is proposed here.

- Project test suite: not run before/after, since no source change was made this session (nothing
  to verify a fix against - see "the fix" above).

## What's not done / open questions

- The i128/u128 non-native (`dyncast_array`) branch of `gcc_icmp` (`src/int.rs:449-604`, above the
  branch investigated here) has its own separate sign-handling logic (`native_int_type.to_unsigned`
  on the low-limb comparison) and is already covered by candidate 003
  (`i128_div_rem_non_native_sign_check`, a different operation family) and candidate 018's own
  scope notes - not re-litigated here since this candidate's scope was specifically the native
  (non-i128) comparison path the backlog note named.
- The stale-`a_type`-after-reassigning-`lhs`/`rhs` pattern in the function-pointer special case
  (`src/int.rs` around the `dyncast_function_ptr_type` branch) is harmless *today* only because
  `to_signed`/`to_unsigned`'s fallback is a no-op identity cast for non-integer types - if a future
  change ever made those helpers sign-aware for pointer-like types too, this stale read would
  silently become live. Not worth a defensive fix on its own (nothing is currently broken), but
  worth a one-line comment if anyone touches that function-pointer branch again; not applied here
  since "the one rule that matters most" reserves actual source edits for confirmed bugs with a
  verified fix, and this is a Not-reproducible candidate.
- Ordering comparisons (`<`/`>`) between two *distinct-signature* function pointer types (rather
  than same-signature `fn()` as tested here) were not separately checked - `a_type != b_type` for
  two different function-pointer types might exercise a different sub-branch of the mismatched-type
  handling; low value to chase further since Rust's own ordering guarantee for `fn` pointers is
  already "unspecified" regardless of backend.
