# image-gif/src/reader/decoder.rs:507:31

- crate: https://github.com/image-rs/image-gif @ a5f89ddc2b29424334244c97456b4a26e208aa7d
- kind: panic
- location: `image-gif/src/reader/decoder.rs:507:31`
- last API call: `gif::streaming_decoder::StreamingDecoder::current_frame_mut`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6754737278276853222

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gif = { git = "https://github.com/image-rs/image-gif", rev = "a5f89ddc2b29424334244c97456b4a26e208aa7d" }
```

`src/main.rs`:

```rust
use gif::streaming_decoder::StreamingDecoder;

fn main() {
    let mut streaming_decoder = StreamingDecoder::new();
    let _ = streaming_decoder.current_frame_mut();
}
```

Run with `cargo run` in this directory (package `repro_image_gif_src_reader_decoder_rs_507_31_27137a`).

## Output

```
thread 'main' (315783) panicked at src/main.rs:5:31:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<&mut gif::common::Frame>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <gif::reader::decoder::StreamingDecoder>::current_frame_mut
             at /home/rafal/.cargo/git/checkouts/image-gif-b1971d000e94ed23/a5f89dd/src/reader/decoder.rs:507:31
   6: repro_image_gif_src_reader_decoder_rs_507_31_27137a::main
             at ./src/main.rs:5:31
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
