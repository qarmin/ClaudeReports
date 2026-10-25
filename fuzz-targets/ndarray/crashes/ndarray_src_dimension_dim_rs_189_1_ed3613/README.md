# ndarray/src/dimension/dim.rs:189:1

- crate: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab
- kind: panic
- location: `ndarray/src/dimension/dim.rs:189:1`
- last API call: `ndarray::prelude::Dim<[usize; 1]>::sub<usize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
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
    let _ = <ndarray::prelude::Dim<[usize; 1]> as ::core::ops::Sub<usize>>::sub(<ndarray::prelude::Dim<[usize; 1]> as ::core::default::Default>::default(), 1usize);
}
```

Run with `cargo run` in this directory (package `repro_ndarray_src_dimension_dim_rs_189_1_ed3613`).

## Output

```
thread 'main' (355186) panicked at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dim.rs:189:1:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ndarray::dimension::dim::Dim<[usize; 1]> as core::ops::arith::SubAssign<usize>>::sub_assign
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dim.rs:178:9
   4: <ndarray::dimension::dim::Dim<[usize; 1]> as core::ops::arith::Sub<usize>>::sub
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dim.rs:178:9
   5: repro_ndarray_src_dimension_dim_rs_189_1_ed3613::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
