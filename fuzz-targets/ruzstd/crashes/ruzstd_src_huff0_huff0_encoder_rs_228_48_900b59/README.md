# ruzstd/src/huff0/huff0_encoder.rs:228:48

- crate: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774
- kind: panic
- location: `ruzstd/src/huff0/huff0_encoder.rs:228:48`
- last API call: `ruzstd::huff0::huff0_encoder::HuffmanTable::build_from_weights`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ruzstd = { git = "https://github.com/KillingSpark/zstd-rs", rev = "91a3a6eb4cb8a0252ef73f5a561e30c790635774", features = ["dict_builder", "fuzz_exports", "hash", "rustc-dep-of-std", "std"] }
```

`src/main.rs`:

```rust
use ruzstd::huff0::huff0_encoder::HuffmanTable;

fn main() {
    let _ = HuffmanTable::build_from_weights(&[1000000000000000000usize]);
}
```

Run with `cargo run` in this directory (package `repro_ruzstd_src_huff0_huff0_encoder_rs_228_48_900b59`).

## Output

```
thread 'main' (414097) panicked at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:228:48:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:228:48
   4: core::iter::adapters::map::map_fold::<&<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry, usize, usize, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}, <usize as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry>, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}>>::{closure#0}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:28
   5: <core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry> as core::iter::traits::iterator::Iterator>::fold::<usize, core::iter::adapters::map::map_fold<&<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry, usize, usize, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}, <usize as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry>, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}>>::{closure#0}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   6: <core::iter::adapters::map::Map<core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry>, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}> as core::iter::traits::iterator::Iterator>::fold::<usize, <usize as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry>, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}>>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   7: <usize as core::iter::traits::accum::Sum>::sum::<core::iter::adapters::map::Map<core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry>, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:52:22
   8: <core::iter::adapters::map::Map<core::slice::iter::Iter<<ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::SortEntry>, <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights::{closure#1}> as core::iter::traits::iterator::Iterator>::sum::<usize>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3670:9
   9: <ruzstd::huff0::huff0_encoder::HuffmanTable>::build_from_weights
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/huff0/huff0_encoder.rs:228:69
  10: repro_ruzstd_src_huff0_huff0_encoder_rs_228_48_900b59::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
