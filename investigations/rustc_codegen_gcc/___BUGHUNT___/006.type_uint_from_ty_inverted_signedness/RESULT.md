# type_uint_from_ty_inverted_signedness

`CodegenCx::type_uint_from_ty` (`src/type_of.rs:47-56`, `#[cfg(feature = "master")]`) maps every
`ty::UintTy` variant to a **signed** GCC scalar type (`type_isize()`/`type_i8()`/.../`type_i128()`)
instead of the matching unsigned one, mirroring `type_int_from_ty` (lines 35-44) by what looks like
a copy-paste that never got the type names swapped. This was flagged (but not investigated) by
candidate 002 (`simd_div_rem_missing_sign_cast`) as a distinct latent bug found in passing. This
candidate investigates it directly: `type_uint_from_ty` has exactly one live call site
(`vector_ty`, `src/intrinsic/simd.rs:903-916`, used only by `simd_scatter`'s element-pointer-type
derivation at `src/intrinsic/simd.rs:1197` when the scattered pointer has exactly one level of
indirection) plus a second call site in `simd_saturating_add`/`simd_saturating_sub`
(`src/intrinsic/simd.rs:1256`) whose returned value (`elem_ty`) is provably dead in the unsigned
case (only the `signed == true` match arms in that function ever read `elem_ty`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced both call sites of `type_uint_from_ty` to determine whether the wrong sign is ever observable | Static, no build | `simd_saturating_add`/`simd_saturating_sub`'s unsigned branch never reads `elem_ty` - confirmed dead there. `simd_scatter`'s `vector_ty` call feeds `elem_type.make_pointer()`, used to bitcast the raw scattered address and as the type of the pointer's `dereference()` target for a `gcc_jit_block_add_assignment` store - a live use of the (wrong) sign. |
| Manual boundary-value construction: real end-to-end compile of a standalone Rust source file through the real backend (`./y.sh rustc`, release build, real `libgccjit.so`), using `core::intrinsics::simd::simd_scatter` on a `u32x4` scattered through `*mut u32` pointers with an `i32x4` all-true mask | Release build of the backend; single run; `repro/simd_scatter_u32.rs` | Real libgccjit compile-time error: `gcc_jit_block_add_assignment: mismatching types: assignment to *(...) (type: __int32_t...) from shufflevector (..., type: __uint32_t...)` - compilation fails, `rustc` aborts with "aborting due to 1 previous error", no binary produced. Reproduced 3/3 runs (deterministic, same build). |
| Same construction at a different element width (`u8x4`/`*mut u8`/`i8x4`), to vary the input within the same suspected bug class rather than relying on the one triggering width | Same release build; single run; `repro/simd_scatter_u8.rs` | Same class of libgccjit error at the `u8`-typed assignment (`__int8_t` vs `__uint8_t`) - confirms the bug isn't specific to the 32-bit width. |

## Status

**Confirmed.** Reproduces 3/3 runs (u32 case) under the project's own real `release`-mode backend
build (`./y.sh build --sysroot --release` then `CHANNEL=release ./y.sh rustc <file>`), 1/1 run for
the u8-width variant (same deterministic compile-time libgccjit type error, re-verified on request
- see "The fix" below for a repeated after-fix run). The failure mode is deterministic given the
same source and the same unpatched build: libgccjit's `gcc_jit_block_add_assignment` unconditionally
rejects the assignment because the store's declared pointee type (`__int32_t`/`__int8_t`, wrongly
signed) doesn't match the stored value's actual type (`__uint32_t`/`__uint8_t`), aborting
compilation before the backend even reaches the linker.

**Fully reproducible**: Yes. `repro/simd_scatter_u32.rs` and `repro/simd_scatter_u8.rs` are plain,
public-API Rust source files compiled through the project's actual, unmodified end-to-end entry
point (`rustc -Zcodegen-backend=<real librustc_codegen_gcc.so>`, wrapped by the project's own
`./y.sh rustc`) - the real affected compiler backend reacting to a real Rust program, not a
hand-built stand-in for one. This is a single-participant finding (a compiler compiling a source
file it's handed), so CONFIRMATION.md section 11's two-participant bar doesn't apply.

## Class

crash (a compile-time libgccjit abort/hard error - valid, well-typed Rust source that should
compile successfully instead fails to produce a binary at all).

## Root cause

`type_uint_from_ty` returns the *signed* GCC type for every `ty::UintTy` variant instead of the
unsigned one (`self.type_isize()` for `Usize`, `self.type_i8()` for `U8`, etc. - compare against
the correctly-unsigned `type_from_unsigned_integer` helper three lines above it in the same file,
and against `type_int_from_ty` immediately below it, which this function's body is structurally
identical to except for the wrong type names). `simd_scatter`'s lowering (`src/intrinsic/simd.rs`)
calls this to compute the pointee type for the raw address it bitcasts each lane's pointer to
(`elem_type.make_pointer()`, then `ptr.dereference(None)` as the assignment target), while the
*value* being stored into that location keeps its correct, unsigned GCC type (derived elsewhere,
from the actual `default`/values-vector argument's real layout). libgccjit's own
`gcc_jit_block_add_assignment` enforces that an assignment's LHS and RHS types match exactly (not
just size/ABI-compatible) - a signed/unsigned mismatch of the same width is rejected outright at
libgccjit level with a hard, unrecoverable compile-time error, not a silent bit-reinterpretation.
This is a stronger check than the codebase's own `SignType`/`is_compatible_with` machinery
(`src/common.rs`) elsewhere appears to assume signed/unsigned same-width types are freely
interchangeable for libgccjit's purposes - that assumption does not hold for a direct
`gcc_jit_block_add_assignment`.

## Introduced by

fac57d9a066de1bb3e32dd7949ef825859bdfe70 "Merge commit
'e8dca3e87d164d2806098c462c6ce41301341f68' into sync_from_cg_gcc" (2022-06-06, author Antoni
Boucher, per `git blame -L 46,56 -- src/type_of.rs`). This is the earliest commit in this
repository's tracked history containing `type_uint_from_ty` in its current (buggy) form - `git log
-S "pub fn type_uint_from_ty" -- src/type_of.rs` finds no other candidate, and this commit is a
single-parent subtree-sync-style import (its own message reads like a merge but `git show
--format=%P` reports exactly one parent), so it is plausible the function was actually authored
even earlier in the original standalone `rustc_codegen_gcc` repository's own history before being
imported here - not independently verifiable from this checkout's history alone. Best-effort per
`COMMANDS.md`; not escalated to `git bisect` since `git log -S`/`git blame` already converge
cleanly on a single commit and the file has apparently never had a correct version of this
function.

## The fix

Changed `type_uint_from_ty` to call the unsigned type constructors (`type_u8`/`type_u16`/
`type_u32`/`type_u64`/`type_u128`, matching `type_from_unsigned_integer`'s pattern) instead of the
signed ones, and changed the `Usize` arm from `self.type_isize()` to `self.usize_type` (the
existing unsigned size-type field already used elsewhere in this file, e.g.
`uncached_gcc_type`'s pointer-in-vector fallback) - there is no `type_usize()` trait method in this
codebase (`BaseTypeCodegenMethods` only defines `type_isize`, since LLVM's IR doesn't distinguish
signed/unsigned integer types the way libgccjit does), so `usize_type` is the correct unsigned
counterpart already available on `CodegenCx`.

```
src/type_of.rs
-            ty::UintTy::Usize => self.type_isize(),
-            ty::UintTy::U8 => self.type_i8(),
-            ty::UintTy::U16 => self.type_i16(),
-            ty::UintTy::U32 => self.type_i32(),
-            ty::UintTy::U64 => self.type_i64(),
-            ty::UintTy::U128 => self.type_i128(),
+            ty::UintTy::Usize => self.usize_type,
+            ty::UintTy::U8 => self.type_u8(),
+            ty::UintTy::U16 => self.type_u16(),
+            ty::UintTy::U32 => self.type_u32(),
+            ty::UintTy::U64 => self.type_u64(),
+            ty::UintTy::U128 => self.type_u128(),
```

Verified against both repro files (u32 and u8 element widths, per CONFIRMATION.md section 7 -
not just the one width that first triggered the bug): both now compile cleanly through
`CHANNEL=release ./y.sh rustc <file>` and the resulting binaries exit 0 (their internal
`assert_eq!` on the scattered data passes) - see `output.txt` for the actual captured before/after
run. Re-ran the u32 case 3/3 times against the patched build with the same result each time.

- Project test suite (`./y.sh test --release --cargo-tests`, which runs `tests/lang_tests.rs`'s
  `lang_tester`-driven `tests/run/*.rs` suite in both debug and release profiles): **41 passed; 0
  failed; 1 ignored** in both profiles, identical before and after the fix - no regressions. (The
  full `./y.sh test` rustc-testsuite target was not run this session - it requires a full upstream
  `rust` checkout under `build/` beyond what this workspace's shared `build/` symlink provides, and
  `--cargo-tests` is the test target that actually exercises this backend's own codegen paths via
  real compiles, which is what this fix touches.)

## What's not done / open questions

- `simd_saturating_add`/`simd_saturating_sub`'s call site (`src/intrinsic/simd.rs:1256`) is
  confirmed dead for the unsigned case as of this commit (only the `signed == true` match arms
  read `elem_ty`) - the fix still corrects it for defense-in-depth (it's the same function, one
  `match`), but no separate repro was built for it since there is currently no reachable path that
  would observe a difference there.
- Only `simd_scatter`'s `pointer_count == 1` branch was checked (the branch that actually calls
  `vector_ty`/`type_uint_from_ty`); the `pointer_count > 1` branch (nested pointers) takes a
  different path (`bx.context.new_vector_type(bx.usize_type, in_len)`) and was not affected by this
  bug in the first place.
- `simd_gather` does not call `vector_ty`/`type_uint_from_ty` at all (only `simd_scatter` does,
  confirmed via `grep -n "vector_ty(" src/intrinsic/simd.rs`), so the read side of masked
  load/store was not a concern for this specific function.
- Whether any other currently-dead call site could be added in the future by upstream `rustc`
  changes to `rustc_codegen_ssa`'s SIMD intrinsic lowering (which this backend's `simd.rs` mirrors)
  that *would* observe the element type's sign directly (e.g. a signed comparison built from this
  type) was not exhaustively ruled out beyond the two call sites that exist in the current source -
  the fix removes the underlying landmine regardless, so this is now moot going forward.
