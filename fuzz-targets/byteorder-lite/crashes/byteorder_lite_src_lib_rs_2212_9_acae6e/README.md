# byteorder-lite/src/lib.rs:2212:9

- crate: https://github.com/image-rs/byteorder-lite @ 7d44da06a48fe10d7e23efd9a932d18a7a2def63
- kind: panic
- location: `byteorder-lite/src/lib.rs:2212:9`
- last API call: `byteorder_lite::LittleEndian::write_i64_into`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion `left == right` failed
  left: 8
 right: 0
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
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_i64_into(&[0i64], &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_byteorder_lite_src_lib_rs_2212_9_acae6e`).

## Output

```
thread 'main' (330773) panicked at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:2212:9:
assertion `left == right` failed
  left: 8
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_i64_into
             at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:1784:9
   5: repro_byteorder_lite_src_lib_rs_2212_9_acae6e::main
             at ./src/main.rs:2:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
