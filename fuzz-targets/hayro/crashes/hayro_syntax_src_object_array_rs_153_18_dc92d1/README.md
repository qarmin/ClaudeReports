# hayro-syntax/src/object/array.rs:153:18

- crate: https://github.com/LaurenzV/hayro @ e0cd0f7c56de05bdf87b8252a6abbe31d870aa65
- kind: panic
- location: `hayro-syntax/src/object/array.rs:153:18`
- last API call: `hayro_syntax::Pdf::new_with_password`
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
hayro = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
hayro-syntax = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
```

`src/main.rs`:

```rust
use hayro_syntax::Pdf;
use hayro_syntax::PdfData;

fn main() {
    let _ = Pdf::new_with_password(PdfData::from(include_bytes!("data_0.bin").to_vec()), "");
}
```

Run with `cargo run` in this directory (package `repro_hayro_syntax_src_object_array_rs_153_18_dc92d1`).

## Output

```
thread 'main' (235944) panicked at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/object/array.rs:153:18:
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
   4: <core::option::Option<hayro_syntax::object::ref::MaybeRef<hayro_syntax::object::Object>>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <hayro_syntax::object::array::ArrayIter as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/object/array.rs:153:18
   6: hayro_syntax::page::resolve_pages
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/page.rs:133:36
   7: <hayro_syntax::page::Pages>::new
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/page.rs:56:9
   8: <hayro_syntax::page::cached::CachedPages>::new::{closure#0}
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/page.rs:532:31
   9: <core::option::Option<hayro_syntax::object::dict::Dict>>::and_then::<hayro_syntax::page::Pages, <hayro_syntax::page::cached::CachedPages>::new::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1542:24
  10: <hayro_syntax::page::cached::CachedPages>::new
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/page.rs:532:18
  11: <hayro_syntax::pdf::Pdf>::new_with_password::<hayro_syntax::data::PdfData>
             at /home/rafal/.cargo/git/checkouts/hayro-296364bfe862bcd9/e0cd0f7/hayro-syntax/src/pdf.rs:61:21
  12: repro_hayro_syntax_src_object_array_rs_153_18_dc92d1::main
             at ./src/main.rs:5:13
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
