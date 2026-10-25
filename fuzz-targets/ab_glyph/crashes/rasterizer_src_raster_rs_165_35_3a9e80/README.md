# rasterizer/src/raster.rs:165:35

- crate: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e
- kind: panic
- location: `rasterizer/src/raster.rs:165:35`
- last API call: `ab_glyph_rasterizer::Rasterizer::draw_cubic`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8829444633921214082

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
    let mut rasterizer = Rasterizer::new(1usize, 10000usize);
    rasterizer.draw_cubic(Point { x: 1.0f32, y: 65536.0f32 }, Point { x: 0.0f32, y: -7110000000.0f32 }, Point { x: 0.0f32, y: 0.0f32 }, Point { x: -3.736144e16f32, y: 0.0f32 });
}
```

Run with `cargo run` in this directory (package `repro_rasterizer_src_raster_rs_165_35_3a9e80`).

## Output

```
thread 'main' (427730) panicked at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:165:35:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ab_glyph_rasterizer::raster::Rasterizer>::draw_line_scalar
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:165:35
   4: ab_glyph_rasterizer::raster::draw_line_avx2
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:309:10
   5: <ab_glyph_rasterizer::raster::Rasterizer>::draw_line
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:100:18
   6: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:241:18
   7: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
   8: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
   9: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  10: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  11: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  12: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  13: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  14: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  15: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  16: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  17: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  18: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  19: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  20: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  21: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  22: <ab_glyph_rasterizer::raster::Rasterizer>::tessellate_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:238:18
  23: <ab_glyph_rasterizer::raster::Rasterizer>::draw_cubic
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:214:14
  24: repro_rasterizer_src_raster_rs_165_35_3a9e80::main
             at ./src/main.rs:6:16
  25: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
