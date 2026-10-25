# ndarray/src/dimension/dimension_trait.rs:943:1

- crate: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab
- kind: panic
- location: `ndarray/src/dimension/dimension_trait.rs:943:1`
- last API call: `ndarray::prelude::Dim<[usize; 6]>::zeros`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 0
 right: 6
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ndarray = { git = "https://github.com/rust-ndarray/ndarray", rev = "bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 6]> as ndarray::Dimension>::zeros(0usize);
}
```

Run with `cargo run` in this directory (package `repro_ndarray_src_dimension_dimension_trait_rs_943_1_39ec6d`).

## Output

```
thread 'main' (1258045) panicked at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dimension_trait.rs:943:1:
assertion `left == right` failed
  left: 0
 right: 6
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <ndarray::dimension::dim::Dim<[usize; 6]> as ndarray::dimension::dimension_trait::Dimension>::zeros
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dimension_trait.rs:924:17
   5: repro_ndarray_src_dimension_dimension_trait_rs_943_1_39ec6d::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
