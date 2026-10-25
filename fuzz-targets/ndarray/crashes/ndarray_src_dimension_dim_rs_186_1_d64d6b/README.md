# ndarray/src/dimension/dim.rs:186:1

- crate: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab
- kind: panic
- location: `ndarray/src/dimension/dim.rs:186:1`
- last API call: `ndarray::prelude::Dim<[usize; 5]>::add<ndarray::prelude::Dim<[usize; 5]>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ndarray = { git = "https://github.com/rust-ndarray/ndarray", rev = "bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab" }
```

`src/main.rs`:

```rust
use ndarray::Axis;
use ndarray::prelude::Ix5;

fn main() {
    let axis = Axis(1usize);
    let usize_v = axis.index();
    let _ = <ndarray::prelude::Dim<[usize; 5]> as ::core::ops::Add<ndarray::prelude::Dim<[usize; 5]>>>::add(Ix5(0usize, 0usize, usize_v, 0usize, usize::MAX), Ix5(0usize, 0usize, 0usize, usize_v, 1usize));
}
```

Run with `cargo run` in this directory (package `repro_ndarray_src_dimension_dim_rs_186_1_d64d6b`).

## Output

```
thread 'main' (355228) panicked at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dim.rs:186:1:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ndarray::dimension::dim::Dim<[usize; 5]> as core::ops::arith::AddAssign<&ndarray::dimension::dim::Dim<[usize; 5]>>>::add_assign
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dim.rs:173:9
   4: <ndarray::dimension::dim::Dim<[usize; 5]> as core::ops::arith::Add>::add
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dim.rs:173:9
   5: repro_ndarray_src_dimension_dim_rs_186_1_d64d6b::main
             at ./src/main.rs:7:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
