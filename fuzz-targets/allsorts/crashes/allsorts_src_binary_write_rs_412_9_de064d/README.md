# allsorts/src/binary/write.rs:412:9

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/binary/write.rs:412:9`
- last API call: `allsorts::binary::write::WriteCounter::write_zeros`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8075668940676216115

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
use allsorts::tables::cmap::owned::CmapSubtable;
use allsorts::tables::cmap::owned::CmapSubtableFormat12;

fn main() {
    let mut write_counter = WriteCounter::new();
    let _ = <allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteCounter>(&mut write_counter, CmapSubtable::Format12(CmapSubtableFormat12 { language: 0u32, groups: vec![] }));
    let _ = <allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext>::write_zeros(&mut write_counter, usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_binary_write_rs_412_9_de064d`).

## Output

```
thread 'main' (81859) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:412:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext>::write_zeros
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/write.rs:412:9
   4: repro_allsorts_src_binary_write_rs_412_9_de064d::main
             at ./src/main.rs:8:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
