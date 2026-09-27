# Repro scripts for candidate 004 (atomic_fetch_max_min_sign_bug)

Both files are standalone Rust source files (no external crates, `std` only), compiled directly
with the codegen-under-test via the project's own `y.sh rustc` entry point - the real, unmodified
end-to-end path a user hits when compiling any Rust program with this backend (`rustc
-Zcodegen-backend=librustc_codegen_gcc.so`, wrapped by `y.sh rustc`). No second participant is
involved (this is a single-process codegen-correctness question, not one participant reacting to
another's input), so `CONFIRMATION.md` section 11 does not apply here.

- `atomic_extremum.rs` - stops at the first wrong result via `assert_eq!` (exit code 101 on
  failure). Used for the repeat-run/determinism check in `../output.txt`.
- `atomic_extremum_report.rs` - the same set of boundary-value checks, but prints `OK`/`FAIL` for
  every case instead of stopping at the first failure, so a single run shows the full pattern
  (every `Atomic{U8,U16,U32,U64}::fetch_max`/`fetch_min` case near/above the sign bit fails, every
  `Atomic{I8,I16,I32,I64}` case passes) - this is the file used for the before/after comparison in
  `../output.txt` and `../RESULT.md`.

Both exercise `core::sync::atomic::Atomic{U,I}{8,16,32,64}::fetch_max`/`fetch_min` at values
straddling the type's sign bit (100 vs. 200 for `u8`, `i32::MIN`/`u32::MAX`, `i64::MIN`/`u64::MAX`,
etc.), which is what a real caller of these public `std` atomic APIs produces - `rustc_codegen_ssa`
lowers `fetch_max`/`fetch_min` to a single `AtomicRmw` MIR/codegen operation
(`AtomicRmwBinOp::Atomic{Max,Min,UMax,UMin}`), which this backend's `Builder::atomic_rmw`
(`src/builder.rs:1774`) dispatches straight to `atomic_extremum` (`src/builder.rs:96`) - the exact
function this candidate targets.

## Running

Requires the project's own toolchain already set up (see `___BUGHUNT___/README.md`):

```
./y.sh prepare
./y.sh build --sysroot --release
CHANNEL=release ./y.sh rustc repro/atomic_extremum_report.rs -o /tmp/atomic_extremum_report && /tmp/atomic_extremum_report
```

Before the fix (`patch.diff` applied), several `Atomic{U8,U16,U32,U64}::fetch_max`/`fetch_min`
lines print `FAIL` (see `../output.txt`); with `patch.diff` applied and the backend rebuilt, all
lines print `OK`, matching plain `rustc`/LLVM. See `../RESULT.md` for the full before/after and the
root cause.
