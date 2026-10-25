# krilla/src/interactive/annotation.rs:224:10

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/interactive/annotation.rs:224:10`
- last API call: `krilla::annotation::LinkAnnotation::new_with_quad_points`
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
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::annotation::LinkAnnotation;
use krilla::annotation::Target;
use krilla::destination::Destination;
use krilla::destination::XyzDestination;
use krilla::geom::Point;
use krilla::geom::Quadrilateral;

fn main() {
    let _ = LinkAnnotation::new_with_quad_points(vec![Quadrilateral([Point::from_xy(0.0f32, 0.0f32), Point { x: 0.0f32, y: 0.0f32 }, Point::from_xy(0.0f32, 0.0f32), Point { x: 0.0f32, y: f32::MIN }])], Target::Destination(Destination::Xyz(XyzDestination::new(0usize, Point::from_xy(0.0f32, 0.0f32)))));
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_interactive_annotation_rs_224_10_100312`).

## Output

```
thread 'main' (250446) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/annotation.rs:224:10:
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
   4: <core::option::Option<krilla::geom::Rect>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <krilla::interactive::annotation::LinkAnnotation>::new_with_quad_points
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/interactive/annotation.rs:224:10
   6: repro_krilla_src_interactive_annotation_rs_224_10_100312::main
             at ./src/main.rs:9:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
