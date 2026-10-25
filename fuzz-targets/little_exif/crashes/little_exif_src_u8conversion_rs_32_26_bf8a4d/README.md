# little_exif/src/u8conversion.rs:32:26

- crate: https://github.com/TechnikTobi/little_exif @ 2c86b66d2feeb1498180ec33cca5aab60239dfbd
- kind: panic
- location: `little_exif/src/u8conversion.rs:32:26`
- last API call: `little_exif::exif_tag::ExifTag::from_u16_with_data`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
from_u8_vec: Mangled EXIF data encountered!
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
little_exif = { git = "https://github.com/TechnikTobi/little_exif", rev = "2c86b66d2feeb1498180ec33cca5aab60239dfbd" }
```

`src/main.rs`:

```rust
use little_exif::endian::Endian;
use little_exif::exif_tag::ExifTag;
use little_exif::exif_tag_format::ExifTagFormat;
use little_exif::ifd::ExifTagGroup;

fn main() {
    let _ = ExifTag::from_u16_with_data(0u16, &ExifTagFormat::FLOAT, &vec![0u8], &Endian::Little, &ExifTagGroup::GENERIC);
}
```

Run with `cargo run` in this directory (package `repro_little_exif_src_u8conversion_rs_32_26_bf8a4d`).

## Output

```
thread 'main' (582357) panicked at /home/rafal/.cargo/git/checkouts/little_exif-2786a8835c2e0b1c/2c86b66/src/u8conversion.rs:32:26:
from_u8_vec: Mangled EXIF data encountered!
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <alloc::vec::Vec<f32> as little_exif::u8conversion::U8conversion<alloc::vec::Vec<f32>>>::from_u8_vec
             at /home/rafal/.cargo/git/checkouts/little_exif-2786a8835c2e0b1c/2c86b66/src/u8conversion.rs:32:26
   3: <little_exif::exif_tag::ExifTag>::from_u16_with_data
             at /home/rafal/.cargo/git/checkouts/little_exif-2786a8835c2e0b1c/2c86b66/src/exif_tag/mod.rs:201:69
   4: repro_little_exif_src_u8conversion_rs_32_26_bf8a4d::main
             at ./src/main.rs:7:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
