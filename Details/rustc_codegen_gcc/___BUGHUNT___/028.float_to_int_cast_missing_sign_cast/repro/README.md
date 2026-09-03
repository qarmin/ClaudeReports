# Repro for candidate 028 (float_to_int_cast_missing_sign_cast)

`boundary_values.rs` is a plain Rust source compiled through the project's real backend
(`./y.sh rustc`). It exercises `CodegenCx::float_to_int_cast`'s native-type early-return branch
(`f64 as u32/i32/u64/i64/usize/isize`) at boundary values where a wrong destination sign would
show up as a wrong numeric result (values near `u64::MAX`/`i64::MIN`, and specifically the native
`usize`/`isize` destination types the candidate description called out).

This is **not** a crash repro - it's a differential/boundary-value check. Expected output is
`OK` and exit code 0 in every configuration tried (see `RESULT.md`): this candidate's suspected
bug did not reproduce.
