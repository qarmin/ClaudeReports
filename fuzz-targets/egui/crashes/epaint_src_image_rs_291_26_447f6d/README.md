# epaint/src/image.rs:291:26

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/image.rs:291:26`
- last API call: `epaint::ColorImage::region_by_pixels`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1598155733189217454

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use emath::Vec2;
use epaint::ColorImage;

fn main() {
    let color_image = ColorImage { size: [usize::MAX, 18446744073709551614usize], source_size: Vec2::from((1.0000001f32, -1.0f32)), pixels: vec![Color32::BLACK, Color32::GREEN, Color32::from_additive_luminance(86u8)] };
    let _ = color_image.region_by_pixels([16777214usize, 65536usize], [31usize, 8191usize]);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_image_rs_291_26_447f6d`).

## Output

```
thread 'main' (385408) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:291:26:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <epaint::image::ColorImage>::region_by_pixels
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:291:26
   4: repro_epaint_src_image_rs_291_26_447f6d::main
             at ./src/main.rs:7:25
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
