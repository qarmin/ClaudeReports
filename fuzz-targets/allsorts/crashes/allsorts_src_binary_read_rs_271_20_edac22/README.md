# allsorts/src/binary/read.rs:271:20

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/binary/read.rs:271:20`
- last API call: `allsorts::binary::read::ReadScope::offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17389532439948326755

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
use allsorts::binary::read::ReadScope;
use allsorts::binary::read::ReadScopeOwned;

fn main() {
    let vec = vec![];
    let read_scope = ReadScope::new(&vec);
    let read_scope_owned = ReadScopeOwned::new(read_scope.offset(1usize));
    let read_scope2 = read_scope_owned.scope();
    let _ = read_scope2.offset(usize::MAX);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_binary_read_rs_271_20_edac22`).

## Output

```
thread 'main' (67769) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/read.rs:271:20:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::binary::read::ReadScope>::offset
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/read.rs:271:20
   4: repro_allsorts_src_binary_read_rs_271_20_edac22::main
             at ./src/main.rs:9:25
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
