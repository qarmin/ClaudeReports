# bytes/src/bytes_mut.rs:296:28

- crate: av-data 0.4.4
- kind: panic
- location: `bytes/src/bytes_mut.rs:296:28`
- source: https://docs.rs/crate/bytes/1.12.1/source/src/bytes_mut.rs#296
- last API call: `av_data::frame::Frame::new_default_frame`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
av-data = { version = "=0.4.4" }
```

`src/main.rs`:

```rust
use av_data::audiosample::ChannelMap;
use av_data::audiosample::ChannelType;
use av_data::audiosample::formats::S16;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    let mut channel_map = ChannelMap::new();
    channel_map.add_channel(ChannelType::Cs);
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(AudioInfo { samples: 8341552203014512160usize, sample_rate: 0usize, map: channel_map.clone(), format: std::sync::Arc::new(S16), block_len: None }), None);
}
```

Run with `cargo run` in this directory (package `repro_bytes_src_bytes_mut_rs_296_28_4be6bb`).

## Output

```
thread 'main' (339772) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   5: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   6: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   7: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   8: <bytes::bytes_mut::BytesMut>::zeroed
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.12.1/src/bytes_mut.rs:296:28
   9: <av_data::frame::DefaultFrameBuffer>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/av-data-0.4.4/src/frame.rs:432:27
  10: <av_data::frame::Frame>::new_default_frame::<av_data::frame::MediaKind>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/av-data-0.4.4/src/frame.rs:479:19
  11: repro_bytes_src_bytes_mut_rs_296_28_4be6bb::main
             at ./src/main.rs:11:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
