# symphonia-format-riff/src/wave/chunks.rs:100:36

- crate: https://github.com/pdeljanov/Symphonia @ ee35874b571a35a9a6e15d3bc9a3aaf8f11fbeee
- kind: panic
- location: `symphonia-format-riff/src/wave/chunks.rs:100:36`
- last API call: `symphonia_format_riff::WavReader::try_new`
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
symphonia-core = { git = "https://github.com/pdeljanov/Symphonia", rev = "ee35874b571a35a9a6e15d3bc9a3aaf8f11fbeee" }
symphonia-format-riff = { git = "https://github.com/pdeljanov/Symphonia", rev = "ee35874b571a35a9a6e15d3bc9a3aaf8f11fbeee" }
```

`src/main.rs`:

```rust
use symphonia_format_riff::WavReader;

fn main() {
    let _ = WavReader::try_new({ let bytes = include_bytes!("data_0.bin").to_vec(); symphonia_core::io::MediaSourceStream::new(Box::new(std::io::Cursor::new(bytes)), Default::default()) }, { symphonia_core::formats::FormatOptions::default() });
}
```

Run with `cargo run` in this directory (package `repro_symphonia_format_riff_src_wave_chunks_rs_100_36_14ca92`).

## Output

```
thread 'main' (3045573) panicked at /home/rafal/.cargo/git/checkouts/symphonia-6dce4e040d3ff5a1/ee35874/symphonia-format-riff/src/wave/chunks.rs:100:36:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <symphonia_format_riff::wave::chunks::WaveFormatChunk>::read_pcm_fmt::<symphonia_core::io::media_source_stream::MediaSourceStream>
             at /home/rafal/.cargo/git/checkouts/symphonia-6dce4e040d3ff5a1/ee35874/symphonia-format-riff/src/wave/chunks.rs:100:36
   4: <symphonia_format_riff::wave::chunks::WaveFormatChunk as symphonia_format_riff::common::ParseChunk>::parse::<symphonia_core::io::media_source_stream::MediaSourceStream>
             at /home/rafal/.cargo/git/checkouts/symphonia-6dce4e040d3ff5a1/ee35874/symphonia-format-riff/src/wave/chunks.rs:439:17
   5: <symphonia_format_riff::common::ChunkParser<symphonia_format_riff::wave::chunks::WaveFormatChunk>>::parse::<symphonia_core::io::media_source_stream::MediaSourceStream>
             at /home/rafal/.cargo/git/checkouts/symphonia-6dce4e040d3ff5a1/ee35874/symphonia-format-riff/src/common.rs:184:9
   6: <symphonia_format_riff::wave::WavReader>::try_new
             at /home/rafal/.cargo/git/checkouts/symphonia-6dce4e040d3ff5a1/ee35874/symphonia-format-riff/src/wave/mod.rs:113:38
   7: repro_symphonia_format_riff_src_wave_chunks_rs_100_36_14ca92::main
             at ./src/main.rs:4:13
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
