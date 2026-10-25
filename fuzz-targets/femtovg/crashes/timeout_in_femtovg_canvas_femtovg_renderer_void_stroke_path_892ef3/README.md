# timeout in femtovg::Canvas<femtovg::renderer::Void>::stroke_path

- crate: femtovg 0.27.0
- kind: timeout
- last API call: `femtovg::Canvas<femtovg::renderer::Void>::stroke_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling femtovg::Canvas<femtovg::renderer::Void>::stroke_path
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
femtovg = { version = "=0.27.0" }
```

`src/main.rs`:

```rust
use femtovg::Atlas;
use femtovg::BlendFactor;
use femtovg::Color;
use femtovg::CompositeOperationState;
use femtovg::Paint;
use femtovg::Path;
use femtovg::Solidity;
use femtovg::renderer::Void;

fn main() {
    let _ = Solidity::default();
    let _ = Void;
    let mut path = Path::new();
    path.ellipse(-1.0f32, 0.5f32, 0.0f32, 0.121f32);
    let _ = Path::new();
    let _ = CompositeOperationState::with_blend_factors(BlendFactor::One, BlendFactor::Zero);
    let _ = Atlas::new(11081732678320183081usize, 13980417984119302665usize);
    let mut canvas = <femtovg::Canvas<femtovg::renderer::Void>>::new(Void).unwrap();
    canvas.stroke_path(&path, &Paint::linear_gradient(9.282809e-22f32, 5.801334e-35f32, 520.0f32, 0.049f32, Color::hex("\r\n\t").premultiplied(), Color::rgbf(-1.1754944e-38f32, 956.0f32, 0.9999999f32)).with_line_dash(&[1.1920929e-7f32]));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_femtovg_canvas_femtovg_renderer_void_stroke_path_892ef3`).

