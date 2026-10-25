# num-bigint/src/biguint/convert.rs:176:5

- crate: https://github.com/rust-num/num-bigint @ 41f0b035d12971c5c7f7501917e31f0f6439c26c
- kind: panic
- location: `num-bigint/src/biguint/convert.rs:176:5`
- last API call: `num_bigint::BigInt::from_radix_le`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
The radix must be within 2...256
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint = { git = "https://github.com/rust-num/num-bigint", rev = "41f0b035d12971c5c7f7501917e31f0f6439c26c" }
```

`src/main.rs`:

```rust
use num_bigint::BigInt;
use num_bigint::Sign;

fn main() {
    let _ = BigInt::from_radix_le(Sign::NoSign, &vec![], 0u32);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_biguint_convert_rs_176_5_5bd037`).

## Output

```
thread 'main' (580701) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-97d9861697b6c00e/41f0b03/src/biguint/convert.rs:176:5:
The radix must be within 2...256
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: num_bigint::biguint::convert::from_radix_le
             at /home/rafal/.cargo/git/checkouts/num-bigint-97d9861697b6c00e/41f0b03/src/biguint/convert.rs:176:5
   3: <num_bigint::biguint::BigUint>::from_radix_le
             at /home/rafal/.cargo/git/checkouts/num-bigint-97d9861697b6c00e/41f0b03/src/biguint.rs:538:9
   4: <num_bigint::bigint::BigInt>::from_radix_le
             at /home/rafal/.cargo/git/checkouts/num-bigint-97d9861697b6c00e/41f0b03/src/bigint.rs:751:17
   5: repro_num_bigint_src_biguint_convert_rs_176_5_5bd037::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
