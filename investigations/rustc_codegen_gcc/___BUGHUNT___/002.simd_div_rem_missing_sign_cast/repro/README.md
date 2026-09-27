# Repro scripts for candidate 002 (simd_div_rem_missing_sign_cast)

Both files are standalone Rust source files (no external crates, `std` only), compiled directly
with the codegen-under-test via the project's own `y.sh rustc` entry point - the real, unmodified
end-to-end path a user hits when compiling any Rust program with this backend (`rustc
-Zcodegen-backend=librustc_codegen_gcc.so`, wrapped by `y.sh rustc`).

- `single_case.rs` - one `i32x4`/`u32x4` pair, transmuted from a negative `i32x4` into a `u32x4`,
  then divided/remaindered with `core::intrinsics::simd::simd_div`. Checks the result against the
  correct unsigned semantics.
- `all_widths.rs` - the same idea generalized: 8/16/32/64-bit element widths, both directions
  (a signed vector transmuted from an unsigned bit pattern, and vice versa), for both `simd_div`
  and `simd_rem`.

Both use `unsafe { transmute(...) }` to reinterpret the bit pattern of one repr(simd) vector as
another with the opposite element signedness - mirroring `tests/run/ptr_to_int_div.rs`
(commit 6f389ff) but for SIMD vectors instead of a scalar pointer-to-integer transmute, per this
candidate's hypothesis.

## Running

Requires the project's own toolchain already set up (see `___BUGHUNT___/README.md`):

```
./y.sh prepare
./y.sh build --sysroot --release
CHANNEL=release ./y.sh rustc repro/single_case.rs -o /tmp/single_case && /tmp/single_case
CHANNEL=release ./y.sh rustc repro/all_widths.rs -o /tmp/all_widths && /tmp/all_widths
```

Both currently print `OK`/matching values and exit 0 - see `../output.txt` for the actual captured
run. See `../RESULT.md` for why this candidate's specific hypothesis did not reproduce, and for a
third, code-level check (temporary debug print in `division_operation`) that independently confirms
the vector's GCC-level element type is already correctly signed by the time it reaches division,
which is the reason the missing cast in `division_operation`'s vector branch is currently harmless.
