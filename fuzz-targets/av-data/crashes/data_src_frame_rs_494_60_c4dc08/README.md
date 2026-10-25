# data/src/frame.rs:494:60

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/frame.rs:494:60`
- last API call: `av_data::frame::Frame::copy_plane_to_buffer`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: InvalidIndex
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
av-data = { git = "https://github.com/rust-av/rust-av", rev = "d44bb083ffc543d80397c8244f2e8c534f609308" }
```

`src/main.rs`:

```rust
use av_data::audiosample::ChannelMap;
use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let channel_map = ChannelMap::new();
    let usize_v = channel_map.len();
    let color_model = ColorModel::LAB;
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(VideoInfo::new(0usize, 0usize, false, FrameType::OTHER, std::sync::Arc::new(Formaton::new(color_model, &[], 0u8, false, false, false)))), None), 0usize, &mut vec![], usize_v);
}
```

Run with `cargo run` in this directory (package `repro_data_src_frame_rs_494_60_c4dc08`).

## Output

```
thread 'main' (2311316) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:494:60:
called `Result::unwrap()` on an `Err` value: InvalidIndex
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<&[u8], av_data::frame::FrameError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:494:60
   5: repro_data_src_frame_rs_494_60_c4dc08::main
             at ./src/main.rs:13:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
