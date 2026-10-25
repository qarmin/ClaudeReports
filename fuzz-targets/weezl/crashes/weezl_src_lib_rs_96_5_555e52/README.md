# weezl/src/lib.rs:96:5

- crate: https://github.com/image-rs/weezl @ 606f9c79b054176ed409152a4852b2e047460d2b
- kind: panic
- location: `weezl/src/lib.rs:96:5`
- last API call: `weezl::decode::Configuration::with_tiff_size_switch`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Maximum code size 12 required, got 100
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
weezl = { git = "https://github.com/image-rs/weezl", rev = "606f9c79b054176ed409152a4852b2e047460d2b" }
```

`src/main.rs`:

```rust
use weezl::BitOrder;
use weezl::decode::Configuration;

fn main() {
    let _ = Configuration::with_tiff_size_switch(BitOrder::Lsb, 100u8);
}
```

Run with `cargo run` in this directory (package `repro_weezl_src_lib_rs_96_5_555e52`).

## Output

```
thread 'main' (566409) panicked at /home/rafal/.cargo/git/checkouts/weezl-78c4ebe2635a1a2c/606f9c7/src/lib.rs:96:5:
Maximum code size 12 required, got 100
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: weezl::assert_decode_size
             at /home/rafal/.cargo/git/checkouts/weezl-78c4ebe2635a1a2c/606f9c7/src/lib.rs:96:5
   3: <weezl::decode::Configuration>::with_tiff_size_switch
             at /home/rafal/.cargo/git/checkouts/weezl-78c4ebe2635a1a2c/606f9c7/src/decode.rs:238:9
   4: repro_weezl_src_lib_rs_96_5_555e52::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
