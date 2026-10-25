# image-png/src/adam7.rs:392:17

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/adam7.rs:392:17`
- last API call: `png::benchable_apis::adam7`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
png = { git = "https://github.com/image-rs/image-png", rev = "b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc", features = ["benchmarks", "zlib-rs"] }
```

`src/main.rs`:

```rust
fn main() {
    png::benchable_apis::adam7(&mut vec![], &vec![0u8], 1u32, 1u32, 8u8);
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_adam7_rs_392_17_eaf3d9`).

## Output

```
thread 'main' (1264114) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/adam7.rs:392:17:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: png::adam7::expand_pass
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/adam7.rs:392:17
   4: png::benchable_apis::adam7
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/benchable_apis.rs:30:9
   5: repro_image_png_src_adam7_rs_392_17_eaf3d9::main
             at ./src/main.rs:2:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
