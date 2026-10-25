# lopdf/src/filters/png.rs:136:87

- crate: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1
- kind: panic
- location: `lopdf/src/filters/png.rs:136:87`
- last API call: `lopdf::filters::png::encode_row`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lopdf = { git = "https://github.com/J-F-Liu/lopdf", rev = "bebffeff2d9e54b00325014a4fda07510eac2cb1" }
```

`src/main.rs`:

```rust
use lopdf::filters::png::FilterType;

fn main() {
    lopdf::filters::png::encode_row(FilterType::Avg, 0usize, &vec![], &mut vec![0u8]);
}
```

Run with `cargo run` in this directory (package `repro_lopdf_src_filters_png_rs_136_87_07e947`).

## Output

```
thread 'main' (396267) panicked at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/filters/png.rs:136:87:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: lopdf::filters::png::encode_row
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/filters/png.rs:136:87
   4: repro_lopdf_src_filters_png_rs_136_87_07e947::main
             at ./src/main.rs:4:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
