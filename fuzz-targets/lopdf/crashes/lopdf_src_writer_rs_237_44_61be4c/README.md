# lopdf/src/writer.rs:237:44

- crate: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1
- kind: panic
- location: `lopdf/src/writer.rs:237:44`
- last API call: `lopdf::IncrementalDocument::save_to`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lopdf = { git = "https://github.com/J-F-Liu/lopdf", rev = "bebffeff2d9e54b00325014a4fda07510eac2cb1" }
```

`src/main.rs`:

```rust
use lopdf::Document;
use lopdf::IncrementalDocument;

fn main() {
    let mut document = Document::new();
    document.renumber_objects_with(u32::MAX);
    let mut incremental_document = IncrementalDocument::create_from(vec![], document.clone());
    let _ = incremental_document.save_to::<Vec<u8>>(&mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_lopdf_src_writer_rs_237_44_61be4c`).

## Output

```
thread 'main' (1294424) panicked at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/writer.rs:237:44:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <lopdf::document::Document>::write_cross_reference_stream::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/writer.rs:237:44
   4: <lopdf::document::Document>::write_xref_and_trailer::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/writer.rs:106:22
   5: <lopdf::incremental_document::IncrementalDocument>::save_internal::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/writer.rs:391:14
   6: <lopdf::incremental_document::IncrementalDocument>::save_to::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/lopdf-49bfd002b5e4fbb4/bebffef/src/writer.rs:293:14
   7: repro_lopdf_src_writer_rs_237_44_61be4c::main
             at ./src/main.rs:8:34
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
