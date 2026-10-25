# timeout in ab_glyph_rasterizer::Rasterizer::draw_cubic

- crate: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e
- kind: timeout
- last API call: `ab_glyph_rasterizer::Rasterizer::draw_cubic`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11591744400571705566

## Message

```
process ended abnormally (timeout) while calling ab_glyph_rasterizer::Rasterizer::draw_cubic
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
    let mut rasterizer = Rasterizer::new(0usize, usize::MAX);
    rasterizer.draw_cubic(<ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(Point { x: 2.8e16f32, y: 1e-40f32 }, Point { x: f32::NAN, y: -2187873800.0f32 }), Point::default(), Point { x: -1e30f32, y: 1e30f32 }, <ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(ab_glyph_rasterizer::point(1.0f32, -6.4f32), ab_glyph_rasterizer::point(65536.0f32, 5.2e-6f32)));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_ab_glyph_rasterizer_rasterizer_draw_cubic_42c57a`).

