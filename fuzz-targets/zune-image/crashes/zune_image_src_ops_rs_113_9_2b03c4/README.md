# zune-image/src/ops.rs:113:9

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/ops.rs:113:9`
- last API call: `zune_image::image::Image::sub<zune_image::image::Image>`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9595792436205066163

## Message

```
assertion `left == right` failed: Dimensions mismatch, (0, 0) != (0, 1)
  left: (0, 0)
 right: (0, 1)
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
    let _ = <zune_image::image::Image as ::core::ops::Sub<zune_image::image::Image>>::sub(Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 0usize, ColorSpace::HSL), Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 1usize, ColorSpace::HSL));
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_ops_rs_113_9_2b03c4`).

## Output

```
thread 'main' (1684469) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/ops.rs:113:9:
assertion `left == right` failed: Dimensions mismatch, (0, 0) != (0, 1)
  left: (0, 0)
 right: (0, 1)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<(usize, usize), (usize, usize)>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: zune_image::ops::check_full_compatibility
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/ops.rs:32:5
   5: <zune_image::image::Image as core::ops::arith::Sub>::sub
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/ops.rs:113:9
   6: repro_zune_image_src_ops_rs_113_9_2b03c4::main
             at ./src/main.rs:6:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
