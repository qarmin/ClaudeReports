# printpdf/src/serialize.rs:370:55

- crate: https://github.com/fschutt/printpdf @ 5cc7cfd6d37c06862543952fb93aaeb6d1f99bb6
- kind: panic
- location: `printpdf/src/serialize.rs:370:55`
- last API call: `printpdf::PdfDocument::save_writer`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { git = "https://github.com/fschutt/printpdf", rev = "5cc7cfd6d37c06862543952fb93aaeb6d1f99bb6" }
```

`src/main.rs`:

```rust
use printpdf::PdfDocument;
use printpdf::serialize::PdfSaveOptions;

fn main() {
    let mut pdf_document = PdfDocument::default();
    let _ = pdf_document.add_bookmark("", 0usize);
    pdf_document.save_writer::<Vec<u8>>(&mut vec![], &PdfSaveOptions { optimize: false, subset_fonts: false, secure: false, image_optimization: None }, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_printpdf_src_serialize_rs_370_55_e58ce5`).

## Output

```
thread 'main' (1279606) panicked at /home/rafal/.cargo/git/checkouts/printpdf-d5271e4ecfa3944f/5cc7cfd/src/serialize.rs:370:55:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<(u32, u16)>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: printpdf::serialize::to_lopdf_doc
             at /home/rafal/.cargo/git/checkouts/printpdf-d5271e4ecfa3944f/5cc7cfd/src/serialize.rs:370:55
   6: printpdf::serialize::serialize_pdf::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/printpdf-d5271e4ecfa3944f/5cc7cfd/src/serialize.rs:96:19
   7: <printpdf::PdfDocument>::save_writer::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/printpdf-d5271e4ecfa3944f/5cc7cfd/src/lib.rs:758:9
   8: repro_printpdf_src_serialize_rs_370_55_e58ce5::main
             at ./src/main.rs:7:18
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
