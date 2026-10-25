# timeout in num_bigint_dig::BigInt::rem<isize>

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: timeout
- last API call: `num_bigint_dig::BigInt::rem<isize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling num_bigint_dig::BigInt::rem<isize>
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2" }
```

`src/main.rs`:

```rust
use num_bigint_dig::BigInt;

fn main() {
    let _ = <num_bigint_dig::BigInt as ::core::ops::Rem<isize>>::rem(BigInt::from_signed_bytes_le(&vec![]), 0isize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_num_bigint_dig_bigint_rem_isize_500b6f`).

## Output

```
thread 'main' (346209) has overflowed its stack
fatal runtime error: stack overflow, aborting
```
