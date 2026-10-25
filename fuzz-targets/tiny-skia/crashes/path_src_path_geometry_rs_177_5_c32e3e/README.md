# path/src/path_geometry.rs:177:5

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `path/src/path_geometry.rs:177:5`
- last API call: `tiny_skia_path::path_geometry::chop_cubic_at2`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 4 but the index is 4
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
use tiny_skia_path::Point;

fn main() {
    tiny_skia_path::path_geometry::chop_cubic_at2(&[Point::from((1.0f32, 0.0f32)), Point::from((0.0f32, 0.0f32)), Point::zero(), Point::from((0.0f32, 0.0f32))], NormalizedF32Exclusive::HALF, &mut vec![Point { x: 0.0f32, y: 0.0f32 }, Point::default(), Point::zero(), Point::default()]);
}
```

Run with `cargo run` in this directory (package `repro_path_src_path_geometry_rs_177_5_c32e3e`).

## Output

```
thread 'main' (19600) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:177:5:
index out of bounds: the len is 4 but the index is 4
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: tiny_skia_path::path_geometry::chop_cubic_at2
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:177:5
   4: repro_path_src_path_geometry_rs_177_5_c32e3e::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
