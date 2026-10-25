# ttf-parser/src/parser.rs:932:5

- crate: https://github.com/harfbuzz/ttf-parser @ 0c7291223fe9e0bc2808f0255cb991066eedd796
- kind: panic
- location: `ttf-parser/src/parser.rs:932:5`
- last API call: `ttf_parser::NormalizedCoordinate::from<f32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: val.is_finite()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ttf-parser = { git = "https://github.com/harfbuzz/ttf-parser", rev = "0c7291223fe9e0bc2808f0255cb991066eedd796" }
```

`src/main.rs`:

```rust
use ttf_parser::NormalizedCoordinate;

fn main() {
    let _ = NormalizedCoordinate::from(f32::NEG_INFINITY);
}
```

Run with `cargo run` in this directory (package `repro_ttf_parser_src_parser_rs_932_5_b6d2a2`).

## Output

```
thread 'main' (1330755) panicked at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/parser.rs:932:5:
assertion failed: val.is_finite()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: ttf_parser::parser::f32_bound
             at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/parser.rs:932:5
   4: <ttf_parser::NormalizedCoordinate as core::convert::From<f32>>::from
             at /home/rafal/.cargo/git/checkouts/ttf-parser-a65fc4bf0fbc20c0/0c72912/src/lib.rs:177:31
   5: repro_ttf_parser_src_parser_rs_932_5_b6d2a2::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
