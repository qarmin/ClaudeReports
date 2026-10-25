# timeout in num_bigint_dig::prime::probably_prime_miller_rabin

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: timeout
- last API call: `num_bigint_dig::prime::probably_prime_miller_rabin`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling num_bigint_dig::prime::probably_prime_miller_rabin
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2", features = ["arbitrary", "fuzz", "i128", "nightly", "prime", "rand", "serde", "std", "u64_digit", "zeroize"] }
```

`src/main.rs`:

```rust
use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&<num_bigint_dig::BigUint as ::core::ops::Add<u8>>::add(BigUint::from_bytes_le(&vec![]), 7u8), 1000000000usize, false);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_num_bigint_dig_prime_probably_prime_miller_rabin_206241`).

