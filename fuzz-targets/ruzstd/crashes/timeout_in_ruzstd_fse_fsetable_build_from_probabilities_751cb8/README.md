# timeout in ruzstd::fse::FSETable::build_from_probabilities

- crate: https://github.com/KillingSpark/zstd-rs @ 91a3a6eb4cb8a0252ef73f5a561e30c790635774
- kind: timeout
- last API call: `ruzstd::fse::FSETable::build_from_probabilities`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling ruzstd::fse::FSETable::build_from_probabilities
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ruzstd = { git = "https://github.com/KillingSpark/zstd-rs", rev = "91a3a6eb4cb8a0252ef73f5a561e30c790635774", features = ["dict_builder", "fuzz_exports", "hash", "rustc-dep-of-std", "std"] }
```

`src/main.rs`:

```rust
use ruzstd::decoding::DEFAULT_MAX_WINDOW_SIZE;
use ruzstd::decoding::errors::BlockSizeError;
use ruzstd::decoding::errors::FSEDecoderError;
use ruzstd::encoding::CompressionLevel;
use ruzstd::fse::FSETable;
use ruzstd::huff0::huff0_encoder::HuffmanTable;

fn main() {
    let mut f_s_e_table = FSETable::new(64u8);
    let _ = f_s_e_table.build_from_probabilities(26u8, &[7i32, 2041339626i32, 45i32, i32::MAX, 1i32, 1i32]);
    let _ = HuffmanTable::build_from_weights(&[3usize, 6180167714942147986usize, 2usize]);
    let _ = BlockSizeError::BlockSizeTooLarge { size: 0u32 };
    let _ = DEFAULT_MAX_WINDOW_SIZE;
    let _ = CompressionLevel::Default;
    let _ = FSEDecoderError::TableIsUninitialized;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_ruzstd_fse_fsetable_build_from_probabilities_751cb8`).

