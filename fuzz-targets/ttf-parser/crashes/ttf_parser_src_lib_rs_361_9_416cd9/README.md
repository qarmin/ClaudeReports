# ttf-parser/src/lib.rs:361:9

- crate: https://github.com/harfbuzz/ttf-parser @ 0c7291223fe9e0bc2808f0255cb991066eedd796
- kind: panic
- location: `ttf-parser/src/lib.rs:361:9`
- last API call: `ttf_parser::Rect::height`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ttf-parser = { git = "https://github.com/harfbuzz/ttf-parser", rev = "0c7291223fe9e0bc2808f0255cb991066eedd796" }
```

`src/main.rs`:

```rust
use ttf_parser::Rect;

fn main() {
    let rect = Rect { x_min: 0i16, y_min: i16::MAX, x_max: 0i16, y_max: -10000i16 };
    let _ = rect.height();
}
```

Run with `cargo run` in this directory (package `repro_ttf_parser_src_lib_rs_361_9_416cd9`).

## Output

```
thread 'main' (1330857) panicked at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/lib.rs:361:9:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ttf_parser::Rect>::height
             at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/lib.rs:361:9
   4: repro_ttf_parser_src_lib_rs_361_9_416cd9::main
             at ./src/main.rs:5:18
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
