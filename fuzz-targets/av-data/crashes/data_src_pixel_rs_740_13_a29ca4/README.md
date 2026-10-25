# data/src/pixel.rs:740:13

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/pixel.rs:740:13`
- last API call: `av_data::pixel::Formaton::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
too many components
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
av-data = { git = "https://github.com/rust-av/rust-av", rev = "d44bb083ffc543d80397c8244f2e8c534f609308" }
```

`src/main.rs`:

```rust
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;
use av_data::pixel::TrichromaticEncodingSystem;

fn main() {
    let _ = Formaton::new(ColorModel::Trichromatic(TrichromaticEncodingSystem::RGB), &[Chromaton::new(0u8, 0u8, false, 0u8, 0u8, 0u8, 0u8), Chromaton::packrgb(0u8, 0u8, 0u8, 0u8), Chromaton::packrgb(0u8, 0u8, 0u8, 0u8), Chromaton { h_ss: 0u8, v_ss: 0u8, packed: false, depth: 0u8, shift: 0u8, comp_offs: 0u8, next_elem: 0u8 }, Chromaton::yuvhb(0u8, 0u8, 0u8, 0u8), Chromaton::yuv8(0u8, 0u8, 0u8)], 0u8, false, false, false);
}
```

Run with `cargo run` in this directory (package `repro_data_src_pixel_rs_740_13_a29ca4`).

## Output

```
thread 'main' (2311030) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/pixel.rs:740:13:
too many components
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <av_data::pixel::Formaton>::new
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/pixel.rs:740:13
   3: repro_data_src_pixel_rs_740_13_a29ca4::main
             at ./src/main.rs:7:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
