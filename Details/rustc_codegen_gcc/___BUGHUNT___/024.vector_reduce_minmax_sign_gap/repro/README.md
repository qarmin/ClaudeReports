Modeled on candidate 020's `repro/all_widths.rs` (`simd_shr`), applied to
`vector_reduce_min`/`vector_reduce_max` instead.

`vector_reduce_minmax_sign.rs` builds, for each of the 8/16/32/64-bit integer widths, `#[repr(simd)]`
vectors whose elements have their high bit set (so a signed vs. unsigned interpretation of the same
bit pattern genuinely differs), then calls `core::intrinsics::simd::simd_reduce_min`/
`simd_reduce_max` on them and checks the result against a plain-Rust `[T]::iter().min()/max()`
reference oracle (no LLVM/SIMD codegen involved on the oracle side, so it is trivially correct):

- `check_direct!` builds the unsigned vector and the signed vector (same bit pattern, same width)
  independently, via ordinary struct construction - the baseline case.
- `check_transmuted!` builds one sign's vector, then `mem::transmute`s it into the opposite-signed
  vector type of the same width before reducing - the shape candidate 024's backlog note flagged as
  not yet ruled out (does the GCC-level element sign get re-derived from the transmute's destination
  type, or does it stay stale from the source type - the same question candidate 020 answered "no
  bug" for `simd_shr`).
- A final block repeats `check_direct!` with extreme boundary values (`0`/`MAX`, and the exact
  overflow edge for the `difference_or_zero` Hacker's-Delight-style arithmetic
  `vector_reduce_min`/`max` use internally: `a - b` for the widest and narrowest same-width values in
  one vector).

Run against the real backend end-to-end: `rustc -Zcodegen-backend=.../librustc_codegen_gcc.so
--sysroot .../build_sysroot/sysroot -C link-arg=-lc -o <out> vector_reduce_minmax_sign.rs`
(a real `rustc` invocation using the real GCC codegen backend), then run `<out>` as a real native
binary - not a hand-simulated codegen step. All checks passed (`exit 0`) in every configuration
tried - see `output.txt`.
