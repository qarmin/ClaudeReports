# ndarray/src/dimension/remove_axis.rs:68:1

- crate: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab
- kind: panic
- location: `ndarray/src/dimension/remove_axis.rs:68:1`
- last API call: `ndarray::prelude::Dim<[usize; 3]>::remove_axis`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: axis.index() < self.ndim()
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
use ndarray::prelude::Ix3;
use ndarray::prelude::Ix5;

fn main() {
    let dim = Ix5(0usize, 0usize, 0usize, 1usize, 1usize);
    let usize_v = <ndarray::prelude::Dim<[usize; 5]> as ndarray::Dimension>::ndim(&dim);
    let _ = <ndarray::prelude::Dim<[usize; 3]> as ndarray::RemoveAxis>::remove_axis(&Ix3(0usize, 0usize, 0usize), Axis(usize_v));
}
```

Run with `cargo run` in this directory (package `repro_ndarray_src_dimension_remove_axis_rs_68_1_de4e0f`).

## Output

```
thread 'main' (355645) panicked at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/remove_axis.rs:68:1:
assertion failed: axis.index() < self.ndim()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <ndarray::dimension::dim::Dim<[usize; 3]> as ndarray::dimension::remove_axis::RemoveAxis>::remove_axis
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/remove_axis.rs:53:17
   4: repro_ndarray_src_dimension_remove_axis_rs_68_1_de4e0f::main
             at ./src/main.rs:8:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
