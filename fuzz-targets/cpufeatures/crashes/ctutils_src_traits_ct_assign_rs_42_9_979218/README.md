# ctutils/src/traits/ct_assign.rs:42:9

- crate: cpufeatures 0.3.1
- kind: panic
- location: `ctutils/src/traits/ct_assign.rs:42:9`
- last API call: `ctutils::Choice::ct_assign_slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1316400172363284171

## Message

```
assertion `left == right` failed: source slice length (0) does not match destination slice length (1)
  left: 1
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ctutils = { version = "=0.4.3" }
```

`src/main.rs`:

```rust
use ctutils::Choice;

fn main() {
    <ctutils::Choice as ctutils::CtAssignSlice>::ct_assign_slice(&mut vec![Choice::from_u32_lsb(1u32)], &[], Choice::FALSE);
}
```

Run with `cargo run` in this directory (package `repro_ctutils_src_traits_ct_assign_rs_42_9_979218`).

## Output

```
thread 'main' (1019315) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ctutils-0.4.3/src/traits/ct_assign.rs:42:9:
assertion `left == right` failed: source slice length (0) does not match destination slice length (1)
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <ctutils::choice::Choice as ctutils::traits::ct_assign::CtAssignSlice>::ct_assign_slice
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ctutils-0.4.3/src/traits/ct_assign.rs:42:9
   5: repro_ctutils_src_traits_ct_assign_rs_42_9_979218::main
             at ./src/main.rs:4:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
