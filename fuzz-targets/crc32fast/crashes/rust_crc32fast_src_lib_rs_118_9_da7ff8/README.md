# rust-crc32fast/src/lib.rs:118:9

- crate: https://github.com/srijs/rust-crc32fast @ ad14266cf782ba6936d1af37fc9fd6382ef73b7c
- kind: panic
- location: `rust-crc32fast/src/lib.rs:118:9`
- last API call: `crc32fast::Hasher::update`
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
crc32fast = { git = "https://github.com/srijs/rust-crc32fast", rev = "ad14266cf782ba6936d1af37fc9fd6382ef73b7c" }
```

`src/main.rs`:

```rust
use crc32fast::Hasher;

fn main() {
    let u32_v = crc32fast::hash(&vec![]);
    let mut hasher = Hasher::new_with_initial_len(u32_v, u64::MAX);
    hasher.update(&vec![34u8]);
}
```

Run with `cargo run` in this directory (package `repro_rust_crc32fast_src_lib_rs_118_9_da7ff8`).

## Output

```
thread 'main' (1446222) panicked at /home/rafal/.cargo/git/checkouts/rust-crc32fast-b88769cd0919912e/ad14266/src/lib.rs:118:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <crc32fast::Hasher>::update
             at /home/rafal/.cargo/git/checkouts/rust-crc32fast-b88769cd0919912e/ad14266/src/lib.rs:118:9
   4: repro_rust_crc32fast_src_lib_rs_118_9_da7ff8::main
             at ./src/main.rs:6:12
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
