# epaint/src/viewport.rs:38:31

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: panic
- location: `epaint/src/viewport.rs:38:31`
- last API call: `epaint::PaintCallbackInfo::clip_rect_in_pixels`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 9243965788510865915

## Message

```
min > max. min = 0, max = -1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
emath = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use emath::Rect;
use epaint::PaintCallbackInfo;

fn main() {
    let paint_callback_info = PaintCallbackInfo { viewport: Rect::everything_right_of(0.0f32), clip_rect: Rect::ZERO, pixels_per_point: 0.0f32, screen_size_px: [u32::MAX, 0u32] };
    let _ = paint_callback_info.clip_rect_in_pixels();
}
```

Run with `cargo run` in this directory (package `repro_epaint_src_viewport_rs_38_31_a9e982`).

## Output

```
thread 'main' (147975) panicked at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/viewport.rs:38:31:
min > max. min = 0, max = -1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <i32 as core::cmp::Ord>::clamp::do_panic::runtime
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:172:21
   3: <i32 as core::cmp::Ord>::clamp::do_panic
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics/mod.rs:2636:9
   4: <i32 as core::cmp::Ord>::clamp
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic.rs:177:9
   5: <epaint::viewport::ViewportInPixels>::from_points
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/viewport.rs:38:31
   6: <epaint::shapes::paint_callback::PaintCallbackInfo>::clip_rect_in_pixels
             at /home/rafal/.cargo/git/checkouts/egui-9c90d2f9798966b7/86099ac/crates/epaint/src/shapes/paint_callback.rs:52:9
   7: repro_epaint_src_viewport_rs_38_31_a9e982::main
             at ./src/main.rs:6:33
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
