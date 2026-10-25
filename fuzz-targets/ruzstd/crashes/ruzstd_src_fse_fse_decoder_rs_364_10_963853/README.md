# ruzstd/src/fse/fse_decoder.rs:364:10

- crate: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774
- kind: panic
- location: `ruzstd/src/fse/fse_decoder.rs:364:10`
- last API call: `ruzstd::fse::FSETable::build_from_probabilities`
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
ruzstd = { git = "https://github.com/KillingSpark/zstd-rs", rev = "91a3a6eb4cb8a0252ef73f5a561e30c790635774", features = ["dict_builder", "fuzz_exports", "hash", "rustc-dep-of-std", "std"] }
```

`src/main.rs`:

```rust
use ruzstd::fse::FSETable;

fn main() {
    let mut f_s_e_table = FSETable::new(0u8);
    let _ = f_s_e_table.build_from_probabilities(21u8, &[1i32]);
}
```

Run with `cargo run` in this directory (package `repro_ruzstd_src_fse_fse_decoder_rs_364_10_963853`).

## Output

```
thread 'main' (414809) panicked at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/fse/fse_decoder.rs:364:10:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: ruzstd::fse::fse_decoder::calc_baseline_and_numbits
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/fse/fse_decoder.rs:364:10
   4: <ruzstd::fse::fse_decoder::FSETable>::build_decoding_table
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/fse/fse_decoder.rs:209:28
   5: <ruzstd::fse::fse_decoder::FSETable>::build_from_probabilities
             at /home/rafal/.cargo/git/checkouts/zstd-rs-0eb1b7c086324bbe/91a3a6e/ruzstd/src/fse/fse_decoder.rs:136:14
   6: repro_ruzstd_src_fse_fse_decoder_rs_364_10_963853::main
             at ./src/main.rs:5:25
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
