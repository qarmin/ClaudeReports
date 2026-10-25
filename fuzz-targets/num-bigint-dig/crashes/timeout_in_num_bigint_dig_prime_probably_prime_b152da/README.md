# timeout in num_bigint_dig::prime::probably_prime

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: timeout
- last API call: `num_bigint_dig::prime::probably_prime`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling num_bigint_dig::prime::probably_prime
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2", features = ["arbitrary", "fuzz", "i128", "nightly", "prime", "rand", "serde", "std", "u64_digit", "zeroize"] }
```

`src/main.rs`:

```rust
use num_bigint_dig::BigInt;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime(&<num_bigint_dig::BigInt as num_bigint_dig::IntoBigUint>::into_biguint(<num_bigint_dig::BigInt as ::core::ops::Rem<isize>>::rem(BigInt::from(0i8), 0isize)).unwrap(), 0usize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_num_bigint_dig_prime_probably_prime_b152da`).

## Output

```
thread 'main' (346621) has overflowed its stack
fatal runtime error: stack overflow, aborting
```
