# int_to_float_cast_missing_sign_cast

`CodegenCx::int_to_float_cast` (`src/int.rs`) is the shared implementation behind
`Builder::uitofp`/`sitofp` (`src/builder.rs:1352-1358`). This candidate checks the family this
workspace's risk map explicitly flags as under-audited: "any place a value's GCC type sign is
constructed/asserted/cast, ask whether the sign is actually guaranteed to match, or just assumed" -
`int_to_float_cast`'s native-type branch does exactly that assuming, with no forcing cast, unlike
the already-fixed `division_operation`/`gcc_lshr`/`gcc_ashr` (candidates 004/006/018 and the
division fix) which all force the correct sign before their native operation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading, cross-referencing candidate 018's confirmed finding that `mem::transmute::<*const (), usize>` leaves the resulting `usize` with a signed GCC-declared type (via `ptrtoint` bridging through libgccjit's signed `isize`) | Static reading only | `int_to_float_cast`'s native branch (`src/int.rs:951-953`, before this candidate's fix) returns `self.context.new_cast(None, value, dest_typ)` unconditionally, completely ignoring the `signed` parameter for any native GCC int type - a plausible live instance of the same bug shape as candidate 018 |
| Manual boundary-value construction: real end-to-end compile of a standalone Rust source through the real backend (`./y.sh cargo build` against the real sysroot, then confirmed via `./y.sh rustc` for the final repro) - `transmute::<*const (), usize>(ptr) as f64` compared against `usize::MAX as f64` | Release build; 3 runs | Real mismatch: `unsigned as f64` printed/asserted `-1.0` instead of the correct `18446744073709551615.0` (`1.8446744073709552e19` after f64 rounding) - reproduced 3/3 identical runs, deterministic |
| Final `repro/int_to_float_cast.rs` compiled+run via the project's actual entry point (`./y.sh rustc`, matching this workspace's own convention from candidate 018), covering the transmuted-`usize` `uitofp` case, the mirror transmuted-`isize` `sitofp` case, and four non-transmuted boundary-value checks (`u64::MAX`, `i64::MIN`, `0u32`, `-1i32`) to confirm the fix does not regress the already-correct ordinary case | Release backend build, 3 runs before the fix, 3 runs after | Before: 3/3 identical panics (exit 101) on the transmuted-`usize` assertion, all other assertions passing (never reached due to the panic). After: 3/3 identical clean exits (exit 0, all assertions pass) |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic, real release-mode build of the actual backend
(`./y.sh build --sysroot --release` then `CHANNEL=release ./y.sh rustc
repro/int_to_float_cast.rs`).

**Fully reproducible**: Yes. `repro/int_to_float_cast.rs` is a plain, public-API-only Rust source
file compiled through the project's actual, unmodified end-to-end entry point
(`rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>`, wrapped by the project's own
`./y.sh rustc`) - the real affected compiler backend reacting to a real Rust program it is handed,
not a hand-built stand-in for one. This is a single-participant finding (a compiler compiling a
source file it's handed), so `CONFIRMATION.md` section 11's two-participant bar doesn't apply.

## Class

logic (a well-typed, safe `usize as f64`/`isize as f64` cast silently produces a wrong numeric
result - no crash, no compile-time diagnostic).

## Root cause

`CodegenCx::int_to_float_cast` takes a `signed: bool` parameter describing which conversion the
*caller* (`gcc_uint_to_float_cast`/`gcc_int_to_float_cast`, in turn `Builder::uitofp`/`sitofp`)
actually wants, but for any *native* GCC int type it ignores that flag entirely and does a plain
`self.context.new_cast(None, value, dest_typ)`. GCC's native cast from an integer to a
floating-point type is determined by the operand's *declared* C-level signed/unsigned type, not by
any flag passed alongside it - the same GCC-vs-LLVM semantic gap behind every other confirmed bug
in this family (LLVM has no sign in its integer types at all; the choice of `uitofp` vs `sitofp`
instruction alone decides the conversion kind there).

A value's GCC-declared sign can diverge from what the Rust-level operation actually needs whenever
it arrives through a path that doesn't preserve sign for its own reasons - candidate 018 already
demonstrated this exact divergence for `mem::transmute::<*const (), usize>`, whose result keeps a
signed GCC type (via `Builder::ptrtoint`'s intermediate `isize` bridge) even though it's a
semantically-unsigned `usize`. Feeding that value to `unsigned as f64` calls `uitofp` ->
`gcc_uint_to_float_cast` -> `int_to_float_cast(signed=false, ...)`, which - before this candidate's
fix - performed the native cast using the value's own (wrongly signed) GCC type instead of the
caller-requested unsigned interpretation, converting `usize::MAX`'s bit pattern as if it were the
signed value `-1` and producing `-1.0` instead of the correct large positive float.

This is the same family already confirmed for `division_operation` (fixed upstream, 6f389ff),
`atomic_extremum` (candidate 004), `type_uint_from_ty` (candidate 006), and `gcc_lshr`/`gcc_ashr`
(candidate 018) - `int_to_float_cast` is simply another native operation whose correctness depends
on the operand's sign that never received the equivalent forcing-cast treatment.

## Introduced by

41f20fa "Support 128-bit integers on platforms without native support (#103)" (2022-01-30) - the
commit that introduced `int_to_float_cast` itself, with this exact unconditional
`new_cast(None, value, dest_typ)` shape for native types from day one (confirmed via `git show
41f20fa -- src/int.rs`).

## The fix

Added a forcing cast inside `int_to_float_cast`'s native-type branch: when `signed` is true but
`value`'s declared GCC type is currently unsigned, cast it to the signed counterpart first; when
`signed` is false but the declared type is currently signed, cast it to the unsigned counterpart
first - leaving `value` untouched when its declared sign already matches what the caller asked for
(the overwhelmingly common case, so no extra cast is inserted for ordinary, already-correctly-signed
casts). This mirrors `gcc_lshr`/`gcc_ashr`'s existing sign-forcing pattern (candidate 018) applied
to this sibling native operation.

```
src/int.rs
     fn int_to_float_cast(...) -> RValue<'gcc> {
         let value_type = value.get_type();
         if self.is_native_int_type_or_bool(value_type) {
+            let value = if signed && !value_type.is_signed(self) {
+                self.context.new_cast(None, value, value_type.to_signed(self))
+            } else if !signed && value_type.is_signed(self) {
+                self.context.new_cast(None, value, value_type.to_unsigned(self))
+            } else {
+                value
+            };
             return self.context.new_cast(None, value, dest_typ);
         }
```

(See `patch.diff` for the exact diff including comments.)

Verified against `repro/int_to_float_cast.rs`, which covers both directions (transmuted `usize`
via `uitofp`, transmuted `isize` via `sitofp`) plus four non-transmuted boundary-value checks
(`u64::MAX`, `i64::MIN`, `0u32`, `-1i32`) to confirm the fix does not regress the already-correct
ordinary case. Before the fix: exit 101 (assertion failure on the transmuted-`usize` check,
`left: -1.0, right: 1.8446744073709552e19`). After the fix: exit 0, re-run 3/3 times with identical
(silent, all-pass) result each time. See `output.txt` for the actual captured output.

- Project test suite: `CHANNEL=release ./y.sh test --release --mini-tests` passes identically
  before and after (smoke test only, unrelated to this cast). `CHANNEL=release ./y.sh test
  --release --cargo-tests`: **9 passed/0 failed** (unit tests), **41 passed/0 failed/1 ignored**
  (debug lang_tests), **41 passed/0 failed/1 ignored** (release lang_tests) - identical before and
  after, no regressions (the existing suite doesn't happen to cover this cast path with a
  wrongly-signed operand, the same observation candidate 018 made about its own sibling bug).
  `--test-libcore` was also run against the fixed build: **578 passed/0 failed**.

## What's not done / open questions

- Only the pointer-transmute-to-integer trigger was exercised end-to-end (the same trigger
  candidate 018 used for `lshr`/`ashr`), consistent with this workspace's risk map. Other ways a
  value could reach `uitofp`/`sitofp` with a GCC type sign that disagrees with the Rust-level
  operation (e.g. via a SIMD lane extraction, or some other bitcast-heavy path) were not
  separately swept for the same class of bug.
- The sibling function `float_to_int_cast` (`src/int.rs`, immediately below `int_to_float_cast`,
  same file, same commit) has an analogous native-type early return
  (`if self.is_native_int_type_or_bool(dest_typ) { return self.context.new_cast(None, value,
  dest_typ); }`) that also does not force `dest_typ`'s sign to match the `signed` parameter before
  casting - not independently investigated this session (this candidate's fix does not touch it).
  Worth a dedicated candidate: the failure mode there would be a float-to-int cast producing the
  wrong result when the *destination* type ends up with the "wrong" GCC-declared sign (e.g. `f64 as
  usize` where the destination register happens to be treated as signed for some downstream
  reason) - not yet constructed or confirmed as a live trigger.
- Non-native (128-bit, `supports_128bit_integers = false`) `int_to_float_cast`/`float_to_int_cast`
  paths dispatch to a fixed extern function name built from the `signed` flag directly (like
  candidate 003's division dispatch), so they are very likely sign-correct by construction - not
  independently re-verified this session, following the same reasoning already established for
  candidate 003 in this exact codebase.
