# zune-image/src/frame.rs:354:17

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/frame.rs:354:17`
- last API call: `zune_image::codecs::hdr::HdrEncoder::encode_to_result`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18422929471204501166

## Message

```
assertion `left == right` failed
  left: 3
 right: 12
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
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::hdr::HdrEncoder;
use zune_image::image::Image;

fn main() {
    let mut hdr_encoder = HdrEncoder::default();
    let _ = <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut hdr_encoder, &Image::fill::<f32>(0.0f32, ColorSpace::Luma, 1usize, 1usize));
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_frame_rs_354_17_982464`).

## Output

```
thread 'main' (266050) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/frame.rs:354:17:
assertion `left == right` failed
  left: 3
 right: 12
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <zune_image::frame::Frame>::flatten::<f32>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/frame.rs:354:17
   5: <zune_image::image::Image>::flatten_frames::<f32>::{closure#0}
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:189:44
   6: core::iter::adapters::map::map_fold::<&zune_image::frame::Frame, alloc::vec::Vec<f32>, (), <zune_image::image::Image>::flatten_frames<f32>::{closure#0}, core::iter::traits::iterator::Iterator::for_each::call<alloc::vec::Vec<f32>, <alloc::vec::Vec<alloc::vec::Vec<f32>>>::extend_trusted<core::it... (+177 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:28
   7: <core::slice::iter::Iter<zune_image::frame::Frame> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<&zune_image::frame::Frame, alloc::vec::Vec<f32>, (), <zune_image::image::Image>::flatten_frames<f32>::{closure#0}, core::iter::traits::iterator::Iterato... (+282 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   8: <core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<alloc::vec::Vec<f32>, <alloc::vec::Vec<... (+211 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   9: <core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}> as core::iter::traits::iterator::Iterator>::for_each::<<alloc::vec::Vec<alloc::vec::Vec<f32>>>::extend_trusted<core::iter::adapters::map::Map<core::s... (+120 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:887:14
  10: <alloc::vec::Vec<alloc::vec::Vec<f32>>>::extend_trusted::<core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4132:26
  11: <alloc::vec::Vec<alloc::vec::Vec<f32>> as alloc::vec::spec_extend::SpecExtend<alloc::vec::Vec<f32>, core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:27:14
  12: <alloc::vec::Vec<alloc::vec::Vec<f32>> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<alloc::vec::Vec<f32>, core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter_nested.rs:60:16
  13: <alloc::vec::Vec<alloc::vec::Vec<f32>> as alloc::vec::spec_from_iter::SpecFromIter<alloc::vec::Vec<f32>, core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}>>>::from_iter
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_iter.rs:33:9
  14: <alloc::vec::Vec<alloc::vec::Vec<f32>> as core::iter::traits::collect::FromIterator<alloc::vec::Vec<f32>>>::from_iter::<core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3996:9
  15: <core::iter::adapters::map::Map<core::slice::iter::Iter<zune_image::frame::Frame>, <zune_image::image::Image>::flatten_frames<f32>::{closure#0}> as core::iter::traits::iterator::Iterator>::collect::<alloc::vec::Vec<alloc::vec::Vec<f32>>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2086:9
  16: <zune_image::image::Image>::flatten_frames::<f32>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:189:55
  17: <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_inner::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/codecs/hdr.rs:98:27
  18: <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode::<&mut alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:395:18
  19: <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_to_result
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:420:25
  20: repro_zune_image_src_frame_rs_354_17_982464::main
             at ./src/main.rs:7:13
  21: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
