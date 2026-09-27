# simd_div_rem_missing_sign_cast

`src/int.rs:218-237` (`division_operation`) is the entry point used by `gcc_udiv`/`gcc_sdiv`/
`gcc_urem`/`gcc_srem`, which back `simd_div`/`simd_rem` (`src/intrinsic/simd.rs:1218-1219`). It
only applies its sign-correcting cast (added by commit 6f389ff, "Use the correct sign for the
division") when `self.is_native_int_type(a_type)` is true - and `is_native_int_type`
(`src/context.rs:374`) only matches the 8 native scalar int types, never vector types. So for any
SIMD `a_type`, `division_operation` falls straight into `multiplicative_operation`'s vector branch
(`src/int.rs:249-261`), which does a plain `new_binary_op` on `a_type` with no signedness cast -
structurally the same gap 6f389ff fixed for scalars, just not extended to vectors. The hypothesis
was that a value whose GCC-level vector element type has the "wrong" declared sign (e.g. produced
via `transmute`/bitcast from a vector of opposite signedness, mirroring
`tests/run/ptr_to_int_div.rs`) would get divided/remaindered with the wrong sign.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: single `i32x4`→`u32x4` transmute of a negative value, divided/remaindered via `core::intrinsics::simd::simd_div` | `release` build of the backend, `CHANNEL=release ./y.sh rustc`; single run | Correct unsigned result (`(-4i32 as u32) / 2 = 2147483646`, matched) - no sign bug observed |
| Differential-style construction across all native widths and both directions: 8/16/32/64-bit element vectors, transmuting a signed vector from an unsigned bit pattern and vice versa, for both `simd_div` and `simd_rem`, each checked against the scalar reference computed with a plain `as` cast | Same `release` build; 8 checks (4 widths x 2 directions), each checking div+rem | All 8 checks passed - correct signed/unsigned results in every case, no sign bug observed |
| Direct code-level verification: temporary `eprintln!` added inside `division_operation` printing `signed`, `is_native_int_type(a_type)`, and (for vectors) the actual GCC element type's own `is_signed`, then rebuilt and re-ran the single-case repro | Same `release` build, instrumented; reverted after (`git checkout -- src/int.rs`, confirmed via `git status`/`git diff --stat` clean before continuing) | Confirmed the exact code path the hypothesis describes is taken (`is_native=false`, `is_vector=true`, cast-fixup branch skipped) - but also showed the vector's GCC element type is *already* correctly unsigned (`elem_signed=Some(Some(false))`) by the time it reaches `division_operation`, which is why the missing cast has no observable effect here |

## Status

**Not reproducible.** All three approaches above were tried; none produced a wrong-signed
division/remainder result. Covered: element widths 8/16/32/64 bits, both div and rem, and both
transmute directions (signed-bit-pattern-viewed-as-unsigned and vice versa) - i.e. the same
boundary-value space `tests/run/ptr_to_int_div.rs` covers for scalars, applied to vectors.

## Root cause (why the suspect turned out to be safe)

The scalar bug this candidate is modeled on (6f389ff) matters because a scalar integer's GCC type
can end up with the "wrong" declared sign relative to its Rust-level meaning after certain
conversions. For SIMD vectors, the GCC-level type of a `repr(simd)` value is built by
`scalar_gcc_type_at` (`src/type_of.rs:263-272`), which switches on `scalar.primitive()`'s own
`Int(width, signed)` flag - a property of the *destination* layout, not something a `transmute`
can leave stale. `BuilderMethods::bitcast` (`src/builder.rs:1379-1381`, used by
`rustc_codegen_ssa` for immediate-to-immediate transmute) always retypes the value to the
destination layout's own GCC type, so a transmuted SIMD vector's element type always reflects the
sign of whatever Rust type it was transmuted *into*, not the type it came from. This differs from
the scalar pointer case: a raw pointer's GCC type is a genuine pointer type with no
signed/unsigned distinction, so `transmute::<*const _, usize/isize>` goes through a scalar
int-to-int adjustment path where an intermediate value's sign could legitimately end up wrong
before 6f389ff's fix; vectors have no equivalent intermediate step that could introduce the same
mismatch. The debug-print check above confirms this invariant holds in practice: by the time a
vector operand reaches `division_operation`, its GCC element type already carries the correct
sign, so the missing cast-fixup for vectors is currently dead weight (harmless), not a live bug.

This does **not** mean `division_operation`'s vector branch is defensively correct - it is missing
the same safety net the scalar branch has, and would misbehave the moment any future code path
manages to hand it a vector operand whose GCC type doesn't match its Rust-level element sign (see
"What's not done" below). It just means no such path currently exists for `simd_div`/`simd_rem`
specifically.

## Introduced by

N/A - not applicable, no bug was confirmed. The vector-branch gap being described (no
sign-correcting cast for vector operands in `division_operation`) has existed since the vector
branch itself was introduced in `multiplicative_operation`; 6f389ff (commit 6f389ff6a4474bca77a6897518c0b0dfddbdae71) only added the cast for the scalar branch, and never claimed
to cover vectors.

## The fix

No fix - nothing to fix; see Status. A future session could still add the same sign-correcting
cast to the vector branch as pure defense-in-depth even without a live reproduction: cast
`a`/`b`'s vector element type to `to_signed`/`to_unsigned` (per `signed`) inside `division_operation`
when `a_type.is_vector()`, mirroring the existing native-int branch. This was not applied here
since there's no reproduction to verify it against, and per "the one rule that matters most" a fix
is not committed to the real source without a demonstrated bug to prove it actually would have
mattered - a purely speculative hardening patch without a verifying repro would not meet this
skill's own bar for a bounded, *verified* fix (step 7 needs a before/after that changes).

- Project test suite: N/A - no fix was applied, so no before/after comparison was run.

## What's not done / open questions

- Found, but out of scope for this candidate, while investigating the same helper function: a
  distinct, unrelated latent bug in `type_uint_from_ty` (`src/type_of.rs:47-53`, `#[cfg(feature =
  "master")]`) - it maps every `ty::UintTy` variant (`Usize`, `U8`, `U16`, `U32`, `U64`, `U128`) to
  a **signed** GCC type (`type_isize()`/`type_i8()`/.../`type_i128()`) instead of the matching
  unsigned one. Checked both of its current call sites in `src/intrinsic/simd.rs` (the
  `simd_gather`/`simd_masked_load` pointer-vector helper at line ~1197, and the unsigned branch of
  `simd_saturating_add`/`simd_saturating_sub` at line ~1256): neither actually uses the element
  type's sign for arithmetic in the unsigned case (the first only uses it to build a pointer type;
  the second's `(false, _)` match arms don't reference `elem_ty` at all), so this is currently dead
  weight with no observable effect - but it's a landmine for any future caller that does rely on
  the returned type's sign. Not investigated further as its own candidate this session; worth a
  future `IDEAS.md` entry.
- Only `simd_div`/`simd_rem` were checked; `division_operation` is also used by `gcc_urem`/
  `gcc_srem` (128-bit scalar remainder) which already goes through the native-int branch since it's
  scalar, so it wasn't a concern here.
- If `division_operation`'s vector branch is ever hardened anyway (see "The fix" above) as
  proactive defense-in-depth unrelated to a specific bug, that would be a `perf-optimize`/general
  maintenance change, not a bug-hunt fix - this workspace doesn't apply unverified changes to real
  source per its own ground rules.
