# enum_niche_layout_codegen

Enum/niche-optimized layout codegen was flagged as entirely unexplored territory in this
workspace's own knowledge file (`projects/rustc_codegen_gcc.md`'s risk map: "Not yet explored at
all as of session 2: enum/niche layout codegen"). Given this project's dominant confirmed bug
family so far is GCC-declared-signedness-vs-Rust-semantic-signedness mismatches (candidates
004/006), and `rustc_codegen_ssa`'s own niche-discriminant decoding algorithm
(`codegen_get_discr` in `compiler/rustc_codegen_ssa/src/mir/operand.rs`) explicitly branches
between unsigned (`IntULE`/`IntUGE`) and signed (`IntSLE`/`IntSGE`) tag comparisons depending on
whether the tag's valid range wraps as unsigned vs. signed, this looked like a promising place for
exactly that bug family to hide in a spot no prior candidate had looked at.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading: traced every backend-specific primitive `rustc_codegen_ssa`'s shared `codegen_get_discr`/`codegen_set_discr`/`codegen_tag_value` (read directly from the installed `rustc-dev` component, `compiler/rustc_codegen_ssa/src/mir/{operand,place}.rs`) actually calls into this backend for niche/enum discriminant handling: `icmp` (`src/int.rs:449` `gcc_icmp`), `intcast` (`src/builder.rs:1383`), `const_uint`/`const_uint_big`/`gcc_uint`/`gcc_uint_big` (`src/common.rs`, `src/int.rs:803-837`), `scalar_to_backend_with_pac` (`src/common.rs:330`), `select` (`src/builder.rs:1554`), and `switch` (`src/builder.rs:627`, including its dedicated i128/u128 if-ladder fallback) | - | `gcc_icmp`'s native-int path explicitly re-casts both operands to the signed or unsigned GCC type the requested `IntPredicate` needs (`src/int.rs:620-648`) before comparing, regardless of the operands' original declared sign - so both the `IntULE`/`IntUGE` and `IntSLE`/`IntSGE` branches `codegen_get_discr` can take are handled correctly by construction. `intcast` (`src/builder.rs:1383-1399`) already normalizes the source value's sign before the final cast, per the fix candidate 011 found already applied (commit 035ff85). `const_uint_big`/`scalar_to_backend_with_pac` construct/bitcast values by raw bit pattern, not by a value-preserving cast, so sign mismatches can't skew the bits. `type_uint_from_ty`/`type_int_from_ty` (`src/type_of.rs:35-56`, confirmed still inverted-sign by candidate 006, not re-litigated here) are only called from `src/intrinsic/simd.rs`, never from any enum/niche discriminant code path - so that already-known bug doesn't extend into this area. No suspicious spot found in this pass. |
| Manual boundary-value / differential construction against a known-correct oracle (Rust's own documented enum/cast semantics, checked via inline `assert_eq!`, not a second compiler): four self-checking Rust programs (`repro/enum_niche_test.rs`, `repro/many_niche.rs`, `repro/i128_enum.rs`, `repro/async_test.rs`) covering: a `repr(i32)` enum with negative discriminants read through `Direct` tag encoding and `as i64` (sign-extension exercise); a hand-built niche source (`repr(i8)` enum using only `{-1,0,1}`) wrapped by two filler variants placed both below and above the used range, to hit the niche encoding in both directions; the same niche source wrapped by 140 fieldless filler variants, deliberately sized (per `rustc_codegen_ssa`'s own doc-comment in `codegen_get_discr`, `mir/operand.rs:588-650`) to make the tag's valid range fail the unsigned-no-wraparound check but pass the signed one, forcing the backend through the `IntSLE`/`IntSGE` comparison branch specifically; a `repr(i128)` enum with `i128::MIN`/`i128::MAX` discriminants exercising the non-native (array-based) `gcc_icmp`/`switch` path (`src/int.rs`'s `dyncast_array` code); and a real coroutine (`async fn`) state machine with an internal `Option`-niched `match`, run through 3 `.await` yield points. All four were compiled and run through the real `rustc_codegen_gcc` backend built in this environment (`./y.sh build --sysroot --release`, `libgccjit` via `download-gccjit = true`) at every `-C opt-level` from 0 to 3, via `./y.sh rustc <file> -C opt-level=N` | Every program's assertions passed at every optimization level (0/1/2/3), 4/4 programs x 4 opt levels = 16/16 runs correct, real captured output in `output.txt`. `println!("size={}", ...)` in `many_niche.rs` also confirmed the 140-filler-variant type actually stayed 1 byte (niche optimization genuinely applied, not silently falling back to a discriminant+payload layout that would trivially sidestep the code path under test). |

## Status

**Not reproducible.** Covered: `Direct` tag encoding with a negative signed discriminant read
through a widening cast (`as i64`); `Niche` tag encoding with filler variants both below and above
the niche source's used range; the specific signed-vs-unsigned-wraparound branch split that
`rustc_codegen_ssa`'s own source comments call out as the trickiest part of niche decoding, forced
via a 140-variant enum; the non-native (array-based) 128-bit-integer tag comparison/switch path via
a `repr(i128)` enum; and a real coroutine state machine (the most common real-world source of
niche-heavy enum layouts) driven through several `.await` yield points. No sanitizer/fuzzing was
used - per this workspace's own `README.md`, fuzzing doesn't fit this project's bug surface well;
differential/boundary-value construction against a specified-semantics oracle (used here) is the
tool `HEURISTICS.md` recommends for this bug class, and static reading of every backend primitive
the shared niche-decoding algorithm actually depends on backs up why none of the constructed
inputs found a mismatch.

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: `rustc_codegen_ssa`'s niche/enum discriminant encode/decode logic
(`codegen_get_discr`/`codegen_set_discr`/`codegen_tag_value`) is shared, backend-agnostic code:
this backend only supplies the low-level primitives it calls into (`icmp`, `intcast`,
`const_uint`/`const_uint_big`, `scalar_to_backend`, `select`, `switch`), and each of those was
already independently verified (by earlier candidates 006/011, and re-confirmed by reading them
again here) to correctly normalize operand sign to whatever the caller's requested
signed/unsigned semantics require, rather than trusting the operand's pre-existing declared GCC
type. Because those primitives are correct, the shared algorithm above them - including its
tag-range-signedness-dependent choice between `IntULE`/`IntUGE` and `IntSLE`/`IntSGE` - produces
correct results regardless of which branch it takes. `type_uint_from_ty`/`type_int_from_ty`
(candidate 006's confirmed inverted-sign bug) is not reachable from any of this code, since it is
only ever called from `src/intrinsic/simd.rs`.

## Introduced by

N/A (Not reproducible - no bug to trace to an introducing commit).

## The fix

N/A - no bug found, no fix attempted.

- Project test suite: N/A - this candidate never reached the fix/verify step (nothing to fix), so
  the project's existing test suite was not run before/after for this candidate. (The four repro
  programs above are what was actually run in its place, per the "What was tried" table.)

## What's not done / open questions

- Only `Scalar`-backed single-field niches were tested, matching what `codegen_tag_value` itself
  asserts (`BackendRepr::Scalar` is the only representation it accepts for a niche field) - no
  other niche shape exists to test.
- Coroutine/async-fn state machines were tested with one real example (3 yield points, an `Option`
  match inside); a much larger, more state-heavy coroutine (many `.await` points, nested
  `match`/loop control flow) was not specifically constructed - the 140-variant synthetic test is
  the closest stand-in for "many states", but a real coroutine with a comparably large number of
  states was not built.
- `Direct` tag encoding was tested for a `repr(i32)` and a `repr(i128)` enum but not for every
  signed/unsigned integer width (`i8`/`i16`/`i64`/`u8`/`u16`/`u32`/`u64`/`u128`) - the `intcast`
  fix candidate 011 already re-confirmed makes width-specific behavior unlikely to differ, so this
  wasn't treated as a priority gap.
- This was investigated as a single, time-boxed session per this skill's soft cap on
  tools/approaches (2 used: static reading, then differential/boundary construction) - a longer,
  dedicated fuzzing-style sweep over many more randomly-generated enum shapes (variant counts,
  niche-source types, repr widths) was not attempted and might still be worth pointing a future
  session at if enum/niche codegen is revisited.
