# epaint/src/image.rs:281:9

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/image.rs:281:9`
- last API call: `epaint::ColorImage::region_by_pixels`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9210768307512943331

## Message

```
y + h should be <= self.height(), but y: 0, h: 1, height: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
ecolor = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use ecolor::Color32;
use epaint::ColorImage;

fn main() {
    let _ = ColorImage::filled([1usize, 0usize], Color32::KHAKI).region_by_pixels([1usize, 0usize], [0usize, 1usize]);
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_image_rs_281_9_7e485c`).

## Output

```
thread 'main' (197504) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:281:9:
y + h should be <= self.height(), but y: 0, h: 1, height: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <epaint::image::ColorImage>::region_by_pixels
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/image.rs:281:9
   3: repro_epaint_src_image_rs_281_9_7e485c::main
             at ./src/main.rs:5:66
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
