# byteorder-lite/src/lib.rs:2124:9

- crate: https://github.com/image-rs/byteorder-lite @ 7d44da06a48fe10d7e23efd9a932d18a7a2def63
- kind: panic
- location: `byteorder-lite/src/lib.rs:2124:9`
- last API call: `byteorder_lite::LittleEndian::write_uint128`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: pack_size128(n) <= nbytes && nbytes <= 16
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
byteorder-lite = { git = "https://github.com/image-rs/byteorder-lite", rev = "7d44da06a48fe10d7e23efd9a932d18a7a2def63" }
```

`src/main.rs`:

```rust
fn main() {
    let u128 = <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_u128(&vec![0u8, 0u8, 0u8, 0u8, 79u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 0u8, 0u8, 69u8, 101u8]);
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_uint128(&mut vec![], u128, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_byteorder_lite_src_lib_rs_2124_9_8c5139`).

## Output

```
thread 'main' (330986) panicked at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:2124:9:
assertion failed: pack_size128(n) <= nbytes && nbytes <= 16
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_uint128
             at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:2124:9
   4: repro_byteorder_lite_src_lib_rs_2124_9_8c5139::main
             at ./src/main.rs:3:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
