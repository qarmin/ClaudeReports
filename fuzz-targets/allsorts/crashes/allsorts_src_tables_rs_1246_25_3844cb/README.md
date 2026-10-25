# allsorts/src/tables.rs:1246:25

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/tables.rs:1246:25`
- last API call: `allsorts::tables::Fixed::from<f64>`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8801206335138779543

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::tables::Fixed;

fn main() {
    let _ = Fixed::from(-32768.0f64);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_tables_rs_1246_25_3844cb`).

## Output

```
thread 'main' (94297) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables.rs:1246:25:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::tables::Fixed as core::convert::From<f64>>::from
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables.rs:1246:25
   4: repro_allsorts_src_tables_rs_1246_25_3844cb::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
