# data/src/frame.rs:143:17

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/frame.rs:143:17`
- last API call: `av_data::frame::Frame::new_default_frame`
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
av-data = { git = "https://github.com/rust-av/rust-av", rev = "d44bb083ffc543d80397c8244f2e8c534f609308" }
```

`src/main.rs`:

```rust
use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let media_kind = MediaKind::from(VideoInfo { width: 1usize, height: 531707584965882534usize, flipped: false, frame_type: FrameType::P, format: std::sync::Arc::new(Formaton::new(ColorModel::CMYK, &[Chromaton::pal8(0u8), Chromaton::packrgb(10u8, 0u8, 0u8, 0u8)], 0u8, false, false, false)), bits: 0u8 });
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(media_kind.clone(), None);
}
```

Run with `cargo run` in this directory (package `repro_data_src_frame_rs_143_17_5e03a9`).

## Output

```
thread 'main' (2312253) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:143:17:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <av_data::frame::VideoInfo>::size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:143:17
   4: <av_data::frame::DefaultFrameBuffer>::new
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:412:34
   5: <av_data::frame::Frame>::new_default_frame::<av_data::frame::MediaKind>
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:479:19
   6: repro_data_src_frame_rs_143_17_5e03a9::main
             at ./src/main.rs:11:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
