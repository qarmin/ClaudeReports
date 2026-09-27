# simd_funnel_shl / simd_funnel_shr missing - hard `span_err` fails the whole build

`src/intrinsics/simd.rs` implements the generic SIMD intrinsics by name and ends in an `_ =>` arm
that calls `fx.tcx.dcx().span_err(span, format!("Unknown SIMD intrinsic {}", intrinsic))` - a hard
error, unlike the unknown-`llvm.x86.*` path in `src/intrinsics/llvm_x86.rs`, which only warns and
emits a run-time trap. `simd_funnel_shl` and `simd_funnel_shr` have no arm, so any crate that
reaches them fails to compile outright. They are reachable from **stable** Rust: stdarch implements
every AVX512F rotate (`_mm{,256,512}_ro[lr]_epi{32,64}`, `_mm*_ro[lr]v_epi*`, and their
`mask`/`maskz` variants) and every AVX512VBMI2 `_mm*_sh[lr]d*` on top of them. Lead came from
`IDEAS.md` / the 2026-09-02 hodow audit (8958 AVX512F + 150 AVX512VBMI2 tests in that suite).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential test vs LLVM, real `std::arch` path (`tools/diffcheck.sh`) | `_mm512_rol_epi32::<1>` behind `#[target_feature(enable = "avx512f")]`, debug/opt-level 0, cg_clif @ 2bc05d45 | Reproduced on the first try: LLVM builds and prints the right answer, cg_clif emits `error: Unknown SIMD intrinsic simd_funnel_shl` (pointing into `stdarch/.../avx512f.rs`) and aborts the build |
| Differential test on the generic intrinsic (`simd_funnel_shl`/`_shr` called directly) | `#[repr(simd)]` u8x16 / i16x8 / u32x4 / i64x2, boundary operands (0, 1, MAX, sign bit, 0x55.., 0xAA..) and shift amounts 0 / 1 / bits/2 / bits-1, debug | Same hard error for both intrinsics; also produced the LLVM reference values the fix was written against. Boundary work is what surfaced the `shift == 0` subtlety (Cranelift masks shift amounts, so a naive `b >> (bits - s)` yields `b` instead of `0`) |
| Generated sweep (new: `tools/gen_funnel_sweep.py`) | 10 lane types (u8/i8/u16/i16/u32/i32/u64/i64/u128/i128) x lane counts 2..64 up to 1024-bit vectors, every shift amount `0..bits-1` on every lane, boundary + xorshift operands; ~51.8k printed lane results; self-checked against a scalar reference in-program | Unpatched: build fails. Patched: cg_clif output byte-identical to LLVM's and both agree with the scalar reference (`selfcheck failures: 0`) |
| Real end-to-end `std::arch` family check (`repro/`) | `_mm512_rol/ror/rolv/rorv_epi32`, `_mm512_shldi/shrdi/shldv_epi32`, immediates 0/1/16/31, run through `dist/cargo-clif` | Unpatched: `error: Unknown SIMD intrinsic simd_funnel_shl` x2, `exit 101`, no binary. Patched: binary runs, `failures: 0`, matches LLVM |

## Status

**Confirmed**

Deterministic: `bash ___BUGHUNT___/tools/diffcheck.sh tmp/c003/rol.rs` exits 1 (cg_clif compile
failure) on **3/3** runs against the unpatched main tree, and exits 0 (identical to LLVM) on
**3/3** runs against the patched worktree. Same debug build (opt-level 0), rustc
nightly-2026-08-24, cg_clif @ 2bc05d45, Cranelift 0.135. A compile-time diagnostic has no
non-determinism to speak of.

**Fully reproducible**: **Yes**. `repro/` is a real crate built by the real `dist/cargo-clif`
wrapper; the failure observed is the real backend's own diagnostic on the real
`library/.../stdarch` source, and after the patch the real generated binary runs and prints its
values, which are then diffed against the plain-rustc build of the same crate. Nothing is
hand-asserted or stubbed.

## Class

compile-error

## Root cause

`codegen_simd_intrinsic_call` in `src/intrinsics/simd.rs` dispatches on the intrinsic `Symbol`.
There is no arm for `sym::simd_funnel_shl` / `sym::simd_funnel_shr`, so both fall into the final
`_ =>` arm at the bottom of the function, which is:

```rust
_ => {
    fx.tcx.dcx().span_err(span, format!("Unknown SIMD intrinsic {}", intrinsic));
    // Prevent verifier error
    fx.bcx.ins().trap(TrapCode::user(1 /* unreachable */).unwrap());
    return;
}
```

`span_err` is a hard error, so the compilation session fails and no artifact is produced. That is a
deliberately different policy from the unimplemented-`llvm.x86.*` path
(`src/intrinsics/llvm_x86.rs`), which emits a `span_warn` plus `codegen_panic_nounwind` - a crate
using an unimplemented vendor intrinsic still builds. The consequence is that one AVX512F rotate
anywhere in a dependency tree fails the whole build under cg_clif, with a diagnostic that points at
stdarch's source rather than at the user's own call.

The intrinsics themselves are not hard to lower: rustc's own semantics (see
`rustc_const_eval/src/interpret/intrinsics/simd.rs`) are, per lane, `funnel_shl(a,b,s) = (a << s) |
(b >>u (bits - s))` and `funnel_shr(a,b,s) = (a << (bits - s)) | (b >>u s)`, with `s >= bits` being
UB, and `bits - s` needing to behave as an *unbounded* shift so that `s == 0` contributes 0 rather
than the whole other operand. The only real trap for a Cranelift lowering is exactly that: `ishl`
and `ushr` mask the shift amount to the operand width, so the plain `bits - s` form silently
returns `a | b` for `s == 0`.

## Introduced by

not determined as a single commit, and cg_clif's own history is not where it happened:
`git log -S simd_funnel --all -- src/` in this repo returns nothing at all - the symbol was never
present in `src/`. The gap opened upstream, when rustc gained `simd_funnel_shl`/`simd_funnel_shr`
and stdarch switched the AVX512 rotates onto them; that rustc PR is not in this repository's
history and `gh` search for it was not available in this sandbox. The first commit here that
records the consequence is 2eccb7a5 "Fix rustc test suite" (2025-07-01, bjorn3), which added
`rm tests/ui/simd/intrinsic/generic-arithmetic-pass.rs # unimplemented simd_funnel_{shl,shr}` to
`scripts/test_rustc_tests.sh`, i.e. the gap was already known and worked around by then.

## The fix

`patch.diff` (+43/-0, `src/intrinsics/simd.rs`) adds a `sym::simd_funnel_shl | sym::simd_funnel_shr`
arm that scalarizes lane by lane through the existing `simd_trio_for_each_lane` helper, matching how
the neighbouring `simd_shl`/`simd_shr` arm works. Per lane:

- `inv = bnot(shift)`. Cranelift masks every shift amount to the operand width, and `!s` masked to
  `log2(bits)` bits is exactly `bits - 1 - s` for the `0 <= s < bits` range the intrinsic requires -
  so no `iconst` is needed and the code works unchanged for `i128` lanes (where `iconst` isn't
  available).
- `shl`: `(a << s) | ((b >>u 1) >>u inv)`. The split `>>u 1` then `>>u (bits-1-s)` totals
  `bits - s` for `s >= 1` and yields 0 for `s == 0`, which is the `unbounded_shr` behaviour rustc's
  semantics require. `ushr` (logical) is used for the `b` half regardless of lane signedness.
- `shr`: `((a << 1) << inv) | (b >>u s)`, the mirror image.

Why it is sufficient beyond the one input that first triggered it (CONFIRMATION.md section 7):

- `tools/gen_funnel_sweep.py` covers 10 lane types x 6 lane counts (u8..i128, 2..64 lanes, i.e.
  16-bit through 1024-bit vectors, so both the native-width and the scalarized >128-bit paths),
  every shift amount `0..bits-1` on every lane, boundary operands (0, 1, MAX, sign bit, MAX>>1,
  0x55.., 0xAA..) plus a deterministic xorshift stream - 51809 printed lines. cg_clif's output is
  byte-identical to LLVM's and both match the in-program scalar reference.
- `repro/` additionally covers the real `std::arch` surface: `_mm512_rol/ror_epi32` at immediates
  0/1/16/31, `_mm512_rolv/rorv_epi32` with a per-lane amount vector, and AVX512VBMI2
  `_mm512_shldi/shrdi_epi32` at 0/1/31 plus `_mm512_shldv_epi32`.
- Separately checked by hand against LLVM: `_mm512_rol_epi64::<1>/<63>`, `_mm512_ror_epi64::<0>`,
  `_mm512_rolv_epi64`, `_mm512_mask_rol_epi32`, `_mm512_maskz_rol_epi32`, `_mm256_rol_epi32`,
  `_mm256_ror_epi32`, `_mm256_rolv_epi64`, `_mm_rol_epi32`, `_mm_ror_epi64`,
  `_mm512_shldi/shrdi_epi16`, `_mm512_shldi/shrdi_epi64`, `_mm512_shrdv_epi32` - all identical to
  LLVM. That probe is kept in this folder as `extra_arch_probe.rs` (single-file, for
  `tools/diffcheck.sh`): exit 1 against the unpatched backend, exit 0 / identical against the
  patched one.

Before/after of `repro/` (captured verbatim in `output.txt`): unpatched cg_clif prints
`error: Unknown SIMD intrinsic simd_funnel_shl` / `..._shr` and exits 101 with no binary; patched
cg_clif builds it, the binary runs and prints `failures: 0` with every line matching the plain-rustc
build.

This folder's own `justfile` was actually run end-to-end from a fresh clone of 2bc05d45 (not just
read): `build-backend` -> `reproduce-before` (LLVM `failures: 0`; cg_clif `error: Unknown SIMD
intrinsic simd_funnel_shl` / `..._shr`, exit 101) -> `apply` (43 insertions) -> `rebuild-backend`
-> `reproduce-after` (cg_clif builds, runs, `failures: 0`, `IDENTICAL to LLVM`), exit 0. Only the
`clone` recipe itself could not run in the sandbox this was verified in (ssh host-key verification
is not set up there); the clone was made with the equivalent
`https://github.com/rust-lang/rustc_codegen_cranelift.git` URL at the same `base_commit`, which the
justfile documents as a `just repo_url=...` override.

- Project test suite: `./y.sh test` in the isolated worktree, **before** the patch: passes
  (exit 0, final in-tree item `test result: ok. 1200 passed; 0 failed`); **after** the patch:
  passes (exit 0, same `1200 passed; 0 failed`). No new failures.

## What's not done / open questions

- The patch implements the two intrinsics; it does **not** change the `_ =>` arm's hard-error
  policy. Whether `span_err` there should become `span_warn` + trap (matching `llvm_x86.rs`) is a
  maintainer decision - it is the reason a single missing generic SIMD intrinsic is a whole-build
  failure rather than a run-time abort, and the same trap is waiting for the next generic SIMD
  intrinsic rustc adds.
- Shift amounts `>= bits` are UB per rustc's semantics; the lowering does not trap on them, it just
  computes a masked-shift result. That matches LLVM's `llvm.fsh[lr]` (which masks) closely enough
  in practice but is not a guarantee - the const-eval interpreter reports UB for that case instead.
- Not measured: whether re-running `tools/hodow_audit/` with this patch turns the 8958 AVX512F +
  150 AVX512VBMI2 tests from "build blocked" into passes, or whether some of them then fail on a
  *different* missing intrinsic. That re-measurement is the natural follow-up (it is also
  `IDEAS.md`'s "correctness of the 27.8% that never executed" row).
- Not checked: `simd_funnel_shl`/`_shr` on `f16`/`f128` or mask-typed lanes (rustc's type check
  should reject non-integer lanes; the new arm asserts that rather than handling it).
- The hodow audit's "2 unsupported generic SIMD intrinsics" are exactly these two, so this
  candidate closes that whole compile-time gap - but only for the generic SIMD surface. The 701
  unsupported `llvm.x86.*` names are a separate, much larger backlog item.

## Upstream status (checked 2026-09-03, after this candidate was written)

Nothing above is retracted - the bug is still live in `HEAD` (2bc05d45) exactly as described - but
the finding is **not new upstream**, which a reader should know before filing anything:

- Issue #1696 "Implement simd_funnel_shl and simd_funnel_shr intrinsics" (opened 2026-08-26)
  reports the same hard error, and names the concrete consequence this candidate reached
  independently: the rustc test `tests/ui/simd/intrinsic/generic-arithmetic-pass.rs` is skipped in
  `scripts/test_rustc_tests.sh` with the comment `unimplemented simd_funnel_{shl,shr}`.
- PR #1698 "Implement SIMD funnel shift intrinsics" (opened 2026-08-29) is **open and unmerged**
  as of 2026-09-03, so `Fixed by` in `SUMMARY.md` stays blank.

The two fixes differ in one respect worth comparing if this is ever taken further: #1698's
description says it handles the `shift == 0` case explicitly, while `patch.diff` here is
branchless - it relies on Cranelift masking shift amounts to the operand width so that
`bnot(shift)` equals `bits - 1 - shift`, plus a fixed shift by one, which yields `0` rather than
the unshifted operand when `shift == 0`. Neither has been differentially tested against the other.
