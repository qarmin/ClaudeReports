# allsorts/src/tables/cmap.rs:398:9

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/tables/cmap.rs:398:9`
- last API call: `allsorts::tables::cmap::owned::CmapSubtable::write[WriteBuffer]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 6591414775470688517

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698" }
```

`src/main.rs`:

```rust
use allsorts::binary::write::WriteBuffer;
use allsorts::tables::cmap::owned::CmapSubtable;
use allsorts::tables::cmap::owned::CmapSubtableFormat4;

fn main() {
    let mut write_buffer = WriteBuffer::new();
    let _ = <allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteBuffer>(&mut write_buffer, CmapSubtable::Format4(CmapSubtableFormat4 { language: 0u16, end_codes: vec![], start_codes: vec![], id_deltas: vec![], id_range_offsets: vec![], glyph_id_array: vec![] }));
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_tables_cmap_rs_398_9_9e2577`).

## Output

```
thread 'main' (22516) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/cmap.rs:398:9:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::tables::cmap::Format4Calculator>::range_shift
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/cmap.rs:398:9
   4: <allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteBuffer>
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/cmap.rs:1138:45
   5: repro_allsorts_src_tables_cmap_rs_398_9_9e2577::main
             at ./src/main.rs:7:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
