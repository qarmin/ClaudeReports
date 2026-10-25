# allsorts/src/woff2.rs:918:22

- crate: https://github.com/yeslogic/allsorts @ efab0fc769287f4ced68bbd723c328c9d9b89698
- kind: panic
- location: `allsorts/src/woff2.rs:918:22`
- last API call: `allsorts::font_specimen::specimen`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17610668232535092861

## Message

```
called `Option::unwrap()` on a `None` value
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
    let _ = allsorts::font_specimen::specimen("", &include_bytes!("data_0.bin").to_vec(), SpecimenOptions { index: 3u32, sample_text: None });
}
```

Run with `cargo run` in this directory (package `repro_allsorts_src_woff2_rs_918_22_b817ba`).

## Output

```
thread 'main' (81149) panicked at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/woff2.rs:918:22:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<u16>, core::option::Option<&allsorts::woff2::TableDirectoryEntry>, <allsorts::woff2::collection::FontEntry>::table_entries::{closure#0}>>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <allsorts::woff2::Woff2TableProvider>::table_directory
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/woff2.rs:918:22
   6: <allsorts::woff2::Woff2TableProvider>::new
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/woff2.rs:884:28
   7: <allsorts::woff2::Woff2Font>::table_provider
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/woff2.rs:199:9
   8: <allsorts::font_data::FontData>::table_provider
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/font_data.rs:85:37
   9: allsorts::font_specimen::specimen
             at /home/rafal/.cargo/git/checkouts/allsorts-3b0a752b668ae2bb/efab0fc/src/font_specimen.rs:177:25
  10: repro_allsorts_src_woff2_rs_918_22_b817ba::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
