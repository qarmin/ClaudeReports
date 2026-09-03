# sext_missing_extend_int_fix

`Builder::sext` (`src/builder.rs:1335`) carries the FIXME "check that it indeed sign extend the
value" and, for non-vector destination types, calls `self.context.new_cast(self.location, value,
dest_ty)` directly - the same shape of bug candidate `004.atomic_fetch_max_min_sign_bug` confirmed
elsewhere (a GCC-level cast whose result depends on the *source* value's own declared signedness,
not the caller's intended one). The hypothesis was that `codegen_ssa` could call `sext` on a value
whose current GCC type happens to be unsigned, silently zero-extending it instead.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static call-graph audit: `grep -rn "\.sext("` across the whole checked-out `rustc` compiler tree (`rustc_codegen_ssa`, `rustc_codegen_llvm`, `rustc_codegen_cranelift`, this backend) for the exact toolchain this project builds against (`nightly-2026-08-04`) | - | `rustc_codegen_ssa` (the shared code this backend actually runs) calls `bx.sext` from exactly one place: `compare_simd_types` (`base.rs:97,110`), always with a SIMD vector `ret_ty`. This backend's own code calls `self.sext` from exactly one other place: `sym::compare_bytes` in `src/intrinsic/mod.rs:504`, casting `memcmp`'s `int` return value to `type_ix(32)` - i.e. always a same-width, already-signed source. `rustc_codegen_llvm`'s own extra `bx.sext` call sites (`intrinsic.rs`) are LLVM-backend-only code, never linked into this backend. Int-to-int casts from Rust source (`CastKind::IntToInt`) go through `bx.intcast(imm, to_backend_ty, is_signed)` (an explicit-sign API), never through `sext`/`zext` directly (`rustc_codegen_ssa/src/mir/rvalue.rs:465`). |
| Code reading of `Builder::sext`'s own vector branch and `Builder::gcc_icmp` (`src/int.rs:449-649`) | - | For a vector destination, `sext` explicitly returns `value` unchanged (`dest_ty.dyncast_vector().is_some() => return value`) - the scalar `new_cast` line is never reached for `compare_simd_types`'s call. `gcc_icmp`'s vector branch (`if !a_type.is_vector()` guards around the sign-fixup casts) leaves vector comparisons operating on the vectors' own original element types, relying on GCC's native vector-comparison semantics (already producing a full-width all-ones/all-zeros mask per lane) - which is exactly why the no-op in `sext` is safe there, not a missed cast. |
| Manual boundary-value construction + real end-to-end differential test: `repro/sext_check.rs`, compiled and run three ways - (1) plain `rustc`/LLVM as ground truth, (2) this backend at `-Copt-level=0`, (3) this backend at `-O` - exercising `simd_lt`/`simd_gt`/`simd_eq` on `i32x4` (values straddling the sign bit incl. `i32::MIN`) and on `u32x4` (values incl. `u32::MAX`, `2^31`), plus `compare_bytes` (backed by `memcmp`) in both ordering directions | `CHANNEL=release ./y.sh rustc repro/sext_check.rs [-O] -o ...`, both opt levels, 3 repeat runs at `-O`; see `output.txt` | Every case (`ALL CHECKS PASSED`, exit 0) matched plain `rustc`/LLVM exactly, at both opt levels, deterministically across 3 repeat runs - no sign-extension bug observed. |
| Runtime instrumentation: temporary `eprintln!` added to `Builder::sext`'s scalar (non-vector) branch, printing the source value's GCC type/signedness/width vs. the destination type, rebuilt, then run against `repro/sext_check.rs` and the project's own full test corpus (`./y.sh test --test-libcore` and `./y.sh test --cargo-tests --release`, i.e. all of libcore's unit tests plus all 41 `tests/run/*.rs` lang-tests) | Same `release` build, instrumented; reverted after (`git diff --stat` confirmed clean before continuing) | The scalar branch fired only from `compare_bytes` call sites (2 in the repro, 11 across `--test-libcore`, more not individually counted across `--cargo-tests`) - every single hit had `value_type=int, value_signed=true, same_width=true` (i.e. an already-signed, non-widening cast). It was never hit with a mismatched-sign source or an actual width change anywhere in this corpus; `--cargo-tests --release` finished 41 passed / 0 failed / 1 ignored, identical to an unmodified build. |

## Status

**Not reproducible.** All three escalating approaches above were tried (static call-graph audit,
real differential compile-and-run boundary-value testing, and direct runtime instrumentation of
the suspect branch itself run across the project's own test corpus) and none found a live call
path where `Builder::sext`'s scalar `new_cast` branch is reached with a value whose declared GCC
sign doesn't already match what the caller needs, or with an actual width change at all. Covered:
both `simd_lt`/`simd_gt`/`simd_eq` (via `compare_simd_types`, the only `rustc_codegen_ssa` caller
of `sext`) at 8/16/32-bit-implied element boundary values (`i32::MIN`, `u32::MAX`, `2^31`), and
`compare_bytes` (the only other caller, in this backend's own code) in both comparison directions,
at both `-Copt-level=0` and `-O`, 3 repeat runs, plus every actual invocation observed across the
project's own libcore unit tests and all 41 lang-tests.

## Class

N/A (not reproducible - no bug class to assign).

## Root cause (why the suspect turned out to be safe)

The FIXME is real (the scalar branch genuinely performs a plain `new_cast` with no explicit
signedness argument, structurally identical to the pattern that *did* cause a real bug in
candidate 004), but it is currently dead in practice for two independent reasons:

1. **The only `rustc_codegen_ssa` caller (`compare_simd_types`) never reaches the scalar branch at
   all.** Every call passes a SIMD vector as `ret_ty`, and `Builder::sext` explicitly short-circuits
   before the `new_cast` line whenever `dest_ty` is a vector (`return value` unchanged) - this is
   safe, not a missed case, because GCC's own vector comparison (`gcc_icmp`'s vector branch, which
   deliberately skips the scalar sign-fixup casts) already produces a full-width all-ones/all-zeros
   mask per lane using the vector's own original element type, matching what `ret_ty` expects with
   no cast needed. `rustc_codegen_ssa`'s int-to-int casts (`CastKind::IntToInt`, the general path
   that actually changes a scalar integer's width) go through `Builder::intcast` instead, which
   *does* take an explicit `is_signed` argument (`rustc_codegen_ssa/src/mir/rvalue.rs:465`) - `sext`
   was simply never plumbed into that path in the first place, so it isn't a candidate for the
   "wrong declared source sign" failure mode `intcast` was written to handle.
2. **This backend's own remaining caller (`compare_bytes`) never changes width.** `self.sext(cmp,
   self.type_ix(32))` casts `memcmp`'s `int` return value (already GCC's native, already-signed
   32-bit `int`) to `type_ix(32)` (also a 32-bit signed type) - same width, same sign, so whether
   the cast is implemented as sign-extension or zero-extension is unobservable here; there is no
   bit pattern a same-width cast could get wrong other than truncation, which doesn't apply.

Unlike candidate 004 (`atomic_extremum`'s comparison, reached through a real, width-relevant
runtime path with observably wrong output), this candidate's suspected mechanism has no live
trigger anywhere in the current `rustc` + `rustc_codegen_gcc` pairing - the FIXME describes a real
gap in the implementation's generality, not a currently reachable bug.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

Not attempted - no bug to fix. (For the record: PR #552, referenced by this candidate's own
description, adds `CodegenCx::extend_int` to fix the sibling `zext`'s identical FIXME - but that PR
is not merged into this checkout's `HEAD` (`git log -S extend_int` finds nothing), and even if it
were, `zext` *does* have live, width-changing call sites in this backend (see "What's not done"
below) unlike `sext`, so the two are not simply mirror images of each other in terms of real-world
impact.)

- Project test suite: N/A, no fix was made. For reference, `./y.sh test --cargo-tests --release`
  was run unmodified (only the temporary probe added/reverted, see above): **41 passed, 0 failed, 1
  ignored** - unchanged from a clean build.

## What's not done / open questions

- This candidate only covers `sext`. Its sibling `Builder::zext` (`src/builder.rs:1885`) carries
  the mirror-image FIXME ("this does not zero-extend") and, unlike `sext`, **does** have live,
  width-changing call sites in this backend's own code: `from_immediate` (`src/builder.rs:1902`,
  `bool`/`i1` -> `i8`) and `src/intrinsic/simd.rs:779` (`zext(result, type_ix(expected_bytes *
  8))`, a genuine narrow-to-wide extension for a SIMD bitmask-to-array conversion). Neither was
  exercised or root-caused by this candidate - worth a separate candidate specifically on `zext`,
  since it has a call graph shape this candidate's `sext` audit shows `sext` itself lacks.
  Suspect noted for the backlog.
- `compare_simd_types`'s float-comparison branch (`bx.fcmp` then `bx.sext`) was exercised only
  indirectly by this candidate's own `simd_lt`/`simd_gt`/`simd_eq` cases, which use integer element
  types; the floating-point path through the same vector no-op in `sext` was not independently
  re-verified with float vectors, though the same structural argument (GCC vector comparison
  already produces a correctly-shaped mask, `ret_ty` is always a vector) applies unchanged.
- Only x86_64 was tested (this environment's only available target); the mechanism argued above
  doesn't depend on target architecture, but this wasn't independently re-verified on another
  triple.
