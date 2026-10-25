# Repro for candidate 036 (vdso_symbol_resolution_unwrap_overflow)

The affected code (`Vdso`/`Vdso::sym()` in `src/backend/linux_raw/vdso.rs`) is
`pub(super)` - it has **no public entry point at all**. It's only ever reached
internally, once per process, by `src/backend/linux_raw/vdso_wrappers.rs`'s
own lazy `init()` (itself private), as a performance optimization underneath
public calls like `rustix::time::clock_gettime()`. There is no way to drive
this exact internal panic from outside the crate through any public API, and
the untrusted input it parses (the vDSO ELF image) is a single externally
-produced artifact handed to this parser once at process start (like a
malformed file - see `CONFIRMATION.md` section 11), not a live second
participant that could instead be exercised directly.

So this repro is a real, in-tree `#[cfg(test)]` unit test added directly to
`src/backend/linux_raw/vdso.rs` - the same file, the same private struct, the
same private `sym()` method the real `init()` calls, exercised with a hand
-built minimal classic-`DT_HASH` symbol table (one bucket, one symbol) built
entirely in-process. The only thing substituted for "a real kernel-supplied
vDSO" is the raw bytes of that table - not the logic, not the entry point,
not the caller.

`add_test.patch` adds exactly one new test function,
`test_vdso_sym_overflow_panics_via_unwrap_before_bounds_assert`, to the end of
`vdso.rs`'s own existing `#[cfg(test)] mod tests` (the same module the
project's own `test_vdso` lives in). It constructs a `Vdso` value whose one
symbol's `st_value` is chosen as the exact two's-complement negation of the
table's own (real) address - `load_addr.wrapping_add(st_value) == 0` - so
`base_plus`'s `checked_add` is guaranteed to overflow and `addr_from_elf`
returns `None`. Calling the real `vdso.sym(name, name)` then panics inside
`.unwrap()` at `src/backend/linux_raw/vdso.rs:394` (the classic-hash branch)
- never reaching the `assert!` right after it, which was clearly meant to be
the actual bounds check for a bogus-but-non-overflowing address.

## Running it

From a clone of the project at `base_commit` (see `../justfile`, which does
this automatically):

```sh
git apply /path/to/repro/add_test.patch
cargo test --lib --features time,stdio,fs backend::vdso::tests::test_vdso_sym_overflow
```

- Against the unpatched tree: the test **panics** (`called
  'Option::unwrap()' on a 'None' value`) - this is the bug, Status: FAILED.
- Against the tree with `../patch.diff` also applied: the test **passes**
  (`sym()` now returns a null pointer instead of panicking) - see
  `../justfile`'s `reproduce-after`.

Verified 3/3 debug-build runs and 1/1 release-build run, both before
(panics every time, deterministic - the boundary value is computed from the
real runtime address every run, not fixed/seeded) and after (passes every
time) applying `../patch.diff`.
