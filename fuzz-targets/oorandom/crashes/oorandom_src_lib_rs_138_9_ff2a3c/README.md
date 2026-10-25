# oorandom/src/lib.rs:138:9

- crate: oorandom 11.1.5
- kind: panic
- location: `oorandom/src/lib.rs:138:9`
- source: https://docs.rs/crate/oorandom/11.1.5/source/src/lib.rs#138
- last API call: `oorandom::Rand32::rand_range`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: range.start < range.end
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
oorandom = { version = "=11.1.5" }
```

`src/main.rs`:

```rust
use oorandom::Rand32;

fn main() {
    let mut rand32 = Rand32::new_inc(0u64, 0u64);
    let _ = rand32.rand_range(0u32..0u32);
}
```

Run with `cargo run` in this directory (package `repro_oorandom_src_lib_rs_138_9_ff2a3c`).

## Output

```
thread 'main' (551228) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/oorandom-11.1.5/src/lib.rs:138:9:
assertion failed: range.start < range.end
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <oorandom::Rand32>::rand_range
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/oorandom-11.1.5/src/lib.rs:138:9
   4: repro_oorandom_src_lib_rs_138_9_ff2a3c::main
             at ./src/main.rs:5:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
