# zune-image/src/codecs/farbfeld.rs:118:34

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/codecs/farbfeld.rs:118:34`
- last API call: `zune_image::codecs::farbfeld::FarbFeldEncoder::encode_to_result`
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
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false, features = ["bmp", "farbfeld", "hdr", "heic", "jpeg", "jpeg-xl", "jpeg_2000", "png", "ppm", "psd", "qoi", "webp", "metadata", "serde-support", "threads", "simd", "log"] }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::bit_depth::BitDepth;
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::codecs::farbfeld::FarbFeldEncoder;
use zune_image::image::Image;

fn main() {
    let color_space = ColorSpace::BGRA;
    let channel = Channel::new_with_bit_type(0usize, BitType::U8);
    let usize_v = channel.len();
    let bit_depth = BitDepth::Eight;
    let mut farb_feld_encoder = FarbFeldEncoder::new();
    let _ = <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut farb_feld_encoder, &Image::new_frames(vec![], bit_depth, 1usize, usize_v, color_space));
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_codecs_farbfeld_rs_118_34_f9c9a6`).

## Output

```
thread 'main' (3374460) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/codecs/farbfeld.rs:118:34:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[alloc::vec::Vec<u8>]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[alloc::vec::Vec<u8>] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<alloc::vec::Vec<u8>> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_inner::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/codecs/farbfeld.rs:118:34
   7: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:395:18
   8: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:420:25
   9: repro_zune_image_src_codecs_farbfeld_rs_118_34_f9c9a6::main
             at ./src/main.rs:14:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
