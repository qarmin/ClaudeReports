# chromaprint/src/audio_processor.rs:180:9

- crate: https://github.com/darksv/rusty-chromaprint @ b1524bf2cf25c892931c0df59aa3226ba4ad651d
- kind: panic
- location: `chromaprint/src/audio_processor.rs:180:9`
- last API call: `rusty_chromaprint::Fingerprinter::consume`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 1
 right: 0
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
use rusty_chromaprint::Fingerprinter;

fn main() {
    let mut fingerprinter = Fingerprinter::new(&Configuration::default());
    let _ = fingerprinter.start(1000000000u32, 10000u32);
    fingerprinter.consume(&[0i16]);
}
```

Run with `cargo run` in this directory (package `repro_chromaprint_src_audio_processor_rs_180_9_42a924`).

## Output

```
thread 'main' (1331180) panicked at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/audio_processor.rs:180:9:
assertion `left == right` failed
  left: 1
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <rusty_chromaprint::audio_processor::AudioProcessor<alloc::boxed::Box<dyn rusty_chromaprint::stages::AudioConsumer<f64, Output = [u32]>>> as rusty_chromaprint::stages::AudioConsumer>::consume
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/audio_processor.rs:180:9
   5: <rusty_chromaprint::fingerprinter::Fingerprinter>::consume
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprinter.rs:208:24
   6: repro_chromaprint_src_audio_processor_rs_180_9_42a924::main
             at ./src/main.rs:7:19
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
