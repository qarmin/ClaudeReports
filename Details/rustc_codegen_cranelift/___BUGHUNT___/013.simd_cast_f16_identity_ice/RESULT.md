# simd_cast_f16_identity_ice

`src/cast.rs`'s `clif_int_or_float_cast` has a guard that hands every cast *touching* `f16`/`f128`
over to `codegen_f16_f128::codegen_cast`, and that guard's `from_ty != to_ty` condition binds to
only one half of an `||`. An *identity* `f16 -> f16` (or `f128 -> f128`) cast therefore reaches
`codegen_cast`, whose float-to-float match has no `(F16, F16)`/`(F128, F128)` arm and falls into
`unreachable!`. `simd_cast`/`simd_as` call `clif_int_or_float_cast` per lane without checking for
the identity case, so `simd_cast::<F16x4, F16x4>` ICEs the backend. Lead came from candidate #010's
notes (7 cases of `tools/simd_type_sweep.py` hit this line); it is a different funnel from
candidate 010's `src/base.rs` `bug!()`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `src/cast.rs:42-46` guard vs `src/codegen_f16_f128.rs:184-198` match | - | Predicted exactly which `(from_ty, to_ty)` pairs reach `unreachable!`: only `(F16, F16)` and `(F128, F128)` |
| Differential test vs LLVM, `___BUGHUNT___/tools/diffcheck.sh` | single file, `simd_cast` on `#[repr(simd)] struct F16x4([f16; 4])`, opt-level 0, cg_clif @ 2bc05d45 | `RESULT: DIFFERENT (llvm exit 0, clif exit 101)`; cg_clif panics at `src/codegen_f16_f128.rs:197:18` `internal error: entered unreachable code: types::F16 -> types::F16` |
| Boundary/shape sweep, new `___BUGHUNT___/tools/f16_f128_cast_sweep.sh` (90 programs) | `simd_cast`+`simd_as` identity on `f16`/`f128` at lane counts 1/2/4/8/16; widening/narrowing `f16<->f32`, `f16<->f128`, `f64<->f128` SIMD casts as controls; scalar `x as f16`/`x as f128` identity; full 10-int x 4-float scalar cast matrix with 0/1/MIN/MAX/NaN/±inf/subnormal values; opt-level 0 | 20/90 differ, all 20 the same ICE - every identity `f16`/`f128` SIMD cast at every lane count, both intrinsics. The other 70 (incl. all non-identity `f16`/`f128` casts) are identical to LLVM |
| MIR inspection of the scalar path | `rustc -Zmir-opt-level=0 -Zunpretty=mir` on `fn go(a: f16) -> f16 { a as f16 }` | rustc lowers an identity float cast to `_0 = copy _1`, no `Rvalue::Cast` - so the `src/base.rs` cast site can never see it; `simd_cast`/`simd_as` in `src/intrinsics/simd.rs` is the only reachable route |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, opt-level 0), run via `dist/cargo-clif` | 14 cast shapes in one binary, `#[inline(never)]` per shape | LLVM prints all 14 lines; unpatched cg_clif does not compile the crate at all (exit 101, ICE dump written) |
| `./y.sh test` in the worktree, before and after the patch | in-tree items + `download/` external repos, worktree `tmp/wt/013.simd_cast_f16_identity_ice` | 1200 passed / 0 failed, exit 0, both before and after |

## Status

**Confirmed**

Deterministic: the ICE reproduced on every attempt - 3/3 consecutive `dist/cargo-clif build` runs of
`repro/` with a wiped `target-clif` each time (exit 101 each), plus 20/20 of the sweep's identity
variants, plus the initial single-file `diffcheck.sh` runs on both the main tree's `dist/` and the
freshly-built pristine worktree's `dist/`. cg_clif @ 2bc05d45 (Cranelift 0.135), rustc
nightly-2026-08-24, target program built at opt-level 0. After the patch: 3/3 runs exit 0 and the
run-time output is byte-identical to LLVM's.

**Fully reproducible**: **Yes**. `repro/` is a real crate compiled by the real `dist/cargo-clif`
wrapper driving the real backend; the failure observed is that real compilation ICEing, and after
the patch the real produced binary runs and prints exactly what the LLVM-built binary prints. There
is no second participant to fake here - the input is Rust source handed to the compiler's real
entry point.

## Class

crash (backend ICE on valid Rust)

## Root cause

`src/cast.rs`, `clif_int_or_float_cast`:

```rust
if matches!(from_ty, types::F16 | types::F128)
    || matches!(to_ty, types::F16 | types::F128) && from_ty != to_ty
{
    return codegen_f16_f128::codegen_cast(fx, from, from_signed, to_ty, to_signed);
}
```

`&&` binds tighter than `||`, so the intended "unless it is a no-op" condition `from_ty != to_ty`
guards only the *second* disjunct. When `from_ty` is itself `F16`/`F128` the first disjunct is true
on its own and the call is dispatched to `codegen_f16_f128::codegen_cast` even when
`from_ty == to_ty`.

`codegen_cast` (`src/codegen_f16_f128.rs:184-198`) enumerates only the *distinct* float pairs
(`F16->F32`, `F16->F64`, `F16->F128`, `F32->F128`, `F64->F128`, `F128->F64`, `F128->F32`,
`F128->F16`, `F64->F16`, `F32->F16`) and ends in `_ => unreachable!("{from_ty:?} -> {to_ty:?}")` at
line 197. `(F16, F16)` and `(F128, F128)` hit that arm. The non-`f16`/`f128` float-to-float path at
the bottom of `clif_int_or_float_cast` (`src/cast.rs:158-168`) does have an identity arm
(`_ => from`), which is why `f32 as f32` has always been fine - the f16/f128 detour simply never
grew the equivalent.

Reachability: `src/intrinsics/simd.rs:45` (`simd_as`/`simd_cast`) calls `clif_int_or_float_cast`
once per lane with whatever the source and destination lane types are, with no identity check, so
`simd_cast::<F16xN, F16xN>` reaches the `unreachable!`. The scalar `src/base.rs:791` cast site
cannot: rustc lowers an identity float `as`-cast to a plain `Rvalue::Use` copy, never an
`Rvalue::Cast` (verified with `-Zmir-opt-level=0 -Zunpretty=mir`). `float_to_int_unchecked`
(`src/intrinsics/mod.rs:1481`) is float-to-int only, so it cannot hit it either.

## Introduced by

e46186f9 "Add support for casting to and from `f16`/`f128`" (2025-05-23) - the commit that added
both the guard in `src/cast.rs` and `src/codegen_f16_f128.rs`'s `codegen_cast`. Found with
`git log -S 'matches!(to_ty, types::F16 | types::F128) && from_ty != to_ty' -- src/cast.rs`, which
returns that single commit; the buggy precedence and the incomplete match were introduced together,
so no bisect was needed.

## The fix

`patch.diff` (+3/-2, one hunk in `src/cast.rs`) reparenthesises the guard so the no-op check
applies to the whole condition:

```rust
if from_ty != to_ty
    && (matches!(from_ty, types::F16 | types::F128)
        || matches!(to_ty, types::F16 | types::F128))
{
```

An identity `F16 -> F16`/`F128 -> F128` cast now falls through to the ordinary float-to-float tail
of `clif_int_or_float_cast`, whose `_ => from` arm already returns the value unchanged - which is
the correct semantics for a no-op cast, and is exactly what that same tail already does for
`f32 as f32`. `codegen_f16_f128::codegen_cast` is left alone, so the diff cannot change the
behaviour of any cast that legitimately needs the libcall/soft-float detour.

Verification (per CONFIRMATION.md section 7 - varied within the bug class, not just the one input):

- `repro/` against the patched worktree: compiles, runs, and all 19 printed lines (14 result lines
  across 14 cast shapes, plus 5 section headers) are byte-identical to the LLVM build's (`just compare` -> "LLVM and cg_clif agree on all 14 cast shapes"). Captured
  in `output.txt`.
- `___BUGHUNT___/tools/f16_f128_cast_sweep.sh`: 20 differing programs out of 90 before the patch,
  **0 out of 90** after. The 20 span both intrinsics (`simd_cast`, `simd_as`), both types (`f16`,
  `f128`), lane counts 1/2/4/8/16, and lane values 0.0/-0.0/NaN/±inf/MIN/MAX/MIN_POSITIVE/subnormal.
- The 70 previously-passing programs still pass, including every non-identity `f16`/`f128` SIMD cast
  and the whole 10-int x 4-float scalar cast matrix at its boundary values - i.e. the widened guard
  did not divert anything away from the soft-float path that needed it.

- Project test suite: `./y.sh test` in `tmp/wt/013.simd_cast_f16_identity_ice` - **1200 passed, 0
  failed, exit 0 before the patch; 1200 passed, 0 failed, exit 0 after**. No new failures.

## What's not done / open questions

- `just clone` in this folder's `justfile` was **not** executed: this session's sandbox has no
  usable SSH known-hosts for github.com (`git ls-remote` -> "Host key verification failed"). Every
  other recipe was run for real (`just llvm`, `just clif` unpatched -> exit 101, `just clif` patched
  -> exit 0, `just compare` -> agree), and `patch.diff` was verified to apply to `base_commit` with
  `git apply --check`. The `clone` recipe is a plain `git clone` + `git checkout`, identical in
  shape to the other candidates in this workspace.
- Only `x86_64-unknown-linux-gnu` was tested. The guard is target-independent, so the ICE should be
  identical on aarch64, but that was not checked.
- The same "identity case missing from an exhaustive match" shape may exist elsewhere in
  `src/codegen_f16_f128.rs`: `codegen_bitcast`, the comparison helpers and the binop helpers were
  not swept for a matching hole. Only the cast entry point was audited here.
- `simd_cast_ptr`/`simd_expose_provenance`/`simd_with_exposed_provenance` and the integer identity
  `simd_cast` paths were not part of the sweep (they do not route through
  `codegen_f16_f128::codegen_cast`); integer identity casts are handled by `clif_intcast`'s explicit
  `(_, _) if from == to => val` arm, so they are safe by construction.
- Not checked against a released rustc: the reproduction needs `#![feature(f16, f128, repr_simd,
  core_intrinsics)]`, so this is nightly-only surface today. It will become reachable from stable
  code if/when `f16`/`f128` plus portable SIMD stabilise.
