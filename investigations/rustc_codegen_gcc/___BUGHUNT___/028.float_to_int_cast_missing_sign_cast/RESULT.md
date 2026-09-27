# float_to_int_cast_missing_sign_cast

`CodegenCx::float_to_int_cast` (`src/int.rs:990-1030`) is the sibling function immediately below
`int_to_float_cast` (candidate 026, Confirmed/fixed) - it backs `Builder::fptoui`/`fptosi`. Its
native-type early return (`if self.is_native_int_type_or_bool(dest_typ) { return
self.context.new_cast(None, value, dest_typ); }`) never forces `dest_typ`'s GCC-declared sign to
match the `signed: bool` parameter, the same anti-pattern candidate 026 fixed on the int-to-float
side. This candidate looks for a live trigger where `dest_typ`'s already-declared sign disagrees
with `signed` at the point of the cast.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced every real call site of `float_to_int_cast` from `rustc_codegen_ssa` (`cast_immediate`'s `(Float(_), Int(_, is_signed)) => bx.cast_float_to_int(is_signed, imm, to_backend_ty)` in `mir/rvalue.rs`, and the `float_to_int_unchecked` intrinsic in `mir/intrinsic.rs`) plus this backend's own two call sites (`fptoui`/`fptosi` in `builder.rs`, in turn feeding `fptoint_sat`'s `int_ty`) | Static, no build | In every real call site, the `signed` flag and the `to_backend_ty`/`dest_ty` passed alongside it are derived from the exact same source at the exact same call: `to_scalar.primitive()` (`rvalue.rs`) or `result_layout.ty` via `int_type_width_signed` (`intrinsic.rs`), and `scalar_gcc_type_at` builds the GCC type from that same scalar's own sign bit (`Int(i, true) => type_from_integer`, `Int(i, false) => type_from_unsigned_integer`). Unlike `int_to_float_cast`'s `value` parameter (a *source* operand that can carry a stale sign from an earlier, independent computation like a `transmute`), `float_to_int_cast`'s `dest_typ` is a *destination* type freshly derived from the cast's own target Rust type at the same call, so it cannot diverge from `signed` by construction in any path this backend actually reaches. Also confirmed `usize`/`isize` (the types named in the candidate description) go through this exact native branch on this 64-bit target (`is_native_int_type` includes `u64_type`/`i64_type`, and `usize`/`isize` are compatible with those) |
| Manual boundary-value construction: real end-to-end compile of a standalone Rust source (`f64 as u32`, `f64 as i32`, `f64 as u64`, `f64 as i64`, `f64 as usize`, `f64 as isize`, each at a boundary value chosen so a wrong destination sign would flip the result, e.g. `1.8446744073709552e19_f64 as usize` should be `usize::MAX`, `-9223372036854775808.0_f64 as isize` should be `isize::MIN`) through the real backend (`./y.sh rustc`, release build of the backend, base commit 4cd7da5), cross-checked against the same source compiled with the host's normal LLVM-backed `rustc` as an oracle | Release build of the backend; 1 run against the real backend + 1 run against host LLVM rustc as oracle | Both printed `OK` and exited 0 - every boundary value matched the LLVM oracle; no sign flip observed |
| Same repro re-run against a `dev`/debug-profile build of the backend itself (`./y.sh build`, no `CHANNEL=release`) as a distinct build configuration, to catch anything specific to how the backend is compiled (not to be confused with a debug-assertions *sysroot*, which this backend's build system does not offer a plain switch for) | Dev/debug backend build; 1 run | Also printed `OK`, exit 0 - identical result to the release backend build |

## Status

**Not reproducible.** All three approaches above are clean negatives: static tracing of every
real call site shows `dest_typ`'s sign and the `signed` flag are always derived from the same
underlying scalar/layout at the exact call site (so they cannot disagree), and two independent
real-build runs (release-profile backend, dev-profile backend) of a boundary-value repro
targeting exactly the branch and the `usize`/`isize` types named in the candidate description
both produced the LLVM-oracle-matching correct result. Not escalated to a fourth approach past
the soft cap since none of the three found even a partial signal (no wrong output, no crash, no
sanitizer-style warning).

**Fully reproducible**: N/A (no reproduction to report - this is a Not reproducible result).

## Class

N/A (Not reproducible).

## Root cause

The suspected bug's premise (candidate 026's own "what's not done" note: "the failure mode there
would be a float-to-int cast producing the wrong result when the destination type ends up with
the wrong GCC-declared sign") requires `dest_typ` to somehow already carry a sign that disagrees
with the `signed` flag by the time `float_to_int_cast` is called. In this codebase's actual
codegen paths, that never happens: `dest_typ` is not a pre-existing value carried over from an
earlier computation (unlike `int_to_float_cast`'s `value`, which genuinely can carry a stale sign
via `transmute`/`ptrtoint` bridging, per candidate 018/026) - it is synthesized fresh, at the same
call site, directly from the destination Rust type's own ABI scalar
(`scalar_gcc_type_at`/`type_from_integer`/`type_from_unsigned_integer`), and the `signed` boolean
passed alongside it is read from that exact same scalar's sign bit
(`to_scalar.primitive()`/`int_type_width_signed`). There is no code path in this backend that
constructs `dest_typ` from one source and `signed` from a different, potentially-disagreeing
source, so the two can never diverge for any input reachable through `fptoui`/`fptosi`/
`fptoui_sat`/`fptosi_sat`/`float_to_int_unchecked`. `int_to_float_cast`'s bug (candidate 026) and
this sibling's non-bug are asymmetric for a specific reason: a *source* integer operand's GCC
type can legitimately be "wrong" per Rust's semantic sign at the moment it's read (it was produced
by some earlier operation with its own reasons for its declared sign), but a *destination* type
requested for a cast is always fabricated on the spot from the thing that also decides `signed`,
so there is nothing upstream that could leave it stale.

## Introduced by

N/A (Not reproducible - no bug to attribute to a commit).

## The fix

N/A (Not reproducible - no fix needed; `float_to_int_cast`'s native branch was left unmodified in
this session's investigation).

- Project test suite: not run for this candidate - no change was made to the source tree (Not
  reproducible, no patch), so there is nothing to verify a before/after delta for. `N/A`.

## What's not done / open questions

- Only the two real call-site families reachable from `rustc_codegen_ssa` (`cast_immediate`'s
  `Float -> Int` arm and the `float_to_int_unchecked` intrinsic) and this backend's own two
  internal callers (`fptoui`/`fptosi`, and `fptoint_sat`'s use of `dest_ty`/`int_ty`) were traced.
  If a future `rustc_codegen_ssa` change ever added a call site that derives `signed` and
  `dest_ty` from two independent sources, this analysis would need to be redone - the underlying
  `float_to_int_cast` code itself is still exactly as unguarded as candidate 026 found it (no
  forcing cast was added), so this is about the current call graph, not a inherent safety of the
  function itself.
- The vector/SIMD case of `fptoui`/`fptosi` (used by `fptoint_sat`'s vector branch) was reasoned
  about but not separately built and run (SIMD float<->int conversions in this backend's own
  `simd_cast`/`simd_as` intrinsic lowering go through `bx.context.convert_vector` directly,
  confirmed via `grep` in `src/intrinsic/simd.rs`, never through `float_to_int_cast` at all) - so
  there is currently no live vector call site for this function either, but this was not verified
  with an actual vector-cast compile+run the way the scalar case was.
- Non-native (128-bit) `float_to_int_cast` dispatch was not re-investigated this session, per the
  same reasoning candidate 026 already gave for its own non-native path (fixed extern function
  names keyed directly on `signed`, sign-correct by construction) - candidate 003 already
  independently confirmed the sibling division dispatch is safe for the same reason.
