# out of memory in av_data::frame::Frame::new_default_frame

- crate: av-data 0.4.4
- kind: out of memory
- last API call: `av_data::frame::Frame::new_default_frame`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling av_data::frame::Frame::new_default_frame
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
    let mut channel_map = ChannelMap::default();
    channel_map.add_channels(&[ChannelType::Cs]);
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Audio(AudioInfo::new(4294967296usize, 0usize, channel_map.clone(), std::sync::Arc::new(S16), None)), None);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_av_data_frame_frame_new_default_frame_8474e5`).

## Output

```
memory allocation of 8589934592 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: <bytes::bytes_mut::BytesMut>::zeroed
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytes-1.12.1/src/bytes_mut.rs:296:28
  10: <av_data::frame::DefaultFrameBuffer>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/av-data-0.4.4/src/frame.rs:432:27
  11: <av_data::frame::Frame>::new_default_frame::<av_data::frame::MediaKind>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/av-data-0.4.4/src/frame.rs:479:19
  12: repro_out_of_memory_in_av_data_frame_frame_new_default_frame_8474e5::main
             at ./src/main.rs:11:13
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
