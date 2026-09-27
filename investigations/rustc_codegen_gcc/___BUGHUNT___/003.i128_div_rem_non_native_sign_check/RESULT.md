# i128_div_rem_non_native_sign_check

`src/int.rs`'s i128/u128 division/remainder codegen, specifically the path taken when
`is_native_int_type` (`src/context.rs:374`) is `false` (GCC/libgccjit on the target doesn't have a
native 128-bit integer type, so i128/u128 are represented as a 2-element array of 64-bit words -
see `src/context.rs:212`). `division_operation`'s cast-to-signed/-unsigned step (added by commit
6f389ff "Use the correct sign for the division", fixing a sign bug in the *native*-int-type path)
is explicitly skipped for non-native types, falling straight into `multiplicative_operation`'s
`dyncast_array` branch (`src/int.rs:262-281`), which calls an extern `__divti3`/`__udivti3`/
`__modti3`/`__umodti3` selected by string-formatting the function name from the `signed: bool`
parameter. This looked suspicious because it's the same general bug class (division sign
selection) as 6f389ff's fix, just in the sibling non-native-int code path that fix didn't touch -
worth checking whether the array-based path picks the correctly-signed libgcc function for a value
whose GCC-level representation is the array variant.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading of `src/int.rs` (`division_operation`/`multiplicative_operation`), `src/context.rs` (`is_native_int_type`/`is_non_native_int_type`), `src/common.rs` (`SignType::is_signed`/`is_i128`/`is_u128`), and the vendored `gccjit` crate's `Type::is_compatible_with`/`unqualified` | - | Found that `gcc_sdiv`/`gcc_udiv`/`gcc_srem`/`gcc_urem` each pass a *literal* `signed: bool` constant into `division_operation`/`multiplicative_operation` (`true` for the `s*` variants, `false` for the `u*` variants) - this constant, not any inspection of `a`'s or `b`'s actual GCC type, is what selects `__divti3` vs `__udivti3` (and `__modti3` vs `__umodti3`) in the `dyncast_array` branch. Since `rustc_codegen_ssa` itself already dispatches to the correct signed/unsigned `Builder` method based on the real Rust-level signedness of the operand type, the extern function name is correct by construction regardless of what GCC-level type (array-based or native) the operand actually has. |
| Manual boundary-value construction: a real Rust program (`repro/i128_boundary.rs`) exercising `i128::MIN.wrapping_div(-1)`, `i128::MIN / 3`, `i128::MIN % 3`, `-7i128 / 2`, `-7i128 % 2`, `u128::MAX / 3`, `u128::MAX % 3`, `(u128::MAX - 1) / 2`, `(u128::MAX - 1) % 2`, `100i128 / -7`, `100i128 % -7` (all through `std::hint::black_box` to defeat constant folding), compiled and run three ways: (1) plain `rustc`/LLVM as ground truth, (2) this backend as-is on this x86_64 host (`supports_128bit_integers = true`, native GCC `__int128`), (3) this backend with `CodegenCx::new`'s `supports_128bit_integers` parameter forced to `false` (`repro/force_non_native.patch`, a 2-line diagnostic patch to `src/context.rs`, **not a fix** - reverted immediately after use) to force every i128/u128 value onto the array-based representation and the `dyncast_array`/`__*ti3` code path on this same host, since no GCC target actually available in this environment lacks native 128-bit int support | Release build, `./y.sh rustc`, real captured stdout, see `output.txt` and `repro/run.sh` | All three runs produced bit-for-bit identical output across every boundary value. Disassembling the forced-non-native binary (`objdump -d`) confirms the array path was genuinely exercised: `main` calls `__divti3` for the two signed-i128 divisions, `__modti3` for the two signed-i128 remainders, `__udivti3`/`__umodti3` for the u128 operations, and `__rust_u128_subo` for the `u128::MAX - 1` subtraction - i.e. this is a real run of the exact code path the candidate targets, not a simulation. |

## Status

**Not reproducible.** Covered: `i128::MIN` divided/modulo'd by a positive divisor (tests the
largest-magnitude negative value), `i128::MIN.wrapping_div(-1)` (the one case that would overflow
if computed as `-MIN`), a small negative dividend with a positive divisor, `u128::MAX` and
`u128::MAX - 1` (top bit set, would misbehave if treated as signed), and a positive dividend with a
negative divisor (tests operand-order sign combination). All matched plain `rustc`/LLVM exactly, in
both the native-128-bit-int GCC path and the forced array-based (non-native) path. No sanitizer/
fuzzing was used - this is a codegen-correctness question (does the array-based path call the
correctly-signed libgcc function), not a memory-safety one, and this project's own
`___BUGHUNT___/README.md` notes fuzzing doesn't fit this codebase's bug surface well; boundary-value
construction against a differential (rustc/LLVM) oracle is the tool that actually applies here, per
`HEURISTICS.md`'s "logic bugs" guidance, and it's what was used.

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: the `signed` parameter that picks `__divti3` vs `__udivti3` (`__modti3` vs `__umodti3`)
in `multiplicative_operation`'s `dyncast_array` branch is a compile-time-literal `bool` fixed by
which wrapper (`gcc_sdiv`/`gcc_udiv`/`gcc_srem`/`gcc_urem`) `rustc_codegen_ssa` calls - and
`rustc_codegen_ssa` always calls the signed variant for a Rust `i128` operand and the unsigned
variant for a Rust `u128` operand, independent of how this backend happens to represent that value
at the GCC level (native `__int128` vs. a 2-element `u64`/`i64` array). Unlike the *native*-int-type
path (where GCC's own binary division operator's behavior depends on the operand's *declared* GCC
type, which is why 6f389ff needed to insert a cast-to-signed/-unsigned step there), the array-based
path never asks GCC to do the division itself - it calls a fixed-signedness external function by
name, so there's no GCC-level type to get wrong. The one remaining type-driven decision in the
array path, `multiplicative_operation`'s `b = self.gcc_int_cast(b, a_type)` (used only if `a_type !=
b_type`, which doesn't occur for `/`/`%` since Rust's `Div`/`Rem` for primitive ints require both
operands to already share the same type), falls through to `gcc_int_cast`'s final `else` branch,
which does a plain bitcast between the i128 and u128 array types - correct, since same-width
signed/unsigned integers share the same bit pattern and no conversion (only reinterpretation) is
ever needed between them.

## Introduced by

N/A - not reproducible, no buggy commit to identify.

## The fix

No fix - nothing to fix. No `patch.diff` in this folder.

- Project test suite: N/A, no fix was applied. The one diagnostic patch used during the
  investigation (`repro/force_non_native.patch`) was applied to the main tree only transiently to
  force the code path under test, then reverted (`git checkout -- src/context.rs`) before this
  candidate's own investigation ended; the backend was rebuilt afterward and `git status` shows the
  main tree clean except for the pre-existing `___BUGHUNT___/` `.gitignore` entry from this
  workspace's own setup.

## What's not done / open questions

- This only checked `/` and `%` (the subject of the candidate). The sibling additive path
  (`gcc_add`/`gcc_sub` -> `additive_operation`'s `dyncast_array` branch, which *does* derive
  `signed` from `a_type.is_compatible_with(self.i128_type)` rather than a caller-supplied literal)
  was read but not independently boundary-tested here - it's a different mechanism (type-inspection
  instead of a literal-per-caller), so a finding here wouldn't necessarily transfer, but it's a
  plausible next candidate for the same "does the array-based i128/u128 path pick the right sign"
  question.
- Not tested: 128-bit shifts (`gcc_lshr`/`gcc_ashr`) and comparisons in the array-based path, which
  also rely on `a_type.is_signed(self)` (type inspection, not a literal) to decide arithmetic vs.
  logical shift/comparison behavior - same caveat as above, a different mechanism than the one this
  candidate targeted, not covered by this session's boundary values.
- The forced-non-native path was exercised on this x86_64 host by patching `CodegenCx::new` to
  ignore the real `supports_128bit_integers` query rather than by testing on an actual GCC
  target that lacks native 128-bit integers (none was available in this environment) - the
  generated array-based codegen itself should be identical either way (the same `is_native_int_type`
  checks and the same `int.rs` code paths run regardless of *why* `supports_128bit_integers` is
  `false`), but this wasn't cross-checked against a genuinely different target triple/GCC build.
