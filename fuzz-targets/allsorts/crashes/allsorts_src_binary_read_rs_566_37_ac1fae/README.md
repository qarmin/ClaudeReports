# allsorts/src/binary/read.rs:566:37

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/binary/read.rs:566:37`
- last API call: `allsorts::binary::read::ReadCtxt::read_array[I16Be]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4847659567121302620

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
use allsorts::binary::read::ReadScope;
use allsorts::binary::read::ReadScopeOwned;

fn main() {
    let vec = vec![];
    let read_scope_owned = ReadScopeOwned::new(ReadScope::new(&vec));
    let read_scope = read_scope_owned.scope();
    let mut read_ctxt = read_scope.ctxt();
    let _ = read_ctxt.read_array::<allsorts::binary::I16Be>(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_binary_read_rs_566_37_ac1fae`).

## Output

```
thread 'main' (45341) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/read.rs:566:37:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::binary::read::ReadCtxt>::read_array::<allsorts::binary::I16Be>
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/read.rs:566:37
   4: repro_allsorts_src_binary_read_rs_566_37_ac1fae::main
             at ./src/main.rs:9:23
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
