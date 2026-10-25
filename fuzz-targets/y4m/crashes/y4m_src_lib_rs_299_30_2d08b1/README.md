# y4m/src/lib.rs:299:30

- crate: https://github.com/image-rs/y4m @ 76635a27866db6bad1ec5f9b08711dfa908f2e6d
- kind: panic
- location: `y4m/src/lib.rs:299:30`
- last API call: `y4m::EncoderBuilder::write_header`
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
y4m = { git = "https://github.com/image-rs/y4m", rev = "76635a27866db6bad1ec5f9b08711dfa908f2e6d" }
```

`src/main.rs`:

```rust
use y4m::Colorspace;
use y4m::EncoderBuilder;
use y4m::Ratio;
use y4m::VendorExtensionString;

fn main() {
    let encoder_builder = EncoderBuilder::new(0usize, usize::MAX, Ratio { num: 0usize, den: 1usize });
    let encoder_builder2 = encoder_builder.with_colorspace(Colorspace::C422p10);
    let encoder_builder3 = encoder_builder2.append_vendor_extension(VendorExtensionString::new(vec![]).unwrap());
    let _ = encoder_builder3.write_header::<Vec<u8>>(vec![]);
}
```

Run with `cargo run` in this directory (package `repro_y4m_src_lib_rs_299_30_2d08b1`).

## Output

```
thread 'main' (522180) panicked at /home/rafal/.cargo/git/checkouts/y4m-1316f13b6ec08d4a/76635a2/src/lib.rs:299:30:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: y4m::get_plane_sizes
             at /home/rafal/.cargo/git/checkouts/y4m-1316f13b6ec08d4a/76635a2/src/lib.rs:299:30
   4: <y4m::EncoderBuilder>::write_header::<alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/git/checkouts/y4m-1316f13b6ec08d4a/76635a2/src/lib.rs:627:37
   5: repro_y4m_src_lib_rs_299_30_2d08b1::main
             at ./src/main.rs:10:30
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
