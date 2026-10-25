# rasterizer/src/raster.rs:187:17

- crate: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e
- kind: panic
- location: `rasterizer/src/raster.rs:187:17`
- last API call: `ab_glyph_rasterizer::Rasterizer::draw_quad`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ab_glyph = { git = "https://github.com/alexheretic/ab-glyph", rev = "3eb21a592819d9ed78e9a13ad7392d46c60ce39e" }
ab_glyph_rasterizer = { git = "https://github.com/alexheretic/ab-glyph", rev = "3eb21a592819d9ed78e9a13ad7392d46c60ce39e" }
```

`src/main.rs`:

```rust
use ab_glyph::Rect;
use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let rect = Rect { min: ab_glyph_rasterizer::point(1.0f32, 0.0f32), max: Point::default() };
    let f32_v = rect.width();
    let mut rasterizer = Rasterizer::new(0usize, 1usize);
    rasterizer.draw_quad(ab_glyph_rasterizer::point(0.0f32, f32_v), Point::default(), Point { x: 1e30f32, y: 0.0f32 });
}
```

Run with `cargo run` in this directory (package `repro_rasterizer_src_raster_rs_187_17_e35e9f`).

## Output

```
thread 'main' (1337134) panicked at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:187:17:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ab_glyph_rasterizer::raster::Rasterizer>::draw_quad
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:187:17
   4: repro_rasterizer_src_raster_rs_187_17_e35e9f::main
             at ./src/main.rs:9:16
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
