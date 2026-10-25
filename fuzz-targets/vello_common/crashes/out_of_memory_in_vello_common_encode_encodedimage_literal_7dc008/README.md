# out of memory in vello_common::encode::EncodedImage {literal}

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::encode::EncodedImage {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5955441818658472731

## Message

```
process ended abnormally (out of memory) while calling vello_common::encode::EncodedImage {literal}
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
peniko = { version = "=0.6.1" }
vello_cpu = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use kurbo::Affine;
use kurbo::Vec2;
use peniko::ImageSampler;
use vello_common::encode::EncodedImage;
use vello_common::paint::ImageSource;
use vello_common::pixmap::Pixmap;
use vello_cpu::Resources;

fn main() {
    let _ = EncodedImage { source: ImageSource::opaque_id(Resources::new().register_image(std::sync::Arc::new(Pixmap::new(32766u16, 16384u16)))), sampler: ImageSampler::default(), may_have_transparency: false, transform: Affine::IDENTITY, x_advance: Vec2::splat(0.0f64), y_advance: Vec2::from_angle(0.0f64), tint: None };
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_encode_encodedimage_literal_7dc008`).

## Output

```
memory allocation of 2147352576 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<color::rgba8::PremulRgba8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<color::rgba8::PremulRgba8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <color::rgba8::PremulRgba8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:14:21
   9: alloc::vec::from_elem::<color::rgba8::PremulRgba8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
  10: <vello_common::pixmap::Pixmap>::new
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/pixmap.rs:99:19
  11: repro_out_of_memory_in_vello_common_encode_encodedimage_literal_7dc008::main
             at ./src/main.rs:10:111
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
