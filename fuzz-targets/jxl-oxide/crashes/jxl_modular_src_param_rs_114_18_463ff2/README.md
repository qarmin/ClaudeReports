# jxl-modular/src/param.rs:114:18

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: panic
- location: `jxl-modular/src/param.rs:114:18`
- last API call: `jxl_modular::ChannelShift::from_jpeg_upsampling`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Invalid jpeg_upsampling value of 1000000000
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-modular = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_modular::ChannelShift;

fn main() {
    let _ = ChannelShift::from_jpeg_upsampling([1000000000u32, 0u32, 0u32], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_jxl_modular_src_param_rs_114_18_463ff2`).

## Output

```
thread 'main' (407275) panicked at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-modular/src/param.rs:114:18:
Invalid jpeg_upsampling value of 1000000000
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <jxl_modular::param::ChannelShift>::from_jpeg_upsampling
             at /home/rafal/.cargo/git/checkouts/jxl-oxide-a116d7996e62170f/c0cc4c7/crates/jxl-modular/src/param.rs:114:18
   3: repro_jxl_modular_src_param_rs_114_18_463ff2::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
