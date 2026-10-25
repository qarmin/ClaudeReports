# SIGSEGV in zlib_rs::deflate::tune

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: SIGSEGV
- last API call: `zlib_rs::deflate::tune`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (SIGSEGV) while calling zlib_rs::deflate::tune
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b", features = ["ZLIB_DEBUG", "__internal-api", "__internal-fuzz", "__internal-fuzz-disable-checksum", "__internal-test", "arbitrary", "avx512", "c-allocator", "lsx", "quickcheck", "rust-allocator", "std", "vpclmulqdq"] }
```

`src/main.rs`:

```rust
use zlib_rs::DeflateConfig;
use zlib_rs::deflate::DeflateStream;

fn main() {
    let mut deflate_stream = DeflateStream::new(DeflateConfig::best_compression());
    let _ = zlib_rs::deflate::end(&mut deflate_stream);
    let _ = zlib_rs::deflate::tune(&mut deflate_stream, 0usize, 0usize, 0usize, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_sigsegv_in_zlib_rs_deflate_tune_9ccc65`).

