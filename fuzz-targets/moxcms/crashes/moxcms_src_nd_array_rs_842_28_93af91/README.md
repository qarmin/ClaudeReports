# moxcms/src/nd_array.rs:842:28

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: panic
- location: `moxcms/src/nd_array.rs:842:28`
- last API call: `moxcms::Cube::trilinear_vec3`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 12884901885 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f" }
```

`src/main.rs`:

```rust
use moxcms::Cube;

fn main() {
    let vec = vec![];
    let cube = Cube::new(&vec, 0usize, 0usize).unwrap();
    let _ = cube.trilinear_vec3(0.0f32, 0.0f32, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_nd_array_rs_842_28_93af91`).

## Output

```
thread 'main' (357385) panicked at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:842:28:
range start index 12884901885 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[f32]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <[f32] as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <moxcms::nd_array::ArrayFetchVector3f as moxcms::nd_array::ArrayFetch<moxcms::matrix::Vector3<f32>>>::fetch
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:842:28
   6: <moxcms::nd_array::Cube>::trilinear::<moxcms::matrix::Vector3<f32>, moxcms::nd_array::ArrayFetchVector3f>
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:972:26
   7: <moxcms::nd_array::Cube>::trilinear_vec3
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:1234:14
   8: repro_moxcms_src_nd_array_rs_842_28_93af91::main
             at ./src/main.rs:6:18
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
