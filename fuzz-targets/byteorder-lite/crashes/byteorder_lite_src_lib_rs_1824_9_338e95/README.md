# byteorder-lite/src/lib.rs:1824:9

- crate: https://github.com/image-rs/byteorder-lite @ 7d44da06a48fe10d7e23efd9a932d18a7a2def63
- kind: panic
- location: `byteorder-lite/src/lib.rs:1824:9`
- last API call: `byteorder_lite::BigEndian::read_uint128`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: 1 <= nbytes && nbytes <= out.len() && nbytes <= buf.len()
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
    let _ = <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_uint128(&vec![], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_byteorder_lite_src_lib_rs_1824_9_338e95`).

## Output

```
thread 'main' (328938) panicked at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:1824:9:
assertion failed: 1 <= nbytes && nbytes <= out.len() && nbytes <= buf.len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_uint128
             at /home/rafal/.cargo/git/checkouts/byteorder-lite-644a59c08c978d9c/7d44da0/src/lib.rs:1824:9
   4: repro_byteorder_lite_src_lib_rs_1824_9_338e95::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
