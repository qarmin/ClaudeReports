# image-png/src/decoder/mod.rs:346:13

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/decoder/mod.rs:346:13`
- last API call: `png::Reader<::std::io::Cursor<Vec<u8>>>::next_frame_info`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 717681547012870426

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
png = { git = "https://github.com/image-rs/image-png", rev = "b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc" }
```

`src/main.rs`:

```rust
fn main() {
    let decoder = <png::Decoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()));
    let mut reader = decoder.read_info().unwrap();
    let _ = reader.finish();
    let _ = reader.next_frame_info();
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_decoder_mod_rs_346_13_43480e`).

## Output

```
thread 'main' (775074) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/decoder/mod.rs:346:13:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <png::decoder::Reader<core::io::cursor::Cursor<alloc::vec::Vec<u8>>>>::next_frame_info
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/decoder/mod.rs:346:13
   4: repro_image_png_src_decoder_mod_rs_346_13_43480e::main
             at ./src/main.rs:5:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
