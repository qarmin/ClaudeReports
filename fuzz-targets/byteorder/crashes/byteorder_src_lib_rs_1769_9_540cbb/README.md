# byteorder/src/lib.rs:1769:9

- crate: https://github.com/BurntSushi/byteorder @ 5a82625fae462e8ba64cec8146b24a372b4d75c6
- kind: panic
- location: `byteorder/src/lib.rs:1769:9`
- last API call: `byteorder::BigEndian::default`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
BigEndian default
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
byteorder = { git = "https://github.com/BurntSushi/byteorder", rev = "5a82625fae462e8ba64cec8146b24a372b4d75c6" }
```

`src/main.rs`:

```rust
use byteorder::BigEndian;

fn main() {
    let _ = BigEndian::default();
}
```

Run with `cargo run` in this directory (package `repro_byteorder_src_lib_rs_1769_9_540cbb`).

## Output

```
thread 'main' (1256050) panicked at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:1769:9:
BigEndian default
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <byteorder::BigEndian as core::default::Default>::default
             at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:1769:9
   3: repro_byteorder_src_lib_rs_1769_9_540cbb::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
