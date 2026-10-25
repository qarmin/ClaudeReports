# image-tiff/src/decoder/mod.rs:1397:23

- crate: https://github.com/image-rs/image-tiff @ 3302936b08e2260d4c66f40f51170efbb1fded4c
- kind: panic
- location: `image-tiff/src/decoder/mod.rs:1397:23`
- last API call: `tiff::decoder::Decoder<::std::io::Cursor<Vec<u8>>>::read_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7630902026757110945

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiff = { git = "https://github.com/image-rs/image-tiff", rev = "3302936b08e2260d4c66f40f51170efbb1fded4c" }
```

`src/main.rs`:

```rust
use tiff::decoder::TiffHeader;
use tiff::tags::IfdPointer;
use tiff::tags::TiffVariant;
use tiff::tags::Type;
use tiff::tags::ValueBuffer;

fn main() {
    let tiff_header = TiffHeader { byte_order: ValueBuffer::empty(Type::BYTE).byte_order(), variant: TiffVariant::BigTiff, first_ifd: IfdPointer(0u64) };
    let mut decoder = tiff_header.open::<::std::io::Cursor<Vec<u8>>>(std::io::Cursor::new(vec![]));
    let _ = decoder.read_string(10000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_image_tiff_src_decoder_mod_rs_1397_23_2cb5da`).

## Output

```
thread 'main' (203324) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   5: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   6: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   7: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   8: <tiff::decoder::Decoder<core::io::cursor::Cursor<alloc::vec::Vec<u8>>>>::read_string
             at /home/rafal/.cargo/git/checkouts/image-tiff-2c13a9b5ecc4a5fb/3302936/src/decoder/mod.rs:1397:23
   9: repro_image_tiff_src_decoder_mod_rs_1397_23_2cb5da::main
             at ./src/main.rs:10:21
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
