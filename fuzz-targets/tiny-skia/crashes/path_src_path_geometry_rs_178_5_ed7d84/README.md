# path/src/path_geometry.rs:178:5

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `path/src/path_geometry.rs:178:5`
- last API call: `tiny_skia_path::path_geometry::chop_cubic_at2`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 5 but the index is 5
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
    let normalized_f32_exclusive = NormalizedF32Exclusive::ANY;
    let point = <tiny_skia_path::Point as ::core::ops::Add<tiny_skia_path::Point>>::add(Point::from_xy(0.0f32, 0.0f32), Point::from_xy(0.0f32, 0.0f32));
    tiny_skia_path::path_geometry::chop_cubic_at2(&[point, point, point, point], normalized_f32_exclusive, &mut vec![point, point, point, point, point]);
}
```

Run with `cargo run` in this directory (package `repro_path_src_path_geometry_rs_178_5_ed7d84`).

## Output

```
thread 'main' (19973) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:178:5:
index out of bounds: the len is 5 but the index is 5
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: tiny_skia_path::path_geometry::chop_cubic_at2
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_geometry.rs:178:5
   4: repro_path_src_path_geometry_rs_178_5_ed7d84::main
             at ./src/main.rs:7:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
