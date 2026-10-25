# chromaprint/src/chroma.rs:25:20

- crate: https://github.com/darksv/rusty-chromaprint @ b1524bf2cf25c892931c0df59aa3226ba4ad651d
- kind: panic
- location: `chromaprint/src/chroma.rs:25:20`
- last API call: `rusty_chromaprint::Fingerprinter::new`
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
rusty-chromaprint = { git = "https://github.com/darksv/rusty-chromaprint", rev = "b1524bf2cf25c892931c0df59aa3226ba4ad651d" }
```

`src/main.rs`:

```rust
use rusty_chromaprint::Configuration;
use rusty_chromaprint::Fingerprinter;

fn main() {
    let _ = Fingerprinter::new(&Configuration::preset_test3().with_frame_size(10000000000000000000usize));
}
```

Run with `cargo run` in this directory (package `repro_chromaprint_src_chroma_rs_25_20_1977cb`).

## Output

```
thread 'main' (1331265) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   8: <rusty_chromaprint::chroma::Chroma<rusty_chromaprint::chroma_filter::ChromaFilter<rusty_chromaprint::chroma_normalizer::ChromaNormalizer<rusty_chromaprint::fingerprint_calculator::FingerprintCalculator>>>>::new
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/chroma.rs:25:20
   9: <rusty_chromaprint::fingerprinter::Fingerprinter>::new
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprinter.rs:185:22
  10: repro_chromaprint_src_chroma_rs_25_20_1977cb::main
             at ./src/main.rs:5:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
