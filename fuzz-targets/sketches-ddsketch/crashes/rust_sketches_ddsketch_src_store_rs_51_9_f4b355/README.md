# rust-sketches-ddsketch/src/store.rs:51:9

- crate: https://github.com/mheffner/rust-sketches-ddsketch @ 9c517a0996152f9f12a609d7b720d7cead01fede
- kind: panic
- location: `rust-sketches-ddsketch/src/store.rs:51:9`
- last API call: `sketches_ddsketch::encoding::decode_from_java_bytes`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
sketches-ddsketch = { git = "https://github.com/mheffner/rust-sketches-ddsketch", rev = "9c517a0996152f9f12a609d7b720d7cead01fede" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&vec![15u8, 56u8, 215u8, 0u8, 137u8, 116u8, 194u8, 60u8, 65u8, 126u8, 83u8, 41u8, 233u8, 100u8, 13u8, 150u8, 174u8, 61u8, 158u8, 109u8, 22u8, 152u8, 242u8, 6u8, 193u8, 140u8, 201u8, 61u8, 117u8, 46u8, 126u8]);
}
```

Run with `cargo run` in this directory (package `repro_rust_sketches_ddsketch_src_store_rs_51_9_f4b355`).

## Output

```
thread 'main' (1278770) panicked at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/store.rs:51:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <sketches_ddsketch::store::Store>::add_count
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/store.rs:51:9
   4: sketches_ddsketch::encoding::decode_store
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/encoding.rs:290:23
   5: sketches_ddsketch::encoding::decode_from_java_bytes
             at /home/rafal/.cargo/git/checkouts/rust-sketches-ddsketch-33e4efe8e394be6c/9c517a0/src/encoding.rs:400:39
   6: repro_rust_sketches_ddsketch_src_store_rs_51_9_f4b355::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
