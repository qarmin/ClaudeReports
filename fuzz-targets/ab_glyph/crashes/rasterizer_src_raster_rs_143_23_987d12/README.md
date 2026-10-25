# rasterizer/src/raster.rs:143:23

- crate: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e
- kind: panic
- location: `rasterizer/src/raster.rs:143:23`
- last API call: `ab_glyph_rasterizer::Rasterizer::draw_line`
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
use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let point = <ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(Point { x: f32::MAX, y: 0.0f32 }, Point::default());
    let mut rasterizer = Rasterizer::new(0usize, 1usize);
    rasterizer.draw_line(Point { x: 0.0f32, y: 6189.0f32 }, point);
}
```

Run with `cargo run` in this directory (package `repro_rasterizer_src_raster_rs_143_23_987d12`).

## Output

```
thread 'main' (1337219) panicked at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:143:23:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ab_glyph_rasterizer::raster::Rasterizer>::draw_line_scalar
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:143:23
   4: ab_glyph_rasterizer::raster::draw_line_avx2
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:309:10
   5: <ab_glyph_rasterizer::raster::Rasterizer>::draw_line
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:100:18
   6: repro_rasterizer_src_raster_rs_143_23_987d12::main
             at ./src/main.rs:7:16
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
