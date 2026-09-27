# overflow_check_intrinsics_gcc_checked_binop

`Builder::checked_binop` / `gcc_checked_binop` (`src/int.rs:294+`) - the codegen for
`checked_add`/`checked_sub`/`checked_mul`, `overflowing_*`, `wrapping_*`, `saturating_*`, and
plain `+`/`-`/`*` under `overflow-checks = true`, across every native integer width plus the i128/
u128 non-native fallback (`operation_with_overflow`, called via `__rust_i128_addo`/`_subo`,
`__rust_u128_addo`/`_subo`, `__mulosi4`/`__mulodi4`, `__rust_i128_mulo`/`__rust_u128_mulo`). This
was explicitly flagged in the project's own `projects/rustc_codegen_gcc.md` knowledge file as
"not yet explored at all", in a codebase whose dominant confirmed bug family so far (candidates
004/006, and 6f389ff upstream) is exactly "GCC-declared-signedness-vs-Rust-semantic-signedness
mismatch" - `gcc_checked_binop` casts `lhs`/`rhs`/the result pointer to a type derived from the
Rust-level `typ` argument (`type_kind_to_gcc_type(new_kind)`), which is precisely the shape of
cast that produced the confirmed bugs elsewhere, and it also carries its own explicit `FIXME`
("is it correct to use rhs type instead of the parameter typ?") calling the intermediate `res`
local's type into question.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static code reading of `gcc_checked_binop`, `operation_with_overflow`, `type_kind_to_gcc_type`, and `is_native_int_type`/`is_non_native_int_type` (`src/context.rs`) | - | The native-path cast-to-`new_type` (derived from the caller-supplied Rust `typ`, not from `lhs.get_type()`) is what actually selects the sign the `__builtin_*_overflow` builtins check against, and it's driven by the Rust-level type the same way the already-fixed 6f389ff division bug's cast now is - looked like a plausible sibling. The `res` local's initial type (`rhs.get_type()`, the FIXME) turns out not to matter for correctness: it is immediately re-cast to `new_type.make_pointer()` before use, and `rhs.get_type()`'s width already always matches `new_type`'s width (isize/usize get `normalize`d to the target's actual pointer width before `type_kind_to_gcc_type`, so no size mismatch is possible) - a real but so-far-harmless code smell, not a live bug. |
| Differential boundary-value test (`repro/overflow_boundary.rs`): for all 12 integer types (`i8`/`i16`/`i32`/`i64`/`i128`/`isize`, `u8`/`u16`/`u32`/`u64`/`u128`/`usize`), a full cross-product of boundary values (`0`, `1`, `-1` (signed only), `MIN`, `MAX`, `MIN+1`, `MAX-1`, `MIN/2`, `MAX/2`) through `checked_add`/`checked_sub`/`checked_mul`, `overflowing_add`/`overflowing_sub`/`overflowing_mul`, `wrapping_add`/`wrapping_sub`/`wrapping_mul`, `saturating_add`/`saturating_sub` - ~2,000+ individual checks per build. The "expected" value for each check is a `const` (evaluated by rustc's own CTFE interpreter, entirely independent of this backend's codegen); the "actual" value is computed at runtime on `std::hint::black_box`-ed operands so it is genuinely routed through this backend's codegen. Run against the backend as shipped (native GCC `__int128` path, the only reachable configuration on a normal x86_64 host, confirmed via libgccjit's real `get_target_info()` query under the default `master` feature) | `CHANNEL=release ./y.sh cargo build`/`cargo run`, both `dev` and `release` Cargo profiles, LTO off, run twice each (deterministic - no seed/layout dependency in this test) | 0 failures across every run (`OVERALL: PASS` every time) |
| Same differential test, forced onto the non-native (array-based) i128/u128 fallback path - the "friends" functions the candidate name specifically calls out (`operation_with_overflow`, `__rust_i128_addo`/`_subo`/`_mulo`, `__rust_u128_addo`/`_subo`/`_mulo`) - which are otherwise unreachable on this x86_64 host since libgccjit genuinely reports native 128-bit support here. First attempt (adding `-m32` to the ephemeral `Context` used only for libgccjit's `master`-feature target-info probe) corrupted unrelated global GCC option state and crashed an unrelated crate (`object`) with a `verify_gimple` ICE ("type mismatch in binary expression... unsigned int / unsigned int / size_t") - not a finding about `gcc_checked_binop`, just this probe hack leaking option state across `Context` instances, so it was abandoned. Second attempt followed candidate 003's precedent: forced `u128_type_supported = false` directly in `src/base.rs`'s `compile_codegen_unit` (`repro/force_non_native.patch`), then did a full sysroot rebuild under that setting (necessary here, unlike 003's division-only case, since `checked_add`&co. can be reached via a precompiled `core` function, not just inlined at each call site) | Same test, `CHANNEL=release ./y.sh build --release --sysroot` rebuilt under the patch, `objdump -d` on the resulting binary confirmed real calls to `__rust_i128_addo`/`__rust_i128_subo`/`__rust_i128_mulo`/`__rust_u128_addo`/`__rust_u128_subo`/`__rust_u128_mulo` (i.e. the fallback branch was genuinely exercised, not just theoretically reachable), run twice | 0 failures (`OVERALL: PASS` every time); disassembly confirms the target branch was actually hit |

## Status

**Not reproducible.** Covered: every native integer width's checked/overflowing/wrapping/
saturating add/sub/mul at every meaningful boundary (zero, one, the two extremes, the extremes
adjusted by one, and the midpoints, which also exercises the "does the overflow direction differ
between signed and unsigned" case since `MAX/2 + MAX/2` and similar combinations cross the
overflow boundary for several of these types) in both the native-`__int128` configuration (the
only one reachable on this x86_64 host under real conditions) and, via a deliberately forced
build, the non-native array-based i128/u128 fallback functions - deterministic, reproduces 2/2 runs
in both `dev` and `release` Cargo profiles for both configurations (a pure `const`-vs-runtime
differential with no seed/layout dependency, so a higher repeat count wouldn't add information).
No sanitizer/fuzzing was used: per this project's own `README.md`, the interesting bugs here are
codegen-correctness bugs found via reasoned boundary-value construction against a trustworthy
oracle, not memory-safety bugs a sanitizer would catch, and that's the tool actually applied.

**Fully reproducible**: N/A (nothing reproduced - this field only applies to `Confirmed` findings).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: `gcc_checked_binop`'s native-path cast to `new_type` is keyed off the caller-supplied
Rust-level `typ` argument (via `type_kind_to_gcc_type(new_kind)`), and `rustc_codegen_ssa` always
calls `checked_binop` with `typ` set to the real Rust type of the operands - so unlike the
already-fixed division bug (6f389ff), where GCC's own binary `/` operator's behavior depended on
the operand's already-existing, potentially-wrong *declared* GCC type, here the sign used for the
overflow check is derived fresh from the trusted Rust-level type on every call, not inherited from
a GCC type that could have gone stale/wrong upstream. The `res` local's initial GCC type being
`rhs.get_type()` instead of the cast-to type (the function's own `FIXME`) doesn't currently cause a
width mismatch (isize/usize are normalized to the actual target pointer width before comparison,
and every other `Int`/`Uint` variant keeps its width unchanged), so the subsequent
`new_cast(res, new_type.make_pointer())` + dereference always reads back a correctly-sized,
correctly-signed value. The 128-bit-specific fallback (`operation_with_overflow`, dispatched by
`new_kind` - i.e. by the Rust-level type name, not by any GCC type inspection - to a fixed extern
function per {width, signedness, op}) is sign-correct by construction for the same reason
candidate 003 found the analogous division fallback safe: the signedness is baked into which named
function gets called, not derived from a GCC type that could be wrong.

## Introduced by

N/A - not reproducible, no buggy commit to identify.

## The fix

No fix - nothing to fix. No `patch.diff` in this folder. `repro/force_non_native.patch` is a
diagnostic patch used only to force the otherwise-unreachable code path under test; it was applied
and reverted (`git checkout -- src/base.rs`) inside a private clone, never against the main
project's own working tree/its shared `build/build_sysroot` cache (a real risk flagged in this
project's own `projects/rustc_codegen_gcc.md` for parallel worktrees - avoided here by always
copying `build_sysroot` into a private, non-symlinked directory before rebuilding under the forced
setting, and restoring the symlink afterward).

- Project test suite (in the investigation worktree, unpatched, after all temporary patches were
  reverted and confirmed absent via `git status`/`git diff`): `CHANNEL=release ./y.sh test
  --release --test-libcore --mini-tests --cargo-tests` -> 2807 + 578 + 9 + 41 (debug lang tests) +
  41 (release lang tests) passed, 0 failed, 2 ignored (baseline only - no fix was applied, so
  there is no "before" to compare against; this just confirms the worktree was left healthy after
  the investigation's temporary patches).

## What's not done / open questions

- The non-native fallback was exercised on this x86_64 host by forcing `u128_type_supported =
  false` in `src/base.rs` rather than by testing on an actual GCC target that genuinely lacks
  native 128-bit integers (none was available in this environment, same caveat candidate 003
  recorded) - the generated fallback codegen itself should be identical either way (the same
  `is_native_int_type`/`gcc_checked_binop` code runs regardless of *why* `u128_type_supported` is
  `false`), and this session additionally confirmed via `objdump` that the intended functions were
  genuinely called (stronger evidence than 003 had, which relied on reasoning alone for the
  overflow-op sibling), but it's still not a real 32-bit target.
- `saturating_add`/`saturating_sub`'s *signed* branch (`src/intrinsic/mod.rs`) reuses
  `__builtin_add_overflow`/`__builtin_sub_overflow`/`operation_with_overflow` directly rather than
  going through `gcc_checked_binop`, and was included in the differential test's coverage as a
  bonus, but `saturating_mul` (not present in the codebase's `IntrinsicCallBuilderMethods` impl,
  presumably handled elsewhere or not yet implemented) was not investigated at all this session.
- Only `i8`-`i128`/`u8`-`u128` and `isize`/`usize` boundary *values* were tested, not exhaustive
  input space - a longer, dedicated fuzzing/property-test campaign (via `fuzz-bug-repro`, run
  separately) over random (not just boundary) operand pairs would cover more ground, though this
  project's own `README.md` notes fuzzing fits this codebase's bug surface poorly compared to
  reasoned boundary-value construction.
