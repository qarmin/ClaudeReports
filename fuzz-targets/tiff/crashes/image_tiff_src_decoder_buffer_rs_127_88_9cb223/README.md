# image-tiff/src/decoder/buffer.rs:127:88

- crate: https://github.com/image-rs/image-tiff @ 3302936b08e2260d4c66f40f51170efbb1fded4c
- kind: panic
- location: `image-tiff/src/decoder/buffer.rs:127:88`
- last API call: `tiff::decoder::DecodingSampleBuffer::as_buffer`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6571379263176733490

## Message

```
range start index 1 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiff = { git = "https://github.com/image-rs/image-tiff", rev = "3302936b08e2260d4c66f40f51170efbb1fded4c" }
```

`src/main.rs`:

```rust
use tiff::decoder::DecodingSampleBuffer;

fn main() {
    let mut decoding_sample_buffer = DecodingSampleBuffer::F32(vec![]);
    let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);
}
```

Run with `cargo run` in this directory (package `repro_image_tiff_src_decoder_buffer_rs_127_88_9cb223`).

## Output

```
thread 'main' (150555) panicked at /home/rafal/.cargo/git/checkouts/image-tiff-2c13a9b5ecc4a5fb/3302936/src/decoder/buffer.rs:127:88:
range start index 1 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[f32]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:558:13
   4: <[f32] as core::ops::index::IndexMut<core::ops::range::RangeFrom<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <alloc::vec::Vec<f32> as core::ops::index::IndexMut<core::ops::range::RangeFrom<usize>>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3943:9
   6: <tiff::decoder::buffer::DecodingSampleBuffer>::as_buffer
             at /home/rafal/.cargo/git/checkouts/image-tiff-2c13a9b5ecc4a5fb/3302936/src/decoder/buffer.rs:127:88
   7: repro_image_tiff_src_decoder_buffer_rs_127_88_9cb223::main
             at ./src/main.rs:5:56
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
