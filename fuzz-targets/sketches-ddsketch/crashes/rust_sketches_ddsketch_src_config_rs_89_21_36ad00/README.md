# rust-sketches-ddsketch/src/config.rs:89:21

- crate: https://github.com/mheffner/rust-sketches-ddsketch @ 9c517a0996152f9f12a609d7b720d7cead01fede
- kind: panic
- location: `rust-sketches-ddsketch/src/config.rs:89:21`
- last API call: `sketches_ddsketch::encoding::decode_from_java_bytes`
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
sketches-ddsketch = { git = "https://github.com/mheffner/rust-sketches-ddsketch", rev = "9c517a0996152f9f12a609d7b720d7cead01fede" }
```

`src/main.rs`:

```rust
use sketches_ddsketch::Config;
use sketches_ddsketch::DDSketch;

fn main() {
    let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&DDSketch::new(Config::new(0.0f64, 0u32, 1.0f64)));
    let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&vec);
}
```

Run with `cargo run` in this directory (package `repro_rust_sketches_ddsketch_src_config_rs_89_21_36ad00`).

## Output

```
thread 'main' (1278327) panicked at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/config.rs:89:21:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <sketches_ddsketch::config::Config>::from_gamma
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/config.rs:89:21
   4: sketches_ddsketch::encoding::decode_from_java_bytes
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/encoding.rs:410:18
   5: repro_rust_sketches_ddsketch_src_config_rs_89_21_36ad00::main
             at ./src/main.rs:6:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
