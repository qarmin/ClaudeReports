# atomic_fetch_max_min_sign_bug

`src/builder.rs`'s `atomic_rmw` (line 1774) dispatches `AtomicRmwBinOp::AtomicMax`/`AtomicMin`
(signed) and `AtomicRmwBinOp::AtomicUMax`/`AtomicUMin` (unsigned) to the identical
`atomic_extremum` helper (line 96) with no signed/unsigned flag - only the `ComparisonOp` direction
(`LessThan` vs `GreaterThan`) differs between max and min, never between signed and unsigned.
`atomic_extremum`'s CAS-loop comparison type comes from `previous_value.get_type()`, which is
whatever `atomic_load` (line 1085) returns - and `atomic_load`'s own `_ty` parameter is explicitly
unused (`// FIXME(antoyo): use ty.`), so it always returns the fixed return type of GCC's
`__atomic_load_N` builtin regardless of the real Rust-level signedness of the atomic. This looked
suspicious because the sibling `atomic_store` (line 1269) documents and works around this exact
generic-type mismatch with an explicit cast, while `atomic_extremum`'s comparison had no equivalent
fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading of `atomic_rmw`/`atomic_extremum`/`atomic_load`/`atomic_store` in `src/builder.rs`, and `SignType::to_signed`/`to_unsigned` in `src/common.rs` | - | Confirmed the dispatch passes no signedness info into `atomic_extremum`, and that `atomic_load` always returns a fixed-signedness type (its `_ty` argument is a documented no-op) - the exact mechanism the candidate description predicted. |
| Manual boundary-value construction: a real Rust program (`repro/atomic_extremum_report.rs`) exercising `AtomicU8/U16/U32/U64::fetch_max`/`fetch_min` at a value pair straddling the sign bit (`100`/`200` for `u8`, `100`/`60000` for `u16`, `100`/`u32::MAX`, `100`/`u64::MAX`) and `AtomicI8/I16/I32/I64::fetch_max`/`fetch_min` with a negative operand (`-1`, `i32::MIN`, `i64::MIN`), compiled and run three ways: (1) plain `rustc`/LLVM as ground truth, (2) this backend as-is (before the fix), (3) this backend with the patch below applied (after the fix) | Real end-to-end compile via `CHANNEL=release ./y.sh rustc repro/atomic_extremum_report.rs -o ... && ...`, both `-Copt-level=0` and `-Copt-level=3` (release) checked, see `output.txt` | Every `AtomicU{8,16,32,64}::fetch_max`/`fetch_min` case printed the wrong result before the fix (unsigned comparison silently ran as signed), every `AtomicI{8,16,32,64}` case already happened to be correct (the fixed builtin-derived comparison type turned out to already be signed) - matching the candidate's prediction exactly. All cases matched plain `rustc`/LLVM after the fix, unchanged by optimization level. |
| Repeat-run determinism check | `repro/atomic_extremum.rs` (assert-based, panics at the first wrong result), run 3x against the same unpatched binary | Bit-for-bit identical failure (same assertion, same values) on all 3 runs - this is deterministic codegen, not a race; see `output.txt`. |

## Status

**Confirmed.** Reproduces 3/3 runs (deterministic - this is a compile-time codegen defect, not a
runtime race), across every tested integer width (8/16/32/64-bit) and both `-Copt-level=0` and
release (`-Copt-level=3`).

**Fully reproducible**: Yes. The repro compiles a small, self-contained, `std`-only Rust source
file through the real, unmodified backend entry point (`y.sh rustc`, i.e. `rustc
-Zcodegen-backend=librustc_codegen_gcc.so`) using only the public `std::sync::atomic` API, and runs
the resulting real binary - not a hand-constructed IR/AST standing in for what a caller would
produce. No second participant is involved (this is a single-process codegen-correctness question),
so `CONFIRMATION.md` section 11's second-participant bar does not apply here.

## Class

logic

## Root cause

`Builder::atomic_rmw` (`src/builder.rs:1789-1802`) routes all four of
`AtomicMax`/`AtomicMin`/`AtomicUMax`/`AtomicUMin` through the same `atomic_extremum` call with no
way for it to know whether the real Rust-level atomic is signed or unsigned - the `ExtremumOperation`
enum only ever encodes max-vs-min. Inside `atomic_extremum` (`src/builder.rs:96`), the loop's
comparison (`previous_var.to_rvalue() < cast(src, previous_value.get_type())` or `>` for min) is
typed by `previous_value.get_type()`, which comes from `atomic_load` (`src/builder.rs:1085`) -
whose own `_ty: Type<'gcc>` parameter is explicitly documented as unused
(`// FIXME(antoyo): use ty.`) and which always returns whatever fixed return type GCC's own
`__atomic_load_N` builtin declares for that byte size, independent of the caller's real signedness.
Empirically that fixed type turned out to be signed for every width tested (1/2/4/8 bytes) - so
every already-signed `Atomic{I8,I16,I32,I64}` case happened to compare correctly by coincidence,
while every unsigned `Atomic{U8,U16,U32,U64}` case silently ran the comparison as signed, giving the
wrong result for any value pair straddling the type's sign bit (e.g. `100` vs `200` for `u8`: `200`
reinterpreted as signed is `-56`, which compares as *less than* `100`, so `fetch_max` picks the
smaller value). The sibling `atomic_store` (`src/builder.rs:1269-1272`) hits the same
generic-builtin-return-type mismatch and works around it with an explicit cast to the builtin
parameter's real type - `atomic_extremum`'s comparison had no equivalent cast.

## Introduced by

afae271 "Initial commit" (2020-05-10). The `atomic_extremum`/`atomic_rmw` dispatch already lacked
a signed/unsigned distinction in the project's very first commit (`git show
afae271:src/builder.rs` shows the same four `AtomicRmwBinOp::Atomic{Max,Min,UMax,UMin}` arms
calling `atomic_extremum` with only `ExtremumOperation::{Max,Min}`, no sign parameter) - the bug
predates all tracked history of this file. `git log -S atomic_extremum --oneline -- src/builder.rs`
confirms no later commit introduced or changed this dispatch; the file has since been reformatted/
moved through several bulk subtree-sync merges (same history shape noted in candidate
`001.lto_driver_failure_segfault`'s own "Introduced by"), but the actual logic is unchanged since
2020.

## The fix

Added a `signed: bool` parameter to `atomic_extremum`, set to `true` for `AtomicMax`/`AtomicMin`
and `false` for `AtomicUMax`/`AtomicUMin` at the two call sites in `atomic_rmw`. Inside
`atomic_extremum`, before building the comparison, the previous-value and incoming-value operands
are cast to the correctly-signed variant of the builtin-derived type (`SignType::to_signed`/
`to_unsigned`, the same trait/helpers this codebase already uses for every other sign-dispatch
decision in `src/int.rs`) so the `ComparisonOp` always runs with the caller's real Rust-level
signedness, regardless of what fixed-signedness type `__atomic_load_N` happens to declare. This
only changes the comparison's operand types (a same-width reinterpretation cast) - the CAS
(`compare_exchange`) and the returned previous value keep their original bit pattern and type
unchanged, so no other code path is affected.

Checked against more than the one triggering input: verified across 4 integer widths (8/16/32/64-
bit), both directions (`fetch_max`/`fetch_min`), both signednesses (`Atomic{U,I}*`), at both
`-Copt-level=0` and release optimization - see `output.txt`. All 11 checks in the boundary-value
repro pass after the fix, matching plain `rustc`/LLVM exactly.

- Project test suite (`./y.sh test --cargo-tests --release`, i.e. `cargo test` driving
  `tests/run/*.rs` via `lang_tester`, both its debug and release passes): **41 passed, 0 failed, 1
  ignored, before and after** (identical in both runs) for the lang-tests groups, and **9 passed, 0
  failed** for the crate's own unit tests, in both before and after - no regression, no new
  failure. No existing test in this suite exercises `fetch_max`/`fetch_min` at all (`grep -ri
  atomic tests/` finds nothing), which is consistent with this bug going unnoticed since the
  project's initial commit.

## What's not done / open questions

- Only `AtomicMax`/`AtomicMin`/`AtomicUMax`/`AtomicUMin` (the only ops that route through
  `atomic_extremum`) were checked. The other `AtomicRmwBinOp` variants (`Add`/`Sub`/`And`/`Nand`/
  `Or`/`Xor`/`Xchg`, handled a few lines below by directly calling `__atomic_fetch_*`/
  `__atomic_exchange_*`) don't have a comparison step at all, so this specific bug class doesn't
  apply to them - not independently re-verified here, but the code shape rules it out by
  inspection.
- `compare_exchange` (`src/builder.rs:156`) and `atomic_load`'s own generic-type FIXME are shared
  by other call sites (plain `AtomicOrdering`-based loads/stores/CAS outside `atomic_extremum`) -
  this candidate only fixed the one place (`atomic_extremum`'s comparison) where the wrong fixed
  type was observably used for a *signed-vs-unsigned comparison*; it did not audit every other use
  of `atomic_load`'s return type for a similar latent issue.
- Not tested: `AtomicMax`/`AtomicMin` on `usize`/`isize` (pointer-width) directly, or on a target
  other than x86_64 - the fix is width-generic (`to_signed`/`to_unsigned` work off the GCC type,
  not a hardcoded width), so no difference is expected, but this wasn't independently verified on
  another target triple.
