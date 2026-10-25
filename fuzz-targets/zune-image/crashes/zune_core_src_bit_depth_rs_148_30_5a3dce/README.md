# zune-core/src/bit_depth.rs:148:30

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-core/src/bit_depth.rs:148:30`
- last API call: `zune_image::core_filters::depth::Depth::execute`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13288233246015010891

## Message

```
Unknown bit type
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let mut image = Image::new(vec![], ImageMetadata::default().depth(), 0usize, 1usize, ColorSpace::ARGB);
    let _ = <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute(&Depth::new(BitDepth::Float32), &mut image);
}
```

Run with `cargo run` in this directory (package `repro_zune_core_src_bit_depth_rs_148_30_5a3dce`).

## Output

```
thread 'main' (903808) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-core/src/bit_depth.rs:148:30:
Unknown bit type
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <zune_core::bit_depth::BitDepth>::bit_type
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-core/src/bit_depth.rs:148:30
   3: <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/traits.rs:187:47
   4: repro_zune_core_src_bit_depth_rs_148_30_5a3dce::main
             at ./src/main.rs:9:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
