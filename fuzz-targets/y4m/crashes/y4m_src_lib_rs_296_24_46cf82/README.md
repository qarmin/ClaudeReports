# y4m/src/lib.rs:296:24

- crate: https://github.com/image-rs/y4m @ 76635a27866db6bad1ec5f9b08711dfa908f2e6d
- kind: panic
- location: `y4m/src/lib.rs:296:24`
- last API call: `y4m::EncoderBuilder::write_header`
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
y4m = { git = "https://github.com/image-rs/y4m", rev = "76635a27866db6bad1ec5f9b08711dfa908f2e6d" }
```

`src/main.rs`:

```rust
use y4m::EncoderBuilder;
use y4m::Ratio;

fn main() {
    let encoder_builder = EncoderBuilder::new(usize::MAX, 12usize, Ratio::new(1usize, 0usize));
    let _ = encoder_builder.write_header::<Vec<u8>>(include_bytes!("mutated_0_Cargo.toml").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_y4m_src_lib_rs_296_24_46cf82`).

## Output

```
thread 'main' (522233) panicked at /home/rafal/.cargo/git/checkouts/y4m-1316f13b6ec08d4a/76635a2/src/lib.rs:296:24:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: y4m::get_plane_sizes
             at /home/rafal/.cargo/git/checkouts/y4m-1316f13b6ec08d4a/76635a2/src/lib.rs:296:24
   4: <y4m::EncoderBuilder>::write_header::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/y4m-1316f13b6ec08d4a/76635a2/src/lib.rs:627:37
   5: repro_y4m_src_lib_rs_296_24_46cf82::main
             at ./src/main.rs:6:29
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
