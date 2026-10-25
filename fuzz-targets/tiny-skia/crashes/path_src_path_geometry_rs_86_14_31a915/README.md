# path/src/path_geometry.rs:86:14

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `path/src/path_geometry.rs:86:14`
- last API call: `tiny_skia_path::path_geometry::chop_quad_at`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 2 but the index is 2
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
use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::PathBuilder;
use tiny_skia_path::Point;

fn main() {
    let f32x2 = tiny_skia_path::f32x2::splat(0.0f32);
    let mut path_builder = PathBuilder::new();
    path_builder.line_to(0.0f32, 0.0f32);
    tiny_skia_path::path_geometry::chop_quad_at(&[Point::from_f32x2(f32x2), Point::from_xy(0.0f32, 0.0f32)], NormalizedF32Exclusive::ANY, &mut [Point { x: 0.0f32, y: 0.0f32 }, path_builder.last_point().unwrap(), Point::default(), path_builder.last_point().unwrap(), path_builder.last_point().unwrap()]);
}
```

Run with `cargo run` in this directory (package `repro_path_src_path_geometry_rs_86_14_31a915`).

## Output

```
thread 'main' (19144) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:86:14:
index out of bounds: the len is 2 but the index is 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: tiny_skia_path::path_geometry::chop_quad_at
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:86:14
   4: repro_path_src_path_geometry_rs_86_14_31a915::main
             at ./src/main.rs:9:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
