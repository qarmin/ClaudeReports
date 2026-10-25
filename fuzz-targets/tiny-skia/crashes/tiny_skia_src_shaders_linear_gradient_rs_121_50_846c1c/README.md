# tiny-skia/src/shaders/linear_gradient.rs:121:50

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `tiny-skia/src/shaders/linear_gradient.rs:121:50`
- last API call: `tiny_skia::LinearGradient::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiny-skia = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
tiny-skia-path = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
```

`src/main.rs`:

```rust
use tiny_skia::Color;
use tiny_skia::ColorU8;
use tiny_skia::GradientStop;
use tiny_skia::LinearGradient;
use tiny_skia::SpreadMode;
use tiny_skia_path::Point;
use tiny_skia_path::Transform;

fn main() {
    let color_u8 = ColorU8::from_rgba(0u8, 0u8, 0u8, 0u8);
    let u8_v = color_u8.alpha();
    let color = Color::BLACK;
    let _ = LinearGradient::new(Point::zero(), Point::default(), vec![GradientStop::new(1.0f32, color), GradientStop::new(0.0f32, Color::from_rgba8(u8_v, 0u8, 0u8, 0u8)), GradientStop::new(0.0f32, Color::WHITE)], SpreadMode::Reflect, Transform::default());
}
```

Run with `cargo run` in this directory (package `repro_tiny_skia_src_shaders_linear_gradient_rs_121_50_846c1c`).

## Output

```
thread 'main' (21091) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/shaders/linear_gradient.rs:121:50:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<tiny_skia::color::Color>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: tiny_skia::shaders::linear_gradient::average_gradient_color::store_color
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/shaders/linear_gradient.rs:121:50
   6: tiny_skia::shaders::linear_gradient::average_gradient_color
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/shaders/linear_gradient.rs:161:5
   7: <tiny_skia::shaders::linear_gradient::LinearGradient>::new
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/shaders/linear_gradient.rs:77:52
   8: repro_tiny_skia_src_shaders_linear_gradient_rs_121_50_846c1c::main
             at ./src/main.rs:13:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
