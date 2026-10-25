# allsorts/src/tables/cmap.rs:1038:44

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/tables/cmap.rs:1038:44`
- last API call: `allsorts::tables::cmap::owned::CmapSubtable::map_glyph`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7840570007542970247

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
use allsorts::tables::cmap::owned::CmapSubtable;
use allsorts::tables::cmap::owned::CmapSubtableFormat12;
use allsorts::tag::LOCA;

fn main() {
    let u32_v = LOCA;
    let cmap_subtable = CmapSubtable::Format12(CmapSubtableFormat12 { language: 0u32, groups: vec![<allsorts::tables::cmap::SequentialMapGroup as allsorts::binary::read::ReadFrom>::read_from((0u32, u32_v, u32::MAX))] });
    let _ = cmap_subtable.map_glyph(u32_v);
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_tables_cmap_rs_1038_44_4d9697`).

## Output

```
thread 'main' (80801) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/cmap.rs:1038:44:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <allsorts::tables::cmap::owned::CmapSubtable>::map_glyph
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/tables/cmap.rs:1038:44
   4: repro_allsorts_src_tables_cmap_rs_1038_44_4d9697::main
             at ./src/main.rs:8:27
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
