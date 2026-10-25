# allsorts/src/tables.rs:1140:22

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/tables.rs:1140:22`
- last API call: `allsorts::tables::Fixed::abs`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 13602012001933610793

## Message

```
attempt to negate with overflow
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
    let _ = <allsorts::tables::Fixed as ::core::ops::Add<allsorts::tables::Fixed>>::add(<allsorts::tables::Fixed as allsorts::binary::read::ReadFrom>::read_from(1i32), <allsorts::tables::Fixed as ::core::ops::Div<allsorts::tables::Fixed>>::div(Fixed::from_raw(0i32), Fixed::from(0i32))).abs();
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_tables_rs_1140_22_7bf1a4`).

## Output

```
thread 'main' (67499) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:429:5:
attempt to negate with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_neg_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32>::abs
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/int_macros.rs:3635:17
   4: <allsorts::tables::Fixed>::abs
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables.rs:1140:22
   5: repro_allsorts_src_tables_rs_1140_22_7bf1a4::main
             at ./src/main.rs:4:288
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
