use num_bigint_dig::BigUint;

fn main() {
    let _ = num_bigint_dig::prime::probably_prime_miller_rabin(&<num_bigint_dig::BigUint as ::core::ops::Add<u8>>::add(BigUint::from_bytes_le(&vec![]), 7u8), 1000000000usize, false);
}
