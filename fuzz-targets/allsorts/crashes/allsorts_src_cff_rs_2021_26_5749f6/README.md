# allsorts/src/cff.rs:2021:26

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/cff.rs:2021:26`
- last API call: `allsorts::cff::Operand::from<f32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 1838052262080775438

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::cff::Operand;

fn main() {
    let _ = Operand::from(f32::NAN);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_cff_rs_2021_26_5749f6`).

## Output

```
thread 'main' (21736) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/cff.rs:2021:26:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <allsorts::cff::Operand>::bcd_encode
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/cff.rs:2021:26
   4: <allsorts::cff::Operand as core::convert::From<f32>>::from
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/cff.rs:2069:9
   5: repro_allsorts_src_cff_rs_2021_26_5749f6::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
