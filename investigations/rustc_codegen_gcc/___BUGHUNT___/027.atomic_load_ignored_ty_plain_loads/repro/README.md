# repro for candidate 027 (atomic_load_ignored_ty_plain_loads)

Three plain, `std`-only Rust source files, each compiled through the real, unmodified backend
entry point (`y.sh rustc`, i.e. `rustc -Zcodegen-backend=librustc_codegen_gcc.so`) and run for real,
then diffed against the same file compiled with plain host `rustc`/LLVM:

- `widen_and_compare.rs` - `AtomicI8/I16/I32/I64::load()` of a negative value and
  `AtomicU8/U16/U32/U64::load()` of a top-bit-set value, each widened (`as i32`/`as i64`) and
  compared against a literal `0`.
- `div_and_pointer_width.rs` - signed/unsigned division directly on a loaded value with no
  intervening cast, `AtomicIsize`/`AtomicUsize` (pointer-width) load + compare + widen, and a
  direct comparison between two separately-loaded `AtomicU8` values (no literal operand).
- `match_and_minmax.rs` - `match`/exclusive-range-pattern dispatch on a loaded negative `i8` and a
  loaded high-bit-set `u8`, and `.min()`/`.max()` on a loaded value.

All three print `OK ...`/`FAIL ...` per check and a final `TOTAL_FAILURES=N` line. Every check
matched plain `rustc`/LLVM in every run tried this session (`output.txt` in the parent folder) -
the suspected bug (candidate 027) did **not** reproduce through any of these consumption patterns.
See `../RESULT.md` for the full analysis of why (the sign-sensitive consumers involved -
`intcast`/`gcc_icmp` - already launder the operand's declared sign independently of what
`atomic_load` returns).
