# chromaprint/src/fingerprinter.rs:139:9

- crate: https://github.com/darksv/rusty-chromaprint @ b1524bf2cf25c892931c0df59aa3226ba4ad651d
- kind: panic
- location: `chromaprint/src/fingerprinter.rs:139:9`
- last API call: `rusty_chromaprint::Segment::start1`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rusty-chromaprint = { git = "https://github.com/darksv/rusty-chromaprint", rev = "b1524bf2cf25c892931c0df59aa3226ba4ad651d" }
```

`src/main.rs`:

```rust
use rusty_chromaprint::Configuration;
use rusty_chromaprint::Segment;

fn main() {
    let configuration = Configuration::preset_test3();
    let configuration2 = configuration.clone().with_frame_overlap(10000usize);
    let configuration3 = configuration2.clone().with_interpolation(false);
    let segment = Segment { offset1: 0usize, offset2: 0usize, items_count: 0usize, score: 0.0f64 };
    let _ = segment.start1(&configuration3);
}
```

Run with `cargo run` in this directory (package `repro_chromaprint_src_fingerprinter_rs_139_9_7139ec`).

## Output

```
thread 'main' (1331223) panicked at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprinter.rs:139:9:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rusty_chromaprint::fingerprinter::Configuration>::samples_in_item
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprinter.rs:139:9
   4: <rusty_chromaprint::fingerprinter::Configuration>::item_duration_in_seconds
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprinter.rs:149:14
   5: <rusty_chromaprint::fingerprint_matcher::Segment>::start1
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprint_matcher.rs:212:16
   6: repro_chromaprint_src_fingerprinter_rs_139_9_7139ec::main
             at ./src/main.rs:9:21
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
