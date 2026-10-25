# path/src/stroker.rs:918:9

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `path/src/stroker.rs:918:9`
- last API call: `tiny_skia_path::PathStroker::stroke`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.segment_count >= 0
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
use tiny_skia_path::PathBuilder;
use tiny_skia_path::PathStroker;
use tiny_skia_path::Stroke;

fn main() {
    let mut path_builder = PathBuilder::default();
    path_builder.quad_to(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    path_builder.cubic_to(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32);
    let _ = PathStroker::default().stroke(&path_builder.clone().finish().unwrap(), &Stroke::default(), 1.0f32);
}
```

Run with `cargo run` in this directory (package `repro_path_src_stroker_rs_918_9_575860`).

## Output

```
thread 'main' (22066) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:918:9:
assertion failed: self.segment_count >= 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <tiny_skia_path::stroker::PathStroker>::pre_join_to
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:918:9
   4: <tiny_skia_path::stroker::PathStroker>::line_to
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:466:18
   5: <tiny_skia_path::stroker::PathStroker>::cubic_to
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:558:18
   6: <tiny_skia_path::stroker::PathStroker>::stroke_inner
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:398:26
   7: <tiny_skia_path::stroker::PathStroker>::stroke
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/stroker.rs:301:14
   8: repro_path_src_stroker_rs_918_9_575860::main
             at ./src/main.rs:9:36
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
