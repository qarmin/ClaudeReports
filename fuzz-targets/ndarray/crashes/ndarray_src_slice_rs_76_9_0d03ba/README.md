# ndarray/src/slice.rs:76:9

- crate: https://github.com/rust-ndarray/ndarray @ bd3ade99c1f6d1fbd0f31866153e2d155e7b75ab
- kind: panic
- location: `ndarray/src/slice.rs:76:9`
- last API call: `ndarray::Slice::step_by`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left != right` failed: Slice::step_by: step must be nonzero
  left: 0
 right: 0
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
use ndarray::Slice;

fn main() {
    let axis = Axis(0usize);
    let usize_v = axis.index();
    let slice = Slice::from(usize_v..0usize);
    let _ = slice.step_by(0isize);
}
```

Run with `cargo run` in this directory (package `repro_ndarray_src_slice_rs_76_9_0d03ba`).

## Output

```
thread 'main' (355054) panicked at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/slice.rs:76:9:
assertion `left != right` failed: Slice::step_by: step must be nonzero
  left: 0
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<isize, isize>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <ndarray::slice::Slice>::step_by
             at /home/rafal/.cargo/git/checkouts/ndarray-28e6343d23a7e0ef/bd3ade9/src/slice.rs:76:9
   5: repro_ndarray_src_slice_rs_76_9_0d03ba::main
             at ./src/main.rs:8:19
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
