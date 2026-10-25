# data/src/audiosample.rs:28:6

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/audiosample.rs:28:6`
- last API call: `av_data::frame::AudioInfo::size`
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
use av_data::audiosample::ChannelMap;
use av_data::audiosample::formats::S32;
use av_data::frame::AudioInfo;

fn main() {
    let audio_info = AudioInfo::new(1usize, 0usize, ChannelMap::default(), std::sync::Arc::new(S32), Some(usize::MAX));
    let usize_v = audio_info.get_block_len().unwrap();
    let _ = audio_info.size(usize_v);
}
```

Run with `cargo run` in this directory (package `repro_data_src_audiosample_rs_28_6_be96a8`).

## Output

```
thread 'main' (2311734) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:28:6:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: av_data::audiosample::align
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:28:6
   4: <av_data::audiosample::Soniton>::get_audio_size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:59:9
   5: <av_data::frame::AudioInfo>::size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:220:21
   6: repro_data_src_audiosample_rs_28_6_be96a8::main
             at ./src/main.rs:8:24
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
