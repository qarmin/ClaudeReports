# out of memory in rusty_chromaprint::Fingerprinter::new

- crate: https://github.com/darksv/rusty-chromaprint @ b1524bf2cf25c892931c0df59aa3226ba4ad651d
- kind: out of memory
- last API call: `rusty_chromaprint::Fingerprinter::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling rusty_chromaprint::Fingerprinter::new
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
    let configuration = Configuration::preset_test3();
    let _ = Fingerprinter::new(&configuration.clone().with_frame_size(1000000000usize));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_rusty_chromaprint_fingerprinter_new_c2ef4f`).

## Output

```
memory allocation of 8000000000 bytes failed
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
   6: <alloc::raw_vec::RawVec<f64>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <f64 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:24:31
   8: alloc::vec::from_elem::<f64>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: <rusty_chromaprint::chroma::Chroma<rusty_chromaprint::chroma_filter::ChromaFilter<rusty_chromaprint::chroma_normalizer::ChromaNormalizer<rusty_chromaprint::fingerprint_calculator::FingerprintCalculator>>>>::new
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/chroma.rs:26:25
  10: <rusty_chromaprint::fingerprinter::Fingerprinter>::new
             at /home/rafal/.cargo/git/checkouts/rusty-chromaprint-f136767f8c3e3e9b/b1524bf/chromaprint/src/fingerprinter.rs:185:22
  11: repro_out_of_memory_in_rusty_chromaprint_fingerprinter_new_c2ef4f::main
             at ./src/main.rs:6:13
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
