# data/src/pixel.rs:674:29

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/pixel.rs:674:29`
- last API call: `av_data::frame::Frame::new_default_frame`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
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
use av_data::pixel::TrichromaticEncodingSystem;

fn main() {
    let trichromatic_encoding_system = TrichromaticEncodingSystem::RGB;
    let media_kind = MediaKind::Video(VideoInfo { width: 0usize, height: 0usize, flipped: false, frame_type: FrameType::B, format: std::sync::Arc::new(Formaton::new(ColorModel::Trichromatic(trichromatic_encoding_system), &[Chromaton { h_ss: 0u8, v_ss: 100u8, packed: false, depth: 0u8, shift: 0u8, comp_offs: 0u8, next_elem: 0u8 }], 0u8, false, false, false)), bits: 0u8 });
    let media_kind2 = MediaKind::from(media_kind.get_video_info().unwrap());
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Video(media_kind2.get_video_info().unwrap()), None);
}
```

Run with `cargo run` in this directory (package `repro_data_src_pixel_rs_674_29_c91ed1`).

## Output

```
thread 'main' (2311452) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/pixel.rs:674:29:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <av_data::pixel::Chromaton>::get_data_size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/pixel.rs:674:29
   4: <av_data::frame::VideoInfo>::size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:143:27
   5: <av_data::frame::DefaultFrameBuffer>::new
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:412:34
   6: <av_data::frame::Frame>::new_default_frame::<av_data::frame::MediaKind>
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:479:19
   7: repro_data_src_pixel_rs_674_29_c91ed1::main
             at ./src/main.rs:14:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
