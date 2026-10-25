# SIGSEGV in zlib_rs::deflate::set_dictionary

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: SIGSEGV
- last API call: `zlib_rs::deflate::set_dictionary`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (SIGSEGV) while calling zlib_rs::deflate::set_dictionary
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
    let mut deflate_stream = DeflateStream::new(DeflateConfig::default());
    let _ = zlib_rs::deflate::end(&mut deflate_stream);
    let _ = zlib_rs::deflate::set_dictionary(&mut deflate_stream, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_sigsegv_in_zlib_rs_deflate_set_dictionary_946fb5`).

