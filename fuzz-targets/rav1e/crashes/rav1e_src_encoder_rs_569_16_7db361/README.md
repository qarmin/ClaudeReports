# rav1e/src/encoder.rs:569:16

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/encoder.rs:569:16`
- last API call: `rav1e::bench::encoder::SegmentationState::update_threshold`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range end index 10 out of range for slice of length 8
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
use rav1e::bench::encoder::SegmentationState;
use rav1e::bench::rdo::DistortionScale;

fn main() {
    let mut segmentation_state = SegmentationState { enabled: false, update_data: false, update_map: false, preskip: false, last_active_segid: 0u8, features: [[false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, false]], data: [[0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16], [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16]], threshold: [rav1e::bench::rdo::distortion_scale_for(0.0f64, 0.0f64), rav1e::bench::rdo::distortion_scale_for(0.0f64, 1.0f64), DistortionScale::from(0.0f64), DistortionScale::default(), DistortionScale(0u32), rav1e::bench::rdo::distortion_scale_for(0.0f64, 1.0f64), DistortionScale::from(0.0f64)], min_segment: 0u8, max_segment: 10u8 };
    segmentation_state.update_threshold(1u8, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_encoder_rs_569_16_7db361`).

## Output

```
thread 'main' (309407) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/encoder.rs:569:16:
range end index 10 out of range for slice of length 8
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeInclusive<usize> as core::slice::index::SliceIndex<[[i16; 8]]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:689:9
   4: <core::ops::range::RangeToInclusive<usize> as core::slice::index::SliceIndex<[[i16; 8]]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:780:24
   5: <[[i16; 8]] as core::ops::index::Index<core::ops::range::RangeToInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   6: <[[i16; 8]; 8] as core::ops::index::Index<core::ops::range::RangeToInclusive<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:391:9
   7: <rav1e::encoder::SegmentationState>::update_threshold
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/encoder.rs:569:16
   8: repro_rav1e_src_encoder_rs_569_16_7db361::main
             at ./src/main.rs:6:24
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
