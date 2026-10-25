# zune-image/src/iters.rs:134:33

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/iters.rs:134:33`
- last API call: `zune_image::image::Image::pixels[f32]`
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
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let image = Image::new_frames(vec![], BitDepth::Unknown, 0usize, 0usize, ColorSpace::LumaA);
    let _ = image.pixels::<f32>();
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_iters_rs_134_33_8849a8`).

## Output

```
thread 'main' (3372900) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/iters.rs:134:33:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[zune_image::frame::Frame]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[zune_image::frame::Frame] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<zune_image::frame::Frame> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <zune_image::image::Image>::pixels::<f32>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/iters.rs:134:33
   7: repro_zune_image_src_iters_rs_134_33_8849a8::main
             at ./src/main.rs:7:19
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
