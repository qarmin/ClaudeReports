# signtype_is_compatible_with_foundation_check

`SignType` (`src/common.rs:398-457`, `is_signed`/`is_unsigned`/`to_signed`/`to_unsigned`) is built
entirely on `is_i8`/`is_u8`/etc (`src/common.rs:527+`), which each call
`Type::is_compatible_with` - a thin wrapper (`gccjit` crate, `src/types.rs:353`) around
libgccjit's `gcc_jit_compatible_types` C entry point, not strict pointer/type-object equality.
Since nearly every sign-sensitive decision in `src/int.rs` (division/remainder sign-fixing casts,
`gcc_lshr`/`gcc_shl` signed-vs-unsigned casts, `gcc_icmp`'s comparison casts) sits on top of this,
the concern was that `gcc_jit_compatible_types`'s cross-sign semantics might be looser than
assumed - specifically, that `gcc_jit_compatible_types(int8_t, uint8_t)` could return `true`
(compatible) rather than `false`, which would silently make every sign-dispatch decision built on
`SignType` non-functional without any single call site looking wrong on its own.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct library query: a standalone Rust binary using the `gccjit` crate (the exact same crate/version this project depends on, `gccjit = "6.1.0"`, `dlopen` feature) to `dlopen` this project's own already-built `libgccjit.so` and call `Type::is_compatible_with` directly for every native signed/unsigned pair (`i8/u8`, `i16/u16`, `i32/u32`, `i64/u64`, `i128/u128`), constructed via `context.new_c_type(CType::IntNt/UIntNt)` - the exact construction path `src/context.rs`'s `create_type` uses to build `CodegenCx::i8_type`/`u8_type`/etc, the types `SignType`'s `is_i8`/`is_u8`/etc actually compare against | Release build, run against `build/libgccjit/badf78d09d16e66f4ca07971c51aa6a227558d4f/libgccjit.so` (this project's own bundled `download-gccjit = true` library) | Every signed/unsigned pair returned `is_compatible_with = false` in both directions; every type returned `is_compatible_with = true` against itself |
| Same check via the alternate construction path `context.new_type::<iN>()`/`context.new_int_type()` (maps to `gcc_jit_context_get_int_type` instead of `gcc_jit_context_get_type`, used elsewhere in the `gccjit` crate's `Typeable` impls and by `CodegenCx`'s non-native i128 fallback) - to check whether the two different libgccjit type-construction entry points produce type objects that disagree on compatibility with each other | Same binary, same library, same run | `i8(CType)/u8(Typeable)` cross-construction pair: still `false` in both directions, same as same-path pairs; `i128/u128` via `new_int_type` alone: also `false` in both directions |
| Repeatability check: re-ran the same binary against the same library 3 times total | Same release build, same library path, no source change between runs | Byte-for-byte identical output all 3 runs (`diff -q` confirmed) |

## Status

**Not reproducible.** The suspected foundational flaw - that `gcc_jit_compatible_types` might
treat a same-width signed/unsigned pair as "compatible" and thereby silently break `SignType`'s
sign dispatch - does not hold against this project's own bundled libgccjit build. Covered: all
four native scalar widths (8/16/32/64-bit) plus 128-bit, both directions
(`signed.is_compatible_with(unsigned)` and the reverse), both of the two distinct libgccjit type-
construction paths this codebase actually uses (`gcc_jit_context_get_type` via `CType`, and
`gcc_jit_context_get_int_type` via `Typeable`/`new_int_type`), and cross-construction-path
comparison. Reproduces deterministically 3/3 runs (same result every time, as expected for a pure
query against a fixed, non-randomized library).

**Fully reproducible**: Yes - `repro/` runs the real `gccjit` crate's real public
`Type::is_compatible_with` against this project's actual bundled `libgccjit.so`, the same crate,
version, and shared library the real backend links against; no second participant is being
simulated (this is a single-process query against a library, not an interaction between two
peers).

## Class

N/A - no bug confirmed.

## Root cause (why the suspect turned out to be safe)

`gcc_jit_compatible_types` genuinely does distinguish declared signedness for same-width integer
types in this libgccjit build: an `int8_t`-typed GCC type object is never reported compatible with
a `uint8_t`-typed one (or any other differently-signed same-width pair), in either call direction,
regardless of which of the two libgccjit type-construction entry points
(`gcc_jit_context_get_type(GCC_JIT_TYPE_INT8_T)` vs `gcc_jit_context_get_int_type(1, true)`) was
used to build the operand. This means `SignType::is_i8`/`is_u8`/etc (and everything built on top
- `is_signed`/`is_unsigned`/`to_signed`/`to_unsigned`) correctly discriminate sign for every
concrete integer width this backend actually constructs, at least for this libgccjit
version/build. The dominant sign-mismatch bug family already found in this project this session
(candidates 004/006) is real, but its root cause lives one level up from here - in code that
constructs or dispatches on a value's declared GCC type incorrectly (e.g. `type_uint_from_ty`
returning the wrong type outright, or a fixed-return-type builtin masking the real operand type) -
not in `is_compatible_with`/`gcc_jit_compatible_types` itself misreporting sign compatibility.

## Introduced by

N/A - not applicable, no bug was confirmed.

## The fix

No fix - nothing to fix; see Status.

- Project test suite: N/A - no fix was applied, so no before/after comparison was run. (Also N/A
  in the sense that this candidate's own repro is a standalone binary outside the main project's
  crate/test suite - it doesn't touch or depend on `rustc_codegen_gcc`'s own test harness.)

## What's not done / open questions

- Only checked against the one libgccjit build/version bundled by this project's own
  `download-gccjit = true` setup (commit hash `badf78d09d16e66f4ca07971c51aa6a227558d4f` per the
  build directory name). `gcc_jit_compatible_types`'s exact semantics are an implementation detail
  of libgccjit itself (not part of the project under test), so this result does not by itself rule
  out a different libgccjit version ever behaving differently - though the pattern found (correct
  sign discrimination) is exactly the documented/intended contract of "compatible types" in a C
  type system with signed/unsigned as genuinely distinct types, so a regression there would be a
  libgccjit-level bug, not something this project's own code could reasonably guard against.
- Only integer types were checked (the actual concern raised by the candidate, since `SignType` is
  entirely about `is_iN`/`is_uN`). Non-integer type compatibility (pointers, floats, `bool`,
  qualified/aligned variants) was not checked here - out of scope for this specific foundational
  question, which was narrowly about signed-vs-unsigned same-width integers.
- This rules out the *foundation* being broken; it does not re-verify every individual call site
  in `src/int.rs` built on top of `SignType` (division/remainder casts, `gcc_lshr`/`gcc_shl`,
  `gcc_icmp`) - those remain open, unexplored leads for a future candidate per the project's own
  `IDEAS.md` backlog entry ("Integer width casts elsewhere in `src/int.rs`... not yet explored at
  all this session").
