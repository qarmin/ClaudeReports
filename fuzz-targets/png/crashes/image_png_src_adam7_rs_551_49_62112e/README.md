# image-png/src/adam7.rs:551:49

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/adam7.rs:551:49`
- last API call: `png::splat_interlaced_row`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 8 out of range for slice of length 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
png = { git = "https://github.com/image-rs/image-png", rev = "b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc" }
```

`src/main.rs`:

```rust
use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8], 1usize, &vec![0u8], &Adam7Info::new(1u8, 1u32, 1000000000u32), 8u8);
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_adam7_rs_551_49_62112e`).

## Output

```
thread 'main' (1264924) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/adam7.rs:551:49:
range end index 8 out of range for slice of length 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:427:13
   4: <core::ops::range::RangeTo<usize> as core::slice::index::SliceIndex<[u8]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:509:23
   5: <[u8] as core::ops::index::IndexMut<core::ops::range::RangeTo<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   6: png::adam7::expand_pass_splat
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/adam7.rs:551:49
   7: repro_image_png_src_adam7_rs_551_49_62112e::main
             at ./src/main.rs:4:5
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
