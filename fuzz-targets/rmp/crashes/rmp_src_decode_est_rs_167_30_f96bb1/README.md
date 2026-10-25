# rmp/src/decode/est.rs:167:30

- crate: https://github.com/3Hren/msgpack-rust @ cf880019f72e35a884659a9b29d9eb89b862670b
- kind: panic
- location: `rmp/src/decode/est.rs:167:30`
- last API call: `rmp::decode::MessageLen::len_of`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not yet implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rmp = { git = "https://github.com/3Hren/msgpack-rust", rev = "cf880019f72e35a884659a9b29d9eb89b862670b" }
```

`src/main.rs`:

```rust
use rmp::decode::MessageLen;

fn main() {
    let _ = MessageLen::len_of(&vec![201u8]);
}
```

Run with `cargo run` in this directory (package `repro_rmp_src_decode_est_rs_167_30_f96bb1`).

## Output

```
thread 'main' (446846) panicked at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/decode/est.rs:167:30:
not yet implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <rmp::decode::est::MessageLen>::read_one_item
             at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/decode/est.rs:167:30
   4: <rmp::decode::est::MessageLen>::incremental_len
             at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/decode/est.rs:117:37
   5: <rmp::decode::est::MessageLen>::len_of
             at /home/rafal/.cargo/git/checkouts/msgpack-rust-ae36e625e5c730e6/cf88001/rmp/src/decode/est.rs:89:42
   6: repro_rmp_src_decode_est_rs_167_30_f96bb1::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
