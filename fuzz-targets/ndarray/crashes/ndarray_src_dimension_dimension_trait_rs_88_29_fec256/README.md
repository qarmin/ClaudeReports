# ndarray/src/dimension/dimension_trait.rs:88:29

- crate: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab
- kind: panic
- location: `ndarray/src/dimension/dimension_trait.rs:88:29`
- last API call: `ndarray::prelude::Dim<[usize; 6]>::size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ndarray = { git = "https://github.com/rust-ndarray/ndarray", rev = "bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab" }
```

`src/main.rs`:

```rust
use ndarray::prelude::Ix6;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 6]> as ndarray::Dimension>::size(&Ix6(1usize, 1usize, 1usize, usize::MAX, 256usize, 4294967296usize));
}
```

Run with `cargo run` in this directory (package `repro_ndarray_src_dimension_dimension_trait_rs_88_29_fec256`).

## Output

```
thread 'main' (355555) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:206:1:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <usize as core::ops::arith::Mul>::mul
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:350:45
   4: <usize as core::ops::arith::Mul<&usize>>::mul
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/internal_macros.rs:39:17
   5: <usize as core::iter::traits::accum::Product<&usize>>::product::<core::slice::iter::Iter<usize>>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:88:28
   6: <core::slice::iter::Iter<usize> as core::iter::traits::iterator::Iterator>::fold::<usize, <usize as core::iter::traits::accum::Product<&usize>>::product<core::slice::iter::Iter<usize>>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   7: <usize as core::iter::traits::accum::Product<&usize>>::product::<core::slice::iter::Iter<usize>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:85:22
   8: <core::slice::iter::Iter<usize> as core::iter::traits::iterator::Iterator>::product::<usize>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3702:9
   9: <ndarray::dimension::dim::Dim<[usize; 6]> as ndarray::dimension::dimension_trait::Dimension>::size
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/dimension/dimension_trait.rs:88:29
  10: repro_ndarray_src_dimension_dimension_trait_rs_88_29_fec256::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
