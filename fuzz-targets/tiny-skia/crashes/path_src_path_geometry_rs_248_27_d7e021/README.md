# path/src/path_geometry.rs:248:27

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `path/src/path_geometry.rs:248:27`
- last API call: `tiny_skia_path::PathStroker::stroke`
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
use tiny_skia_path::LineCap;
use tiny_skia_path::LineJoin;
use tiny_skia_path::PathBuilder;
use tiny_skia_path::PathStroker;
use tiny_skia_path::Stroke;
use tiny_skia_path::Transform;

fn main() {
    let line_cap = LineCap::default();
    let mut path_stroker = PathStroker::new();
    let _ = path_stroker.stroke(&PathBuilder::from_circle(0.0f32, 0.0f32, 1.0f32).unwrap().transform(Transform::from_scale(-1e30f32, 4.5584144e30f32)).unwrap(), &Stroke { width: 1.0f32, miter_limit: 0.0f32, line_cap: line_cap, line_join: LineJoin::Round, dash: None }, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_path_src_path_geometry_rs_248_27_d7e021`).

## Output

```
thread 'main' (21362) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:248:27:
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
   4: <core::option::Option<strict_num::NormalizedF32>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: tiny_skia_path::path_geometry::find_quad_max_curvature
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:248:27
   6: tiny_skia_path::stroker::check_quad_linear
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:1801:13
   7: <tiny_skia_path::stroker::PathStroker>::quad_to
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:478:43
   8: <tiny_skia_path::stroker::PathStroker>::stroke_inner
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:394:26
   9: <tiny_skia_path::stroker::PathStroker>::stroke
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:301:14
  10: repro_path_src_path_geometry_rs_248_27_d7e021::main
             at ./src/main.rs:11:26
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
