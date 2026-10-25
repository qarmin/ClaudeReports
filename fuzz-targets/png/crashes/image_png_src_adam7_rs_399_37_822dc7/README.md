# image-png/src/adam7.rs:399:37

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/adam7.rs:399:37`
- last API call: `png::expand_interlaced_row`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
copy_from_slice: source slice length (1) does not match destination slice length (2)
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
    png::expand_interlaced_row(&mut vec![0u8, 80u8], 0usize, &vec![68u8], &Adam7Info::new(1u8, 0u32, 1u32), 16u8);
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_adam7_rs_399_37_822dc7`).

## Output

```
thread 'main' (1265023) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/adam7.rs:399:37:
copy_from_slice: source slice length (1) does not match destination slice length (2)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::copy_from_slice_impl::len_mismatch_fail::do_panic::runtime
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:172:21
   3: core::slice::copy_from_slice_impl::len_mismatch_fail::do_panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/intrinsics/mod.rs:2636:9
   4: core::slice::copy_from_slice_impl::len_mismatch_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   5: core::slice::copy_from_slice_impl::<u8>
   6: <[u8]>::copy_from_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:4344:18
   7: png::adam7::expand_pass
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/adam7.rs:399:37
   8: repro_image_png_src_adam7_rs_399_37_822dc7::main
             at ./src/main.rs:4:5
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
