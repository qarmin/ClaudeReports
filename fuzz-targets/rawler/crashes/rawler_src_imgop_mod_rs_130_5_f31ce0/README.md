# rawler/src/imgop/mod.rs:130:5

- crate: rawler 0.8.0
- kind: panic
- location: `rawler/src/imgop/mod.rs:130:5`
- source: https://docs.rs/crate/rawler/0.8.0/source/src/imgop/mod.rs#130
- last API call: `rawler::imgop::Rect::new_with_points`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: p1.x <= p2.x
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rawler = { version = "=0.8.0" }
```

`src/main.rs`:

```rust
use rawler::imgop::Point;
use rawler::imgop::Rect;

fn main() {
    let _ = Rect::new_with_points(Point::new(1usize, 0usize), Point::new(0usize, 0usize));
}
```

Run with `cargo run` in this directory (package `repro_rawler_src_imgop_mod_rs_130_5_f31ce0`).

## Output

```
thread 'main' (431681) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/imgop/mod.rs:130:5:
assertion failed: p1.x <= p2.x
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <rawler::imgop::Rect>::new_with_points
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rawler-0.8.0/src/imgop/mod.rs:130:5
   4: repro_rawler_src_imgop_mod_rs_130_5_f31ce0::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
