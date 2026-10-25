# allsorts/src/font_specimen.rs:185:50

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/font_specimen.rs:185:50`
- last API call: `allsorts::font_specimen::specimen`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17135140159275885014

## Message

```
range start index 99 out of range for slice of length 80
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
allsorts = { git = "https://github.com/yeslogic/allsorts", rev = "efab0fc769287f4ced68bbd723c328c9d9b89698", features = ["flate2", "flate2_rust", "flate2_zlib", "flate2_zlib-rs", "outline", "prince", "specimen"] }
```

`src/main.rs`:

```rust
use allsorts::font_specimen::SpecimenOptions;

fn main() {
    let _ = allsorts::font_specimen::specimen("", &include_bytes!("data_0.bin").to_vec(), SpecimenOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_font_specimen_rs_185_50_6fc4d2`).

## Output

```
thread 'main' (80475) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/font_specimen.rs:185:50:
range start index 99 out of range for slice of length 80
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::RangeFrom<usize> as core::slice::index::SliceIndex<[u8]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:545:13
   4: <[u8] as core::ops::index::Index<core::ops::range::RangeFrom<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: allsorts::font_specimen::specimen
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/font_specimen.rs:185:50
   6: repro_allsorts_src_font_specimen_rs_185_50_6fc4d2::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
