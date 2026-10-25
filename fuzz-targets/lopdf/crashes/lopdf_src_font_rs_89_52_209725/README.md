# lopdf/src/font.rs:89:52

- crate: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1
- kind: panic
- location: `lopdf/src/font.rs:89:52`
- last API call: `lopdf::FontData::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Failed to parse font file: OutOfBounds
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lopdf = { git = "https://github.com/J-F-Liu/lopdf", rev = "bebffeff2d9e54b00325014a4fda07510eac2cb1", features = ["async", "chrono", "chrono-clock", "embed_image", "font_embedding", "image", "jiff", "rayon", "serde", "time", "tokio", "wasm_js"] }
```

`src/main.rs`:

```rust
use lopdf::FontData;

fn main() {
    let _ = FontData::new(&vec![], "".to_string());
}
```

Run with `cargo run` in this directory (package `repro_lopdf_src_font_rs_89_52_209725`).

## Output

```
thread 'main' (395631) panicked at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/font.rs:89:52:
Failed to parse font file: OutOfBounds
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<read_fonts::FontRef, read_fonts::read::ReadError>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <lopdf::font::FontData>::new
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/font.rs:89:52
   5: repro_lopdf_src_font_rs_89_52_209725::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
