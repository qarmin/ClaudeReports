# blake2/src/lib.rs:40:1

- crate: https://github.com/RustCrypto/hashes @ ff2b46e7d972c7f8f59d0650e83db6d80a7783f0
- kind: panic
- location: `blake2/src/lib.rs:40:1`
- last API call: `blake2::Blake2bVarCore::new_with_params`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: persona.len() <= length
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
blake2 = { git = "https://github.com/RustCrypto/hashes", rev = "ff2b46e7d972c7f8f59d0650e83db6d80a7783f0" }
```

`src/main.rs`:

```rust
use blake2::Blake2bVarCore;

fn main() {
    let _ = Blake2bVarCore::new_with_params(&vec![], &vec![73u8, 73u8, 42u8, 0u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], 0usize, 0usize);
}
```

Run with `cargo run` in this directory (package `repro_blake2_src_lib_rs_40_1_cf342d`).

## Output

```
thread 'main' (562703) panicked at /home/rafal/.cargo/git/checkouts/hashes-1d556dee1f65bd53/ff2b46e/blake2/src/lib.rs:40:1:
assertion failed: persona.len() <= length
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <blake2::Blake2bVarCore>::new_with_params
             at /home/rafal/.cargo/git/checkouts/hashes-1d556dee1f65bd53/ff2b46e/blake2/src/macros.rs:39:17
   4: repro_blake2_src_lib_rs_40_1_cf342d::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
