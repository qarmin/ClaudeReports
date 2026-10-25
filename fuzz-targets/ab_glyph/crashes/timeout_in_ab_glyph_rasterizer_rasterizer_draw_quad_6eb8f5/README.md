# timeout in ab_glyph_rasterizer::Rasterizer::draw_quad

- crate: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e
- kind: timeout
- last API call: `ab_glyph_rasterizer::Rasterizer::draw_quad`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3000089465530431569

## Message

```
process ended abnormally (timeout) while calling ab_glyph_rasterizer::Rasterizer::draw_quad
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
    let mut rasterizer = Rasterizer::new(0usize, 10000usize);
    rasterizer.draw_quad(ab_glyph_rasterizer::point(f32::INFINITY, 65536.0f32), ab_glyph_rasterizer::point(0.0f32, f32::NAN), Point::default());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_ab_glyph_rasterizer_rasterizer_draw_quad_6eb8f5`).

