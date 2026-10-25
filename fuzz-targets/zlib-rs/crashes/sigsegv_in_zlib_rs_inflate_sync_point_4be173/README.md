# SIGSEGV in zlib_rs::inflate::sync_point

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: SIGSEGV
- last API call: `zlib_rs::inflate::sync_point`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (SIGSEGV) while calling zlib_rs::inflate::sync_point
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b", features = ["ZLIB_DEBUG", "__internal-api", "__internal-fuzz", "__internal-fuzz-disable-checksum", "__internal-test", "arbitrary", "avx512", "c-allocator", "lsx", "quickcheck", "rust-allocator", "std", "vpclmulqdq"] }
```

`src/main.rs`:

```rust
use zlib_rs::InflateConfig;
use zlib_rs::inflate::InflateStream;

fn main() {
    let mut inflate_stream = InflateStream::new(InflateConfig::default());
    let _ = zlib_rs::inflate::end(&mut inflate_stream);
    let _ = zlib_rs::inflate::sync_point(&mut inflate_stream);
}
```

Run with `cargo run` in this directory (package `repro_sigsegv_in_zlib_rs_inflate_sync_point_4be173`).

