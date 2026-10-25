# timeout in zlib_rs::crc32::crc32_combine_op

- crate: https://github.com/trifectatechfoundation/zlib-rs @ 7909c0fc48f5d29f6610770e31d6f0c924f9ec3b
- kind: timeout
- last API call: `zlib_rs::crc32::crc32_combine_op`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling zlib_rs::crc32::crc32_combine_op
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zlib-rs = { git = "https://github.com/trifectatechfoundation/zlib-rs", rev = "7909c0fc48f5d29f6610770e31d6f0c924f9ec3b" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = zlib_rs::crc32::crc32_combine_op(0u32, 0u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_zlib_rs_crc32_crc32_combine_op_82506d`).

