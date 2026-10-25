# data/src/audiosample.rs:273:17

- crate: https://github.com/rust-av/rust-av @ d44bb083ffc543d80397c8244f2e8c534f609308
- kind: panic
- location: `data/src/audiosample.rs:273:17`
- last API call: `av_data::audiosample::ChannelMap::get_channel`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
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

fn main() {
    let channel_map = ChannelMap::new();
    let _ = channel_map.get_channel(0usize);
}
```

Run with `cargo run` in this directory (package `repro_data_src_audiosample_rs_273_17_f306c7`).

## Output

```
thread 'main' (2311072) panicked at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:273:17:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[av_data::audiosample::ChannelType]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[av_data::audiosample::ChannelType] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<av_data::audiosample::ChannelType> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <av_data::audiosample::ChannelMap>::get_channel
             at /home/rafal/.cargo/git/checkouts/rust-av-a70395d595c3033d/d44bb08/data/src/audiosample.rs:273:17
   7: repro_data_src_audiosample_rs_273_17_f306c7::main
             at ./src/main.rs:5:25
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
