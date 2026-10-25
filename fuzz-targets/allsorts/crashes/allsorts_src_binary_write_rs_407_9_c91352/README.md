# allsorts/src/binary/write.rs:407:9

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/binary/write.rs:407:9`
- last API call: `allsorts::tables::F2Dot14::write[WriteCounter]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15647671126722936457

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::binary::write::WriteCounter;
use allsorts::tables::F2Dot14;

fn main() {
    let mut write_counter = WriteCounter::new();
    let _ = <allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext>::write_zeros(&mut write_counter, usize::MAX);
    let _ = <allsorts::tables::F2Dot14 as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteCounter>(&mut write_counter, F2Dot14::from(0i16));
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_binary_write_rs_407_9_c91352`).

## Output

```
thread 'main' (82157) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:407:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext>::write_bytes
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:407:9
   4: <allsorts::binary::I16Be as allsorts::binary::write::WriteBinary<i16>>::write::<allsorts::binary::write::WriteCounter>
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:241:14
   5: <allsorts::tables::F2Dot14 as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteCounter>
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables.rs:1100:9
   6: repro_allsorts_src_binary_write_rs_407_9_c91352::main
             at ./src/main.rs:7:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
