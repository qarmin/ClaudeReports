# stack overflow in kurbo::CubicBez::approx_spline

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: stack overflow
- last API call: `kurbo::CubicBez::approx_spline`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 1809143021950043356

## Message

```
process ended abnormally (stack overflow) while calling kurbo::CubicBez::approx_spline
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Rect;

fn main() {
    let bez_path = <kurbo::Rect as kurbo::Shape>::into_path(Rect::new(0.0f64, f64::MIN, 1.0f64, 0.0f64), 1.0f64);
    let path_seg = bez_path.get_seg(1usize).unwrap();
    let path_seg2 = path_seg.reverse();
    let cubic_bez = path_seg2.to_cubic();
    let _ = cubic_bez.approx_spline(0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_stack_overflow_in_kurbo_cubicbez_approx_spline_fa7294`).

## Output

```
thread 'main' (236842) has overflowed its stack
fatal runtime error: stack overflow, aborting
```
