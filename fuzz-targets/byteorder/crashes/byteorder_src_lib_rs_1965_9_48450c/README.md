# byteorder/src/lib.rs:1965:9

- crate: https://github.com/BurntSushi/byteorder @ 5a82625fae462e8ba64cec8146b24a372b4d75c6
- kind: panic
- location: `byteorder/src/lib.rs:1965:9`
- last API call: `byteorder::BigEndian::write_uint128`
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
byteorder = { git = "https://github.com/BurntSushi/byteorder", rev = "5a82625fae462e8ba64cec8146b24a372b4d75c6" }
```

`src/main.rs`:

```rust
fn main() {
    let u128 = <byteorder::BigEndian as byteorder::ByteOrder>::read_u128(&vec![73u8, 73u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    <byteorder::BigEndian as byteorder::ByteOrder>::write_uint128(&mut vec![], u128, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_byteorder_src_lib_rs_1965_9_48450c`).

## Output

```
thread 'main' (1256811) panicked at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:1965:9:
assertion failed: pack_size128(n) <= nbytes && nbytes <= 16
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <byteorder::BigEndian as byteorder::ByteOrder>::write_uint128
             at /home/rafal/.cargo/git/checkouts/byteorder-b6cec9872925adfc/5a82625/src/lib.rs:1965:9
   4: repro_byteorder_src_lib_rs_1965_9_48450c::main
             at ./src/main.rs:3:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
