# out of memory in av_data::frame::Frame::copy_plane_to_buffer

- crate: av-data 0.4.4
- kind: out of memory
- last API call: `av_data::frame::Frame::copy_plane_to_buffer`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling av_data::frame::Frame::copy_plane_to_buffer
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
use av_data::audiosample::formats::S32;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Audio(AudioInfo::new(1000000000usize, 1usize, ChannelMap::default_map(1usize), std::sync::Arc::new(S32), None)), None), 0usize, &mut vec![], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_av_data_frame_frame_copy_plane_to_buffer_6068fb`).

## Output

```
memory allocation of 4000000000 bytes failed
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
  12: repro_out_of_memory_in_av_data_frame_frame_copy_plane_to_buffer_6068fb::main
             at ./src/main.rs:8:87
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
