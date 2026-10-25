# jxl-image/src/lib.rs:469:37

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: panic
- location: `jxl-image/src/lib.rs:469:37`
- last API call: `jxl_image::BitDepth::parse_integer_sample`
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
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-image = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_image::BitDepth;

fn main() {
    let bit_depth = BitDepth::FloatSample { bits_per_sample: 0u32, exp_bits: 0u32 };
    let _ = bit_depth.parse_integer_sample(0i32);
}
```

Run with `cargo run` in this directory (package `repro_jxl_image_src_lib_rs_469_37_11c5d9`).

## Output

```
thread 'main' (1298810) panicked at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-image/src/lib.rs:469:37:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <jxl_image::BitDepth>::parse_integer_sample
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-image/src/lib.rs:469:37
   4: repro_jxl_image_src_lib_rs_469_37_11c5d9::main
             at ./src/main.rs:5:23
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
