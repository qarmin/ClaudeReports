# out of memory in krilla::embed::EmbedError::Existence

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: out of memory
- last API call: `krilla::embed::EmbedError::Existence`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14117882295492016762

## Message

```
process ended abnormally (out of memory) while calling krilla::embed::EmbedError::Existence
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::Data;
use krilla::document::Document;
use krilla::embed::EmbedError;
use krilla::image::Image;

fn main() {
    let document = Document::default();
    let _ = document.finish();
    let _ = Image::from_webp(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), false);
    let _ = EmbedError::Existence;
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_krilla_embed_embederror_existence_0a69a4`).

## Output

```
memory allocation of 12058784000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: krilla::graphics::image::decode_webp
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/image/mod.rs:797:27
  10: <krilla::graphics::image::Image>::from_webp::{closure#0}
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/image/mod.rs:313:42
  11: <krilla::util::deferred::Deferred<core::result::Result<krilla::graphics::image::Repr, alloc::string::String>>>::new::<<krilla::graphics::image::Image>::from_webp::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/util.rs:188:18
  12: <krilla::graphics::image::Image>::from_webp
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/graphics/image/mod.rs:313:20
  13: repro_out_of_memory_in_krilla_embed_embederror_existence_0a69a4::main
             at ./src/main.rs:9:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
