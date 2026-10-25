# timeout in kurbo::Arc::winding

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Arc::winding`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 17870330840229693831

## Message

```
process ended abnormally (timeout) while calling kurbo::Arc::winding
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let point = Point::ZERO;
    let _ = <kurbo::Arc as kurbo::Shape>::winding(&Arc::new(point, Vec2::ZERO, 0.0f64, f64::INFINITY, 0.0f64), point);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_arc_winding_7a58c3`).

