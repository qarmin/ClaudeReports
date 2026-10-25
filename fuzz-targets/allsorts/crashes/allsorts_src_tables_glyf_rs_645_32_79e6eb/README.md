# allsorts/src/tables/glyf.rs:645:32

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/tables/glyf.rs:645:32`
- last API call: `allsorts::tables::glyf::Glyph::read`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3418943528902209027

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

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let read_scope = ReadScope::new(&vec);
    let mut read_ctxt = read_scope.ctxt();
    let _ = <allsorts::woff::WoffHeader as allsorts::binary::read::ReadBinary>::read(&mut read_ctxt);
    let _ = <allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary>::read(&mut read_ctxt);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_tables_glyf_rs_645_32_79e6eb`).

## Output

```
thread 'main' (100405) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/glyf.rs:645:32:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::tables::glyf::SimpleGlyph as allsorts::binary::read::ReadBinaryDep>::read_dep
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/glyf.rs:645:32
   4: <allsorts::binary::read::ReadCtxt>::read_dep::<allsorts::tables::glyf::SimpleGlyph>
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/binary/read.rs:448:9
   5: <allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary>::read
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/glyf.rs:540:30
   6: repro_allsorts_src_tables_glyf_rs_645_32_79e6eb::main
             at ./src/main.rs:8:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
