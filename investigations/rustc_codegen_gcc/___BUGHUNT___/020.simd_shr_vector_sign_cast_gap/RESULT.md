# simd_shr_vector_sign_cast_gap

`simd_shr`'s dispatch (`src/intrinsic/simd.rs:1221`, `Uint => lshr, Int => ashr`) and the actual
`lshr`/`ashr` `BuilderMethods` implementations (`src/builder.rs:921-928`, both of which just call
`self.gcc_lshr(a, b)` - there is no separate `gcc_ashr`, GCC's `>>` operator picks arithmetic vs
logical shift from the operand's own declared sign) route every vector shift through
`gcc_lshr`'s vector branch (`src/int.rs:96-97`: `a_type.is_vector() && b_type.is_vector()` ->
plain `a >> b`, no sign-correcting cast, unlike the native-scalar branch a few lines above it).
This is structurally the same gap candidate 002 investigated for `simd_div`/`simd_rem`'s vector
branch in `division_operation` - and this candidate was raised specifically because a byte-level
transmute/bitcast of a SIMD vector (e.g. `Simd::<u64,N>::from_bits`, or transmuting a signed SIMD
vector into an unsigned one) was flagged as the one reachability shape candidate 002 did **not**
rule out, since that's exactly the shape that (per this same session's candidate 018) broke the
analogous *scalar* pointer-to-int case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential-style boundary-value construction across all native widths and both directions: 8/16/32/64-bit element vectors, transmuting a signed vector (built from a negative value) into the same-width unsigned vector type and vice versa, then calling `core::intrinsics::simd::simd_shr` on the transmuted vector and checking against a scalar reference computed with a plain Rust `as`-cast + `>>` (8 checks total: 4 widths x 2 directions) | `release` build of the backend (`CHANNEL=release ./y.sh build --release`, in an isolated git worktree at `4cd7da5`), single run via `CHANNEL=release ./y.sh rustc repro/all_widths.rs -o <out>` then executing `<out>` directly | All 8 checks passed - correct shift kind (logical zero-fill for the unsigned destination, arithmetic sign-fill for the signed destination) in every case, no sign bug observed |
| Direct code-level verification: temporary `eprintln!` added inside `gcc_lshr`'s vector branch (`src/int.rs:96-97`) printing whether the branch is reached and the actual GCC vector element type's own `is_signed()`, then rebuilt (`./y.sh build --release`) and re-ran the same 8-check repro | Same `release` build, instrumented; reverted after (`git checkout -- src/int.rs`, confirmed via `git status`/`git diff --stat` clean before continuing) | Confirmed the vector branch is reached exactly 8 times (once per check) and that the GCC-level element type's sign matches the Rust-level type dispatch decided *for every single check*: `is_signed=Some(false)` for all 4 `check_unsigned_shr!` calls (which dispatch through the `Uint => lshr` arm), `is_signed=Some(true)` for all 4 `check_signed_shr!` calls (`Int => ashr` arm) - i.e. the missing cast has no observable effect because the operand handed to `a >> b` is already correctly typed |

Two approaches were enough here since the second one gave a clean, non-flaky mechanistic
explanation for the first's negative result (not just a repeated negative) - no third tool was
needed per the soft-cap guidance ("3 clean negatives... move on", and a live mechanistic
explanation is stronger than a 3rd blind negative).

## Status

**Not reproducible.** Covered: element widths 8/16/32/64 bits, both shift directions
(signed-bit-pattern-transmuted-to-unsigned and vice versa) - the same boundary-value space
candidate 002 covered for div/rem and `tests/run/ptr_to_int_div.rs` covers for the scalar
pointer case, applied to `simd_shr`.

## Root cause (why the suspect turned out to be safe)

Same underlying invariant candidate 002 established for `division_operation`'s vector branch,
confirmed again here for `gcc_lshr`'s vector branch: a `repr(simd)` value's GCC-level type is
built from the *destination* layout by `scalar_gcc_type_at` (`src/type_of.rs:263-272`), which
reads `scalar.primitive()`'s own `Int(width, signed)` flag. `BuilderMethods::bitcast`
(`src/builder.rs:1379-1381`, the path `rustc_codegen_ssa` uses for an immediate-to-immediate
`transmute`) always retypes the value to the destination layout's own GCC type - so a transmuted
SIMD vector's element type always reflects the sign of whatever Rust type it was transmuted
*into*, never the type it came from. Concretely for this candidate: the instrumentation shows
that by the time `simd_shr`'s dispatch macro (`src/intrinsic/simd.rs`) has already chosen
`lshr` vs `ashr` based on `in_elem.kind()` (the Rust-level element type), the GCC-level vector
operand handed to `gcc_lshr` already carries the matching sign, so plain `a >> b` (which relies on
the GCC type's own sign to pick arithmetic vs logical shift) happens to do the right thing anyway.

This differs from the scalar pointer/int case (candidate 018): a raw pointer's GCC type has no
signed/unsigned distinction at all, so `transmute::<*const _, usize/isize>` goes through a scalar
int-to-int adjustment path where an intermediate value's declared sign can end up stale/wrong
before a fix like 6f389ff is applied. A SIMD vector transmute has no equivalent intermediate step -
`bitcast` always re-derives the GCC vector type fresh from the destination Rust layout, so there is
no "stale sign carried over from the source type" state for it to ever be in.

This does **not** mean `gcc_lshr`'s vector branch is defensively correct - it is missing the same
safety net the native-scalar branch has a few lines above it, and would misbehave the moment any
future code path manages to hand it a vector operand whose GCC type doesn't match its Rust-level
element sign. It just means, like candidate 002 found for div/rem, no such path currently exists
for `simd_shr` specifically (nor, by the same mechanism, for any `simd_*` intrinsic that goes
through an ordinary typed SIMD immediate).

## Introduced by

N/A - not applicable, no bug was confirmed. The vector-branch gap being described (no
sign-correcting cast for vector operands in `gcc_lshr`, unlike the native-int branch a few lines
above it) has existed since the vector branch itself was added; it was never claimed to need the
same fixup, and this session's investigation found no live path that would need it.

## The fix

No fix - nothing to fix; see Status. As with candidate 002, a future session could still add a
sign-correcting cast to `gcc_lshr`'s vector branch as pure defense-in-depth (cast `a`/`b` to the
vector's own declared-signed/unsigned type per the intended shift direction before `a >> b`) even
without a live reproduction, but per "the one rule that matters most" a fix is not committed to
the real source without a demonstrated bug to prove it actually would have mattered - this was not
applied here since there is no reproduction to verify it against.

- Project test suite: N/A - no fix was applied, so no before/after comparison was run.

## What's not done / open questions

- Only `simd_shr` was checked here; `gcc_lshr`'s vector branch is also reachable from any other
  `rustc_codegen_ssa` call site that ends up calling `BuilderMethods::lshr`/`ashr` on a vector
  operand - not separately enumerated this session, but the same construction mechanism
  (`scalar_gcc_type_at` + `bitcast` always retyping to the destination layout) applies uniformly
  to any ordinary typed SIMD immediate regardless of call site, so this is expected to generalize.
- As with candidate 002, this candidate does not rule out some other route to a SIMD vector value
  whose GCC element type's sign is stale relative to its current Rust-level type (i.e. a route
  that does NOT go through `BuilderMethods::bitcast`'s normal retyping) - none was found or
  hypothesized this session; if one is ever found, `gcc_lshr`'s vector branch would inherit the
  same wrong-shift-kind bug this candidate looked for and failed to trigger.
