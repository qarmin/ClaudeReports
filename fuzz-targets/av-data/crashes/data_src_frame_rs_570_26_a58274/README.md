# data/src/frame.rs:570:26

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/frame.rs:570:26`
- last API call: `av_data::frame::Frame::copy_plane_to_buffer`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
chunk size must be non-zero
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
av-data = { git = "https://github.com/rust-av/rust-av", rev = "d44bb083ffc543d80397c8244f2e8c534f609308" }
```

`src/main.rs`:

```rust
use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::ChromaLocation;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::ColorPrimaries;
use av_data::pixel::Formaton;
use av_data::pixel::MatrixCoefficients;
use av_data::pixel::TransferCharacteristic;

fn main() {
    let video_info = VideoInfo { width: 0usize, height: 0usize, flipped: false, frame_type: FrameType::OTHER, format: std::sync::Arc::new(Formaton { model: ColorModel::LAB, primaries: ColorPrimaries::BT2020, xfer: TransferCharacteristic::BT1361E, matrix: MatrixCoefficients::BT709, chroma_location: ChromaLocation::Center, components: 0u8, comp_info: [None, None, None, None, Some(Chromaton::pal8(1u8))], elem_size: 0u8, be: false, alpha: false, palette: false }), bits: 0u8 };
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Video(video_info.clone()), None), 0usize, &mut vec![], 1usize);
}
```

Run with `cargo run` in this directory (package `repro_data_src_frame_rs_570_26_a58274`).

## Output

```
thread 'main' (2312310) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:570:26:
chunk size must be non-zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <[u8]>::chunks
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:1155:9
   3: av_data::frame::copy_plane
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:570:26
   4: <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:497:13
   5: repro_data_src_frame_rs_570_26_a58274::main
             at ./src/main.rs:15:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
