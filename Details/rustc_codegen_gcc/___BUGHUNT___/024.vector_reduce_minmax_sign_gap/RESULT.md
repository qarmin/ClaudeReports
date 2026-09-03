# vector_reduce_minmax_sign_gap

`vector_reduce_min`/`vector_reduce_max` (`src/builder.rs:2348-2363`), dispatched from
`simd_reduce_min`/`simd_reduce_max` via the `minmax_red!` macro (`src/intrinsic/simd.rs:1425-1436`,
which routes both `ty::Int` and `ty::Uint` element types through the *same* function, unlike LLVM's
separate `smin`/`umin` intrinsics), looked superficially similar to the confirmed candidate 004
(`atomic_fetch_max_min_sign_bug`) - a single implementation shared across signed/unsigned element
types with no separate variant. `difference_or_zero`'s `ComparisonOp::GreaterThanEquals`
(`src/builder.rs:2497-2510`) relies entirely on the GCC-level declared sign of its `a`/`b` operands
to pick the comparison kind, so this was flagged for formal verification: is that GCC-level sign
ever stale relative to the Rust-level element type, in particular when the vector reaches this code
via a `mem::transmute`/bitcast (the one gap candidate 020 left open for the analogous `simd_shr`
case)?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Black-box boundary-value construction: `#[repr(simd)]` vectors of 8/16/32/64-bit signed and unsigned integers with the high bit set (so signed vs. unsigned interpretation genuinely differs), built directly (`check_direct!`) and via `mem::transmute` from the opposite-signed same-width vector type (`check_transmuted!` - the candidate-020-style shape), calling `core::intrinsics::simd::simd_reduce_min`/`simd_reduce_max` and comparing against a plain-Rust `[T]::iter().min()/max()` oracle (32 assertions total: 4 widths x {direct, transmuted} x {min, max} x {unsigned, signed}) | `release` build of the backend (`./y.sh build --release`, isolated git worktree at 4cd7da5), compiled with `rustc -Zcodegen-backend=librustc_codegen_gcc.so --sysroot build/build_sysroot/sysroot -C link-arg=-lc` and run directly | All 32 assertions passed - correct min/max picked for both interpretations of the identical bit pattern in every width/construction-path combination |
| Same repro extended with extreme boundary values (`0`/`TYPE::MAX` and the exact same-vector overflow edge for the `difference_or_zero`-internal `a - b` subtraction: `u8::MAX`/`0`, `i32::MIN`/`i32::MAX`, `i64::MIN`/`i64::MAX` in the same vector) via 3 more `check_direct!` calls | Same build/run setup | All additional assertions passed - no observable effect from the internal subtraction's own two's-complement wraparound |
| Direct code-level verification: temporary `eprintln!` added inside `vector_reduce_min` (`src/builder.rs:2348`) printing the GCC-level vector element type's own `is_signed`/`is_unsigned` (via the existing `SignType` trait) on every call, rebuilt (`./y.sh build --release`) and re-ran the same repro, then reverted (`git checkout -- src/builder.rs`, confirmed via `git status`/`git diff --stat` clean before continuing) | Same `release` build, instrumented | All 22 `vector_reduce_min` invocations (11 unsigned-destination calls, 11 signed-destination calls, including the 8 that reach the function via a `mem::transmute` from the opposite-signed type) reported an `is_signed`/`is_unsigned` value matching the Rust-level destination type exactly - no case where the GCC-level sign was stale relative to the call's actual destination type |

Three approaches were used because the third (instrumentation) gave a clean mechanistic
explanation consistent with the first two's negative results, matching the same invariant
candidate 020 already established for `simd_shr`'s vector branch - not a repeated blind negative.

## Status

**Not reproducible.** Covered: element widths 8/16/32/64 bits, both `vector_reduce_min` and
`vector_reduce_max`, direct construction and `mem::transmute`-based construction (the specific gap
this candidate was raised to check), plus extreme/overflow-edge boundary values for the internal
`difference_or_zero` subtraction.

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed).

## Root cause (why the suspect turned out to be safe)

Same underlying invariant candidate 020 established for `gcc_lshr`'s vector branch, confirmed here
for `vector_reduce_min`/`max`: a `repr(simd)` value's GCC-level type is built from the *destination*
Rust layout by `scalar_gcc_type_at` (`src/type_of.rs:263-272`), and `BuilderMethods::bitcast`
(`src/builder.rs:1379-1381`, the path `rustc_codegen_ssa` uses for an immediate-to-immediate
`transmute`) always retypes the value to that destination layout's own GCC type. By the time
`args[0].immediate()` reaches `minmax_red!`'s dispatch (which only reads `in_elem.kind()` - the
Rust-level type - to decide whether to call `vector_reduce_min`/`max` at all, since both branches
call the *same* function), the operand handed to `vector_reduce_min`/`max` already carries a
GCC-level element type whose declared sign matches that Rust-level type, regardless of whether the
value arrived via ordinary construction or via a `transmute`. `difference_or_zero`'s
`ComparisonOp::GreaterThanEquals` therefore always compares with the correct signedness by
construction - there is no intermediate state (unlike the scalar pointer/int case fixed by
6f389ff) where a stale sign from an earlier type could survive into the comparison.

This is a different mechanism than candidate 004's atomic fetch_max/min bug: that bug's `Signed`
argument was a hard-coded literal at the call site, entirely independent of the atomic's actual
Rust-level (un)signedness, so the wrong branch could be taken outright. Here there is only ever one
code path (`minmax_red!` never chooses between two variants of `vector_reduce_min`/`max` - it's the
same function either way), and its correctness rests entirely on the GCC operand's own declared
sign - which this candidate's instrumentation confirms is always freshly and correctly derived from
the current Rust-level type, never a leftover from an earlier one.

## Introduced by

N/A - not applicable, no bug was confirmed.

## The fix

No fix - nothing to fix; see Status. As with candidate 020, a future session could still add a
defensive re-derivation of the comparison's sign in `difference_or_zero` even without a live
reproduction, but per this workspace's "one rule that matters most" a fix is not committed to the
real source without a demonstrated bug to justify it - this was not applied here since there is no
reproduction to verify it against.

- Project test suite: N/A - no fix was applied, so no before/after comparison was run.

## What's not done / open questions

- Only `vector_reduce_min`/`vector_reduce_max` (the `simd_reduce_min`/`simd_reduce_max`
  intrinsics) were checked here. `vector_extremum` (`src/builder.rs:2363-2394`, backing
  `vector_minimum_number_nsz`/similar float min/max intrinsics) uses a structurally different
  bitcast-to-mask-type approach and was not investigated as part of this candidate.
- As with candidate 020, this does not rule out some other route to a SIMD vector value whose GCC
  element type's sign is stale relative to its current Rust-level type (i.e. a route that does
  NOT go through `BuilderMethods::bitcast`'s normal retyping) - none was found or hypothesized
  this session; if one is ever found, `vector_reduce_min`/`max` would inherit the same
  wrong-comparison-kind bug this candidate looked for and failed to trigger.
