# out of memory in tiff::decoder::Decoder<::std::io::Cursor<Vec<u8>>>::read_string

- crate: https://github.com/image-rs/image-tiff @ 3302936b08e2260d4c66f40f51170efbb1fded4c
- kind: out of memory
- last API call: `tiff::decoder::Decoder<::std::io::Cursor<Vec<u8>>>::read_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 1267744405182176062

## Message

```
process ended abnormally (out of memory) while calling tiff::decoder::Decoder<::std::io::Cursor<Vec<u8>>>::read_string
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiff = { git = "https://github.com/image-rs/image-tiff", rev = "3302936b08e2260d4c66f40f51170efbb1fded4c" }
```

`src/main.rs`:

```rust
use tiff::decoder::Limits;
use tiff::decoder::TiffHeader;
use tiff::tags::ByteOrder;
use tiff::tags::IfdPointer;
use tiff::tags::TiffVariant;

fn main() {
    let tiff_header = TiffHeader { byte_order: ByteOrder::BigEndian, variant: TiffVariant::BigTiff, first_ifd: IfdPointer(0u64) };
    let decoder = tiff_header.open::<::std::io::Cursor<Vec<u8>>>(std::io::Cursor::new(vec![]));
    let decoder2 = decoder.with_limits(Limits::default());
    let mut decoder3 = decoder2.with_limits(Limits::unlimited());
    let _ = decoder3.read_string(100000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_tiff_decoder_decoder_std_io_cursor_vec_u8_r_709f04`).

## Output

```
memory allocation of 100000000000000 bytes failed
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
   9: <tiff::decoder::Decoder<core::io::cursor::Cursor<alloc::vec::Vec<u8>>>>::read_string
             at /home/rafal/.cargo/git/checkouts/image-tiff-2c13a9b5ecc4a5fb/3302936/src/decoder/mod.rs:1397:23
  10: repro_out_of_memory_in_tiff_decoder_decoder_std_io_cursor_vec_u8_r_709f04::main
             at ./src/main.rs:12:22
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
