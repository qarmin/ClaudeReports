# rawler/src/pixarray.rs:77:35

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/pixarray.rs:77:35`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/pixarray.rs#77
- last API call: `rawler::pixarray::Pix2D<f32>::new_uninit`
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
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <rawler::pixarray::Pix2D<f32>>::new_uninit(10000000000000000000usize, 65536usize);
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_pixarray_rs_77_35_64a27c`).

## Output

```
thread 'main' (430939) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/pixarray.rs:77:35:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rawler::pixarray::Pix2D<f32>>::new_uninit
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/pixarray.rs:77:35
   4: repro_rawler_src_pixarray_rs_77_35_64a27c::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
