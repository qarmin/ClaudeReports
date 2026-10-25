# rust-sketches-ddsketch/src/ddsketch.rs:203:9

- crate: https://github.com/mheffner/rust-sketches-ddsketch @ 9c517a0996152f9f12a609d7b720d7cead01fede
- kind: panic
- location: `rust-sketches-ddsketch/src/ddsketch.rs:203:9`
- last API call: `sketches_ddsketch::DDSketch::merge`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
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
    let vec = sketches_ddsketch::encoding::encode_to_java_bytes(&DDSketch::new(Config::default()));
    let mut d_d_sketch = DDSketch::default();
    let mut d_d_sketch2 = sketches_ddsketch::encoding::decode_from_java_bytes(&vec).unwrap();
    d_d_sketch2.add_with_count(0.0f64, u64::MAX);
    d_d_sketch.add(0.0f64);
    let _ = d_d_sketch.merge(&d_d_sketch2);
}
```

Run with `cargo run` in this directory (package `repro_rust_sketches_ddsketch_src_ddsketch_rs_203_9_a70f5c`).

## Output

```
thread 'main' (1278863) panicked at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/ddsketch.rs:203:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <sketches_ddsketch::ddsketch::DDSketch>::merge
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/ddsketch.rs:203:9
   4: repro_rust_sketches_ddsketch_src_ddsketch_rs_203_9_a70f5c::main
             at ./src/main.rs:10:24
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
