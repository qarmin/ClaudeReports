# jxl-color/src/icc/synthesize.rs:98:9

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: panic
- location: `jxl-color/src/icc/synthesize.rs:98:9`
- last API call: `jxl_color::icc::colour_encoding_to_icc`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not yet implemented: ICC profile for XYB color space is not supported yet
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-color = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-image = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_image::color::EnumColourEncoding;
use jxl_image::color::RenderingIntent;

fn main() {
    let _ = jxl_color::icc::colour_encoding_to_icc(&EnumColourEncoding::xyb(RenderingIntent::Perceptual));
}
```

Run with `cargo run` in this directory (package `repro_jxl_color_src_icc_synthesize_rs_98_9_0e9ae2`).

## Output

```
thread 'main' (407239) panicked at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-color/src/icc/synthesize.rs:98:9:
not yet implemented: ICC profile for XYB color space is not supported yet
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: jxl_color::icc::synthesize::colour_encoding_to_icc
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-color/src/icc/synthesize.rs:98:9
   3: repro_jxl_color_src_icc_synthesize_rs_98_9_0e9ae2::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
