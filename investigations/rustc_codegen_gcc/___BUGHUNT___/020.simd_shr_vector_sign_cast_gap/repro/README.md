Modeled on `tests/run/ptr_to_int_div.rs` and on candidate 002's `repro/all_widths.rs`, applied to
`simd_shr` instead of `simd_div`/`simd_rem`.

`all_widths.rs` builds, for each of the 8/16/32/64-bit widths, a `#[repr(simd)]` vector of one
sign, `transmute`s it to the opposite-signed vector type of the same width (so the GCC-level
element type at the point of the shift is whatever the *destination* Rust type's construction
gives it, exactly the shape candidate 018 found broken for the scalar case), then calls
`core::intrinsics::simd::simd_shr` on it and checks the result against a plain `as`-cast scalar
reference computed with Rust's own (guaranteed-correct) `>>` operator semantics:

- unsigned destination -> `simd_shr` must dispatch to a **logical** shift (zero-fill), even though
  the raw bit pattern's top bit is set (it came from a negative value).
- signed destination -> `simd_shr` must dispatch to an **arithmetic** shift (sign-fill), even
  though the raw bit pattern came from a large unsigned value.

Run against the real backend end-to-end: `CHANNEL=release ./y.sh rustc repro/all_widths.rs -o
<out>` (a real `rustc` invocation using the real `-Zcodegen-backend=librustc_codegen_gcc.so`),
then run `<out>` as a real native binary - not a hand-simulated codegen step.

All 8 checks (4 widths x 2 directions) passed with the correct shift kind - see `output.txt`.
