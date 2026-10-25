# data/src/audiosample.rs:56:13

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/audiosample.rs:56:13`
- last API call: `av_data::frame::AudioInfo::size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
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
use av_data::audiosample::formats::S16;
use av_data::frame::AudioInfo;

fn main() {
    let audio_info = AudioInfo { samples: usize::MAX, sample_rate: 7usize, map: ChannelMap::default(), format: std::sync::Arc::new(S16), block_len: None };
    let _ = audio_info.size(65536usize);
}
```

Run with `cargo run` in this directory (package `repro_data_src_audiosample_rs_56_13_e870c5`).

## Output

```
thread 'main' (2311593) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:56:13:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <av_data::audiosample::Soniton>::get_audio_size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:56:13
   4: <av_data::frame::AudioInfo>::size
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/frame.rs:220:21
   5: repro_data_src_audiosample_rs_56_13_e870c5::main
             at ./src/main.rs:7:24
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
