# zune-image/src/image.rs:205:65

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/image.rs:205:65`
- last API call: `zune_image::codecs::farbfeld::FarbFeldEncoder::encode_to_result`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17975080713688976539

## Message

```
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x1f814bf16f812452b283244dea52bee6) from expected TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14). This indicates you are converting a channel
             to a type it wasn't instantiated with
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
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::farbfeld::FarbFeldEncoder;
use zune_image::image::Image;

fn main() {
    let mut image = Image::fill::<f32>(0.0f32, ColorSpace::RGBA, 0usize, 0usize);
    image.set_depth(BitDepth::Sixteen);
    let mut farb_feld_encoder = FarbFeldEncoder::new();
    let _ = <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut farb_feld_encoder, &image);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_image_rs_205_65_d5c4f1`).

## Output

```
thread 'main' (537299) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:205:65:
called `Result::unwrap()` on an `Err` value: Different type id TypeId(0x1f814bf16f812452b283244dea52bee6) from expected TypeId(0x20eff0baf1c4a6b94cfebd6565ec4c14). This indicates you are converting a channel
             to a type it wasn't instantiated with
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<alloc::vec::Vec<u8>, zune_image::channel::ChannelErrors>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <zune_image::image::Image>::to_u8::{closure#0}
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:205:65
   5: core::iter::adapters::map::map_fold::<&zune_image::frame::Frame, alloc::vec::Vec<u8>, (), <zune_image::image::Image>::to_u8::{closure#0}, core::iter::traits::iterator::Iterator::for_each::call<alloc::vec::Vec<u8>, <alloc::vec::Vec<alloc::vec::Vec<u8>>>::extend_trusted<core::iter::adapters::map... (+146 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:28
   6: <core::slice::iter::Iter<zune_image::frame::Frame> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<&zune_image::frame::Frame, alloc::vec::Vec<u8>, (), <zune_image::image::Image>::to_u8::{closure#0}, core::iter::traits::iterator::Iterator::for_each::ca... (+251 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   7: <core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<alloc::vec::Vec<u8>, <alloc::vec::Vec<alloc::vec::Vec... (+181 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   8: <core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}> as core::iter::traits::iterator::Iterator>::for_each::<<alloc::vec::Vec<alloc::vec::Vec<u8>>>::extend_trusted<core::iter::adapters::map::Map<core::slice::iter::Ite... (+91 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:887:14
   9: <alloc::vec::Vec<alloc::vec::Vec<u8>>>::extend_trusted::<core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4132:26
  10: <alloc::vec::Vec<alloc::vec::Vec<u8>> as alloc::vec::spec_extend::SpecExtend<alloc::vec::Vec<u8>, core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:27:14
  11: <alloc::vec::Vec<alloc::vec::Vec<u8>> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<alloc::vec::Vec<u8>, core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:60:16
  12: <alloc::vec::Vec<alloc::vec::Vec<u8>> as alloc::vec::spec_from_iter::SpecFromIter<alloc::vec::Vec<u8>, core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  13: <alloc::vec::Vec<alloc::vec::Vec<u8>> as core::iter::traits::collect::FromIterator<alloc::vec::Vec<u8>>>::from_iter::<core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  14: <core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::to_u8::{closure#0}> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<alloc::vec::Vec<u8>>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  15: <zune_image::image::Image>::to_u8
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:206:18
  16: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_inner::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/codecs/farbfeld.rs:118:27
  17: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:397:18
  18: <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:420:25
  19: repro_zune_image_src_image_rs_205_65_d5c4f1::main
             at ./src/main.rs:10:13
  20: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
