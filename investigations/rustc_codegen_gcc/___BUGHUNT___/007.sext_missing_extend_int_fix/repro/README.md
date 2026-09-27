# sext_check.rs

Differential check exercising every real call site of `BuilderMethods::sext` reachable from
`rustc_codegen_ssa`/this backend's own intrinsic lowering:

- `simd_lt`/`simd_gt`/`simd_eq` (lowered via `rustc_codegen_ssa::base::compare_simd_types`, which
  calls `bx.sext(cmp, ret_ty)`), exercised with values straddling the sign bit for both a signed
  (`i32x4`) and an unsigned (`u32x4`) element type - the scenario the candidate hypothesized could
  get zero-extended instead of sign-extended if `sext` naively cast a value whose current GCC type
  is unsigned.
- `core::intrinsics::compare_bytes` (this backend's own `sym::compare_bytes` arm in
  `src/intrinsic/mod.rs` explicitly calls `self.sext(cmp, self.type_ix(32))` on the `memcmp`
  builtin's return value), checked for the correct sign of the returned ordering.

Run directly (no `Cargo.toml` needed, it's a single free-standing `#![feature(...)]` file, same
convention as this project's own `tests/run/*.rs`):

```
# ground truth, plain rustc/LLVM
rustc +nightly-2026-08-04 -O sext_check.rs -o sext_check_llvm && ./sext_check_llvm

# through this backend (from the project root, after ./y.sh build --sysroot --release)
CHANNEL=release ./y.sh rustc repro/sext_check.rs -O -o /tmp/sext_check_gcc && /tmp/sext_check_gcc
```

Both are expected to print `ALL CHECKS PASSED` and exit 0 - see `../output.txt` for the actual
captured run of both, at both `-Copt-level=0` and `-O`, plus 3 repeat runs. This candidate did not
reproduce the hypothesized bug; see `../RESULT.md` for why (the scalar cast branch inside `sext`
that the FIXME warns about turns out to be unreachable with a mismatched sign/width through any
call path this backend or `rustc_codegen_ssa` currently has).
